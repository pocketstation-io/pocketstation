// CoreAudio process tap implementation — macOS 14.2+.
// Primary capture path for PocketStation on Sonoma and later.
// All tap functions are guarded by @available(macOS 14.2, *).
// Callers check pks_process_tap_available() before using any tap API.

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunguarded-availability"

#import <Foundation/Foundation.h>
#import <math.h>
#import <CoreAudio/CoreAudio.h>
#import <CoreAudio/CATapDescription.h>
#import <CoreAudio/AudioHardwareTapping.h>
#import <AppKit/NSRunningApplication.h>
#import <AVFoundation/AVFoundation.h>
#import <CoreMedia/CoreMedia.h>
#import <libproc.h>
#import <mach/mach_time.h>
#import <stdatomic.h>
#import <string.h>
#import <stdlib.h>

#include "source_discovery.h"

// Optional observation is scoped to synchronous control calls only.
static uint64_t pks_observe_begin(const PksNativeCallObserver *observer, uint32_t operation) {
    return observer ? observer->begin(observer->context, operation) : 0;
}
static void pks_observe_end(const PksNativeCallObserver *observer, uint64_t call,
                            int32_t status, uint8_t has_status) {
    if (observer) observer->end(observer->context, call, status, has_status);
}
static OSStatus pks_property_size(const PksNativeCallObserver *observer, uint32_t operation,
    AudioObjectID object, const AudioObjectPropertyAddress *address,
    UInt32 qualifier_size, const void *qualifier, UInt32 *size) {
    uint64_t call = pks_observe_begin(observer, operation);
    OSStatus status = AudioObjectGetPropertyDataSize(object, address, qualifier_size, qualifier, size);
    pks_observe_end(observer, call, status, 1);
    return status;
}
static OSStatus pks_property_data(const PksNativeCallObserver *observer, uint32_t operation,
    AudioObjectID object, const AudioObjectPropertyAddress *address,
    UInt32 qualifier_size, const void *qualifier, UInt32 *size, void *data) {
    uint64_t call = pks_observe_begin(observer, operation);
    OSStatus status = AudioObjectGetPropertyData(object, address, qualifier_size, qualifier, size, data);
    pks_observe_end(observer, call, status, 1);
    return status;
}
static OSStatus pks_property_settable(const PksNativeCallObserver *observer, uint32_t operation,
    AudioObjectID object, const AudioObjectPropertyAddress *address, Boolean *settable) {
    uint64_t call = pks_observe_begin(observer, operation);
    OSStatus status = AudioObjectIsPropertySettable(object, address, settable);
    pks_observe_end(observer, call, status, 1);
    return status;
}
static OSStatus pks_property_set(const PksNativeCallObserver *observer, uint32_t operation,
    AudioObjectID object, const AudioObjectPropertyAddress *address,
    UInt32 qualifier_size, const void *qualifier, UInt32 size, const void *data) {
    uint64_t call = pks_observe_begin(observer, operation);
    OSStatus status = AudioObjectSetPropertyData(object, address, qualifier_size, qualifier, size, data);
    pks_observe_end(observer, call, status, 1);
    return status;
}

uint64_t pks_process_start_time_ns(int32_t process_id) {
    return pks_process_start_time_ns_observed(process_id, NULL);
}
uint64_t pks_process_start_time_ns_observed(int32_t process_id, const PksNativeCallObserver *observer) {
    pid_t pid = (pid_t)process_id;
    struct proc_bsdinfo info;
    uint64_t call = pks_observe_begin(observer, PKS_NATIVE_CALL_PROCESS_START_TIME);
    int bytes = proc_pidinfo(
        pid,
        PROC_PIDTBSDINFO,
        0,
        &info,
        (int)sizeof(info));
    pks_observe_end(observer, call, 0, 0);
    if (bytes != (int)sizeof(info)
            || info.pbi_start_tvusec >= 1000000u
            || info.pbi_start_tvsec > UINT64_MAX / 1000000000u) {
        return 0u;
    }
    uint64_t seconds_ns = info.pbi_start_tvsec * 1000000000u;
    uint64_t microseconds_ns = info.pbi_start_tvusec * 1000u;
    if (seconds_ns > UINT64_MAX - microseconds_ns) {
        return 0u;
    }
    return seconds_ns + microseconds_ns;
}

// ─── Availability ───────────────────────────────────────────────────────────

int pks_process_tap_available(void) {
    if (@available(macOS 14.2, *)) { return 1; }
    return 0;
}

// ─── SPSC ring buffer ───────────────────────────────────────────────────────
// Writer: CoreAudio IO callback thread.
// Reader: Rust reader thread (sole consumer).

#define TAP_RING_FRAMES   65536u
#define TAP_RING_MASK     (TAP_RING_FRAMES - 1u)
#define TAP_RING_CHANNELS 2u

typedef struct {
    _Atomic uint64_t write_head;
    uint64_t         read_head;   // single-consumer, non-atomic
    _Atomic uint64_t drop_count;  // reader-observed overwritten sample frames
    _Atomic uint32_t sample_rate;
    _Atomic uint32_t level_bits;  // float RMS stored as uint32_t for atomic access
    _Atomic uint64_t timeline_sequence;
    _Atomic uint64_t anchor_frame_position;
    _Atomic uint64_t anchor_host_time;
    float data[TAP_RING_FRAMES * TAP_RING_CHANNELS];
} PksTapRing;

static uint64_t pks_host_time_to_ns(uint64_t host_time) {
    mach_timebase_info_data_t timebase;
    if (host_time == 0 || mach_timebase_info(&timebase) != KERN_SUCCESS
            || timebase.denom == 0) {
        return 0;
    }
    __uint128_t scaled = (__uint128_t)host_time * (__uint128_t)timebase.numer;
    scaled /= (__uint128_t)timebase.denom;
    return scaled > UINT64_MAX ? UINT64_MAX : (uint64_t)scaled;
}

uint64_t pks_tap_current_host_time_ns(void) {
    return pks_host_time_to_ns(mach_absolute_time());
}

static inline float ring_load_level(const PksTapRing *r) {
    uint32_t bits = atomic_load_explicit(&r->level_bits, memory_order_relaxed);
    float f; memcpy(&f, &bits, sizeof(f)); return f;
}

static inline void ring_store_level(PksTapRing *r, float v) {
    uint32_t bits; memcpy(&bits, &v, sizeof(bits));
    atomic_store_explicit(&r->level_bits, bits, memory_order_relaxed);
}

// ─── Tap handle ─────────────────────────────────────────────────────────────

struct PksProcessTapHandle {
    AudioObjectID       tap_id;
    AudioObjectID       agg_device_id;
    AudioDeviceIOProcID io_proc_id;
    PksTapControl control;
    uint32_t            io_buffer_before_frames;
    uint32_t            io_buffer_requested_frames;
    uint32_t            io_buffer_applied_frames;
    uint32_t            io_buffer_min_frames;
    uint32_t            io_buffer_max_frames;
    PksTapRing          ring;
};

// These synchronous operations run only on the owning control thread.
int pks_tap_control_start_observed(PksTapControl *state,
    const PksTapControlOperations *ops, void *context,
    PksTapCancellationCheck cancelled, void *cancel_context,
    int32_t *out_status, uint8_t *out_stage, const PksNativeCallObserver *observer) {
    if (out_status) *out_status = 0;
    if (out_stage) *out_stage = 0;
    if (state->registered || state->registration_uncertain
            || state->cleanup_attempted || state->retained) {
        if (out_status) *out_status = kAudioHardwareIllegalOperationError;
        if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_IO_PROC;
        return -1;
    }
    if (cancelled && cancelled(cancel_context)) return -2;
    // An RPC error need not establish that remote registration was rejected.
    state->registration_uncertain = 1;
    uint64_t call = pks_observe_begin(observer, PKS_NATIVE_CALL_REGISTER_IO_PROC);
    int32_t status = ops->register_io(context);
    pks_observe_end(observer, call, status, 1);
    if (status != 0) {
        if (state->registration_uncertain) state->cleanup_status = status;
        if (out_status) *out_status = status;
        if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_IO_PROC;
        return -1;
    }
    state->registration_uncertain = 0;
    state->registered = 1;
    if (cancelled && cancelled(cancel_context)) return -2;
    state->start_attempted = 1;
    call = pks_observe_begin(observer, PKS_NATIVE_CALL_START_DEVICE);
    status = ops->start(context);
    pks_observe_end(observer, call, status, 1);
    if (status != 0) {
        if (out_status) *out_status = status;
        if (out_stage) *out_stage = PKS_TAP_STAGE_START_DEVICE;
        return -1;
    }
    state->started = 1;
    return cancelled && cancelled(cancel_context) ? -2 : 0;
}

static int pks_tap_retain(PksTapControl *state, int32_t status, uint8_t stage,
                         int32_t *out_status, uint8_t *out_stage) {
    state->retained = 1;
    state->cleanup_status = status;
    state->cleanup_stage = stage;
    if (out_status) *out_status = status;
    if (out_stage) *out_stage = stage;
    return -1;
}

int pks_tap_control_cleanup_observed(PksTapControl *state,
    const PksTapControlOperations *ops, void *context,
    int32_t *out_status, uint8_t *out_stage, const PksNativeCallObserver *observer) {
    if (out_status) *out_status = state->cleanup_status;
    if (out_stage) *out_stage = state->cleanup_stage;
    if (state->cleanup_attempted) return state->retained ? -1 : 0;
    state->cleanup_attempted = 1;
    if (state->registration_uncertain) {
        return pks_tap_retain(state, state->cleanup_status,
            PKS_TAP_STAGE_REGISTRATION_UNCERTAIN, out_status, out_stage);
    }
    int32_t status;
    if (state->start_attempted) {
        uint64_t call = pks_observe_begin(observer, PKS_NATIVE_CALL_STOP_DEVICE);
        status = ops->stop(context);
        pks_observe_end(observer, call, status, 1);
        if (status != 0) return pks_tap_retain(state, status,
            PKS_TAP_STAGE_STOP_DEVICE, out_status, out_stage);
        state->started = 0;
        state->start_attempted = 0;
    }
    if (state->registered) {
        uint64_t call = pks_observe_begin(observer, PKS_NATIVE_CALL_UNREGISTER_IO_PROC);
        status = ops->unregister_io(context);
        pks_observe_end(observer, call, status, 1);
        if (status != 0) return pks_tap_retain(state, status,
            PKS_TAP_STAGE_DESTROY_IO_PROC, out_status, out_stage);
        state->registered = 0;
    }
    if (state->aggregate_owned) {
        uint64_t call = pks_observe_begin(observer, PKS_NATIVE_CALL_DESTROY_AGGREGATE_DEVICE);
        status = ops->destroy_aggregate(context);
        pks_observe_end(observer, call, status, 1);
        if (status != 0) return pks_tap_retain(state, status,
            PKS_TAP_STAGE_DESTROY_AGGREGATE, out_status, out_stage);
        state->aggregate_owned = 0;
    }
    if (state->tap_owned) {
        uint64_t call = pks_observe_begin(observer, PKS_NATIVE_CALL_DESTROY_PROCESS_TAP);
        status = ops->destroy_tap(context);
        pks_observe_end(observer, call, status, 1);
        if (status != 0) return pks_tap_retain(state, status,
            PKS_TAP_STAGE_DESTROY_TAP, out_status, out_stage);
        state->tap_owned = 0;
    }
    // The caller frees only after this return; state may be embedded in context.
    return 0;
}

int pks_tap_control_start(PksTapControl *state, const PksTapControlOperations *ops,
    void *context, PksTapCancellationCheck cancelled, void *cancel_context,
    int32_t *status, uint8_t *stage) {
    return pks_tap_control_start_observed(state, ops, context, cancelled, cancel_context,
                                        status, stage, NULL);
}
int pks_tap_control_cleanup(PksTapControl *state, const PksTapControlOperations *ops,
    void *context, int32_t *status, uint8_t *stage) {
    return pks_tap_control_cleanup_observed(state, ops, context, status, stage, NULL);
}

// ─── IO callback — real-time thread, no ObjC/alloc/lock/log ────────────────

static OSStatus tap_io_proc(
    AudioDeviceID           dev,
    const AudioTimeStamp   *now,
    const AudioBufferList  *input,
    const AudioTimeStamp   *input_time,
    AudioBufferList        *output,
    const AudioTimeStamp   *output_time,
    void                   *user_data)
{
    (void)dev; (void)output; (void)output_time;

    PksTapRing *ring = (PksTapRing *)user_data;
    if (!ring || !input || input->mNumberBuffers == 0) return noErr;

    const AudioBuffer *buf0 = &input->mBuffers[0];
    uint32_t frames;
    uint64_t head = atomic_load_explicit(&ring->write_head, memory_order_relaxed);

    bool interleaved = (input->mNumberBuffers == 1 && buf0->mNumberChannels > 1);

    if (interleaved) {
        uint32_t srcCh = buf0->mNumberChannels;
        frames = buf0->mDataByteSize / (srcCh * sizeof(float));
        if (frames == 0) return noErr;
        if (frames > TAP_RING_FRAMES) {
            atomic_fetch_add_explicit(
                &ring->drop_count, frames, memory_order_relaxed);
            return noErr;
        }
        const float *src = (const float *)buf0->mData;
        for (uint32_t i = 0; i < frames; i++) {
            uint32_t slot = (uint32_t)((head + i) & TAP_RING_MASK);
            for (uint32_t c = 0; c < TAP_RING_CHANNELS; c++) {
                float s = (c < srcCh) ? src[i * srcCh + c] : 0.0f;
                ring->data[slot * TAP_RING_CHANNELS + c] = s;
            }
        }
    } else {
        // Non-interleaved: one buffer per channel.
        uint32_t nbufs = input->mNumberBuffers;
        frames = buf0->mDataByteSize / sizeof(float);
        if (frames == 0) return noErr;
        if (frames > TAP_RING_FRAMES) {
            atomic_fetch_add_explicit(
                &ring->drop_count, frames, memory_order_relaxed);
            return noErr;
        }
        for (uint32_t i = 0; i < frames; i++) {
            uint32_t slot = (uint32_t)((head + i) & TAP_RING_MASK);
            for (uint32_t c = 0; c < TAP_RING_CHANNELS; c++) {
                float s = 0.0f;
                if (c < nbufs) {
                    const float *chBuf = (const float *)input->mBuffers[c].mData;
                    s = chBuf[i];
                }
                ring->data[slot * TAP_RING_CHANNELS + c] = s;
            }
        }
    }

    uint64_t host_time = 0;
    if (input_time && (input_time->mFlags & kAudioTimeStampHostTimeValid) != 0) {
        host_time = input_time->mHostTime;
    } else if (now && (now->mFlags & kAudioTimeStampHostTimeValid) != 0) {
        host_time = now->mHostTime;
    }
    atomic_fetch_add_explicit(&ring->timeline_sequence, 1, memory_order_relaxed);
    atomic_store_explicit(&ring->anchor_frame_position, head, memory_order_relaxed);
    atomic_store_explicit(&ring->anchor_host_time, host_time, memory_order_relaxed);
    atomic_fetch_add_explicit(&ring->timeline_sequence, 1, memory_order_release);
    atomic_store_explicit(&ring->write_head, head + frames, memory_order_release);
    return noErr;
}

// ─── Helper: look up AudioObjectID for a given PID ──────────────────────────

// Returns kAudioObjectUnknown if no process object with that PID is found.
static AudioObjectID pks_audio_object_id_for_pid(pid_t target_pid, const PksNativeCallObserver *observer) {
    AudioObjectPropertyAddress addr = {
        kAudioHardwarePropertyProcessObjectList,
        kAudioObjectPropertyScopeGlobal,
        kAudioObjectPropertyElementMain
    };
    uint32_t dataSize = 0;
    if (pks_property_size(observer, PKS_NATIVE_CALL_RESOLVE_PROCESS_LIST_SIZE, kAudioObjectSystemObject, &addr, 0, NULL, &dataSize) != noErr)
        return kAudioObjectUnknown;
    if (dataSize == 0) return kAudioObjectUnknown;

    uint32_t count = dataSize / sizeof(AudioObjectID);
    AudioObjectID *objs = (AudioObjectID *)malloc(dataSize);
    if (!objs) return kAudioObjectUnknown;

    if (pks_property_data(observer, PKS_NATIVE_CALL_RESOLVE_PROCESS_LIST_DATA, kAudioObjectSystemObject, &addr, 0, NULL, &dataSize, objs) != noErr) {
        free(objs);
        return kAudioObjectUnknown;
    }

    AudioObjectPropertyAddress pidAddr = {
        kAudioProcessPropertyPID,
        kAudioObjectPropertyScopeGlobal,
        kAudioObjectPropertyElementMain
    };

    AudioObjectID result = kAudioObjectUnknown;
    for (uint32_t i = 0; i < count; i++) {
        pid_t pid = 0;
        uint32_t sz = sizeof(pid);
        if (pks_property_data(observer, PKS_NATIVE_CALL_RESOLVE_PROCESS_ID, objs[i], &pidAddr, 0, NULL, &sz, &pid) == noErr) {
            if (pid == target_pid) {
                result = objs[i];
                break;
            }
        }
    }
    free(objs);
    return result;
}

// ─── Source discovery ───────────────────────────────────────────────────────

static int pks_append_input_devices(PksCaptureSourceInfo *out, int max, int written, const PksNativeCallObserver *observer) {
    int initialWritten = written;
    if (@available(macOS 14.0, *)) {
        uint64_t call = pks_observe_begin(observer, PKS_NATIVE_CALL_SOURCE_DEVICE_DISCOVERY);
        AVCaptureDeviceDiscoverySession *session =
            [AVCaptureDeviceDiscoverySession
                discoverySessionWithDeviceTypes:@[AVCaptureDeviceTypeMicrophone]
                mediaType:AVMediaTypeAudio
                position:AVCaptureDevicePositionUnspecified];
        pks_observe_end(observer, call, 0, 0);
        for (AVCaptureDevice *device in session.devices) {
            if (written >= max) break;
            PksCaptureSourceInfo *info = &out[written];
            memset(info, 0, sizeof(*info));
            info->kind = PKS_SOURCE_KIND_INPUT_DEVICE;
            info->state = device.connected
                ? PKS_SOURCE_STATE_AVAILABLE
                : PKS_SOURCE_STATE_UNAVAILABLE;
            if (![device.uniqueID getCString:info->bundle_id
                                  maxLength:sizeof(info->bundle_id)
                                   encoding:NSUTF8StringEncoding]
                    || info->bundle_id[0] == '\0') {
                continue;
            }
            [device.localizedName getCString:info->name
                                   maxLength:sizeof(info->name)
                                    encoding:NSUTF8StringEncoding];
            if (info->name[0] == '\0') {
                strncpy(info->name, info->bundle_id, sizeof(info->name) - 1);
            }
            for (AVCaptureDeviceFormat *format in device.formats) {
                const AudioStreamBasicDescription *description =
                    CMAudioFormatDescriptionGetStreamBasicDescription(format.formatDescription);
                if (!description || description->mSampleRate <= 0.0
                        || description->mSampleRate > (Float64)UINT32_MAX
                        || description->mChannelsPerFrame == 0) {
                    continue;
                }
                info->sample_rate = (uint32_t)description->mSampleRate;
                info->channels = description->mChannelsPerFrame > UINT16_MAX
                    ? UINT16_MAX
                    : (uint16_t)description->mChannelsPerFrame;
                break;
            }
            if (info->sample_rate == 0 || info->channels == 0) {
                continue;
            }
            written++;
        }
    }
    if (written > initialWritten) return written;

    AudioObjectPropertyAddress devicesAddr = {
        kAudioHardwarePropertyDevices,
        kAudioObjectPropertyScopeGlobal,
        kAudioObjectPropertyElementMain
    };
    uint32_t devicesSize = 0;
    if (pks_property_size(observer, PKS_NATIVE_CALL_DEVICE_LIST_SIZE, kAudioObjectSystemObject, &devicesAddr, 0, NULL, &devicesSize) != noErr
            || devicesSize < sizeof(AudioObjectID)) {
        return written;
    }
    AudioObjectID *devices = (AudioObjectID *)malloc(devicesSize);
    if (!devices) return written;
    if (pks_property_data(observer, PKS_NATIVE_CALL_DEVICE_LIST_DATA, kAudioObjectSystemObject, &devicesAddr, 0, NULL,
            &devicesSize, devices) != noErr) {
        free(devices);
        return written;
    }

    uint32_t deviceCount = devicesSize / sizeof(AudioObjectID);
    for (uint32_t i = 0; i < deviceCount && written < max; i++) {
        AudioObjectID device = devices[i];
        AudioObjectPropertyAddress streamsAddr = {
            kAudioDevicePropertyStreamConfiguration,
            kAudioDevicePropertyScopeInput,
            kAudioObjectPropertyElementMain
        };
        uint32_t streamsSize = 0;
        if (pks_property_size(observer, PKS_NATIVE_CALL_DEVICE_STREAMS_SIZE, device, &streamsAddr, 0, NULL, &streamsSize) != noErr
                || streamsSize < sizeof(AudioBufferList)) {
            continue;
        }
        AudioBufferList *streams = (AudioBufferList *)malloc(streamsSize);
        if (!streams) continue;
        if (pks_property_data(observer, PKS_NATIVE_CALL_DEVICE_STREAMS_DATA, device, &streamsAddr, 0, NULL, &streamsSize, streams) != noErr) {
            free(streams);
            continue;
        }
        uint32_t channels = 0;
        for (uint32_t buffer = 0; buffer < streams->mNumberBuffers; buffer++) {
            channels += streams->mBuffers[buffer].mNumberChannels;
        }
        free(streams);
        if (channels == 0) continue;

        PksCaptureSourceInfo *info = &out[written];
        memset(info, 0, sizeof(*info));
        info->audio_object_id = device;
        info->kind = PKS_SOURCE_KIND_INPUT_DEVICE;
        info->state = PKS_SOURCE_STATE_AVAILABLE;
        info->channels = channels > UINT16_MAX ? UINT16_MAX : (uint16_t)channels;

        AudioObjectPropertyAddress uidAddr = {
            kAudioDevicePropertyDeviceUID,
            kAudioObjectPropertyScopeGlobal,
            kAudioObjectPropertyElementMain
        };
        CFStringRef uid = NULL;
        uint32_t valueSize = sizeof(uid);
        if (pks_property_data(observer, PKS_NATIVE_CALL_DEVICE_UID, device, &uidAddr, 0, NULL, &valueSize, &uid) != noErr || !uid) {
            continue;
        }
        bool copiedUID = CFStringGetCString(
            uid, info->bundle_id, sizeof(info->bundle_id), kCFStringEncodingUTF8);
        CFRelease(uid);
        if (!copiedUID || info->bundle_id[0] == '\0') continue;

        AudioObjectPropertyAddress nameAddr = {
            kAudioObjectPropertyName,
            kAudioObjectPropertyScopeGlobal,
            kAudioObjectPropertyElementMain
        };
        CFStringRef name = NULL;
        valueSize = sizeof(name);
        if (pks_property_data(observer, PKS_NATIVE_CALL_DEVICE_NAME, device, &nameAddr, 0, NULL, &valueSize, &name) == noErr && name) {
            CFStringGetCString(
                name, info->name, sizeof(info->name), kCFStringEncodingUTF8);
            CFRelease(name);
        }
        if (info->name[0] == '\0') {
            strncpy(info->name, info->bundle_id, sizeof(info->name) - 1);
        }

        AudioObjectPropertyAddress rateAddr = {
            kAudioDevicePropertyNominalSampleRate,
            kAudioObjectPropertyScopeGlobal,
            kAudioObjectPropertyElementMain
        };
        Float64 rate = 0.0;
        valueSize = sizeof(rate);
        if (pks_property_data(observer, PKS_NATIVE_CALL_DEVICE_SAMPLE_RATE, device, &rateAddr, 0, NULL, &valueSize, &rate) == noErr
                && rate > 0.0 && rate <= (Float64)UINT32_MAX) {
            info->sample_rate = (uint32_t)rate;
        } else {
            info->sample_rate = 48000;
        }
        written++;
    }
    free(devices);
    return written;
}

int pks_discover_sources(PksCaptureSourceInfo *out, int max) {
    return pks_discover_sources_observed(out, max, NULL);
}
int pks_discover_sources_observed(PksCaptureSourceInfo *out, int max, const PksNativeCallObserver *observer) {
    if (!out || max <= 0) return 0;

    @autoreleasepool {
        int written = pks_append_input_devices(out, max, 0, observer);
        AudioObjectPropertyAddress addr = {
            kAudioHardwarePropertyProcessObjectList,
            kAudioObjectPropertyScopeGlobal,
            kAudioObjectPropertyElementMain
        };
        uint32_t dataSize = 0;
        OSStatus err = pks_property_size(observer, PKS_NATIVE_CALL_PROCESS_LIST_SIZE, kAudioObjectSystemObject, &addr, 0, NULL, &dataSize);
        if (err != noErr || dataSize == 0) return written;

        uint32_t count = dataSize / sizeof(AudioObjectID);
        AudioObjectID *objs = (AudioObjectID *)malloc(dataSize);
        if (!objs) return written;

        err = pks_property_data(observer, PKS_NATIVE_CALL_PROCESS_LIST_DATA, kAudioObjectSystemObject, &addr, 0, NULL, &dataSize, objs);
        if (err != noErr) { free(objs); return written; }

        for (uint32_t i = 0; i < count && written < max; i++) {
            AudioObjectID obj = objs[i];
            PksCaptureSourceInfo *info = &out[written];
            memset(info, 0, sizeof(*info));
            info->audio_object_id = obj;
            info->kind   = PKS_SOURCE_KIND_APPLICATION;
            info->state  = PKS_SOURCE_STATE_AVAILABLE;
            info->sample_rate = 48000;
            info->channels    = 2;

            // PID
            AudioObjectPropertyAddress pidAddr = {
                kAudioProcessPropertyPID,
                kAudioObjectPropertyScopeGlobal,
                kAudioObjectPropertyElementMain
            };
            pid_t pid = 0;
            uint32_t sz = sizeof(pid);
            if (pks_property_data(observer, PKS_NATIVE_CALL_PROCESS_ID, obj, &pidAddr, 0, NULL, &sz, &pid) != noErr)
                continue;
            info->pid = (int32_t)pid;
            info->process_start_time_ns = pks_process_start_time_ns_observed(pid, observer);

            // Bundle ID
            AudioObjectPropertyAddress bidAddr = {
                kAudioProcessPropertyBundleID,
                kAudioObjectPropertyScopeGlobal,
                kAudioObjectPropertyElementMain
            };
            CFStringRef bidRef = NULL;
            sz = sizeof(bidRef);
            if (pks_property_data(observer, PKS_NATIVE_CALL_PROCESS_BUNDLE_ID, obj, &bidAddr, 0, NULL, &sz, &bidRef) == noErr && bidRef) {
                CFStringGetCString(bidRef, info->bundle_id, sizeof(info->bundle_id),
                                   kCFStringEncodingUTF8);
                CFRelease(bidRef);
            }

            // Is running output?
            AudioObjectPropertyAddress runAddr = {
                kAudioProcessPropertyIsRunningOutput,
                kAudioObjectPropertyScopeGlobal,
                kAudioObjectPropertyElementMain
            };
            UInt32 running = 0;
            sz = sizeof(running);
            pks_property_data(observer, PKS_NATIVE_CALL_PROCESS_RUNNING_OUTPUT, obj, &runAddr, 0, NULL, &sz, &running);
            info->state = running ? PKS_SOURCE_STATE_PLAYING : PKS_SOURCE_STATE_SILENT;

            // Friendly label only. Identity remains the bundle/PID fields.
            uint64_t label_call = pks_observe_begin(observer, PKS_NATIVE_CALL_APPLICATION_LABEL);
            NSRunningApplication *pidApp =
                [NSRunningApplication runningApplicationWithProcessIdentifier:pid];
            pks_observe_end(observer, label_call, 0, 0);
            if (pidApp && pidApp.localizedName) {
                [pidApp.localizedName getCString:info->name
                                       maxLength:sizeof(info->name)
                                        encoding:NSUTF8StringEncoding];
            }
            if (info->bundle_id[0] != '\0') {
                NSString *bid = [NSString stringWithUTF8String:info->bundle_id];
                label_call = pks_observe_begin(observer, PKS_NATIVE_CALL_APPLICATION_LABEL);
                NSArray<NSRunningApplication *> *apps =
                    [NSRunningApplication runningApplicationsWithBundleIdentifier:bid];
                pks_observe_end(observer, label_call, 0, 0);
                NSRunningApplication *app = apps.firstObject;
                if (info->name[0] == '\0' && app && app.localizedName) {
                    [app.localizedName getCString:info->name
                                       maxLength:sizeof(info->name)
                                        encoding:NSUTF8StringEncoding];
                }
            }
            if (info->name[0] == '\0') {
                uint64_t name_call = pks_observe_begin(observer, PKS_NATIVE_CALL_PROCESS_NAME);
                proc_name(pid, info->name, (uint32_t)sizeof(info->name));
                pks_observe_end(observer, name_call, 0, 0);
            }
            if (info->name[0] == '\0')
                strncpy(info->name, info->bundle_id[0] ? info->bundle_id : "unknown",
                        sizeof(info->name) - 1);

            written++;
        }
        free(objs);

        return written;
    }
}

// ─── Process tap creation ────────────────────────────────────────────────────

static OSStatus pks_failure_status(OSStatus status) {
    return status == noErr ? kAudioHardwareUnspecifiedError : status;
}

static PksProcessTapHandle *pks_failed_create(PksProcessTapHandle *tap,
    int32_t *cleanup_status, uint8_t *cleanup_stage, const PksNativeCallObserver *observer) {
    pks_destroy_process_tap_observed(tap, cleanup_status, cleanup_stage, observer);
    return NULL;
}

PksProcessTapHandle *pks_create_process_tap_observed(const int32_t *pids, int pid_count,
    int32_t *out_status, uint8_t *out_stage,
    int32_t *cleanup_status, uint8_t *cleanup_stage,
    PksTapCancellationCheck cancelled, void *cancel_context, const PksNativeCallObserver *observer) {
    if (cleanup_status) *cleanup_status = 0;
    if (cleanup_stage) *cleanup_stage = 0;
    if (out_status) *out_status = noErr;
    if (out_stage) *out_stage = 0;
    if (cancelled && cancelled(cancel_context)) return NULL;
    if (@available(macOS 14.2, *)) {
        @autoreleasepool {
            CATapDescription *tapDesc;
            if (pid_count == 0 || !pids) {
                // Global system tap: capture all output, exclude nothing.
                uint64_t description_call = pks_observe_begin(observer, PKS_NATIVE_CALL_GLOBAL_TAP_DESCRIPTION);
                tapDesc = [[CATapDescription alloc]
                    initStereoGlobalTapButExcludeProcesses:@[]];
                pks_observe_end(observer, description_call, 0, 0);
            } else {
                // Tap specific processes by PID.
                // CATapDescription takes AudioObjectIDs, not PIDs.
                NSMutableArray<NSNumber *> *objIDs =
                    [NSMutableArray arrayWithCapacity:(NSUInteger)pid_count];
                for (int i = 0; i < pid_count; i++) {
                    if (cancelled && cancelled(cancel_context)) return NULL;
                    AudioObjectID objID = pks_audio_object_id_for_pid((pid_t)pids[i], observer);
                    if (objID != kAudioObjectUnknown)
                        [objIDs addObject:@(objID)];
                }
                if (objIDs.count == 0) {
                    if (out_status) *out_status = kAudioHardwareBadObjectError;
                    if (out_stage) *out_stage = PKS_TAP_STAGE_RESOLVE_PROCESS;
                    return NULL;
                }
                uint64_t description_call = pks_observe_begin(observer, PKS_NATIVE_CALL_PROCESS_TAP_DESCRIPTION);
                tapDesc = [[CATapDescription alloc]
                    initStereoMixdownOfProcesses:objIDs];
                pks_observe_end(observer, description_call, 0, 0);
            }
            if (!tapDesc) {
                if (out_status) *out_status = kAudioHardwareUnspecifiedError;
                if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_PROCESS_TAP;
                return NULL;
            }
            // Keep the source playing (CATapUnmuted = 0).
            uint64_t mute_call = pks_observe_begin(observer, PKS_NATIVE_CALL_TAP_MUTE_BEHAVIOR);
            tapDesc.muteBehavior = CATapUnmuted;
            pks_observe_end(observer, mute_call, 0, 0);

            PksProcessTapHandle *h = calloc(1, sizeof(PksProcessTapHandle));
            if (!h) {
                if (out_status) *out_status = kAudioHardwareUnspecifiedError;
                if (out_stage) *out_stage = PKS_TAP_STAGE_ALLOCATE_HANDLE;
                return NULL;
            }
            if (cancelled && cancelled(cancel_context))
                return pks_failed_create(h, cleanup_status, cleanup_stage, observer);
            AudioObjectID tapID = kAudioObjectUnknown;
            uint64_t call = pks_observe_begin(observer, PKS_NATIVE_CALL_CREATE_PROCESS_TAP);
            OSStatus err = AudioHardwareCreateProcessTap(tapDesc, &tapID);
            pks_observe_end(observer, call, err, 1);
            h->tap_id = tapID;
            h->control.tap_owned = tapID != kAudioObjectUnknown;
            if (err != noErr || tapID == kAudioObjectUnknown) {
                if (tapID == kAudioObjectUnknown) {
                    h->control.registration_uncertain = 1;
                    h->control.cleanup_status = pks_failure_status(err);
                }
                if (out_status) *out_status = pks_failure_status(err);
                if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_PROCESS_TAP;
                return pks_failed_create(h, cleanup_status, cleanup_stage, observer);
            }

            if (cancelled && cancelled(cancel_context))
                return pks_failed_create(h, cleanup_status, cleanup_stage, observer);
            // Get the tap's UID for the aggregate device descriptor.
            AudioObjectPropertyAddress uidAddr = {
                kAudioTapPropertyUID,
                kAudioObjectPropertyScopeGlobal,
                kAudioObjectPropertyElementMain
            };
            CFStringRef tapUID = NULL;
            uint32_t uidSz = sizeof(tapUID);
            err = pks_property_data(observer, PKS_NATIVE_CALL_TAP_UID, tapID, &uidAddr, 0, NULL, &uidSz, &tapUID);
            if (err != noErr || !tapUID) {
                if (tapUID) CFRelease(tapUID);
                if (out_status) *out_status = pks_failure_status(err);
                if (out_stage) *out_stage = PKS_TAP_STAGE_READ_TAP_UID;
                return pks_failed_create(h, cleanup_status, cleanup_stage, observer);
            }

            NSString *tapUIDStr = (__bridge_transfer NSString *)tapUID;
            if (cancelled && cancelled(cancel_context))
                return pks_failed_create(h, cleanup_status, cleanup_stage, observer);
            NSString *aggUID = [NSString stringWithFormat:
                @"io.pocketstation.tap.%@", tapUIDStr];

            // Use @() to convert bare C-string macros to NSString literals.
            NSDictionary *subTap = @{
                @(kAudioSubTapUIDKey):               tapUIDStr,
                @(kAudioSubTapDriftCompensationKey): @YES
            };
            NSDictionary *aggDesc = @{
                @(kAudioAggregateDeviceUIDKey):       aggUID,
                @(kAudioAggregateDeviceNameKey):      @"PocketStation Tap",
                @(kAudioAggregateDeviceIsPrivateKey): @YES,
                @(kAudioAggregateDeviceTapListKey):   @[subTap]
            };

            if (cancelled && cancelled(cancel_context))
                return pks_failed_create(h, cleanup_status, cleanup_stage, observer);
            AudioObjectID aggID = kAudioObjectUnknown;
            call = pks_observe_begin(observer, PKS_NATIVE_CALL_CREATE_AGGREGATE_DEVICE);
            err = AudioHardwareCreateAggregateDevice(
                (__bridge CFDictionaryRef)aggDesc, &aggID);
            pks_observe_end(observer, call, err, 1);
            h->agg_device_id = aggID;
            h->control.aggregate_owned = aggID != kAudioObjectUnknown;
            if (err != noErr || aggID == kAudioObjectUnknown) {
                if (aggID == kAudioObjectUnknown) {
                    h->control.registration_uncertain = 1;
                    h->control.cleanup_status = pks_failure_status(err);
                }
                if (out_status) *out_status = pks_failure_status(err);
                if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_AGGREGATE_DEVICE;
                return pks_failed_create(h, cleanup_status, cleanup_stage, observer);
            }

            atomic_store(&h->ring.write_head, 0);
            h->ring.read_head = 0;
            atomic_store(&h->ring.drop_count, 0);
            atomic_store(&h->ring.sample_rate, 48000u);
            atomic_store(&h->ring.timeline_sequence, 0);
            atomic_store(&h->ring.anchor_frame_position, 0);
            atomic_store(&h->ring.anchor_host_time, 0);
            ring_store_level(&h->ring, 0.0f);
            return h;
        }
    }
    if (out_status) *out_status = kAudioHardwareUnsupportedOperationError;
    if (out_stage) *out_stage = PKS_TAP_STAGE_PLATFORM_SUPPORT;
    return NULL;
}

// ─── Start IO ────────────────────────────────────────────────────────────────

static void pks_prefer_io_buffer_size(PksProcessTapHandle *tap,
                                      double sample_rate,
                                      uint16_t requested_io_duration_ms, const PksNativeCallObserver *observer) {
    AudioObjectPropertyAddress size_addr = {
        kAudioDevicePropertyBufferFrameSize,
        kAudioObjectPropertyScopeGlobal,
        kAudioObjectPropertyElementMain
    };
    uint32_t property_size = sizeof(uint32_t);
    uint32_t current_frames = 0;
    if (pks_property_data(observer, PKS_NATIVE_CALL_IO_BUFFER_SIZE_BEFORE, tap->agg_device_id, &size_addr, 0, NULL,
                                   &property_size, &current_frames) != noErr) {
        return;
    }
    tap->io_buffer_before_frames = current_frames;
    tap->io_buffer_applied_frames = current_frames;

    AudioObjectPropertyAddress range_addr = {
        kAudioDevicePropertyBufferFrameSizeRange,
        kAudioObjectPropertyScopeGlobal,
        kAudioObjectPropertyElementMain
    };
    AudioValueRange range = {0};
    property_size = sizeof(range);
    if (pks_property_data(observer, PKS_NATIVE_CALL_IO_BUFFER_SIZE_RANGE, tap->agg_device_id, &range_addr, 0, NULL,
                                   &property_size, &range) != noErr
            || !isfinite(range.mMinimum)
            || !isfinite(range.mMaximum)
            || range.mMinimum <= 0
            || range.mMaximum < range.mMinimum
            || range.mMaximum > UINT32_MAX) {
        return;
    }
    tap->io_buffer_min_frames = (uint32_t)ceil(range.mMinimum);
    tap->io_buffer_max_frames = (uint32_t)floor(range.mMaximum);

    double requested_frames_exact =
        sample_rate * (double)requested_io_duration_ms / 1000.0;
    if (!isfinite(requested_frames_exact)
            || requested_frames_exact < 1
            || requested_frames_exact > UINT32_MAX) {
        return;
    }
    uint32_t requested_frames = (uint32_t)llround(requested_frames_exact);
    tap->io_buffer_requested_frames = requested_frames;

    // A smaller native callback can reduce capture delay. A larger callback
    // cannot improve the requested Session cadence, so preserve the current
    // device setting instead of increasing it for the 20 ms profile.
    if (requested_frames >= current_frames
            || requested_frames < tap->io_buffer_min_frames
            || requested_frames > tap->io_buffer_max_frames) {
        return;
    }

    Boolean settable = false;
    if (pks_property_settable(observer, PKS_NATIVE_CALL_IO_BUFFER_SIZE_SETTABLE, tap->agg_device_id, &size_addr, &settable) != noErr
            || !settable) {
        return;
    }
    property_size = sizeof(requested_frames);
    if (pks_property_set(observer, PKS_NATIVE_CALL_IO_BUFFER_SIZE_SET, tap->agg_device_id, &size_addr, 0, NULL,
                                   property_size, &requested_frames) != noErr) {
        return;
    }

    uint32_t applied_frames = 0;
    property_size = sizeof(applied_frames);
    if (pks_property_data(observer, PKS_NATIVE_CALL_IO_BUFFER_SIZE_APPLIED, tap->agg_device_id, &size_addr, 0, NULL,
                                   &property_size, &applied_frames) == noErr
            && applied_frames > 0) {
        tap->io_buffer_applied_frames = applied_frames;
    }
}

static int32_t pks_register_io(void *context) {
    PksProcessTapHandle *tap = context;
    OSStatus status = AudioDeviceCreateIOProcID(
        tap->agg_device_id, tap_io_proc, &tap->ring, &tap->io_proc_id);
    if (tap->io_proc_id) {
        tap->control.registered = 1;
        tap->control.registration_uncertain = 0;
    }
    if (status != noErr || !tap->io_proc_id) {
        tap->control.cleanup_status = pks_failure_status(status);
        return pks_failure_status(status);
    }
    return noErr;
}
static int32_t pks_start_io(void *context) {
    PksProcessTapHandle *tap = context;
    return AudioDeviceStart(tap->agg_device_id, tap->io_proc_id);
}
static int32_t pks_stop_io(void *context) {
    PksProcessTapHandle *tap = context;
    return AudioDeviceStop(tap->agg_device_id, tap->io_proc_id);
}
static int32_t pks_unregister_io(void *context) {
    PksProcessTapHandle *tap = context;
    OSStatus status = AudioDeviceDestroyIOProcID(tap->agg_device_id, tap->io_proc_id);
    if (status == noErr) tap->io_proc_id = NULL;
    return status;
}
static int32_t pks_destroy_aggregate(void *context) {
    PksProcessTapHandle *tap = context;
    OSStatus status = AudioHardwareDestroyAggregateDevice(tap->agg_device_id);
    if (status == noErr) tap->agg_device_id = kAudioObjectUnknown;
    return status;
}
static int32_t pks_destroy_tap(void *context) {
    PksProcessTapHandle *tap = context;
    OSStatus status = AudioHardwareDestroyProcessTap(tap->tap_id);
    if (status == noErr) tap->tap_id = kAudioObjectUnknown;
    return status;
}
static const PksTapControlOperations pks_native_tap_operations = {
    pks_register_io, pks_start_io, pks_stop_io, pks_unregister_io,
    pks_destroy_aggregate, pks_destroy_tap
};

PksProcessTapHandle *pks_create_process_tap_checked(const int32_t *pids, int count,
    int32_t *status, uint8_t *stage, int32_t *cleanup_status, uint8_t *cleanup_stage,
    PksTapCancellationCheck cancelled, void *cancel_context) {
    return pks_create_process_tap_observed(pids, count, status, stage, cleanup_status,
        cleanup_stage, cancelled, cancel_context, NULL);
}

PksProcessTapHandle *pks_create_process_tap(const int32_t *pids, int pid_count,
    int32_t *out_status, uint8_t *out_stage) {
    return pks_create_process_tap_checked(pids, pid_count, out_status, out_stage,
                                         NULL, NULL, NULL, NULL);
}

int pks_tap_start_observed(PksProcessTapHandle *tap,
    uint16_t requested_io_duration_ms, PksTapCancellationCheck cancelled,
    void *cancel_context, int32_t *out_status, uint8_t *out_stage, const PksNativeCallObserver *observer) {
    if (out_status) *out_status = noErr;
    if (out_stage) *out_stage = 0;
    if (!tap || tap->control.registered || tap->control.registration_uncertain
            || tap->control.cleanup_attempted || tap->control.retained) {
        if (out_status) *out_status = kAudioHardwareIllegalOperationError;
        if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_IO_PROC;
        return -1;
    }
    if (cancelled && cancelled(cancel_context)) return -2;
    if (@available(macOS 14.2, *)) {
        AudioObjectPropertyAddress address = {
            kAudioDevicePropertyStreamFormat, kAudioObjectPropertyScopeInput,
            kAudioObjectPropertyElementMain
        };
        AudioStreamBasicDescription format = {0};
        uint32_t size = sizeof(format);
        if (pks_property_data(observer, PKS_NATIVE_CALL_STREAM_FORMAT, tap->agg_device_id, &address, 0, NULL,
            &size, &format) == noErr && format.mSampleRate > 0)
            atomic_store(&tap->ring.sample_rate, (uint32_t)format.mSampleRate);
        pks_prefer_io_buffer_size(tap,
            format.mSampleRate > 0 ? format.mSampleRate : 48000.0,
            requested_io_duration_ms, observer);
        return pks_tap_control_start_observed(&tap->control, &pks_native_tap_operations,
            tap, cancelled, cancel_context, out_status, out_stage, observer);
    }
    if (out_status) *out_status = kAudioHardwareUnsupportedOperationError;
    if (out_stage) *out_stage = PKS_TAP_STAGE_PLATFORM_SUPPORT;
    return -1;
}
int pks_tap_start_cancellable(PksProcessTapHandle *tap, uint16_t duration_ms,
    PksTapCancellationCheck cancelled, void *cancel_context, int32_t *status, uint8_t *stage) {
    return pks_tap_start_observed(tap, duration_ms, cancelled, cancel_context, status, stage, NULL);
}
int pks_tap_start(PksProcessTapHandle *tap, uint16_t duration_ms,
    int32_t *out_status, uint8_t *out_stage) {
    return pks_tap_start_cancellable(tap, duration_ms, NULL, NULL,
                                    out_status, out_stage);
}

// ─── Read ────────────────────────────────────────────────────────────────────

uint32_t pks_tap_read_frames_timed(PksProcessTapHandle *tap, float *out,
                                   uint32_t frame_count,
                                   uint64_t *out_source_frame_position,
                                   uint64_t *out_anchor_frame_position,
                                   uint64_t *out_anchor_host_time_ns) {
    if (!tap || !out) return 0;
    PksTapRing *ring = &tap->ring;

    uint64_t wHead = atomic_load_explicit(&ring->write_head, memory_order_acquire);
    uint64_t rHead = ring->read_head;
    uint64_t avail = wHead - rHead;
    if (avail == 0) return 0;
    if (avail > TAP_RING_FRAMES) {
        atomic_fetch_add_explicit(
            &ring->drop_count, avail - TAP_RING_FRAMES, memory_order_relaxed);
        rHead = wHead - TAP_RING_FRAMES;
        avail = TAP_RING_FRAMES;
    }

    if (out_source_frame_position) *out_source_frame_position = rHead;
    if (out_anchor_frame_position || out_anchor_host_time_ns) {
        uint64_t sequence_before;
        uint64_t sequence_after;
        uint64_t anchor_frame_position;
        uint64_t anchor_host_time;
        for (;;) {
            sequence_before = atomic_load_explicit(
                &ring->timeline_sequence, memory_order_acquire);
            if ((sequence_before & 1u) != 0) continue;
            anchor_frame_position = atomic_load_explicit(
                &ring->anchor_frame_position, memory_order_relaxed);
            anchor_host_time = atomic_load_explicit(
                &ring->anchor_host_time, memory_order_relaxed);
            sequence_after = atomic_load_explicit(
                &ring->timeline_sequence, memory_order_acquire);
            if (sequence_before == sequence_after && (sequence_after & 1u) == 0) break;
        }
        if (out_anchor_frame_position) *out_anchor_frame_position = anchor_frame_position;
        if (out_anchor_host_time_ns) {
            *out_anchor_host_time_ns = pks_host_time_to_ns(anchor_host_time);
        }
    }

    uint32_t toRead = (uint32_t)(avail < (uint64_t)frame_count ? avail : (uint64_t)frame_count);
    for (uint32_t i = 0; i < toRead; i++) {
        uint32_t slot = (uint32_t)((rHead + i) & TAP_RING_MASK);
        for (uint32_t c = 0; c < TAP_RING_CHANNELS; c++)
            out[i * TAP_RING_CHANNELS + c] = ring->data[slot * TAP_RING_CHANNELS + c];
    }
    if (toRead > 0) {
        float sum_sq = 0.0f;
        uint32_t sample_count = toRead * TAP_RING_CHANNELS;
        for (uint32_t sample = 0; sample < sample_count; sample++) {
            sum_sq += out[sample] * out[sample];
        }
        ring_store_level(ring, sqrtf(sum_sq / (float)sample_count));
    }
    ring->read_head = rHead + toRead;
    return toRead;
}

uint32_t pks_tap_read_frames(PksProcessTapHandle *tap, float *out, uint32_t frame_count) {
    return pks_tap_read_frames_timed(tap, out, frame_count, NULL, NULL, NULL);
}

uint64_t pks_tap_drop_count(const PksProcessTapHandle *tap) {
    if (!tap) return 0;
    return atomic_load_explicit(&tap->ring.drop_count, memory_order_relaxed);
}

uint32_t pks_tap_io_buffer_before_frames(const PksProcessTapHandle *tap) {
    return tap ? tap->io_buffer_before_frames : 0;
}

uint32_t pks_tap_io_buffer_requested_frames(const PksProcessTapHandle *tap) {
    return tap ? tap->io_buffer_requested_frames : 0;
}

uint32_t pks_tap_io_buffer_applied_frames(const PksProcessTapHandle *tap) {
    return tap ? tap->io_buffer_applied_frames : 0;
}

uint32_t pks_tap_io_buffer_min_frames(const PksProcessTapHandle *tap) {
    return tap ? tap->io_buffer_min_frames : 0;
}

uint32_t pks_tap_io_buffer_max_frames(const PksProcessTapHandle *tap) {
    return tap ? tap->io_buffer_max_frames : 0;
}

static uint32_t pks_tap_input_device_property(
    const PksProcessTapHandle *tap,
    AudioObjectPropertySelector selector, const PksNativeCallObserver *observer, uint32_t operation) {
    if (!tap) return 0;
    AudioObjectPropertyAddress address = {
        selector,
        kAudioObjectPropertyScopeInput,
        kAudioObjectPropertyElementMain
    };
    uint32_t value = 0;
    uint32_t size = sizeof(value);
    return pks_property_data(observer, operation,
        tap->agg_device_id, &address, 0, NULL, &size, &value) == noErr
        ? value
        : 0;
}

uint32_t pks_tap_input_device_latency_frames(const PksProcessTapHandle *tap) {
    return pks_tap_input_device_latency_frames_observed(tap, NULL);
}
uint32_t pks_tap_input_device_latency_frames_observed(const PksProcessTapHandle *tap, const PksNativeCallObserver *observer) {
    return pks_tap_input_device_property(tap, kAudioDevicePropertyLatency, observer, PKS_NATIVE_CALL_DEVICE_LATENCY);
}

uint32_t pks_tap_input_safety_offset_frames(const PksProcessTapHandle *tap) {
    return pks_tap_input_safety_offset_frames_observed(tap, NULL);
}
uint32_t pks_tap_input_safety_offset_frames_observed(const PksProcessTapHandle *tap, const PksNativeCallObserver *observer) {
    return pks_tap_input_device_property(tap, kAudioDevicePropertySafetyOffset, observer, PKS_NATIVE_CALL_SAFETY_OFFSET);
}

uint8_t pks_tap_input_safety_offset_settable(const PksProcessTapHandle *tap) {
    return pks_tap_input_safety_offset_settable_observed(tap, NULL);
}
uint8_t pks_tap_input_safety_offset_settable_observed(const PksProcessTapHandle *tap, const PksNativeCallObserver *observer) {
    if (!tap) return 0;
    AudioObjectPropertyAddress address = {
        kAudioDevicePropertySafetyOffset,
        kAudioObjectPropertyScopeInput,
        kAudioObjectPropertyElementMain
    };
    Boolean settable = false;
    return pks_property_settable(observer, PKS_NATIVE_CALL_SAFETY_OFFSET_SETTABLE, tap->agg_device_id, &address, &settable) == noErr && settable;
}

uint32_t pks_tap_input_stream_latency_frames(const PksProcessTapHandle *tap) {
    return pks_tap_input_stream_latency_frames_observed(tap, NULL);
}
uint32_t pks_tap_input_stream_latency_frames_observed(const PksProcessTapHandle *tap, const PksNativeCallObserver *observer) {
    if (!tap) return 0;
    AudioObjectPropertyAddress streams_address = {
        kAudioDevicePropertyStreams,
        kAudioObjectPropertyScopeInput,
        kAudioObjectPropertyElementMain
    };
    uint32_t streams_size = 0;
    if (pks_property_size(observer, PKS_NATIVE_CALL_STREAM_LIST_SIZE, tap->agg_device_id, &streams_address,
            0,
            NULL,
            &streams_size) != noErr
            || streams_size == 0
            || streams_size % sizeof(AudioStreamID) != 0) {
        return 0;
    }
    AudioStreamID *streams = (AudioStreamID *)malloc(streams_size);
    if (!streams) return 0;
    if (pks_property_data(observer, PKS_NATIVE_CALL_STREAM_LIST_DATA, tap->agg_device_id, &streams_address,
            0,
            NULL,
            &streams_size,
            streams) != noErr) {
        free(streams);
        return 0;
    }

    const uint32_t stream_count = streams_size / sizeof(AudioStreamID);
    uint32_t maximum_latency_frames = 0;
    AudioObjectPropertyAddress latency_address = {
        kAudioStreamPropertyLatency,
        kAudioObjectPropertyScopeGlobal,
        kAudioObjectPropertyElementMain
    };
    for (uint32_t index = 0; index < stream_count; index++) {
        uint32_t latency_frames = 0;
        uint32_t latency_size = sizeof(latency_frames);
        if (pks_property_data(observer, PKS_NATIVE_CALL_STREAM_LATENCY, streams[index], &latency_address,
                0,
                NULL,
                &latency_size,
                &latency_frames) == noErr
                && latency_frames > maximum_latency_frames) {
            maximum_latency_frames = latency_frames;
        }
    }
    free(streams);
    return maximum_latency_frames;
}

uint32_t pks_tap_sample_rate(const PksProcessTapHandle *tap) {
    if (!tap) return 48000;
    return atomic_load_explicit(&tap->ring.sample_rate, memory_order_relaxed);
}

uint32_t pks_tap_channels(const PksProcessTapHandle *tap) {
    (void)tap; return TAP_RING_CHANNELS;
}

float pks_tap_level(const PksProcessTapHandle *tap) {
    if (!tap) return 0.0f;
    return ring_load_level(&tap->ring);
}

// ─── Destroy ─────────────────────────────────────────────────────────────────

int pks_destroy_process_tap_observed(PksProcessTapHandle *tap,
    int32_t *out_status, uint8_t *out_stage, const PksNativeCallObserver *observer) {
    if (!tap) return 0;
    int result = pks_tap_control_cleanup_observed(&tap->control, &pks_native_tap_operations,
                                        tap, out_status, out_stage, observer);
    if (result == 0) free(tap);
    return result;
}
int pks_destroy_process_tap_checked(PksProcessTapHandle *tap, int32_t *status, uint8_t *stage) {
    return pks_destroy_process_tap_observed(tap, status, stage, NULL);
}
void pks_destroy_process_tap(PksProcessTapHandle *tap) {
    pks_destroy_process_tap_checked(tap, NULL, NULL);
}

#pragma clang diagnostic pop
