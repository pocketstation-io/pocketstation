#pragma once
#include <stdint.h>
#include <stdatomic.h>

#if ATOMIC_INT_LOCK_FREE != 2 \
    || ATOMIC_LONG_LOCK_FREE != 2 \
    || ATOMIC_LLONG_LOCK_FREE != 2
#error "PocketStation ASP requires always-lock-free 32-bit and 64-bit shared atomics"
#endif

/* Shared memory name — both plugin and pks must agree on this. */
#define PKS_SHM_NAME        "/pocketstation-loopback-v3"
#define PKS_RING_ABI_MAGIC  0x504b5352u
#define PKS_RING_ABI_VERSION 3u

/* Power-of-2 ring: 65536 frames × 2 channels × 4 bytes = 512 KB. */
#define PKS_RING_FRAMES     65536u
#define PKS_RING_MASK       (PKS_RING_FRAMES - 1u)
#define PKS_MAX_CHANNELS    2u

/* Cache-line size to pad atomics to prevent false sharing. */
#define PKS_CACHE_LINE      64u

/*
 * Layout of the POSIX shared memory region.
 *
 * The region is created by the plugin in coreaudiod and opened read/write
 * by the single pks capture reader. Total size = sizeof(PksLoopbackRing).
 *
 * write_head: absolute frame index of the next slot the plugin will write.
 *             Written only by the plugin (single producer).
 * read_head: absolute frame index of the next slot the pks reader will read.
 *            Written only by the attached reader (single consumer).
 * reader_attached: one while a pks reader owns read_head. A second reader is
 *                  rejected so the producer always observes one authority.
 * drop_count: number of frames rejected before publication because writing
 *             would overwrite unread reader-owned data or the HAL buffer was
 *             already stale.
 * timeline_reject_callback_count: number of whole callbacks rejected because
 *             their native source-frame position was invalid or moved
 *             backwards. Their frames are included once in drop_count.
 * sample_rate / channels: set once in StartIO, read by pks after open.
 *
 * Audio data layout: interleaved f32, channels = sample_rate/channels field.
 * Frame N occupies data[(N & PKS_RING_MASK) * channels .. +channels].
 * source_frame_positions uses the same slot and publication edge as audio, so
 * its capacity cannot diverge from the audio ring.
 */
typedef struct {
    _Atomic uint64_t    write_head;             /* +0   producer only */
    uint8_t             _pad0[PKS_CACHE_LINE - sizeof(uint64_t)];

    _Atomic uint64_t    read_head;              /* +64  consumer only */
    uint8_t             _pad1[PKS_CACHE_LINE - sizeof(uint64_t)];

    _Atomic uint64_t    drop_count;             /* +128 producer only */
    _Atomic uint64_t    timeline_reject_callback_count; /* +136 producer */
    uint8_t             _pad2[PKS_CACHE_LINE - 2*sizeof(uint64_t)];

    _Atomic uint64_t    published_source_frame_end; /* +192 producer */
    _Atomic uint32_t    source_position_initialized; /* +200 producer */
    uint8_t             _pad3[
        PKS_CACHE_LINE - sizeof(uint64_t) - sizeof(uint32_t)];

    _Atomic uint32_t    sample_rate;            /* +256 set in StartIO */
    _Atomic uint32_t    channels;               /* +260 set in StartIO */
    _Atomic uint32_t    io_running;             /* +264 1 when IO active */
    _Atomic uint32_t    reader_attached;         /* +268 reader ownership */
    _Atomic uint32_t    abi_magic;              /* +272 */
    _Atomic uint32_t    abi_version;            /* +276 */
    _Atomic uint32_t    abi_struct_size;         /* +280 */
    uint8_t             _pad4[PKS_CACHE_LINE - 7*sizeof(uint32_t)];

    /* Native Core Audio sample-frame position for each corresponding slot. */
    uint64_t            source_frame_positions[PKS_RING_FRAMES]; /* +320 */

    /* Audio data: PKS_RING_FRAMES * PKS_MAX_CHANNELS floats. */
    float               data[PKS_RING_FRAMES * PKS_MAX_CHANNELS]; /* +524608 */
} PksLoopbackRing;

static inline int pks_ring_reject_invalid_timeline(
    PksLoopbackRing* ring,
    uint32_t frame_count)
{
    atomic_fetch_add_explicit(
        &ring->timeline_reject_callback_count,
        1u,
        memory_order_relaxed);
    atomic_fetch_add_explicit(
        &ring->drop_count,
        (uint64_t)frame_count,
        memory_order_relaxed);
    return 0;
}

/*
 * Publishes one interleaved stereo callback without overwriting unread data.
 *
 * The producer writes samples before the release-store to write_head. The
 * consumer acquire-loads write_head before reading samples and release-stores
 * read_head only after copying them. This is the sole sample visibility
 * protocol for the shared ring.
 *
 * Returns 1 when every frame is published. Returns 0 and counts every rejected
 * frame when the whole callback cannot fit. Partial callback publication is
 * forbidden.
 */
static inline int pks_ring_try_write(
    PksLoopbackRing* ring,
    const float* source,
    uint64_t source_frame_position,
    uint32_t frame_count)
{
    uint64_t write_head = atomic_load_explicit(
        &ring->write_head,
        memory_order_relaxed);
    int timeline_reject =
        write_head > UINT64_MAX - (uint64_t)frame_count
        || source_frame_position > UINT64_MAX - (uint64_t)frame_count;
    if (!timeline_reject && atomic_load_explicit(
            &ring->source_position_initialized,
            memory_order_relaxed) != 0u) {
        uint64_t published_source_frame_end = atomic_load_explicit(
            &ring->published_source_frame_end,
            memory_order_relaxed);
        timeline_reject =
            source_frame_position < published_source_frame_end;
    }
    if (timeline_reject) {
        return pks_ring_reject_invalid_timeline(ring, frame_count);
    }

    int reject = frame_count > PKS_RING_FRAMES;
    if (!reject && atomic_load_explicit(
            &ring->reader_attached,
            memory_order_acquire) != 0u) {
        uint64_t read_head = atomic_load_explicit(
            &ring->read_head,
            memory_order_acquire);
        uint64_t occupied_frames = write_head - read_head;
        reject = occupied_frames > PKS_RING_FRAMES
            || frame_count > PKS_RING_FRAMES - occupied_frames;
    }
    if (reject) {
        atomic_fetch_add_explicit(
            &ring->drop_count,
            (uint64_t)frame_count,
            memory_order_relaxed);
        return 0;
    }

    for (uint32_t frame = 0; frame < frame_count; frame++) {
        uint32_t slot = (uint32_t)((write_head + frame) & PKS_RING_MASK);
        ring->source_frame_positions[slot] = source_frame_position + frame;
        for (uint32_t channel = 0; channel < PKS_MAX_CHANNELS; channel++) {
            ring->data[slot * PKS_MAX_CHANNELS + channel] =
                source[frame * PKS_MAX_CHANNELS + channel];
        }
    }
    atomic_store_explicit(
        &ring->published_source_frame_end,
        source_frame_position + frame_count,
        memory_order_relaxed);
    atomic_store_explicit(
        &ring->source_position_initialized,
        1u,
        memory_order_relaxed);
    atomic_store_explicit(
        &ring->write_head,
        write_head + frame_count,
        memory_order_release);
    return 1;
}

/* Total shared memory size. */
#define PKS_SHM_SIZE        sizeof(PksLoopbackRing)

#if defined(__cplusplus)
static_assert(PKS_SHM_SIZE <= UINT32_MAX, "ASP ABI size field must not truncate");
#else
_Static_assert(PKS_SHM_SIZE <= UINT32_MAX, "ASP ABI size field must not truncate");
#endif
