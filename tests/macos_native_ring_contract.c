#include <assert.h>
#include <pthread.h>
#include <sched.h>
#include <stddef.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdlib.h>

#include "../native/macos/asp/shm_reader.c"

#define PKS_CONCURRENT_FRAME_COUNT (PKS_RING_FRAMES * 4u)

_Static_assert(offsetof(PksLoopbackRing, write_head) == 0u, "write head offset");
_Static_assert(offsetof(PksLoopbackRing, read_head) == 64u, "read head offset");
_Static_assert(offsetof(PksLoopbackRing, drop_count) == 128u, "drop count offset");
_Static_assert(
    offsetof(PksLoopbackRing, published_source_frame_end) == 192u,
    "source end offset");
_Static_assert(offsetof(PksLoopbackRing, sample_rate) == 256u, "config offset");
_Static_assert(
    offsetof(PksLoopbackRing, source_frame_positions) == 320u,
    "source position offset");
_Static_assert(offsetof(PksLoopbackRing, data) == 524608u, "audio data offset");

typedef struct {
    PksLoopbackRing* ring;
    _Atomic uint32_t producer_finished;
} ConcurrentFixture;

static void initialize_reader(
    PksLoopbackRing* ring,
    PksReader* reader)
{
    atomic_store_explicit(&ring->channels, PKS_MAX_CHANNELS, memory_order_relaxed);
    atomic_store_explicit(&ring->reader_attached, 1u, memory_order_release);
    reader->fd = -1;
    reader->ring = ring;
    reader->read_head = 0u;
    atomic_store_explicit(&ring->read_head, 0u, memory_order_release);
}

static void fill_source(float* source, uint64_t first_frame, uint32_t frame_count) {
    for (uint32_t frame = 0; frame < frame_count; frame++) {
        float value = (float)(first_frame + frame);
        source[frame * PKS_MAX_CHANNELS] = value;
        source[frame * PKS_MAX_CHANNELS + 1u] = value;
    }
}

static void test_full_ring_rejects_whole_callback_without_overwrite(void) {
    PksLoopbackRing* ring = calloc(1u, sizeof(PksLoopbackRing));
    assert(ring != NULL);
    PksReader reader;
    initialize_reader(ring, &reader);

    float source[512u * PKS_MAX_CHANNELS];
    for (uint64_t first_frame = 0u;
         first_frame < PKS_RING_FRAMES;
         first_frame += 512u) {
        fill_source(source, first_frame, 512u);
        assert(pks_ring_try_write(ring, source, first_frame, 512u) == 1);
    }
    assert(atomic_load_explicit(&ring->write_head, memory_order_acquire)
        == PKS_RING_FRAMES);

    fill_source(source, PKS_RING_FRAMES, 512u);
    assert(pks_ring_try_write(
        ring,
        source,
        PKS_RING_FRAMES,
        512u) == 0);
    assert(atomic_load_explicit(&ring->write_head, memory_order_acquire)
        == PKS_RING_FRAMES);
    assert(pks_asp_drop_count(&reader) == 512u);

    float output[512u * PKS_MAX_CHANNELS];
    uint64_t source_frame_position = UINT64_MAX;
    assert(pks_asp_read_frames(
        &reader,
        output,
        512u,
        &source_frame_position) == 512u);
    assert(source_frame_position == 0u);
    for (uint32_t frame = 0; frame < 512u; frame++) {
        assert(output[frame * PKS_MAX_CHANNELS] == (float)frame);
        assert(output[frame * PKS_MAX_CHANNELS + 1u] == (float)frame);
    }

    fill_source(source, PKS_RING_FRAMES + 512u, 512u);
    assert(pks_ring_try_write(
        ring,
        source,
        PKS_RING_FRAMES + 512u,
        512u) == 1);

    uint64_t expected_frame = 512u;
    uint64_t consumed_frames = 512u;
    while (consumed_frames < PKS_RING_FRAMES + 512u) {
        uint32_t received = pks_asp_read_frames(
            &reader,
            output,
            512u,
            &source_frame_position);
        assert(received > 0u);
        assert(source_frame_position == expected_frame
            || (expected_frame == PKS_RING_FRAMES
                && source_frame_position == PKS_RING_FRAMES + 512u));
        for (uint32_t frame = 0; frame < received; frame++) {
            if (expected_frame == PKS_RING_FRAMES) {
                expected_frame += 512u;
            }
            assert(output[frame * PKS_MAX_CHANNELS] == (float)expected_frame);
            assert(output[frame * PKS_MAX_CHANNELS + 1u] == (float)expected_frame);
            expected_frame++;
        }
        consumed_frames += received;
    }
    assert(reader.read_head
        == atomic_load_explicit(&ring->write_head, memory_order_acquire));
    assert(atomic_load_explicit(&ring->read_head, memory_order_acquire)
        == reader.read_head);
    assert(pks_asp_drop_count(&reader) == 512u);
    free(ring);
}

static void test_invalid_head_order_fails_closed(void) {
    PksLoopbackRing* ring = calloc(1u, sizeof(PksLoopbackRing));
    assert(ring != NULL);
    atomic_store_explicit(&ring->reader_attached, 1u, memory_order_release);
    atomic_store_explicit(&ring->write_head, 7u, memory_order_relaxed);
    atomic_store_explicit(&ring->read_head, 8u, memory_order_release);
    float source[PKS_MAX_CHANNELS] = {1.0f, 1.0f};

    assert(pks_ring_try_write(ring, source, 7u, 1u) == 0);
    assert(atomic_load_explicit(&ring->write_head, memory_order_acquire) == 7u);
    assert(atomic_load_explicit(&ring->drop_count, memory_order_relaxed) == 1u);
    free(ring);
}

static void* produce_concurrently(void* context) {
    ConcurrentFixture* fixture = context;
    float source[PKS_MAX_CHANNELS];
    for (uint64_t frame = 0u; frame < PKS_CONCURRENT_FRAME_COUNT; frame++) {
        fill_source(source, frame, 1u);
        (void)pks_ring_try_write(fixture->ring, source, frame, 1u);
    }
    atomic_store_explicit(
        &fixture->producer_finished,
        1u,
        memory_order_release);
    return NULL;
}

static void test_release_acquire_visibility_under_concurrency(void) {
    PksLoopbackRing* ring = calloc(1u, sizeof(PksLoopbackRing));
    assert(ring != NULL);
    PksReader reader;
    initialize_reader(ring, &reader);
    ConcurrentFixture fixture = {
        .ring = ring,
        .producer_finished = 0u,
    };
    pthread_t producer;
    assert(pthread_create(&producer, NULL, produce_concurrently, &fixture) == 0);

    uint64_t consumed_frames = 0u;
    float previous_value = -1.0f;
    float output[127u * PKS_MAX_CHANNELS];
    uint64_t expected_source_frame_position = 0u;
    for (;;) {
        uint64_t source_frame_position = UINT64_MAX;
        uint32_t received = pks_asp_read_frames(
            &reader,
            output,
            127u,
            &source_frame_position);
        if (received > 0u) {
            assert(source_frame_position >= expected_source_frame_position);
            expected_source_frame_position = source_frame_position + received;
        }
        for (uint32_t frame = 0; frame < received; frame++) {
            float left = output[frame * PKS_MAX_CHANNELS];
            float right = output[frame * PKS_MAX_CHANNELS + 1u];
            assert(left == right);
            assert(left > previous_value);
            previous_value = left;
            consumed_frames++;
        }
        uint32_t producer_finished = atomic_load_explicit(
            &fixture.producer_finished,
            memory_order_acquire);
        uint64_t published_frames = atomic_load_explicit(
            &ring->write_head,
            memory_order_acquire);
        if (producer_finished != 0u && reader.read_head == published_frames) {
            break;
        }
        if (received == 0u) {
            sched_yield();
        }
    }
    assert(pthread_join(producer, NULL) == 0);
    uint64_t dropped_frames = atomic_load_explicit(
        &ring->drop_count,
        memory_order_relaxed);
    assert(consumed_frames + dropped_frames == PKS_CONCURRENT_FRAME_COUNT);
    assert(reader.read_head
        == atomic_load_explicit(&ring->read_head, memory_order_acquire));
    free(ring);
}

static void test_discontinuity_splits_reader_batches(void) {
    PksLoopbackRing* ring = calloc(1u, sizeof(PksLoopbackRing));
    assert(ring != NULL);
    PksReader reader;
    initialize_reader(ring, &reader);
    float source[8u * PKS_MAX_CHANNELS];
    float output[16u * PKS_MAX_CHANNELS];
    uint64_t source_frame_position = UINT64_MAX;

    fill_source(source, 100u, 8u);
    assert(pks_ring_try_write(ring, source, 100u, 8u) == 1);
    fill_source(source, 116u, 8u);
    assert(pks_ring_try_write(ring, source, 116u, 8u) == 1);

    assert(pks_asp_read_frames(
        &reader,
        output,
        16u,
        &source_frame_position) == 8u);
    assert(source_frame_position == 100u);
    assert(pks_asp_read_frames(
        &reader,
        output,
        16u,
        &source_frame_position) == 8u);
    assert(source_frame_position == 116u);
    free(ring);
}

static void test_invalid_timeline_rejects_once_without_publication(void) {
    PksLoopbackRing* ring = calloc(1u, sizeof(PksLoopbackRing));
    assert(ring != NULL);
    float source[8u * PKS_MAX_CHANNELS];
    fill_source(source, 100u, 8u);
    assert(pks_ring_try_write(ring, source, 100u, 8u) == 1);

    fill_source(source, 99u, 8u);
    assert(pks_ring_try_write(ring, source, 99u, 8u) == 0);
    assert(atomic_load_explicit(&ring->write_head, memory_order_acquire) == 8u);
    assert(atomic_load_explicit(
        &ring->timeline_reject_callback_count,
        memory_order_relaxed) == 1u);
    assert(atomic_load_explicit(
        &ring->drop_count,
        memory_order_relaxed) == 8u);

    assert(pks_ring_try_write(ring, source, UINT64_MAX - 3u, 8u) == 0);
    assert(atomic_load_explicit(
        &ring->timeline_reject_callback_count,
        memory_order_relaxed) == 2u);
    assert(atomic_load_explicit(
        &ring->drop_count,
        memory_order_relaxed) == 16u);
    free(ring);
}

static void test_abi_validation_fails_closed(void) {
    PksLoopbackRing* ring = calloc(1u, sizeof(PksLoopbackRing));
    assert(ring != NULL);
    assert(pks_asp_ring_compatible(ring) == 0);

    atomic_store_explicit(
        &ring->abi_magic,
        PKS_RING_ABI_MAGIC,
        memory_order_relaxed);
    atomic_store_explicit(
        &ring->abi_version,
        PKS_RING_ABI_VERSION,
        memory_order_relaxed);
    atomic_store_explicit(
        &ring->abi_struct_size,
        (uint32_t)PKS_SHM_SIZE,
        memory_order_relaxed);
    atomic_store_explicit(&ring->io_running, 1u, memory_order_release);
    assert(pks_asp_ring_compatible(ring) == 1);

    atomic_store_explicit(
        &ring->abi_version,
        PKS_RING_ABI_VERSION + 1u,
        memory_order_relaxed);
    assert(pks_asp_ring_compatible(ring) == 0);
    free(ring);
}

int main(void) {
    test_full_ring_rejects_whole_callback_without_overwrite();
    test_invalid_head_order_fails_closed();
    test_release_acquire_visibility_under_concurrency();
    test_discontinuity_splits_reader_batches();
    test_invalid_timeline_rejects_once_without_publication();
    test_abi_validation_fails_closed();
    return 0;
}
