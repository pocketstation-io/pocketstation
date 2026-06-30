#pragma once
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif

// Opaque reader handle
typedef struct PksReader PksReader;

// Returns 1 if the PocketStation HAL plugin is currently running (shm exists), 0 otherwise.
int          pks_asp_is_installed(void);

// Open the shared memory ring for reading. Returns NULL if plugin not running.
PksReader*   pks_asp_open_reader(void);

// Current sample rate reported by the plugin (default 48000 if unavailable).
uint32_t     pks_asp_sample_rate(PksReader* r);

// Channel count (always 2 for PocketStation Loopback).
uint32_t     pks_asp_channels(PksReader* r);

// Number of frames dropped by the plugin due to ring overrun.
uint64_t     pks_asp_drop_count(PksReader* r);

// Read up to frame_count interleaved f32 frames into out_buf.
// Returns frames actually read (0 if ring is empty).
uint32_t     pks_asp_read_frames(PksReader* r, float* out_buf, uint32_t frame_count);

// Close and free the reader. Safe to call with NULL.
void         pks_asp_close_reader(PksReader* r);

#ifdef __cplusplus
}
#endif
