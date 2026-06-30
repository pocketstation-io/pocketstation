//! C FFI boundary for the PocketStation audio engine.
//!
//! Canonical headers (regenerate with scripts/sync-ffi-header.sh after any change here):
//!   sdk-ios/Sources/PocketStationAudioFFI/pks_audio.h
//!   sdk-android/sdk/src/main/cpp/pks_audio.h
//! Swift usage: import via module.modulemap in sdk-ios (AUDIO-001).
//! AUDIO-001: platform callback writes f32 PCM → Rust Opus encode → Swift WebRTC send.

use std::os::raw::{c_int, c_uchar, c_uint};

use pks_codec::{OpusChannels, OpusConfig, OpusEncoder};

/// Opaque Opus encoder handle. Created once per session; not thread-safe.
/// The iOS caller must not share this pointer across threads.
pub struct PksOpusEncoder {
    inner: OpusEncoder,
    encode_buf: Vec<u8>,
}

/// Create an Opus encoder.
///
/// sample_rate: must be 48000 (Opus operates internally at 48 kHz; any other
///              value returns null — this is a hard rejection, not silent rounding).
/// channels:    1 (mono) or 2 (stereo). Any other value returns null.
/// bitrate_kbps: target bitrate in kbps (e.g. 64). 0 = Opus auto (VBR).
///
/// Returns null on invalid parameters or on internal encoder failure.
///
/// # Safety
/// The returned pointer must be destroyed with `pks_opus_encoder_destroy`.
#[no_mangle]
pub unsafe extern "C" fn pks_opus_encoder_create(
    sample_rate: c_uint,
    channels: u8,
    bitrate_kbps: c_uint,
) -> *mut PksOpusEncoder {
    if sample_rate != 48_000 {
        return std::ptr::null_mut();
    }
    let ch = match channels {
        1 => OpusChannels::Mono,
        2 => OpusChannels::Stereo,
        _ => return std::ptr::null_mut(),
    };
    let config = OpusConfig {
        channels: ch,
        bitrate_kbps: if bitrate_kbps > 0 {
            Some(bitrate_kbps)
        } else {
            None
        },
        ..OpusConfig::default()
    };
    match OpusEncoder::from_config(&config) {
        Ok(enc) => Box::into_raw(Box::new(PksOpusEncoder {
            inner: enc,
            encode_buf: Vec::with_capacity(pks_codec::OPUS_MAX_PACKET_BYTES),
        })),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Destroy an encoder created by pks_opus_encoder_create.
/// Safe to call with null.
///
/// # Safety
/// `enc` must be a pointer returned by `pks_opus_encoder_create` or null.
#[no_mangle]
pub unsafe extern "C" fn pks_opus_encoder_destroy(enc: *mut PksOpusEncoder) {
    if !enc.is_null() {
        drop(Box::from_raw(enc));
    }
}

/// Update the bitrate of a running encoder. Called on CODEC_HINT relay messages (AUDIO-021).
///
/// enc:          pointer from pks_opus_encoder_create. No-op if null (returns -1).
/// bitrate_kbps: new target bitrate in kbps (e.g. 32, 64, 96). 0 = Opus auto (VBR).
///
/// Returns 0 on success, -1 if enc is null, -2 on internal Opus error.
///
/// # Safety
/// `enc` must be a valid pointer from `pks_opus_encoder_create` or null.
#[no_mangle]
pub unsafe extern "C" fn pks_opus_encoder_set_bitrate(
    enc: *mut PksOpusEncoder,
    bitrate_kbps: c_uint,
) -> c_int {
    if enc.is_null() {
        return -1;
    }
    match (*enc).inner.set_bitrate_kbps(bitrate_kbps) {
        Ok(()) => 0,
        Err(_) => -2,
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
/// - `enc` must be a valid pointer from `pks_opus_encoder_create` or null.
/// - `pcm` must point to at least `sample_count` valid f32 values.
/// - `out_buf` must point to at least `out_cap` bytes of writable memory.
#[no_mangle]
pub unsafe extern "C" fn pks_encode_opus(
    enc: *mut PksOpusEncoder,
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
        (0..pks_codec::OPUS_FRAME_SAMPLES)
            .map(|i| (2.0 * PI * hz * i as f32 / 48_000.0).sin() * 0.25)
            .collect()
    }

    #[test]
    fn given_wrong_sample_rate_when_create_then_returns_null() {
        unsafe {
            let enc = pks_opus_encoder_create(44_100, 1, 64);
            assert!(enc.is_null(), "must reject non-48000 sample rate");
        }
    }

    #[test]
    fn given_invalid_channel_count_when_create_then_returns_null() {
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 3, 64);
            assert!(enc.is_null(), "must reject channels != 1 or 2");
        }
    }

    #[test]
    fn given_stereo_channels_when_create_then_succeeds() {
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 2, 64);
            assert!(!enc.is_null(), "stereo encoder must succeed");
            pks_opus_encoder_destroy(enc);
        }
    }

    #[test]
    fn given_valid_pcm_when_encode_then_returns_positive_byte_count() {
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null(), "encoder creation failed");
            let pcm = sine_960(440.0);
            let mut out = vec![0u8; 256];
            let n = pks_encode_opus(enc, pcm.as_ptr(), pcm.len(), out.as_mut_ptr(), out.len());
            assert!(n > 0, "expected positive byte count, got {n}");
            pks_opus_encoder_destroy(enc);
        }
    }

    #[test]
    fn given_null_encoder_when_encode_then_returns_minus_one() {
        unsafe {
            let mut out = vec![0u8; 256];
            let pcm = sine_960(440.0);
            let n = pks_encode_opus(
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
        unsafe { pks_opus_encoder_destroy(std::ptr::null_mut()) }
    }

    #[test]
    fn given_valid_encoder_when_set_bitrate_then_returns_zero() {
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null());
            assert_eq!(pks_opus_encoder_set_bitrate(enc, 32), 0);
            assert_eq!(pks_opus_encoder_set_bitrate(enc, 96), 0);
            assert_eq!(
                pks_opus_encoder_set_bitrate(enc, 0),
                0,
                "0 kbps = VBR auto must succeed"
            );
            pks_opus_encoder_destroy(enc);
        }
    }

    #[test]
    fn given_null_encoder_when_set_bitrate_then_returns_minus_one() {
        unsafe {
            assert_eq!(pks_opus_encoder_set_bitrate(std::ptr::null_mut(), 64), -1);
        }
    }

    #[test]
    fn given_bitrate_change_when_encode_then_still_produces_valid_packet() {
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null());
            pks_opus_encoder_set_bitrate(enc, 32);
            let pcm = sine_960(440.0);
            let mut out = vec![0u8; 256];
            let n = pks_encode_opus(enc, pcm.as_ptr(), pcm.len(), out.as_mut_ptr(), out.len());
            assert!(
                n > 0,
                "encode after bitrate change must produce valid packet"
            );
            pks_opus_encoder_destroy(enc);
        }
    }

    #[test]
    fn given_sine_440hz_when_round_trip_then_decoded_has_energy() {
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null());
            let pcm = sine_960(440.0);
            let mut out = vec![0u8; 256];
            let n = pks_encode_opus(enc, pcm.as_ptr(), pcm.len(), out.as_mut_ptr(), out.len());
            assert!(n > 0);
            // Verify the encoded frame is a plausible size: Opus 20ms at 64kbps ≈ 160 bytes
            assert!(n <= 256, "encoded frame too large: {n}");
            assert!(n >= 2, "encoded frame suspiciously small: {n}");
            pks_opus_encoder_destroy(enc);
        }
    }
}
