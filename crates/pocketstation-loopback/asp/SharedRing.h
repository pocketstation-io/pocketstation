#pragma once
#include <stdint.h>
#include <stdatomic.h>

/* Shared memory name — both plugin and pks must agree on this. */
#define PKS_SHM_NAME        "/pocketstation-loopback-v1"

/* Power-of-2 ring: 65536 frames × 2 channels × 4 bytes = 512 KB. */
#define PKS_RING_FRAMES     65536u
#define PKS_RING_MASK       (PKS_RING_FRAMES - 1u)
#define PKS_MAX_CHANNELS    2u

/* Cache-line size to pad atomics to prevent false sharing. */
#define PKS_CACHE_LINE      64u

/*
 * Layout of the POSIX shared memory region.
 *
 * The region is created by the plugin in coreaudiod and opened read-only
 * by the pks capture process. Total size = sizeof(PksLoopbackRing).
 *
 * write_head: absolute frame index of the next slot the plugin will write.
 *             Written only by the plugin (single producer).
 * drop_count: incremented by the plugin when the ring is full and a frame
 *             is dropped. Written only by the plugin.
 * sample_rate / channels: set once in StartIO, read by pks after open.
 *
 * Audio data layout: interleaved f32, channels = sample_rate/channels field.
 * Frame N occupies data[(N & PKS_RING_MASK) * channels .. +channels].
 */
typedef struct {
    _Atomic uint64_t    write_head;             /* +0   producer only */
    uint8_t             _pad0[PKS_CACHE_LINE - sizeof(uint64_t)];

    _Atomic uint64_t    drop_count;             /* +64  producer only */
    uint8_t             _pad1[PKS_CACHE_LINE - sizeof(uint64_t)];

    _Atomic uint32_t    sample_rate;            /* +128 set in StartIO */
    _Atomic uint32_t    channels;               /* +132 set in StartIO */
    _Atomic uint32_t    io_running;             /* +136 1 when IO active */
    uint8_t             _pad2[PKS_CACHE_LINE - 3*sizeof(uint32_t)];

    /* Audio data: PKS_RING_FRAMES * PKS_MAX_CHANNELS floats. */
    float               data[PKS_RING_FRAMES * PKS_MAX_CHANNELS]; /* +192 */
} PksLoopbackRing;

/* Total shared memory size. */
#define PKS_SHM_SIZE        sizeof(PksLoopbackRing)
