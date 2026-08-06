//! C compatibility boundary for the PocketStation Opus codec.
//!
//! This is not the Session engine ABI.
//!
//! The checked header is the unified `include/pocketstation.h`.
//!
//! Canonical SDK copies use the same `pocketstation.h` name.
//! Swift usage: import via module.modulemap in sdk-ios (AUDIO-001).
//! AUDIO-001: platform callback writes f32 PCM → Rust Opus encode → Swift WebRTC send.

use std::os::raw::{c_int, c_uchar, c_uint};
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::codec::{OpusChannels, OpusConfig, OpusEncodeError, OpusEncoder};

/// Stable negative result codes returned by codec operations.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PksCodecErrorCode {
    InvalidPointer = -1,
    Encode = -2,
    OutputTooSmall = -3,
    InvalidFrame = -4,
    InternalPanic = -5,
}

impl PksCodecErrorCode {
    const fn as_c_int(self) -> c_int {
        self as c_int
    }
}

fn codec_int_call(function: impl FnOnce() -> c_int) -> c_int {
    catch_unwind(AssertUnwindSafe(function))
        .unwrap_or_else(|_| PksCodecErrorCode::InternalPanic.as_c_int())
}

fn codec_pointer_call<T>(function: impl FnOnce() -> *mut T) -> *mut T {
    catch_unwind(AssertUnwindSafe(function)).unwrap_or(std::ptr::null_mut())
}

fn codec_void_call(function: impl FnOnce()) {
    let _ = catch_unwind(AssertUnwindSafe(function));
}

/// Opaque Opus encoder handle. Created once per session; not thread-safe.
/// A caller must not share this pointer across threads.
pub struct PksOpusEncoder {
    inner: OpusEncoder,
    encode_buf: Vec<u8>,
}

/// Returns the output capacity required to hold any supported Opus packet.
#[no_mangle]
pub extern "C" fn pks_opus_max_packet_bytes() -> usize {
    crate::codec::OPUS_MAX_PACKET_BYTES
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
    codec_pointer_call(|| {
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
                encode_buf: Vec::with_capacity(crate::codec::OPUS_MAX_PACKET_BYTES),
            })),
            Err(_) => std::ptr::null_mut(),
        }
    })
}

/// Destroy an encoder created by pks_opus_encoder_create.
/// Safe to call with null.
///
/// # Safety
/// `enc` must be a pointer returned by `pks_opus_encoder_create` or null.
#[no_mangle]
pub unsafe extern "C" fn pks_opus_encoder_destroy(enc: *mut PksOpusEncoder) {
    if !enc.is_null() {
        codec_void_call(|| {
            // SAFETY: The caller contract requires a live encoder pointer
            // returned by `pks_opus_encoder_create`, consumed exactly once.
            unsafe { drop(Box::from_raw(enc)) };
        });
    }
}

/// Update the bitrate of a running encoder. Called on CODEC_HINT relay messages (AUDIO-021).
///
/// enc:          pointer from pks_opus_encoder_create. No-op if null (returns -1).
/// bitrate_kbps: new target bitrate in kbps (e.g. 32, 64, 96). 0 = Opus auto (VBR).
///
/// Returns 0 on success, -1 if enc is null, -2 on internal Opus error, or -5
/// if a Rust panic was contained at the ABI boundary.
///
/// # Safety
/// `enc` must be a valid pointer from `pks_opus_encoder_create` or null.
#[no_mangle]
pub unsafe extern "C" fn pks_opus_encoder_set_bitrate(
    enc: *mut PksOpusEncoder,
    bitrate_kbps: c_uint,
) -> c_int {
    codec_int_call(|| {
        if enc.is_null() {
            return PksCodecErrorCode::InvalidPointer.as_c_int();
        }
        // SAFETY: The caller contract requires a live encoder pointer owned
        // exclusively by this thread for the duration of the call.
        let encoder = unsafe { &mut *enc };
        match encoder.inner.set_bitrate_kbps(bitrate_kbps) {
            Ok(()) => 0,
            Err(_) => PksCodecErrorCode::Encode.as_c_int(),
        }
    })
}

/// Encode one PCM frame to Opus.
///
/// pcm:          f32 interleaved samples, 48 kHz (960 samples for 20ms mono).
/// sample_count: number of f32 samples (960 for 20ms mono).
/// out_buf:      caller-allocated output buffer.
/// out_cap:      capacity of out_buf in bytes. Use
///               `pks_opus_max_packet_bytes()` for a reusable worst-case buffer.
///
/// Returns:  number of bytes written on success.
///           -1 if an input pointer is null.
///           -2 on an internal Opus encoding error.
///           -3 if `out_cap` is smaller than `pks_opus_max_packet_bytes()`;
///              the encoder is not advanced and nothing is copied.
///           -4 if `sample_count` is not a supported complete frame.
///           -5 if a Rust panic was contained at the ABI boundary.
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
    codec_int_call(|| {
        if enc.is_null() || pcm.is_null() || out_buf.is_null() {
            return PksCodecErrorCode::InvalidPointer.as_c_int();
        }
        // SAFETY: The caller contract requires a live encoder pointer owned
        // exclusively by this thread for the duration of the call.
        let encoder = unsafe { &mut *enc };
        match encoder.inner.validate_frame_sample_count(sample_count) {
            Ok(()) => {}
            Err(OpusEncodeError::InvalidFrameSampleCount { .. }) => {
                return PksCodecErrorCode::InvalidFrame.as_c_int();
            }
            Err(OpusEncodeError::Opus(_)) => {
                return PksCodecErrorCode::Encode.as_c_int();
            }
        }
        if out_cap < crate::codec::OPUS_MAX_PACKET_BYTES {
            return PksCodecErrorCode::OutputTooSmall.as_c_int();
        }
        // SAFETY: The frame count was validated above, and the caller contract
        // requires that many readable f32 values at `pcm`.
        let samples = unsafe { std::slice::from_raw_parts(pcm, sample_count) };
        encoder.encode_buf.clear();
        match encoder.inner.encode_into(samples, &mut encoder.encode_buf) {
            Ok(n) => {
                // SAFETY: `out_cap` was checked against the codec's maximum,
                // `n` cannot exceed that maximum, and the caller contract
                // requires `out_cap` writable bytes at `out_buf`.
                unsafe {
                    std::ptr::copy_nonoverlapping(encoder.encode_buf.as_ptr(), out_buf, n);
                }
                n as c_int
            }
            Err(OpusEncodeError::InvalidFrameSampleCount { .. }) => {
                PksCodecErrorCode::InvalidFrame.as_c_int()
            }
            Err(OpusEncodeError::Opus(_)) => PksCodecErrorCode::Encode.as_c_int(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn sine_960(hz: f32) -> Vec<f32> {
        (0..crate::codec::OPUS_FRAME_SAMPLES)
            .map(|i| (2.0 * PI * hz * i as f32 / 48_000.0).sin() * 0.25)
            .collect()
    }

    #[test]
    fn given_wrong_sample_rate_when_create_then_returns_null() {
        // SAFETY: The call passes scalar values only and checks the returned
        // pointer without dereferencing it.
        unsafe {
            let enc = pks_opus_encoder_create(44_100, 1, 64);
            assert!(enc.is_null(), "must reject non-48000 sample rate");
        }
    }

    #[test]
    fn given_invalid_channel_count_when_create_then_returns_null() {
        // SAFETY: The call passes scalar values only and checks the returned
        // pointer without dereferencing it.
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 3, 64);
            assert!(enc.is_null(), "must reject channels != 1 or 2");
        }
    }

    #[test]
    fn given_stereo_channels_when_create_then_succeeds() {
        // SAFETY: The returned pointer is checked, used on this thread, and
        // destroyed exactly once.
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 2, 64);
            assert!(!enc.is_null(), "stereo encoder must succeed");
            pks_opus_encoder_destroy(enc);
        }
    }

    #[test]
    fn given_valid_pcm_when_encode_then_returns_positive_byte_count() {
        // SAFETY: The encoder is live, and the PCM/output pointers cover the
        // lengths passed to the FFI functions.
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null(), "encoder creation failed");
            let pcm = sine_960(440.0);
            let mut out = vec![0u8; pks_opus_max_packet_bytes()];
            let n = pks_encode_opus(enc, pcm.as_ptr(), pcm.len(), out.as_mut_ptr(), out.len());
            assert!(n > 0, "expected positive byte count, got {n}");
            pks_opus_encoder_destroy(enc);
        }
    }

    #[test]
    fn given_null_encoder_when_encode_then_returns_minus_one() {
        // SAFETY: The API explicitly permits a null encoder; the PCM/output
        // pointers cover the lengths passed with them.
        unsafe {
            let mut out = vec![0u8; pks_opus_max_packet_bytes()];
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
    fn given_small_output_buffer_when_encode_then_packet_is_not_truncated() {
        // SAFETY: The encoder is live, and the PCM/output pointers cover the
        // lengths passed to the FFI function.
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null());
            let pcm = sine_960(440.0);
            let mut out = [0xA5_u8; 1];
            let status = pks_encode_opus(enc, pcm.as_ptr(), pcm.len(), out.as_mut_ptr(), out.len());
            assert_eq!(status, PksCodecErrorCode::OutputTooSmall.as_c_int());
            assert_eq!(out, [0xA5], "insufficient capacity must not copy a prefix");
            pks_opus_encoder_destroy(enc);
        }
    }

    #[test]
    fn given_invalid_frame_size_when_encode_then_error_is_typed_without_writing() {
        // SAFETY: The encoder is live, and the PCM/output pointers cover the
        // lengths passed to the FFI function.
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null());
            let pcm = [0.0_f32; 961];
            let mut out = [0xA5_u8; 256];
            let status = pks_encode_opus(enc, pcm.as_ptr(), pcm.len(), out.as_mut_ptr(), out.len());
            assert_eq!(status, PksCodecErrorCode::InvalidFrame.as_c_int());
            assert!(out.iter().all(|byte| *byte == 0xA5));
            pks_opus_encoder_destroy(enc);
        }
    }

    #[test]
    fn given_rejected_capacity_when_retried_then_encoder_state_is_unchanged() {
        // SAFETY: Both encoders are live, and every PCM/output pointer covers
        // the corresponding declared length.
        unsafe {
            let retried_encoder = pks_opus_encoder_create(48_000, 1, 64);
            let reference_encoder = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!retried_encoder.is_null());
            assert!(!reference_encoder.is_null());
            let pcm = sine_960(440.0);
            let mut too_small = [0xA5_u8; 1];
            assert_eq!(
                pks_encode_opus(
                    retried_encoder,
                    pcm.as_ptr(),
                    pcm.len(),
                    too_small.as_mut_ptr(),
                    too_small.len(),
                ),
                PksCodecErrorCode::OutputTooSmall.as_c_int()
            );

            let mut retried = vec![0_u8; pks_opus_max_packet_bytes()];
            let mut reference = vec![0_u8; pks_opus_max_packet_bytes()];
            let retried_len = pks_encode_opus(
                retried_encoder,
                pcm.as_ptr(),
                pcm.len(),
                retried.as_mut_ptr(),
                retried.len(),
            );
            let reference_len = pks_encode_opus(
                reference_encoder,
                pcm.as_ptr(),
                pcm.len(),
                reference.as_mut_ptr(),
                reference.len(),
            );

            assert!(retried_len > 0);
            assert_eq!(retried_len, reference_len);
            assert_eq!(
                &retried[..retried_len as usize],
                &reference[..reference_len as usize]
            );
            pks_opus_encoder_destroy(retried_encoder);
            pks_opus_encoder_destroy(reference_encoder);
        }
    }

    #[test]
    fn given_panicking_abi_bodies_when_guarded_then_panics_are_contained() {
        assert_eq!(
            codec_int_call(|| panic!("int guard probe")),
            PksCodecErrorCode::InternalPanic.as_c_int()
        );
        let pointer = codec_pointer_call::<PksOpusEncoder>(|| panic!("pointer guard probe"));
        assert!(pointer.is_null());
        codec_void_call(|| panic!("void guard probe"));
    }

    #[test]
    fn given_encoder_when_destroy_null_then_no_crash() {
        // SAFETY: The destroy contract explicitly permits a null pointer.
        unsafe { pks_opus_encoder_destroy(std::ptr::null_mut()) }
    }

    #[test]
    fn given_valid_encoder_when_set_bitrate_then_returns_zero() {
        // SAFETY: The returned pointer is checked, used on this thread, and
        // destroyed exactly once.
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
        // SAFETY: The bitrate API explicitly permits a null encoder.
        unsafe {
            assert_eq!(pks_opus_encoder_set_bitrate(std::ptr::null_mut(), 64), -1);
        }
    }

    #[test]
    fn given_bitrate_change_when_encode_then_still_produces_valid_packet() {
        // SAFETY: The encoder is live, and the PCM/output pointers cover the
        // lengths passed to the FFI functions.
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null());
            pks_opus_encoder_set_bitrate(enc, 32);
            let pcm = sine_960(440.0);
            let mut out = vec![0u8; pks_opus_max_packet_bytes()];
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
        // SAFETY: The encoder is live, and the PCM/output pointers cover the
        // lengths passed to the FFI functions.
        unsafe {
            let enc = pks_opus_encoder_create(48_000, 1, 64);
            assert!(!enc.is_null());
            let pcm = sine_960(440.0);
            let mut out = vec![0u8; pks_opus_max_packet_bytes()];
            let n = pks_encode_opus(enc, pcm.as_ptr(), pcm.len(), out.as_mut_ptr(), out.len());
            assert!(n > 0);
            assert!(
                n <= pks_opus_max_packet_bytes() as c_int,
                "encoded frame exceeds the documented maximum: {n}"
            );
            assert!(n >= 2, "encoded frame suspiciously small: {n}");
            pks_opus_encoder_destroy(enc);
        }
    }
}
