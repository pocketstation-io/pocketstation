/*
 * shm_reader.c — Rust-side POSIX shared memory reader.
 *
 * bridge.h forward-declares PksReader as an opaque struct.
 * We provide the full definition here, before including bridge.h,
 * so the typedef in bridge.h is compatible (same struct tag).
 */
#include "SharedRing.h"
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>

/* Full definition must come before bridge.h to satisfy the forward declaration. */
struct PksReader {
    int              fd;
    PksLoopbackRing* ring;
    uint64_t         read_head;
};

#include "bridge.h"

static int pks_asp_ring_compatible(PksLoopbackRing* ring) {
    return atomic_load_explicit(&ring->io_running, memory_order_acquire) != 0u
        && atomic_load_explicit(&ring->abi_magic, memory_order_relaxed)
            == PKS_RING_ABI_MAGIC
        && atomic_load_explicit(&ring->abi_version, memory_order_relaxed)
            == PKS_RING_ABI_VERSION
        && atomic_load_explicit(&ring->abi_struct_size, memory_order_relaxed)
            == (uint32_t)PKS_SHM_SIZE;
}

PksReader* pks_asp_open_reader(void) {
    int fd = shm_open(PKS_SHM_NAME, O_RDWR, 0);
    if (fd < 0) return NULL;

    struct stat region_stat;
    if (fstat(fd, &region_stat) != 0
            || region_stat.st_size != (off_t)PKS_SHM_SIZE) {
        close(fd);
        return NULL;
    }

    void* ptr = mmap(NULL, PKS_SHM_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (ptr == MAP_FAILED) { close(fd); return NULL; }

    PksLoopbackRing* ring = (PksLoopbackRing*)ptr;
    if (!pks_asp_ring_compatible(ring)) {
        munmap(ptr, PKS_SHM_SIZE);
        close(fd);
        return NULL;
    }

    PksReader* r = (PksReader*)malloc(sizeof(PksReader));
    if (!r) { munmap(ptr, PKS_SHM_SIZE); close(fd); return NULL; }
    r->fd = fd;
    r->ring = (PksLoopbackRing*)ptr;
    uint32_t expected = 0;
    if (!atomic_compare_exchange_strong_explicit(
            &r->ring->reader_attached,
            &expected,
            1u,
            memory_order_acq_rel,
            memory_order_acquire)) {
        munmap(ptr, PKS_SHM_SIZE);
        close(fd);
        free(r);
        return NULL;
    }
    r->read_head = atomic_load_explicit(&r->ring->write_head, memory_order_acquire);
    atomic_store_explicit(&r->ring->read_head, r->read_head, memory_order_release);
    return r;
}

int pks_asp_is_installed(void) {
    /* Existing but stale or ABI-incompatible regions are not installed. */
    int fd = shm_open(PKS_SHM_NAME, O_RDONLY, 0);
    if (fd < 0) return 0;
    struct stat region_stat;
    if (fstat(fd, &region_stat) != 0
            || region_stat.st_size != (off_t)PKS_SHM_SIZE) {
        close(fd);
        return 0;
    }
    void* ptr = mmap(NULL, PKS_SHM_SIZE, PROT_READ, MAP_SHARED, fd, 0);
    if (ptr == MAP_FAILED) {
        close(fd);
        return 0;
    }
    PksLoopbackRing* ring = (PksLoopbackRing*)ptr;
    int compatible = pks_asp_ring_compatible(ring);
    munmap(ptr, PKS_SHM_SIZE);
    close(fd);
    return compatible;
}

uint32_t pks_asp_sample_rate(PksReader* r) {
    if (!r || !r->ring) return 48000;
    return atomic_load_explicit(&r->ring->sample_rate, memory_order_relaxed);
}

uint32_t pks_asp_channels(PksReader* r) {
    if (!r || !r->ring) return 2;
    return atomic_load_explicit(&r->ring->channels, memory_order_relaxed);
}

uint64_t pks_asp_drop_count(PksReader* r) {
    if (!r || !r->ring) return 0;
    return atomic_load_explicit(&r->ring->drop_count, memory_order_relaxed);
}

uint64_t pks_asp_timeline_reject_callback_count(PksReader* r) {
    if (!r || !r->ring) return 0;
    return atomic_load_explicit(
        &r->ring->timeline_reject_callback_count,
        memory_order_relaxed);
}

/*
 * Read up to `frame_count` frames of interleaved f32 into `out_buf`.
 * Returns number of frames actually read (0 if no data available).
 */
uint32_t pks_asp_read_frames(
    PksReader* r,
    float* out_buf,
    uint32_t frame_count,
    uint64_t* out_source_frame_position)
{
    if (!r || !r->ring || !out_buf || !out_source_frame_position
            || frame_count == 0u) {
        return 0;
    }
    uint64_t wHead = atomic_load_explicit(&r->ring->write_head, memory_order_acquire);
    uint64_t available = wHead - r->read_head;
    if (available == 0) return 0;
    if (available > PKS_RING_FRAMES) {
        /*
         * Compatibility guard for a producer that advanced without observing
         * read_head. The v2 producer normally prevents this condition.
         */
        r->read_head = wHead - PKS_RING_FRAMES;
        atomic_store_explicit(&r->ring->read_head, r->read_head, memory_order_release);
        available = PKS_RING_FRAMES;
    }
    uint32_t ch = atomic_load_explicit(&r->ring->channels, memory_order_relaxed);
    if (ch == 0 || ch > PKS_MAX_CHANNELS) ch = PKS_MAX_CHANNELS;
    uint32_t to_read = (uint32_t)(available < frame_count ? available : frame_count);
    uint32_t first_slot = (uint32_t)(r->read_head & PKS_RING_MASK);
    uint64_t source_frame_position =
        r->ring->source_frame_positions[first_slot];
    for (uint32_t i = 1u; i < to_read; i++) {
        uint32_t slot = (uint32_t)((r->read_head + i) & PKS_RING_MASK);
        if (source_frame_position > UINT64_MAX - (uint64_t)i
                || r->ring->source_frame_positions[slot]
                    != source_frame_position + i) {
            to_read = i;
            break;
        }
    }
    for (uint32_t i = 0; i < to_read; i++) {
        uint32_t slot = (uint32_t)((r->read_head + i) & PKS_RING_MASK);
        for (uint32_t c = 0; c < ch; c++) {
            out_buf[i * ch + c] = r->ring->data[slot * ch + c];
        }
    }
    r->read_head += to_read;
    atomic_store_explicit(&r->ring->read_head, r->read_head, memory_order_release);
    *out_source_frame_position = source_frame_position;
    return to_read;
}

void pks_asp_close_reader(PksReader* r) {
    if (!r) return;
    if (r->ring && r->ring != MAP_FAILED) {
        atomic_store_explicit(&r->ring->reader_attached, 0u, memory_order_release);
        munmap(r->ring, PKS_SHM_SIZE);
    }
    if (r->fd >= 0) close(r->fd);
    free(r);
}
