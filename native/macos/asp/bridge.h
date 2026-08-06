#pragma once
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif

// Opaque reader handle
typedef struct PksReader PksReader;

// Returns 1 if the PocketStation HAL plugin is currently running (shm exists), 0 otherwise.
int          pks_asp_is_installed(void);

// Open the shared memory ring for reading. Returns NULL if the plugin is not
// running or another reader already owns the single-consumer ring.
PksReader*   pks_asp_open_reader(void);

// Current sample rate reported by the plugin (default 48000 if unavailable).
uint32_t     pks_asp_sample_rate(PksReader* r);

// Channel count (always 2 for PocketStation Loopback).
uint32_t     pks_asp_channels(PksReader* r);

// Cumulative number of frames rejected before publication.
uint64_t     pks_asp_drop_count(PksReader* r);

// Cumulative number of callbacks rejected for invalid native timeline data.
uint64_t     pks_asp_timeline_reject_callback_count(PksReader* r);

// Read up to frame_count interleaved f32 frames into out_buf.
// Returns frames actually read (0 if ring is empty). A returned batch never
// crosses a native source-position discontinuity.
uint32_t     pks_asp_read_frames(
    PksReader* r,
    float* out_buf,
    uint32_t frame_count,
    uint64_t* out_source_frame_position);

// Close and free the reader. Safe to call with NULL.
void         pks_asp_close_reader(PksReader* r);

#ifdef __cplusplus
}
#endif
