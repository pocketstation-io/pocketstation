// source_discovery.h — C API for CoreAudio process tap capture.
// macOS 14.2+ only; pks_process_tap_available() returns 0 on older systems.
#pragma once
#include <stdint.h>

typedef enum {
    PKS_SOURCE_KIND_APPLICATION   = 0,
    PKS_SOURCE_KIND_INPUT_DEVICE  = 1,
    PKS_SOURCE_KIND_OUTPUT_DEVICE = 2,
    PKS_SOURCE_KIND_SYSTEM_MIX    = 3,
} PksSourceKind;

typedef enum {
    PKS_SOURCE_STATE_AVAILABLE    = 0,
    PKS_SOURCE_STATE_PLAYING      = 1,
    PKS_SOURCE_STATE_SILENT       = 2,
    PKS_SOURCE_STATE_UNAVAILABLE  = 3,
} PksSourceState;

typedef struct {
    uint32_t  audio_object_id;
    int32_t   pid;
    char      bundle_id[256];
    char      name[256];
    uint8_t   kind;
    uint8_t   state;
    uint32_t  sample_rate;
    uint16_t  channels;
    uint64_t  process_start_time_ns;
} PksCaptureSourceInfo;

// Returns 1 if the process tap API is available (macOS 14.2+), 0 otherwise.
int pks_process_tap_available(void);

// Returns the process start time for one live PID, or zero after that process
// instance exits. Call only from a control or reader thread, never an audio
// callback.
uint64_t pks_process_start_time_ns(int32_t process_id);

// Enumerate live audio source processes. Returns count written (≤ max).
int pks_discover_sources(PksCaptureSourceInfo *out, int max);

// Borrowed only during one synchronous control call. Never stored in a tap,
// ring, IOProc or retained native resource. A zero call ordinal omits a marker.
typedef struct {
    uint64_t (*begin)(void *, uint32_t operation);
    void (*end)(void *, uint64_t call, int32_t status, uint8_t has_status);
    void *context;
} PksNativeCallObserver;

// Static tags mirror NativeCallOperation; no native/private identity is passed.
typedef enum {
    PKS_NATIVE_CALL_SOURCE_DEVICE_DISCOVERY = 1,
    PKS_NATIVE_CALL_DEVICE_LIST_SIZE = 2,
    PKS_NATIVE_CALL_DEVICE_LIST_DATA = 3,
    PKS_NATIVE_CALL_DEVICE_STREAMS_SIZE = 4,
    PKS_NATIVE_CALL_DEVICE_STREAMS_DATA = 5,
    PKS_NATIVE_CALL_DEVICE_UID = 6,
    PKS_NATIVE_CALL_DEVICE_NAME = 7,
    PKS_NATIVE_CALL_DEVICE_SAMPLE_RATE = 8,
    PKS_NATIVE_CALL_PROCESS_LIST_SIZE = 9,
    PKS_NATIVE_CALL_PROCESS_LIST_DATA = 10,
    PKS_NATIVE_CALL_PROCESS_ID = 11,
    PKS_NATIVE_CALL_PROCESS_BUNDLE_ID = 12,
    PKS_NATIVE_CALL_PROCESS_RUNNING_OUTPUT = 13,
    PKS_NATIVE_CALL_PROCESS_START_TIME = 14,
    PKS_NATIVE_CALL_PROCESS_NAME = 15,
    PKS_NATIVE_CALL_APPLICATION_LABEL = 16,
    PKS_NATIVE_CALL_RESOLVE_PROCESS_LIST_SIZE = 17,
    PKS_NATIVE_CALL_RESOLVE_PROCESS_LIST_DATA = 18,
    PKS_NATIVE_CALL_RESOLVE_PROCESS_ID = 19,
    PKS_NATIVE_CALL_CREATE_PROCESS_TAP = 20,
    PKS_NATIVE_CALL_TAP_UID = 21,
    PKS_NATIVE_CALL_CREATE_AGGREGATE_DEVICE = 22,
    PKS_NATIVE_CALL_STREAM_FORMAT = 23,
    PKS_NATIVE_CALL_IO_BUFFER_SIZE_BEFORE = 24,
    PKS_NATIVE_CALL_IO_BUFFER_SIZE_RANGE = 25,
    PKS_NATIVE_CALL_IO_BUFFER_SIZE_SETTABLE = 26,
    PKS_NATIVE_CALL_IO_BUFFER_SIZE_SET = 27,
    PKS_NATIVE_CALL_IO_BUFFER_SIZE_APPLIED = 28,
    PKS_NATIVE_CALL_REGISTER_IO_PROC = 29,
    PKS_NATIVE_CALL_START_DEVICE = 30,
    PKS_NATIVE_CALL_STOP_DEVICE = 31,
    PKS_NATIVE_CALL_UNREGISTER_IO_PROC = 32,
    PKS_NATIVE_CALL_DESTROY_AGGREGATE_DEVICE = 33,
    PKS_NATIVE_CALL_DESTROY_PROCESS_TAP = 34,
    PKS_NATIVE_CALL_DEVICE_LATENCY = 35,
    PKS_NATIVE_CALL_SAFETY_OFFSET = 36,
    PKS_NATIVE_CALL_SAFETY_OFFSET_SETTABLE = 37,
    PKS_NATIVE_CALL_STREAM_LIST_SIZE = 38,
    PKS_NATIVE_CALL_STREAM_LIST_DATA = 39,
    PKS_NATIVE_CALL_STREAM_LATENCY = 40,
    PKS_NATIVE_CALL_GLOBAL_TAP_DESCRIPTION = 41,
    PKS_NATIVE_CALL_PROCESS_TAP_DESCRIPTION = 42,
    PKS_NATIVE_CALL_TAP_MUTE_BEHAVIOR = 43,
} PksNativeCallOperation;
int pks_discover_sources_observed(PksCaptureSourceInfo *, int, const PksNativeCallObserver *);
uint64_t pks_process_start_time_ns_observed(int32_t, const PksNativeCallObserver *);

typedef struct PksProcessTapHandle PksProcessTapHandle;

typedef enum PksTapOperationStage {
    PKS_TAP_STAGE_NONE = 0,
    PKS_TAP_STAGE_RESOLVE_PROCESS = 1,
    PKS_TAP_STAGE_CREATE_PROCESS_TAP = 2,
    PKS_TAP_STAGE_READ_TAP_UID = 3,
    PKS_TAP_STAGE_CREATE_AGGREGATE_DEVICE = 4,
    PKS_TAP_STAGE_ALLOCATE_HANDLE = 5,
    PKS_TAP_STAGE_CREATE_IO_PROC = 6,
    PKS_TAP_STAGE_START_DEVICE = 7,
    PKS_TAP_STAGE_PLATFORM_SUPPORT = 8,
    PKS_TAP_STAGE_STOP_DEVICE = 9,
    PKS_TAP_STAGE_DESTROY_IO_PROC = 10,
    PKS_TAP_STAGE_DESTROY_AGGREGATE = 11,
    PKS_TAP_STAGE_DESTROY_TAP = 12,
    PKS_TAP_STAGE_REGISTRATION_UNCERTAIN = 13,
} PksTapOperationStage;

// Control-thread ownership only. Operation callbacks permit deterministic
// fault tests to execute the same controller without invoking CoreAudio.
typedef struct {
    uint8_t registered, start_attempted, started, cleanup_attempted;
    uint8_t retained, aggregate_owned, tap_owned, registration_uncertain;
    int32_t cleanup_status;
    uint8_t cleanup_stage;
    uint8_t reserved[3];
} PksTapControl;
typedef struct {
    int32_t (*register_io)(void *);
    int32_t (*start)(void *);
    int32_t (*stop)(void *);
    int32_t (*unregister_io)(void *);
    int32_t (*destroy_aggregate)(void *);
    int32_t (*destroy_tap)(void *);
} PksTapControlOperations;
typedef int (*PksTapCancellationCheck)(void *);
int pks_tap_control_start(PksTapControl *, const PksTapControlOperations *, void *,
                         PksTapCancellationCheck, void *, int32_t *, uint8_t *);
// Success authorizes the caller to free context. Failure retains all remaining
// ownership. No operation is retried after an uncertain cleanup.
int pks_tap_control_cleanup(PksTapControl *, const PksTapControlOperations *, void *,
                           int32_t *, uint8_t *);
PksProcessTapHandle *pks_create_process_tap_checked(const int32_t *, int,
    int32_t *, uint8_t *, int32_t *, uint8_t *, PksTapCancellationCheck, void *);
int pks_tap_start_cancellable(PksProcessTapHandle *, uint16_t,
    PksTapCancellationCheck, void *, int32_t *, uint8_t *);
int pks_destroy_process_tap_checked(PksProcessTapHandle *, int32_t *, uint8_t *);

// Create a tap. pids=NULL / pid_count=0 → global system tap (all output).
// Returns NULL on failure or on macOS < 14.2.
PksProcessTapHandle *pks_create_process_tap(const int32_t *pids, int pid_count,
                                            int32_t *out_status, uint8_t *out_stage);

// Start capturing. The requested native IO duration is used to prefer a
// matching aggregate-device size when CoreAudio supports it. Returns 0 on
// success.
int pks_tap_start(PksProcessTapHandle *tap, uint16_t requested_io_duration_ms,
                  int32_t *out_status, uint8_t *out_stage);

// Destroy on confirmed cleanup only; uncertain cleanup retains the handle.
void pks_destroy_process_tap(PksProcessTapHandle *tap);

// Read up to frame_count interleaved f32 stereo frames. Returns frames read.
uint32_t pks_tap_read_frames(PksProcessTapHandle *tap, float *out, uint32_t frame_count);
uint32_t pks_tap_read_frames_timed(PksProcessTapHandle *tap, float *out,
                                   uint32_t frame_count,
                                   uint64_t *out_source_frame_position,
                                   uint64_t *out_anchor_frame_position,
                                   uint64_t *out_anchor_host_time_ns);
uint64_t pks_tap_drop_count(const PksProcessTapHandle *tap);
uint64_t pks_tap_current_host_time_ns(void);
uint32_t pks_tap_io_buffer_before_frames(const PksProcessTapHandle *tap);
uint32_t pks_tap_io_buffer_requested_frames(const PksProcessTapHandle *tap);
uint32_t pks_tap_io_buffer_applied_frames(const PksProcessTapHandle *tap);
uint32_t pks_tap_io_buffer_min_frames(const PksProcessTapHandle *tap);
uint32_t pks_tap_io_buffer_max_frames(const PksProcessTapHandle *tap);
uint32_t pks_tap_input_device_latency_frames(const PksProcessTapHandle *tap);
uint32_t pks_tap_input_safety_offset_frames(const PksProcessTapHandle *tap);
uint8_t pks_tap_input_safety_offset_settable(const PksProcessTapHandle *tap);
uint32_t pks_tap_input_stream_latency_frames(const PksProcessTapHandle *tap);

uint32_t pks_tap_sample_rate(const PksProcessTapHandle *tap);
uint32_t pks_tap_channels(const PksProcessTapHandle *tap);
float    pks_tap_level(const PksProcessTapHandle *tap);

// Observed variants execute the same production controller and native leaves.
int pks_tap_control_start_observed(PksTapControl *, const PksTapControlOperations *, void *,
    PksTapCancellationCheck, void *, int32_t *, uint8_t *, const PksNativeCallObserver *);
int pks_tap_control_cleanup_observed(PksTapControl *, const PksTapControlOperations *, void *,
    int32_t *, uint8_t *, const PksNativeCallObserver *);
PksProcessTapHandle *pks_create_process_tap_observed(const int32_t *, int,
    int32_t *, uint8_t *, int32_t *, uint8_t *, PksTapCancellationCheck, void *,
    const PksNativeCallObserver *);
int pks_tap_start_observed(PksProcessTapHandle *, uint16_t, PksTapCancellationCheck,
    void *, int32_t *, uint8_t *, const PksNativeCallObserver *);
int pks_destroy_process_tap_observed(PksProcessTapHandle *, int32_t *, uint8_t *,
    const PksNativeCallObserver *);
uint32_t pks_tap_input_device_latency_frames_observed(const PksProcessTapHandle *, const PksNativeCallObserver *);
uint32_t pks_tap_input_safety_offset_frames_observed(const PksProcessTapHandle *, const PksNativeCallObserver *);
uint8_t pks_tap_input_safety_offset_settable_observed(const PksProcessTapHandle *, const PksNativeCallObserver *);
uint32_t pks_tap_input_stream_latency_frames_observed(const PksProcessTapHandle *, const PksNativeCallObserver *);
