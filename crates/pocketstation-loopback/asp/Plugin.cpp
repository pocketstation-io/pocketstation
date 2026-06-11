// PocketStation AudioServerPlugin
// Implements AudioServerPlugInDriverInterface directly (no libASPL).
// Architecture matches BlackHole but fixes:
//   - No mutex on RT thread (uses _Atomic instead of pthread_mutex_t in GetZeroTimeStamp)
//   - Per-device state struct (no static locals in DoIOOperation — fixes race with 2 devices)
//   - Power-of-2 ring with bitmask wrap (branch-free)
//   - POSIX shared memory ring (Rust reads without extra IPC)
//   - ZeroTimeStampPeriod derived from sample rate
//   - Drop counter in shared memory

#include <CoreAudio/AudioServerPlugIn.h>
#include <CoreAudio/CoreAudioTypes.h>
#include <CoreFoundation/CoreFoundation.h>
#include <mach/mach_time.h>
#include <pthread.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdatomic.h>
#include <string.h>
#include <dispatch/dispatch.h>

#include "SharedRing.h"

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

#define kPlugIn_BundleID        "io.pocketstation.loopback"
#define kDevice_UID             "PocketStationLoopback_UID"
#define kDevice_ModelUID        "PocketStationLoopback_Model"
#define kDevice_Name            "PocketStation Loopback"
#define kDevice_Manufacturer    "PocketStation"

// Object IDs (arbitrary stable values > 1; 1 is kAudioObjectSystemObject)
#define kObjectID_PlugIn        2u
#define kObjectID_Device        3u
#define kObjectID_Stream_Input  4u
#define kObjectID_Stream_Output 5u

// IO buffer frame size presented to coreaudiod.
// coreaudiod may override this; we accept any value in DoIOOperation.
#define kDevice_IOBufferFrameSize   512u

// Ring: 65536 frames, power-of-2 (defined in SharedRing.h)

// Supported sample rates
static const Float64 kSampleRates[] = { 44100.0, 48000.0, 88200.0, 96000.0 };
#define kNumSampleRates  4u
#define kDefaultSampleRate  48000.0

// ---------------------------------------------------------------------------
// Per-device state (fixes BlackHole's static-local race)
// ---------------------------------------------------------------------------

typedef struct {
    AudioObjectID       deviceID;

    // Sample rate — written on config-change thread, read on RT thread.
    // Protected by dispatch to RT thread exclusion via RequestDeviceConfigurationChange.
    Float64             sampleRate;

    // Requested sample rate (set from SetPropertyData, applied in PerformDeviceConfigurationChange)
    Float64             requestedSampleRate;

    // IO client count
    _Atomic uint32_t    ioClientCount;

    // Zero-timestamp state — no mutex; single writer (GetZeroTimeStamp on RT thread)
    _Atomic uint64_t    tsAnchorHostTime;
    _Atomic uint64_t    tsFrameCounter;   // number of ZTS periods elapsed
    _Atomic uint64_t    tsSeed;

    // Mach timebase
    mach_timebase_info_data_t timebase;

    // POSIX shared memory
    int                 shmFd;
    PksLoopbackRing*    ring;

    // Last write position (for overrun detection, like BlackHole's lastOutputSampleTime)
    _Atomic uint64_t    lastWriteSampleTime;
} PksDevice;

// ---------------------------------------------------------------------------
// Driver state
// ---------------------------------------------------------------------------

typedef struct {
    AudioServerPlugInDriverInterface*   interface;  // must be first
    AudioServerPlugInDriverInterface    vtable;
    AudioServerPlugInHostRef            host;
    _Atomic uint32_t                    refCount;
    PksDevice                           device;
} PksDriver;

// Single global driver instance (coreaudiod loads one plugin instance)
static PksDriver gDriver;

// Forward declarations
static AudioServerPlugInDriverRef pks_driver_ref(void) {
    return (AudioServerPlugInDriverRef)&gDriver;
}

// ---------------------------------------------------------------------------
// Shared memory helpers
// ---------------------------------------------------------------------------

static OSStatus pks_shm_create(PksDevice* dev) {
    // Unlink any stale region from a previous run
    shm_unlink(PKS_SHM_NAME);

    int fd = shm_open(PKS_SHM_NAME, O_CREAT | O_RDWR, 0666);
    if (fd < 0) return kAudioHardwareUnspecifiedError;

    if (ftruncate(fd, (off_t)PKS_SHM_SIZE) != 0) {
        close(fd);
        shm_unlink(PKS_SHM_NAME);
        return kAudioHardwareUnspecifiedError;
    }

    void* ptr = mmap(NULL, PKS_SHM_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (ptr == MAP_FAILED) {
        close(fd);
        shm_unlink(PKS_SHM_NAME);
        return kAudioHardwareUnspecifiedError;
    }

    memset(ptr, 0, PKS_SHM_SIZE);
    dev->shmFd = fd;
    dev->ring = (PksLoopbackRing*)ptr;
    return kAudioHardwareNoError;
}

static void pks_shm_destroy(PksDevice* dev) {
    if (dev->ring && dev->ring != MAP_FAILED) {
        munmap(dev->ring, PKS_SHM_SIZE);
        dev->ring = NULL;
    }
    if (dev->shmFd >= 0) {
        close(dev->shmFd);
        dev->shmFd = -1;
    }
    shm_unlink(PKS_SHM_NAME);
}

// ---------------------------------------------------------------------------
// Timing helpers (lock-free GetZeroTimeStamp — beats BlackHole's mutex)
// ---------------------------------------------------------------------------

static Float64 pks_host_ticks_per_frame(const PksDevice* dev) {
    // Convert: ticks_per_second / sample_rate
    // mach_absolute_time ticks per second = timebase.denom / timebase.numer * 1e9
    Float64 ticksPerNs = (Float64)dev->timebase.denom / (Float64)dev->timebase.numer;
    return (ticksPerNs * 1.0e9) / dev->sampleRate;
}

// ---------------------------------------------------------------------------
// COM IUnknown
// ---------------------------------------------------------------------------

static HRESULT pks_QueryInterface(void* inDriver, REFIID inUUID, LPVOID* outInterface) {
    if (!outInterface) return E_POINTER;
    // kAudioServerPlugInDriverInterfaceUUID — EEA5773D-CC43-49F1-8E00-8F96E7D23B17
    CFUUIDRef ifaceUUID = CFUUIDGetConstantUUIDWithBytes(NULL,
        0xEE, 0xA5, 0x77, 0x3D, 0xCC, 0x43, 0x49, 0xF1,
        0x8E, 0x00, 0x8F, 0x96, 0xE7, 0xD2, 0x3B, 0x17);
    // inUUID is a CFUUIDBytes value (REFIID); wrap it for CFEqual comparison.
    CFUUIDRef requestedUUID = CFUUIDCreateFromUUIDBytes(kCFAllocatorDefault, inUUID);
    bool match = CFEqual(requestedUUID, ifaceUUID);
    CFRelease(requestedUUID);
    if (match) {
        *outInterface = &gDriver.vtable;
        ((PksDriver*)inDriver)->refCount++;
        return S_OK;
    }
    *outInterface = NULL;
    return E_NOINTERFACE;
}

static ULONG pks_AddRef(void* inDriver) {
    return atomic_fetch_add_explicit(&((PksDriver*)inDriver)->refCount, 1u, memory_order_relaxed) + 1u;
}

static ULONG pks_Release(void* inDriver) {
    return atomic_fetch_sub_explicit(&((PksDriver*)inDriver)->refCount, 1u, memory_order_relaxed) - 1u;
}

// ---------------------------------------------------------------------------
// Initialize / CreateDevice
// ---------------------------------------------------------------------------

static OSStatus pks_Initialize(AudioServerPlugInDriverRef inDriver,
                                AudioServerPlugInHostRef inHost) {
    PksDriver* drv = (PksDriver*)inDriver;
    drv->host = inHost;

    PksDevice* dev = &drv->device;
    dev->deviceID         = kObjectID_Device;
    dev->sampleRate       = kDefaultSampleRate;
    dev->requestedSampleRate = kDefaultSampleRate;
    dev->shmFd            = -1;
    dev->ring             = NULL;
    atomic_store(&dev->ioClientCount, 0u);
    atomic_store(&dev->tsAnchorHostTime, 0u);
    atomic_store(&dev->tsFrameCounter, 0u);
    atomic_store(&dev->tsSeed, 1u);
    atomic_store(&dev->lastWriteSampleTime, 0u);
    mach_timebase_info(&dev->timebase);

    return kAudioHardwareNoError;
}

static OSStatus pks_CreateDevice(AudioServerPlugInDriverRef inDriver,
                                  CFDictionaryRef inDescription,
                                  const AudioServerPlugInClientInfo* inClientInfo,
                                  AudioObjectID* outDeviceObjectID) {
    (void)inDriver; (void)inDescription; (void)inClientInfo;
    *outDeviceObjectID = kObjectID_Device;
    return kAudioHardwareNoError;
}

static OSStatus pks_DestroyDevice(AudioServerPlugInDriverRef inDriver, AudioObjectID inDeviceObjectID) {
    (void)inDriver; (void)inDeviceObjectID;
    return kAudioHardwareNoError;
}

static OSStatus pks_AddDeviceClient(AudioServerPlugInDriverRef d, AudioObjectID dev,
                                     const AudioServerPlugInClientInfo* c) {
    (void)d; (void)dev; (void)c; return kAudioHardwareNoError;
}
static OSStatus pks_RemoveDeviceClient(AudioServerPlugInDriverRef d, AudioObjectID dev,
                                        const AudioServerPlugInClientInfo* c) {
    (void)d; (void)dev; (void)c; return kAudioHardwareNoError;
}

// ---------------------------------------------------------------------------
// StartIO / StopIO
// ---------------------------------------------------------------------------

static OSStatus pks_StartIO(AudioServerPlugInDriverRef inDriver,
                              AudioObjectID inDeviceObjectID,
                              UInt32 inClientID) {
    (void)inClientID;
    PksDevice* dev = &((PksDriver*)inDriver)->device;
    if (dev->deviceID != inDeviceObjectID) return kAudioHardwareBadObjectError;

    UInt32 prev = atomic_fetch_add_explicit(&dev->ioClientCount, 1u, memory_order_acq_rel);
    if (prev == 0) {
        // First client — create shared memory and reset timing
        OSStatus err = pks_shm_create(dev);
        if (err != kAudioHardwareNoError) return err;

        atomic_store_explicit(&dev->ring->sample_rate, (uint32_t)dev->sampleRate, memory_order_release);
        atomic_store_explicit(&dev->ring->channels, PKS_MAX_CHANNELS, memory_order_release);
        atomic_store_explicit(&dev->ring->io_running, 1u, memory_order_release);
        atomic_store_explicit(&dev->ring->write_head, 0u, memory_order_release);
        atomic_store_explicit(&dev->ring->drop_count, 0u, memory_order_release);

        atomic_store_explicit(&dev->tsAnchorHostTime, mach_absolute_time(), memory_order_release);
        atomic_store_explicit(&dev->tsFrameCounter, 0u, memory_order_release);
        atomic_store_explicit(&dev->lastWriteSampleTime, 0u, memory_order_release);
    }
    return kAudioHardwareNoError;
}

static OSStatus pks_StopIO(AudioServerPlugInDriverRef inDriver,
                             AudioObjectID inDeviceObjectID,
                             UInt32 inClientID) {
    (void)inClientID;
    PksDevice* dev = &((PksDriver*)inDriver)->device;
    if (dev->deviceID != inDeviceObjectID) return kAudioHardwareBadObjectError;

    UInt32 prev = atomic_fetch_sub_explicit(&dev->ioClientCount, 1u, memory_order_acq_rel);
    if (prev == 1) {
        if (dev->ring) {
            atomic_store_explicit(&dev->ring->io_running, 0u, memory_order_release);
        }
        pks_shm_destroy(dev);
    }
    return kAudioHardwareNoError;
}

// ---------------------------------------------------------------------------
// GetZeroTimeStamp — lock-free (beats BlackHole's pthread_mutex_t)
// ---------------------------------------------------------------------------

static OSStatus pks_GetZeroTimeStamp(AudioServerPlugInDriverRef inDriver,
                                      AudioObjectID inDeviceObjectID,
                                      UInt32 inClientID,
                                      Float64* outSampleTime,
                                      UInt64* outHostTime,
                                      UInt64* outSeed) {
    (void)inClientID;
    PksDevice* dev = &((PksDriver*)inDriver)->device;
    if (dev->deviceID != inDeviceObjectID) return kAudioHardwareBadObjectError;

    UInt64 now = mach_absolute_time();
    Float64 ticksPerFrame = pks_host_ticks_per_frame(dev);
    UInt64 ticksPerPeriod = (UInt64)(ticksPerFrame * (Float64)PKS_RING_FRAMES);

    UInt64 anchor = atomic_load_explicit(&dev->tsAnchorHostTime, memory_order_acquire);
    UInt64 counter = atomic_load_explicit(&dev->tsFrameCounter, memory_order_acquire);

    // Advance counter until anchor is in the past (no mutex — single RT thread writer)
    while (now >= anchor + ticksPerPeriod) {
        anchor += ticksPerPeriod;
        counter++;
    }
    atomic_store_explicit(&dev->tsAnchorHostTime, anchor, memory_order_release);
    atomic_store_explicit(&dev->tsFrameCounter, counter, memory_order_release);

    *outSampleTime = (Float64)(counter * PKS_RING_FRAMES);
    *outHostTime   = anchor;
    *outSeed       = atomic_load_explicit(&dev->tsSeed, memory_order_relaxed);
    return kAudioHardwareNoError;
}

// ---------------------------------------------------------------------------
// WillDoIOOperation
// ---------------------------------------------------------------------------

static OSStatus pks_WillDoIOOperation(AudioServerPlugInDriverRef inDriver,
                                       AudioObjectID inDeviceObjectID,
                                       UInt32 inClientID,
                                       UInt32 inOperationID,
                                       Boolean* outWillDo,
                                       Boolean* outWillDoInPlace) {
    (void)inDriver; (void)inDeviceObjectID; (void)inClientID;
    switch (inOperationID) {
        case kAudioServerPlugInIOOperationReadInput:
        case kAudioServerPlugInIOOperationWriteMix:
            *outWillDo        = true;
            *outWillDoInPlace = true;
            break;
        default:
            *outWillDo        = false;
            *outWillDoInPlace = false;
            break;
    }
    return kAudioHardwareNoError;
}

static OSStatus pks_BeginIOOperation(AudioServerPlugInDriverRef d, AudioObjectID dev,
    UInt32 c, UInt32 op, UInt32 sz, const AudioServerPlugInIOCycleInfo* ci) {
    (void)d;(void)dev;(void)c;(void)op;(void)sz;(void)ci;
    return kAudioHardwareNoError;
}

static OSStatus pks_EndIOOperation(AudioServerPlugInDriverRef d, AudioObjectID dev,
    UInt32 c, UInt32 op, UInt32 sz, const AudioServerPlugInIOCycleInfo* ci) {
    (void)d;(void)dev;(void)c;(void)op;(void)sz;(void)ci;
    return kAudioHardwareNoError;
}

// ---------------------------------------------------------------------------
// DoIOOperation — hot path, no alloc/lock/log
// ---------------------------------------------------------------------------

static OSStatus pks_DoIOOperation(AudioServerPlugInDriverRef inDriver,
                                   AudioObjectID inDeviceObjectID,
                                   AudioObjectID inStreamObjectID,
                                   UInt32 inClientID,
                                   UInt32 inOperationID,
                                   UInt32 inIOBufferFrameSize,
                                   const AudioServerPlugInIOCycleInfo* inIOCycleInfo,
                                   void* ioMainBuffer,
                                   void* ioSecondaryBuffer) {
    (void)inClientID; (void)ioSecondaryBuffer; (void)inStreamObjectID;
    PksDevice* dev = &((PksDriver*)inDriver)->device;
    if (dev->deviceID != inDeviceObjectID) return kAudioHardwareNoError;

    PksLoopbackRing* ring = dev->ring;
    if (!ring) return kAudioHardwareNoError;

    const UInt32 ch = PKS_MAX_CHANNELS;
    const float* src = (const float*)ioMainBuffer;
    float* dst       = (float*)ioMainBuffer;

    if (inOperationID == kAudioServerPlugInIOOperationWriteMix) {
        // Output path: system mix → shared memory ring
        // Power-of-2 ring, bitmask wrap — no branch
        uint64_t head = atomic_load_explicit(&ring->write_head, memory_order_relaxed);

        // Overrun check (matches BlackHole's lastOutputSampleTime logic)
        Float64 currentTime = inIOCycleInfo->mCurrentTime.mSampleTime;
        Float64 outputTime  = inIOCycleInfo->mOutputTime.mSampleTime;
        if (currentTime > outputTime + (Float64)inIOBufferFrameSize) {
            atomic_fetch_add_explicit(&ring->drop_count, 1u, memory_order_relaxed);
            return kAudioHardwareNoError;
        }

        for (UInt32 i = 0; i < inIOBufferFrameSize; i++) {
            UInt32 slot = (UInt32)((head + i) & PKS_RING_MASK);
            for (UInt32 c2 = 0; c2 < ch; c2++) {
                ring->data[slot * ch + c2] = src[i * ch + c2];
            }
        }
        atomic_store_explicit(&ring->write_head, head + inIOBufferFrameSize, memory_order_release);
        atomic_store_explicit(&dev->lastWriteSampleTime,
            (uint64_t)(inIOCycleInfo->mOutputTime.mSampleTime + inIOBufferFrameSize),
            memory_order_relaxed);

    } else if (inOperationID == kAudioServerPlugInIOOperationReadInput) {
        // Input path: shared memory ring → HAL input consumers
        // Silence if writer is too far behind
        uint64_t wHead = atomic_load_explicit(&ring->write_head, memory_order_acquire);
        UInt64 expected = (UInt64)inIOCycleInfo->mInputTime.mSampleTime;

        if (wHead < expected + inIOBufferFrameSize) {
            // Writer hasn't caught up — output silence
            memset(dst, 0, inIOBufferFrameSize * ch * sizeof(float));
        } else {
            UInt64 readPos = expected;
            for (UInt32 i = 0; i < inIOBufferFrameSize; i++) {
                UInt32 slot = (UInt32)((readPos + i) & PKS_RING_MASK);
                for (UInt32 c2 = 0; c2 < ch; c2++) {
                    dst[i * ch + c2] = ring->data[slot * ch + c2];
                }
            }
        }
    }

    return kAudioHardwareNoError;
}

// ---------------------------------------------------------------------------
// Property dispatch
// ---------------------------------------------------------------------------

// Helper: stream format for our device (f32 interleaved stereo at given rate)
static AudioStreamBasicDescription pks_stream_format(Float64 rate) {
    AudioStreamBasicDescription fmt;
    memset(&fmt, 0, sizeof(fmt));
    fmt.mSampleRate       = rate;
    fmt.mFormatID         = kAudioFormatLinearPCM;
    fmt.mFormatFlags      = kAudioFormatFlagIsFloat | kAudioFormatFlagIsPacked;
    fmt.mBytesPerPacket   = PKS_MAX_CHANNELS * sizeof(float);
    fmt.mFramesPerPacket  = 1;
    fmt.mBytesPerFrame    = PKS_MAX_CHANNELS * sizeof(float);
    fmt.mChannelsPerFrame = PKS_MAX_CHANNELS;
    fmt.mBitsPerChannel   = 32;
    return fmt;
}

static Boolean pks_HasProperty(AudioServerPlugInDriverRef inDriver,
                                AudioObjectID inObjectID,
                                pid_t inClientPID,
                                const AudioObjectPropertyAddress* inAddress) {
    (void)inDriver; (void)inClientPID;
    AudioObjectPropertySelector sel = inAddress->mSelector;

    if (inObjectID == kObjectID_PlugIn) {
        switch (sel) {
            case kAudioObjectPropertyBaseClass:
            case kAudioObjectPropertyClass:
            case kAudioObjectPropertyOwner:
            case kAudioObjectPropertyManufacturer:
            case kAudioObjectPropertyOwnedObjects:
            case kAudioPlugInPropertyBoxList:
            case kAudioPlugInPropertyTranslateUIDToBox:
            case kAudioPlugInPropertyDeviceList:
            case kAudioPlugInPropertyTranslateUIDToDevice:
            case kAudioPlugInPropertyResourceBundle:
                return true;
            default: break;
        }
    }
    if (inObjectID == kObjectID_Device) {
        switch (sel) {
            case kAudioObjectPropertyBaseClass:
            case kAudioObjectPropertyClass:
            case kAudioObjectPropertyOwner:
            case kAudioObjectPropertyName:
            case kAudioObjectPropertyManufacturer:
            case kAudioObjectPropertyOwnedObjects:
            case kAudioObjectPropertyIdentify:
            case kAudioObjectPropertyModelName:
            case kAudioObjectPropertySerialNumber:
            case kAudioObjectPropertyFirmwareVersion:
            case kAudioDevicePropertyDeviceUID:
            case kAudioDevicePropertyModelUID:
            case kAudioDevicePropertyTransportType:
            case kAudioDevicePropertyClockDomain:
            case kAudioDevicePropertyDeviceIsAlive:
            case kAudioDevicePropertyDeviceIsRunning:
            case kAudioDevicePropertyDeviceCanBeDefaultDevice:
            case kAudioDevicePropertyDeviceCanBeDefaultSystemDevice:
            case kAudioDevicePropertyLatency:
            case kAudioDevicePropertyStreams:
            case kAudioDevicePropertyIsHidden:
            case kAudioDevicePropertyPreferredChannelsForStereo:
            case kAudioDevicePropertyPreferredChannelLayout:
            case kAudioDevicePropertyZeroTimeStampPeriod:
            case kAudioDevicePropertyNominalSampleRate:
            case kAudioDevicePropertyAvailableNominalSampleRates:
            case kAudioDevicePropertyRelatedDevices:
            case kAudioDevicePropertySafetyOffset:
            case kAudioDevicePropertyIcon:
                return true;
            default: break;
        }
    }
    if (inObjectID == kObjectID_Stream_Input || inObjectID == kObjectID_Stream_Output) {
        switch (sel) {
            case kAudioObjectPropertyBaseClass:
            case kAudioObjectPropertyClass:
            case kAudioObjectPropertyOwner:
            case kAudioObjectPropertyOwnedObjects:
            case kAudioStreamPropertyIsActive:
            case kAudioStreamPropertyDirection:
            case kAudioStreamPropertyTerminalType:
            case kAudioStreamPropertyStartingChannel:
            case kAudioStreamPropertyLatency:
            case kAudioStreamPropertyVirtualFormat:
            case kAudioStreamPropertyPhysicalFormat:
            case kAudioStreamPropertyAvailableVirtualFormats:
            case kAudioStreamPropertyAvailablePhysicalFormats:
                return true;
            default: break;
        }
    }
    return false;
}

static OSStatus pks_IsPropertySettable(AudioServerPlugInDriverRef inDriver,
                                        AudioObjectID inObjectID,
                                        pid_t inClientPID,
                                        const AudioObjectPropertyAddress* inAddress,
                                        Boolean* outIsSettable) {
    (void)inDriver; (void)inClientPID;
    *outIsSettable = false;
    if (inObjectID == kObjectID_Device) {
        if (inAddress->mSelector == kAudioDevicePropertyNominalSampleRate)
            *outIsSettable = true;
        if (inAddress->mSelector == kAudioObjectPropertyIdentify)
            *outIsSettable = true;
    }
    if (inObjectID == kObjectID_Stream_Input || inObjectID == kObjectID_Stream_Output) {
        if (inAddress->mSelector == kAudioStreamPropertyVirtualFormat ||
            inAddress->mSelector == kAudioStreamPropertyPhysicalFormat)
            *outIsSettable = true;
    }
    return kAudioHardwareNoError;
}

static OSStatus pks_GetPropertyDataSize(AudioServerPlugInDriverRef inDriver,
                                         AudioObjectID inObjectID,
                                         pid_t inClientPID,
                                         const AudioObjectPropertyAddress* inAddress,
                                         UInt32 inQualifierDataSize,
                                         const void* inQualifierData,
                                         UInt32* outDataSize) {
    (void)inDriver; (void)inClientPID; (void)inQualifierDataSize; (void)inQualifierData;
    OSStatus err = kAudioHardwareNoError;

    if (inObjectID == kObjectID_PlugIn) {
        switch (inAddress->mSelector) {
            case kAudioObjectPropertyBaseClass:
            case kAudioObjectPropertyClass:       *outDataSize = sizeof(AudioClassID); break;
            case kAudioObjectPropertyOwner:       *outDataSize = sizeof(AudioObjectID); break;
            case kAudioObjectPropertyManufacturer: *outDataSize = sizeof(CFStringRef); break;
            case kAudioObjectPropertyOwnedObjects: *outDataSize = sizeof(AudioObjectID); break;
            case kAudioPlugInPropertyBoxList:      *outDataSize = 0; break;
            case kAudioPlugInPropertyTranslateUIDToBox: *outDataSize = sizeof(AudioObjectID); break;
            case kAudioPlugInPropertyDeviceList:   *outDataSize = sizeof(AudioObjectID); break;
            case kAudioPlugInPropertyTranslateUIDToDevice: *outDataSize = sizeof(AudioObjectID); break;
            case kAudioPlugInPropertyResourceBundle: *outDataSize = sizeof(CFStringRef); break;
            default: err = kAudioHardwareUnknownPropertyError; break;
        }
    } else if (inObjectID == kObjectID_Device) {
        switch (inAddress->mSelector) {
            case kAudioObjectPropertyBaseClass:
            case kAudioObjectPropertyClass:       *outDataSize = sizeof(AudioClassID); break;
            case kAudioObjectPropertyOwner:
            case kAudioDevicePropertyClockDomain:
            case kAudioDevicePropertyTransportType:
            case kAudioDevicePropertyLatency:
            case kAudioDevicePropertySafetyOffset:
            case kAudioDevicePropertyZeroTimeStampPeriod: *outDataSize = sizeof(UInt32); break;
            case kAudioObjectPropertyName:
            case kAudioObjectPropertyManufacturer:
            case kAudioObjectPropertyModelName:
            case kAudioObjectPropertySerialNumber:
            case kAudioObjectPropertyFirmwareVersion:
            case kAudioDevicePropertyDeviceUID:
            case kAudioDevicePropertyModelUID:
            case kAudioDevicePropertyIcon:        *outDataSize = sizeof(CFStringRef); break;
            case kAudioDevicePropertyDeviceIsAlive:
            case kAudioDevicePropertyDeviceIsRunning:
            case kAudioDevicePropertyDeviceCanBeDefaultDevice:
            case kAudioDevicePropertyDeviceCanBeDefaultSystemDevice:
            case kAudioDevicePropertyIsHidden:
            case kAudioObjectPropertyIdentify:    *outDataSize = sizeof(UInt32); break;
            case kAudioDevicePropertyStreams:      *outDataSize = 2 * sizeof(AudioObjectID); break;
            case kAudioDevicePropertyPreferredChannelsForStereo: *outDataSize = 2 * sizeof(UInt32); break;
            case kAudioDevicePropertyPreferredChannelLayout: {
                *outDataSize = (UInt32)(offsetof(AudioChannelLayout, mChannelDescriptions) +
                    PKS_MAX_CHANNELS * sizeof(AudioChannelDescription));
                break;
            }
            case kAudioDevicePropertyNominalSampleRate: *outDataSize = sizeof(Float64); break;
            case kAudioDevicePropertyAvailableNominalSampleRates:
                *outDataSize = kNumSampleRates * sizeof(AudioValueRange); break;
            case kAudioObjectPropertyOwnedObjects:
                *outDataSize = 2 * sizeof(AudioObjectID); break;
            case kAudioDevicePropertyRelatedDevices: *outDataSize = sizeof(AudioObjectID); break;
            default: err = kAudioHardwareUnknownPropertyError; break;
        }
    } else if (inObjectID == kObjectID_Stream_Input || inObjectID == kObjectID_Stream_Output) {
        switch (inAddress->mSelector) {
            case kAudioObjectPropertyBaseClass:
            case kAudioObjectPropertyClass:       *outDataSize = sizeof(AudioClassID); break;
            case kAudioObjectPropertyOwner:
            case kAudioStreamPropertyDirection:
            case kAudioStreamPropertyTerminalType:
            case kAudioStreamPropertyStartingChannel:
            case kAudioStreamPropertyLatency:
            case kAudioStreamPropertyIsActive:    *outDataSize = sizeof(UInt32); break;
            case kAudioObjectPropertyOwnedObjects: *outDataSize = 0; break;
            case kAudioStreamPropertyVirtualFormat:
            case kAudioStreamPropertyPhysicalFormat: *outDataSize = sizeof(AudioStreamBasicDescription); break;
            case kAudioStreamPropertyAvailableVirtualFormats:
            case kAudioStreamPropertyAvailablePhysicalFormats:
                *outDataSize = kNumSampleRates * sizeof(AudioStreamRangedDescription); break;
            default: err = kAudioHardwareUnknownPropertyError; break;
        }
    } else {
        err = kAudioHardwareBadObjectError;
    }
    return err;
}

static OSStatus pks_GetPropertyData(AudioServerPlugInDriverRef inDriver,
                                     AudioObjectID inObjectID,
                                     pid_t inClientPID,
                                     const AudioObjectPropertyAddress* inAddress,
                                     UInt32 inQualifierDataSize,
                                     const void* inQualifierData,
                                     UInt32 inDataSize,
                                     UInt32* outDataSize,
                                     void* outData) {
    (void)inClientPID; (void)inQualifierDataSize; (void)inDataSize;
    PksDriver* drv = (PksDriver*)inDriver;
    PksDevice* dev = &drv->device;
    OSStatus err = kAudioHardwareNoError;

#define RETURN_CFSTR(s) do { \
    *(CFStringRef*)outData = CFStringCreateWithCString(NULL, (s), kCFStringEncodingUTF8); \
    *outDataSize = sizeof(CFStringRef); \
} while(0)

#define RETURN_U32(v) do { *(UInt32*)outData = (UInt32)(v); *outDataSize = sizeof(UInt32); } while(0)
#define RETURN_F64(v) do { *(Float64*)outData = (Float64)(v); *outDataSize = sizeof(Float64); } while(0)
#define RETURN_OID(v) do { *(AudioObjectID*)outData = (AudioObjectID)(v); *outDataSize = sizeof(AudioObjectID); } while(0)
#define RETURN_CID(v) do { *(AudioClassID*)outData = (AudioClassID)(v); *outDataSize = sizeof(AudioClassID); } while(0)

    if (inObjectID == kObjectID_PlugIn) {
        switch (inAddress->mSelector) {
            case kAudioObjectPropertyBaseClass: RETURN_CID(kAudioObjectClassID); break;
            case kAudioObjectPropertyClass:     RETURN_CID(kAudioPlugInClassID); break;
            case kAudioObjectPropertyOwner:     RETURN_OID(kAudioObjectPlugInObject); break;
            case kAudioObjectPropertyManufacturer: RETURN_CFSTR(kDevice_Manufacturer); break;
            case kAudioObjectPropertyOwnedObjects: {
                *(AudioObjectID*)outData = kObjectID_Device;
                *outDataSize = sizeof(AudioObjectID);
                break;
            }
            case kAudioPlugInPropertyBoxList:      *outDataSize = 0; break;
            case kAudioPlugInPropertyTranslateUIDToBox: RETURN_OID(kAudioObjectUnknown); break;
            case kAudioPlugInPropertyDeviceList:   RETURN_OID(kObjectID_Device); break;
            case kAudioPlugInPropertyTranslateUIDToDevice: {
                if (inQualifierDataSize == sizeof(CFStringRef)) {
                    CFStringRef uid = *(CFStringRef*)inQualifierData;
                    CFStringRef myUID = CFStringCreateWithCString(NULL, kDevice_UID, kCFStringEncodingUTF8);
                    *(AudioObjectID*)outData = CFEqual(uid, myUID) ? kObjectID_Device : kAudioObjectUnknown;
                    CFRelease(myUID);
                    *outDataSize = sizeof(AudioObjectID);
                }
                break;
            }
            case kAudioPlugInPropertyResourceBundle: RETURN_CFSTR(""); break;
            default: err = kAudioHardwareUnknownPropertyError; break;
        }
    } else if (inObjectID == kObjectID_Device) {
        switch (inAddress->mSelector) {
            case kAudioObjectPropertyBaseClass:  RETURN_CID(kAudioObjectClassID); break;
            case kAudioObjectPropertyClass:      RETURN_CID(kAudioDeviceClassID); break;
            case kAudioObjectPropertyOwner:      RETURN_OID(kObjectID_PlugIn); break;
            case kAudioObjectPropertyName:       RETURN_CFSTR(kDevice_Name); break;
            case kAudioObjectPropertyManufacturer: RETURN_CFSTR(kDevice_Manufacturer); break;
            case kAudioObjectPropertyModelName:  RETURN_CFSTR(kDevice_Name); break;
            case kAudioObjectPropertySerialNumber: RETURN_CFSTR("00000001"); break;
            case kAudioObjectPropertyFirmwareVersion: RETURN_CFSTR("1.0.0"); break;
            case kAudioObjectPropertyIdentify:   RETURN_U32(0); break;
            case kAudioObjectPropertyOwnedObjects: {
                AudioObjectID* ids = (AudioObjectID*)outData;
                ids[0] = kObjectID_Stream_Output;
                ids[1] = kObjectID_Stream_Input;
                *outDataSize = 2 * sizeof(AudioObjectID);
                break;
            }
            case kAudioDevicePropertyDeviceUID:  RETURN_CFSTR(kDevice_UID); break;
            case kAudioDevicePropertyModelUID:   RETURN_CFSTR(kDevice_ModelUID); break;
            case kAudioDevicePropertyTransportType: RETURN_U32(kAudioDeviceTransportTypeVirtual); break;
            case kAudioDevicePropertyClockDomain:   RETURN_U32(0); break;
            case kAudioDevicePropertyDeviceIsAlive: RETURN_U32(1); break;
            case kAudioDevicePropertyDeviceIsRunning:
                RETURN_U32(atomic_load_explicit(&dev->ioClientCount, memory_order_relaxed) > 0 ? 1u : 0u);
                break;
            case kAudioDevicePropertyDeviceCanBeDefaultDevice: RETURN_U32(1); break;
            case kAudioDevicePropertyDeviceCanBeDefaultSystemDevice: RETURN_U32(1); break;
            case kAudioDevicePropertyLatency:    RETURN_U32(0); break;
            case kAudioDevicePropertySafetyOffset: RETURN_U32(0); break;
            case kAudioDevicePropertyZeroTimeStampPeriod: RETURN_U32(PKS_RING_FRAMES); break;
            case kAudioDevicePropertyStreams: {
                AudioObjectID* ids = (AudioObjectID*)outData;
                ids[0] = kObjectID_Stream_Output;
                ids[1] = kObjectID_Stream_Input;
                *outDataSize = 2 * sizeof(AudioObjectID);
                break;
            }
            case kAudioDevicePropertyIsHidden:   RETURN_U32(0); break;
            case kAudioDevicePropertyPreferredChannelsForStereo: {
                UInt32* ch = (UInt32*)outData;
                ch[0] = 1; ch[1] = 2;
                *outDataSize = 2 * sizeof(UInt32);
                break;
            }
            case kAudioDevicePropertyPreferredChannelLayout: {
                UInt32 sz = (UInt32)(offsetof(AudioChannelLayout, mChannelDescriptions) +
                    PKS_MAX_CHANNELS * sizeof(AudioChannelDescription));
                AudioChannelLayout* layout = (AudioChannelLayout*)outData;
                memset(layout, 0, sz);
                layout->mChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
                *outDataSize = sz;
                break;
            }
            case kAudioDevicePropertyNominalSampleRate: RETURN_F64(dev->sampleRate); break;
            case kAudioDevicePropertyAvailableNominalSampleRates: {
                AudioValueRange* ranges = (AudioValueRange*)outData;
                for (UInt32 i = 0; i < kNumSampleRates; i++) {
                    ranges[i].mMinimum = kSampleRates[i];
                    ranges[i].mMaximum = kSampleRates[i];
                }
                *outDataSize = kNumSampleRates * sizeof(AudioValueRange);
                break;
            }
            case kAudioDevicePropertyRelatedDevices: RETURN_OID(kObjectID_Device); break;
            case kAudioDevicePropertyIcon: RETURN_CFSTR(""); break;
            default: err = kAudioHardwareUnknownPropertyError; break;
        }
    } else if (inObjectID == kObjectID_Stream_Input || inObjectID == kObjectID_Stream_Output) {
        Boolean isInput = (inObjectID == kObjectID_Stream_Input);
        switch (inAddress->mSelector) {
            case kAudioObjectPropertyBaseClass:  RETURN_CID(kAudioObjectClassID); break;
            case kAudioObjectPropertyClass:      RETURN_CID(kAudioStreamClassID); break;
            case kAudioObjectPropertyOwner:      RETURN_OID(kObjectID_Device); break;
            case kAudioObjectPropertyOwnedObjects: *outDataSize = 0; break;
            case kAudioStreamPropertyIsActive:   RETURN_U32(1); break;
            case kAudioStreamPropertyDirection:  RETURN_U32(isInput ? 1u : 0u); break;
            case kAudioStreamPropertyTerminalType:
                RETURN_U32(isInput ? (UInt32)kAudioStreamTerminalTypeMicrophone
                                   : (UInt32)kAudioStreamTerminalTypeSpeaker);
                break;
            case kAudioStreamPropertyStartingChannel: RETURN_U32(1); break;
            case kAudioStreamPropertyLatency:    RETURN_U32(0); break;
            case kAudioStreamPropertyVirtualFormat:
            case kAudioStreamPropertyPhysicalFormat: {
                AudioStreamBasicDescription fmt = pks_stream_format(dev->sampleRate);
                *(AudioStreamBasicDescription*)outData = fmt;
                *outDataSize = sizeof(AudioStreamBasicDescription);
                break;
            }
            case kAudioStreamPropertyAvailableVirtualFormats:
            case kAudioStreamPropertyAvailablePhysicalFormats: {
                AudioStreamRangedDescription* descs = (AudioStreamRangedDescription*)outData;
                for (UInt32 i = 0; i < kNumSampleRates; i++) {
                    descs[i].mFormat = pks_stream_format(kSampleRates[i]);
                    descs[i].mSampleRateRange.mMinimum = kSampleRates[i];
                    descs[i].mSampleRateRange.mMaximum = kSampleRates[i];
                }
                *outDataSize = kNumSampleRates * sizeof(AudioStreamRangedDescription);
                break;
            }
            default: err = kAudioHardwareUnknownPropertyError; break;
        }
    } else {
        err = kAudioHardwareBadObjectError;
    }

#undef RETURN_CFSTR
#undef RETURN_U32
#undef RETURN_F64
#undef RETURN_OID
#undef RETURN_CID

    return err;
}

static OSStatus pks_SetPropertyData(AudioServerPlugInDriverRef inDriver,
                                     AudioObjectID inObjectID,
                                     pid_t inClientPID,
                                     const AudioObjectPropertyAddress* inAddress,
                                     UInt32 inQualifierDataSize,
                                     const void* inQualifierData,
                                     UInt32 inDataSize,
                                     const void* inData) {
    (void)inClientPID; (void)inQualifierDataSize; (void)inQualifierData; (void)inDataSize;
    PksDriver* drv = (PksDriver*)inDriver;
    PksDevice* dev = &drv->device;

    if (inObjectID == kObjectID_Device &&
        inAddress->mSelector == kAudioDevicePropertyNominalSampleRate)
    {
        Float64 newRate = *(const Float64*)inData;
        Boolean valid = false;
        for (UInt32 i = 0; i < kNumSampleRates; i++) {
            if (newRate == kSampleRates[i]) { valid = true; break; }
        }
        if (!valid) return kAudioHardwareIllegalOperationError;

        dev->requestedSampleRate = newRate;
        // Must dispatch asynchronously — SetPropertyData may be on HAL client thread
        dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
            drv->host->RequestDeviceConfigurationChange(drv->host,
                kObjectID_Device, (UInt64)kAudioDevicePropertyNominalSampleRate, NULL);
        });
        return kAudioHardwareNoError;
    }

    if ((inObjectID == kObjectID_Stream_Input || inObjectID == kObjectID_Stream_Output) &&
        (inAddress->mSelector == kAudioStreamPropertyVirtualFormat ||
         inAddress->mSelector == kAudioStreamPropertyPhysicalFormat))
    {
        const AudioStreamBasicDescription* fmt = (const AudioStreamBasicDescription*)inData;
        Boolean validRate = false;
        for (UInt32 i = 0; i < kNumSampleRates; i++) {
            if (fmt->mSampleRate == kSampleRates[i]) { validRate = true; break; }
        }
        if (!validRate) return kAudioHardwareIllegalOperationError;
        if (fmt->mFormatID != kAudioFormatLinearPCM) return kAudioHardwareIllegalOperationError;

        dev->requestedSampleRate = fmt->mSampleRate;
        dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
            drv->host->RequestDeviceConfigurationChange(drv->host,
                kObjectID_Device, (UInt64)kAudioDevicePropertyNominalSampleRate, NULL);
        });
        return kAudioHardwareNoError;
    }

    return kAudioHardwareUnknownPropertyError;
}

// ---------------------------------------------------------------------------
// PerformDeviceConfigurationChange — applies requested sample rate
// ---------------------------------------------------------------------------

static OSStatus pks_PerformDeviceConfigurationChange(AudioServerPlugInDriverRef inDriver,
                                                      AudioObjectID inDeviceObjectID,
                                                      UInt64 inChangeAction,
                                                      void* inChangeInfo) {
    (void)inChangeInfo;
    PksDevice* dev = &((PksDriver*)inDriver)->device;
    if (dev->deviceID != inDeviceObjectID) return kAudioHardwareBadObjectError;

    if ((AudioObjectPropertySelector)inChangeAction == kAudioDevicePropertyNominalSampleRate) {
        dev->sampleRate = dev->requestedSampleRate;
        if (dev->ring) {
            atomic_store_explicit(&dev->ring->sample_rate,
                (uint32_t)dev->sampleRate, memory_order_release);
        }
        // Reset ZTS timing for new rate
        atomic_store_explicit(&dev->tsAnchorHostTime, mach_absolute_time(), memory_order_release);
        atomic_store_explicit(&dev->tsFrameCounter, 0u, memory_order_release);
        atomic_fetch_add_explicit(&dev->tsSeed, 1u, memory_order_release);
    }
    return kAudioHardwareNoError;
}

static OSStatus pks_AbortDeviceConfigurationChange(AudioServerPlugInDriverRef inDriver,
                                                    AudioObjectID inDeviceObjectID,
                                                    UInt64 inChangeAction,
                                                    void* inChangeInfo) {
    (void)inDriver; (void)inDeviceObjectID; (void)inChangeAction; (void)inChangeInfo;
    return kAudioHardwareNoError;
}

// ---------------------------------------------------------------------------
// Driver factory
// ---------------------------------------------------------------------------

static void pks_vtable_init(PksDriver* drv) {
    drv->vtable._reserved                       = NULL;
    drv->vtable.QueryInterface                  = pks_QueryInterface;
    drv->vtable.AddRef                          = pks_AddRef;
    drv->vtable.Release                         = pks_Release;
    drv->vtable.Initialize                      = pks_Initialize;
    drv->vtable.CreateDevice                    = pks_CreateDevice;
    drv->vtable.DestroyDevice                   = pks_DestroyDevice;
    drv->vtable.AddDeviceClient                 = pks_AddDeviceClient;
    drv->vtable.RemoveDeviceClient              = pks_RemoveDeviceClient;
    drv->vtable.PerformDeviceConfigurationChange = pks_PerformDeviceConfigurationChange;
    drv->vtable.AbortDeviceConfigurationChange  = pks_AbortDeviceConfigurationChange;
    drv->vtable.HasProperty                     = pks_HasProperty;
    drv->vtable.IsPropertySettable              = pks_IsPropertySettable;
    drv->vtable.GetPropertyDataSize             = pks_GetPropertyDataSize;
    drv->vtable.GetPropertyData                 = pks_GetPropertyData;
    drv->vtable.SetPropertyData                 = pks_SetPropertyData;
    drv->vtable.StartIO                         = pks_StartIO;
    drv->vtable.StopIO                          = pks_StopIO;
    drv->vtable.GetZeroTimeStamp                = pks_GetZeroTimeStamp;
    drv->vtable.WillDoIOOperation               = pks_WillDoIOOperation;
    drv->vtable.BeginIOOperation                = pks_BeginIOOperation;
    drv->vtable.DoIOOperation                   = pks_DoIOOperation;
    drv->vtable.EndIOOperation                  = pks_EndIOOperation;
    drv->interface = &drv->vtable;
}

void* AudioServerPlugInDriverFactory(CFAllocatorRef inAllocator, CFUUIDRef inTypeUUID) {
    (void)inAllocator;
    // kAudioServerPlugInTypeUUID — 443ABAB8-E7B3-491A-B985-BEB9187030DB
    CFUUIDRef typeUUID = CFUUIDGetConstantUUIDWithBytes(NULL,
        0x44, 0x3A, 0xBA, 0xB8, 0xE7, 0xB3, 0x49, 0x1A,
        0xB9, 0x85, 0xBE, 0xB9, 0x18, 0x70, 0x30, 0xDB);
    if (!CFEqual(inTypeUUID, typeUUID)) return NULL;

    memset(&gDriver, 0, sizeof(gDriver));
    pks_vtable_init(&gDriver);
    atomic_store(&gDriver.refCount, 1u);
    return &gDriver;
}
