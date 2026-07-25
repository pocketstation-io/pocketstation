// CoreAudio process tap implementation — macOS 14.2+.
// Primary capture path for PocketStation on Sonoma and later.
// All tap functions are guarded by @available(macOS 14.2, *).
// Callers check pks_process_tap_available() before using any tap API.

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunguarded-availability"

#import <Foundation/Foundation.h>
#import <CoreAudio/CoreAudio.h>
#import <CoreAudio/CATapDescription.h>
#import <CoreAudio/AudioHardwareTapping.h>
#import <AppKit/NSRunningApplication.h>
#import <libproc.h>
#import <stdatomic.h>
#import <math.h>
#import <string.h>
#import <stdlib.h>

#include "source_discovery.h"

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
    float data[TAP_RING_FRAMES * TAP_RING_CHANNELS];
} PksTapRing;

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
    PksTapRing          ring;
};

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
    (void)dev; (void)now; (void)input_time; (void)output; (void)output_time;

    PksTapRing *ring = (PksTapRing *)user_data;
    if (!ring || !input || input->mNumberBuffers == 0) return noErr;

    const AudioBuffer *buf0 = &input->mBuffers[0];
    uint32_t frames;
    uint64_t head = atomic_load_explicit(&ring->write_head, memory_order_relaxed);
    float sumSq = 0.0f;

    bool interleaved = (input->mNumberBuffers == 1 && buf0->mNumberChannels > 1);

    if (interleaved) {
        uint32_t srcCh = buf0->mNumberChannels;
        frames = buf0->mDataByteSize / (srcCh * sizeof(float));
        if (frames == 0) return noErr;
        const float *src = (const float *)buf0->mData;
        for (uint32_t i = 0; i < frames; i++) {
            uint32_t slot = (uint32_t)((head + i) & TAP_RING_MASK);
            for (uint32_t c = 0; c < TAP_RING_CHANNELS; c++) {
                float s = (c < srcCh) ? src[i * srcCh + c] : 0.0f;
                ring->data[slot * TAP_RING_CHANNELS + c] = s;
                sumSq += s * s;
            }
        }
    } else {
        // Non-interleaved: one buffer per channel.
        uint32_t nbufs = input->mNumberBuffers;
        frames = buf0->mDataByteSize / sizeof(float);
        if (frames == 0) return noErr;
        for (uint32_t i = 0; i < frames; i++) {
            uint32_t slot = (uint32_t)((head + i) & TAP_RING_MASK);
            for (uint32_t c = 0; c < TAP_RING_CHANNELS; c++) {
                float s = 0.0f;
                if (c < nbufs) {
                    const float *chBuf = (const float *)input->mBuffers[c].mData;
                    s = chBuf[i];
                }
                ring->data[slot * TAP_RING_CHANNELS + c] = s;
                sumSq += s * s;
            }
        }
    }

    atomic_store_explicit(&ring->write_head, head + frames, memory_order_release);
    float rms = sqrtf(sumSq / (float)(frames * TAP_RING_CHANNELS + 1));
    ring_store_level(ring, rms);
    return noErr;
}

// ─── Helper: look up AudioObjectID for a given PID ──────────────────────────

// Returns kAudioObjectUnknown if no process object with that PID is found.
static AudioObjectID pks_audio_object_id_for_pid(pid_t target_pid) {
    AudioObjectPropertyAddress addr = {
        kAudioHardwarePropertyProcessObjectList,
        kAudioObjectPropertyScopeGlobal,
        kAudioObjectPropertyElementMain
    };
    uint32_t dataSize = 0;
    if (AudioObjectGetPropertyDataSize(kAudioObjectSystemObject, &addr, 0, NULL, &dataSize) != noErr)
        return kAudioObjectUnknown;
    if (dataSize == 0) return kAudioObjectUnknown;

    uint32_t count = dataSize / sizeof(AudioObjectID);
    AudioObjectID *objs = (AudioObjectID *)malloc(dataSize);
    if (!objs) return kAudioObjectUnknown;

    if (AudioObjectGetPropertyData(kAudioObjectSystemObject, &addr, 0, NULL, &dataSize, objs) != noErr) {
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
        if (AudioObjectGetPropertyData(objs[i], &pidAddr, 0, NULL, &sz, &pid) == noErr) {
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

int pks_discover_sources(PksCaptureSourceInfo *out, int max) {
    if (!out || max <= 0) return 0;

    @autoreleasepool {
        AudioObjectPropertyAddress addr = {
            kAudioHardwarePropertyProcessObjectList,
            kAudioObjectPropertyScopeGlobal,
            kAudioObjectPropertyElementMain
        };
        uint32_t dataSize = 0;
        OSStatus err = AudioObjectGetPropertyDataSize(
            kAudioObjectSystemObject, &addr, 0, NULL, &dataSize);
        if (err != noErr || dataSize == 0) return 0;

        uint32_t count = dataSize / sizeof(AudioObjectID);
        AudioObjectID *objs = (AudioObjectID *)malloc(dataSize);
        if (!objs) return 0;

        err = AudioObjectGetPropertyData(
            kAudioObjectSystemObject, &addr, 0, NULL, &dataSize, objs);
        if (err != noErr) { free(objs); return 0; }

        int written = 0;
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
            if (AudioObjectGetPropertyData(obj, &pidAddr, 0, NULL, &sz, &pid) != noErr)
                continue;
            info->pid = (int32_t)pid;

            // Bundle ID
            AudioObjectPropertyAddress bidAddr = {
                kAudioProcessPropertyBundleID,
                kAudioObjectPropertyScopeGlobal,
                kAudioObjectPropertyElementMain
            };
            CFStringRef bidRef = NULL;
            sz = sizeof(bidRef);
            if (AudioObjectGetPropertyData(obj, &bidAddr, 0, NULL, &sz, &bidRef) == noErr && bidRef) {
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
            AudioObjectGetPropertyData(obj, &runAddr, 0, NULL, &sz, &running);
            info->state = running ? PKS_SOURCE_STATE_PLAYING : PKS_SOURCE_STATE_SILENT;

            // Friendly label only. Identity remains the bundle/PID fields.
            NSRunningApplication *pidApp =
                [NSRunningApplication runningApplicationWithProcessIdentifier:pid];
            if (pidApp && pidApp.localizedName) {
                [pidApp.localizedName getCString:info->name
                                       maxLength:sizeof(info->name)
                                        encoding:NSUTF8StringEncoding];
            }
            if (info->bundle_id[0] != '\0') {
                NSString *bid = [NSString stringWithUTF8String:info->bundle_id];
                NSArray<NSRunningApplication *> *apps =
                    [NSRunningApplication runningApplicationsWithBundleIdentifier:bid];
                NSRunningApplication *app = apps.firstObject;
                if (info->name[0] == '\0' && app && app.localizedName) {
                    [app.localizedName getCString:info->name
                                       maxLength:sizeof(info->name)
                                        encoding:NSUTF8StringEncoding];
                }
            }
            if (info->name[0] == '\0') {
                proc_name(pid, info->name, (uint32_t)sizeof(info->name));
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

PksProcessTapHandle *pks_create_process_tap(const int32_t *pids, int pid_count,
                                            int32_t *out_status, uint8_t *out_stage) {
    if (out_status) *out_status = noErr;
    if (out_stage) *out_stage = 0;
    if (@available(macOS 14.2, *)) {
        @autoreleasepool {
            CATapDescription *tapDesc;
            if (pid_count == 0 || !pids) {
                // Global system tap: capture all output, exclude nothing.
                tapDesc = [[CATapDescription alloc]
                    initStereoGlobalTapButExcludeProcesses:@[]];
            } else {
                // Tap specific processes by PID.
                // CATapDescription takes AudioObjectIDs, not PIDs.
                NSMutableArray<NSNumber *> *objIDs =
                    [NSMutableArray arrayWithCapacity:(NSUInteger)pid_count];
                for (int i = 0; i < pid_count; i++) {
                    AudioObjectID objID = pks_audio_object_id_for_pid((pid_t)pids[i]);
                    if (objID != kAudioObjectUnknown)
                        [objIDs addObject:@(objID)];
                }
                if (objIDs.count == 0) {
                    if (out_status) *out_status = kAudioHardwareBadObjectError;
                    if (out_stage) *out_stage = PKS_TAP_STAGE_RESOLVE_PROCESS;
                    return NULL;
                }
                tapDesc = [[CATapDescription alloc]
                    initStereoMixdownOfProcesses:objIDs];
            }
            if (!tapDesc) {
                if (out_status) *out_status = kAudioHardwareUnspecifiedError;
                if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_PROCESS_TAP;
                return NULL;
            }
            // Keep the source playing (CATapUnmuted = 0).
            tapDesc.muteBehavior = CATapUnmuted;

            AudioObjectID tapID = kAudioObjectUnknown;
            OSStatus err = AudioHardwareCreateProcessTap(tapDesc, &tapID);
            if (err != noErr || tapID == kAudioObjectUnknown) {
                if (out_status) *out_status = pks_failure_status(err);
                if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_PROCESS_TAP;
                return NULL;
            }

            // Get the tap's UID for the aggregate device descriptor.
            AudioObjectPropertyAddress uidAddr = {
                kAudioTapPropertyUID,
                kAudioObjectPropertyScopeGlobal,
                kAudioObjectPropertyElementMain
            };
            CFStringRef tapUID = NULL;
            uint32_t uidSz = sizeof(tapUID);
            err = AudioObjectGetPropertyData(tapID, &uidAddr, 0, NULL, &uidSz, &tapUID);
            if (err != noErr || !tapUID) {
                if (out_status) *out_status = pks_failure_status(err);
                if (out_stage) *out_stage = PKS_TAP_STAGE_READ_TAP_UID;
                AudioHardwareDestroyProcessTap(tapID);
                return NULL;
            }

            NSString *tapUIDStr = (__bridge_transfer NSString *)tapUID;
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

            AudioObjectID aggID = kAudioObjectUnknown;
            err = AudioHardwareCreateAggregateDevice(
                (__bridge CFDictionaryRef)aggDesc, &aggID);
            if (err != noErr || aggID == kAudioObjectUnknown) {
                if (out_status) *out_status = pks_failure_status(err);
                if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_AGGREGATE_DEVICE;
                AudioHardwareDestroyProcessTap(tapID);
                return NULL;
            }

            PksProcessTapHandle *h =
                (PksProcessTapHandle *)calloc(1, sizeof(PksProcessTapHandle));
            if (!h) {
                if (out_status) *out_status = kAudioHardwareUnspecifiedError;
                if (out_stage) *out_stage = PKS_TAP_STAGE_ALLOCATE_HANDLE;
                AudioHardwareDestroyAggregateDevice(aggID);
                AudioHardwareDestroyProcessTap(tapID);
                return NULL;
            }
            h->tap_id        = tapID;
            h->agg_device_id = aggID;
            h->io_proc_id    = NULL;
            atomic_store(&h->ring.write_head, 0);
            h->ring.read_head = 0;
            atomic_store(&h->ring.drop_count, 0);
            atomic_store(&h->ring.sample_rate, 48000u);
            ring_store_level(&h->ring, 0.0f);
            return h;
        }
    }
    if (out_status) *out_status = kAudioHardwareUnsupportedOperationError;
    if (out_stage) *out_stage = PKS_TAP_STAGE_PLATFORM_SUPPORT;
    return NULL;
}

// ─── Start IO ────────────────────────────────────────────────────────────────

int pks_tap_start(PksProcessTapHandle *tap, int32_t *out_status, uint8_t *out_stage) {
    if (out_status) *out_status = noErr;
    if (out_stage) *out_stage = 0;
    if (!tap || tap->io_proc_id) {
        if (out_status) *out_status = kAudioHardwareIllegalOperationError;
        if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_IO_PROC;
        return -1;
    }
    if (@available(macOS 14.2, *)) {
        // Detect the aggregate device's input format to get the actual sample rate.
        AudioObjectPropertyAddress fmtAddr = {
            kAudioDevicePropertyStreamFormat,
            kAudioObjectPropertyScopeInput,
            kAudioObjectPropertyElementMain
        };
        AudioStreamBasicDescription fmt;
        memset(&fmt, 0, sizeof(fmt));
        uint32_t fmtSz = sizeof(fmt);
        if (AudioObjectGetPropertyData(tap->agg_device_id, &fmtAddr, 0, NULL,
                                       &fmtSz, &fmt) == noErr && fmt.mSampleRate > 0)
            atomic_store(&tap->ring.sample_rate, (uint32_t)fmt.mSampleRate);

        OSStatus err = AudioDeviceCreateIOProcID(
            tap->agg_device_id, tap_io_proc, &tap->ring, &tap->io_proc_id);
        if (err != noErr) {
            if (out_status) *out_status = err;
            if (out_stage) *out_stage = PKS_TAP_STAGE_CREATE_IO_PROC;
            return -1;
        }

        err = AudioDeviceStart(tap->agg_device_id, tap->io_proc_id);
        if (err != noErr) {
            if (out_status) *out_status = err;
            if (out_stage) *out_stage = PKS_TAP_STAGE_START_DEVICE;
            AudioDeviceDestroyIOProcID(tap->agg_device_id, tap->io_proc_id);
            tap->io_proc_id = NULL;
            return -1;
        }
        return 0;
    }
    if (out_status) *out_status = kAudioHardwareUnsupportedOperationError;
    if (out_stage) *out_stage = PKS_TAP_STAGE_PLATFORM_SUPPORT;
    return -1;
}

// ─── Read ────────────────────────────────────────────────────────────────────

uint32_t pks_tap_read_frames(PksProcessTapHandle *tap, float *out, uint32_t frame_count) {
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

    uint32_t toRead = (uint32_t)(avail < (uint64_t)frame_count ? avail : (uint64_t)frame_count);
    for (uint32_t i = 0; i < toRead; i++) {
        uint32_t slot = (uint32_t)((rHead + i) & TAP_RING_MASK);
        for (uint32_t c = 0; c < TAP_RING_CHANNELS; c++)
            out[i * TAP_RING_CHANNELS + c] = ring->data[slot * TAP_RING_CHANNELS + c];
    }
    ring->read_head = rHead + toRead;
    return toRead;
}

uint64_t pks_tap_drop_count(const PksProcessTapHandle *tap) {
    if (!tap) return 0;
    return atomic_load_explicit(&tap->ring.drop_count, memory_order_relaxed);
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

void pks_destroy_process_tap(PksProcessTapHandle *tap) {
    if (!tap) return;
    if (tap->io_proc_id) {
        AudioDeviceStop(tap->agg_device_id, tap->io_proc_id);
        AudioDeviceDestroyIOProcID(tap->agg_device_id, tap->io_proc_id);
        tap->io_proc_id = NULL;
    }
    if (tap->agg_device_id != kAudioObjectUnknown) {
        AudioHardwareDestroyAggregateDevice(tap->agg_device_id);
        tap->agg_device_id = kAudioObjectUnknown;
    }
    if (tap->tap_id != kAudioObjectUnknown) {
        if (@available(macOS 14.2, *))
            AudioHardwareDestroyProcessTap(tap->tap_id);
        tap->tap_id = kAudioObjectUnknown;
    }
    free(tap);
}

#pragma clang diagnostic pop
