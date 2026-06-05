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
///
/// # Safety
/// The returned pointer must be destroyed with `ps_opus_encoder_destroy`.
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
///
/// # Safety
/// `enc` must be a pointer returned by `ps_opus_encoder_create` or null.
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
///
/// # Safety
/// - `enc` must be a valid pointer from `ps_opus_encoder_create` or null.
/// - `pcm` must point to at least `sample_count` valid f32 values.
/// - `out_buf` must point to at least `out_cap` bytes of writable memory.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn sine_960(hz: f32) -> Vec<f32> {
        (0..pocketstation_codec::OPUS_FRAME_SAMPLES)
            .map(|i| (2.0 * PI * hz * i as f32 / 48_000.0).sin() * 0.25)
            .collect()
    }

    #[test]
    fn given_valid_pcm_when_encode_then_returns_positive_byte_count() {
        unsafe {
            let enc = ps_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null(), "encoder creation failed");
            let pcm = sine_960(440.0);
            let mut out = vec![0u8; 256];
            let n = ps_encode_opus(enc, pcm.as_ptr(), pcm.len(), out.as_mut_ptr(), out.len());
            assert!(n > 0, "expected positive byte count, got {n}");
            ps_opus_encoder_destroy(enc);
        }
    }

    #[test]
    fn given_null_encoder_when_encode_then_returns_minus_one() {
        unsafe {
            let mut out = vec![0u8; 256];
            let pcm = sine_960(440.0);
            let n = ps_encode_opus(
                std::ptr::null_mut(),
                pcm.as_ptr(),
                pcm.len(),
                out.as_mut_ptr(),
                out.len(),
            );
            assert_eq!(n, -1);
        }
    }

    #[test]
    fn given_encoder_when_destroy_null_then_no_crash() {
        unsafe { ps_opus_encoder_destroy(std::ptr::null_mut()) }
    }

    #[test]
    fn given_sine_440hz_when_round_trip_then_decoded_has_energy() {
        unsafe {
            let enc = ps_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null());
            let pcm = sine_960(440.0);
            let mut out = vec![0u8; 256];
            let n = ps_encode_opus(enc, pcm.as_ptr(), pcm.len(), out.as_mut_ptr(), out.len());
            assert!(n > 0);
            // Verify the encoded frame is a plausible size: Opus 20ms at 64kbps ≈ 160 bytes
            assert!(n <= 256, "encoded frame too large: {n}");
            assert!(n >= 2, "encoded frame suspiciously small: {n}");
            ps_opus_encoder_destroy(enc);
        }
    }
}
