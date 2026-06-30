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
} PksCaptureSourceInfo;

// Returns 1 if the process tap API is available (macOS 14.2+), 0 otherwise.
int pks_process_tap_available(void);

// Enumerate live audio source processes. Returns count written (≤ max).
int pks_discover_sources(PksCaptureSourceInfo *out, int max);

typedef struct PksProcessTapHandle PksProcessTapHandle;

// Create a tap. pids=NULL / pid_count=0 → global system tap (all output).
// Returns NULL on failure or on macOS < 14.2.
PksProcessTapHandle *pks_create_process_tap(const int32_t *pids, int pid_count);

// Start capturing. Returns 0 on success.
int pks_tap_start(PksProcessTapHandle *tap);

// Destroy handle and release all CoreAudio resources.
void pks_destroy_process_tap(PksProcessTapHandle *tap);

// Read up to frame_count interleaved f32 stereo frames. Returns frames read.
uint32_t pks_tap_read_frames(PksProcessTapHandle *tap, float *out, uint32_t frame_count);

uint32_t pks_tap_sample_rate(const PksProcessTapHandle *tap);
uint32_t pks_tap_channels(const PksProcessTapHandle *tap);
float    pks_tap_level(const PksProcessTapHandle *tap);
