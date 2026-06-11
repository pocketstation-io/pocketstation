/*
 * shm_reader.c — Rust-side POSIX shared memory reader.
 *
 * bridge.h forward-declares PksReader as an opaque struct.
 * We provide the full definition here, before including bridge.h,
 * so the typedef in bridge.h is compatible (same struct tag).
 */
#include "SharedRing.h"
#include <sys/mman.h>
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

PksReader* pks_asp_open_reader(void) {
    int fd = shm_open(PKS_SHM_NAME, O_RDONLY, 0);
    if (fd < 0) return NULL;

    void* ptr = mmap(NULL, PKS_SHM_SIZE, PROT_READ, MAP_SHARED, fd, 0);
    if (ptr == MAP_FAILED) { close(fd); return NULL; }

    PksReader* r = (PksReader*)malloc(sizeof(PksReader));
    if (!r) { munmap(ptr, PKS_SHM_SIZE); close(fd); return NULL; }
    r->fd = fd;
    r->ring = (PksLoopbackRing*)ptr;
    r->read_head = atomic_load_explicit(&r->ring->write_head, memory_order_acquire);
    return r;
}

int pks_asp_is_installed(void) {
    /* Check if the shared memory region exists (plugin is running). */
    int fd = shm_open(PKS_SHM_NAME, O_RDONLY, 0);
    if (fd < 0) return 0;
    close(fd);
    return 1;
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

/*
 * Read up to `frame_count` frames of interleaved f32 into `out_buf`.
 * Returns number of frames actually read (0 if no data available).
 */
uint32_t pks_asp_read_frames(PksReader* r, float* out_buf, uint32_t frame_count) {
    if (!r || !r->ring) return 0;
    uint64_t wHead = atomic_load_explicit(&r->ring->write_head, memory_order_acquire);
    uint64_t available = wHead - r->read_head;
    if (available == 0) return 0;
    if (available > PKS_RING_FRAMES) {
        /* Overrun: skip ahead, keep only the most recent PKS_RING_FRAMES */
        r->read_head = wHead - PKS_RING_FRAMES;
        available = PKS_RING_FRAMES;
    }
    uint32_t ch = atomic_load_explicit(&r->ring->channels, memory_order_relaxed);
    if (ch == 0 || ch > PKS_MAX_CHANNELS) ch = PKS_MAX_CHANNELS;
    uint32_t to_read = (uint32_t)(available < frame_count ? available : frame_count);
    for (uint32_t i = 0; i < to_read; i++) {
        uint32_t slot = (uint32_t)((r->read_head + i) & PKS_RING_MASK);
        for (uint32_t c = 0; c < ch; c++) {
            out_buf[i * ch + c] = r->ring->data[slot * ch + c];
        }
    }
    r->read_head += to_read;
    return to_read;
}

void pks_asp_close_reader(PksReader* r) {
    if (!r) return;
    if (r->ring && r->ring != MAP_FAILED) munmap(r->ring, PKS_SHM_SIZE);
    if (r->fd >= 0) close(r->fd);
    free(r);
}
