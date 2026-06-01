//! C FFI boundary for the PocketStation audio engine.
//!
//! Generated header: audio-core/ffi/pocketstation_audio.h (via cbindgen in build.rs).
//! Swift usage: import via module.modulemap in sdk-ios.
//! ADR-001: platform callback writes f32 PCM → Rust Opus encode → Swift WebRTC send.

use std::os::raw::{c_int, c_uchar, c_uint};

use pocketstation_codec::OpusEncoder;

/// Opaque Opus encoder handle. Created once per session; not thread-safe.
/// The iOS caller must not share this pointer across threads.
pub struct PsOpusEncoder {
    inner: OpusEncoder,
    encode_buf: Vec<u8>,
}

/// Create an Opus encoder.
/// sample_rate: must be 48000. channels: 1 (mono) or 2 (stereo).
/// bitrate_kbps: target bitrate in kbps (e.g. 64).
/// Returns null on failure.
#[no_mangle]
pub unsafe extern "C" fn ps_opus_encoder_create(
    _sample_rate: c_uint,
    _channels: u8,
    _bitrate_kbps: c_uint,
) -> *mut PsOpusEncoder {
    match OpusEncoder::new() {
        Ok(enc) => Box::into_raw(Box::new(PsOpusEncoder {
            inner: enc,
            encode_buf: Vec::with_capacity(pocketstation_codec::OPUS_MAX_PACKET_BYTES),
        })),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Destroy an encoder created by ps_opus_encoder_create.
/// Safe to call with null.
#[no_mangle]
pub unsafe extern "C" fn ps_opus_encoder_destroy(enc: *mut PsOpusEncoder) {
    if !enc.is_null() {
        drop(Box::from_raw(enc));
    }
}

/// Encode one PCM frame to Opus.
///
/// pcm:          f32 interleaved samples, 48 kHz (960 samples for 20ms mono).
/// sample_count: number of f32 samples (960 for 20ms mono).
/// out_buf:      caller-allocated output buffer.
/// out_cap:      capacity of out_buf in bytes (256 is sufficient for any Opus frame).
///
/// Returns:  number of bytes written on success.
///           -1 if enc is null.
///           -2 on encoding error.
#[no_mangle]
pub unsafe extern "C" fn ps_encode_opus(
    enc: *mut PsOpusEncoder,
    pcm: *const f32,
    sample_count: usize,
    out_buf: *mut c_uchar,
    out_cap: usize,
) -> c_int {
    if enc.is_null() || pcm.is_null() || out_buf.is_null() {
        return -1;
    }
    let enc = &mut *enc;
    let samples = std::slice::from_raw_parts(pcm, sample_count);
    enc.encode_buf.clear();
    match enc.inner.encode_into(samples, &mut enc.encode_buf) {
        Ok(n) => {
            let to_copy = n.min(out_cap);
            std::ptr::copy_nonoverlapping(enc.encode_buf.as_ptr(), out_buf, to_copy);
            to_copy as c_int
        }
        Err(_) => -2,
    }
}
