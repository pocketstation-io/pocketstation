//! Bounded Windows Session probe using real, enumerated endpoint IDs.
//!
//! This is a device-path classification, not an acoustic-quality test. A
//! successful route says that Windows attested configuration on this opened
//! microphone; it does not establish echo reduction or double-talk quality.

#![cfg(all(target_os = "windows", feature = "wasapi-capture"))]

use std::time::{Duration, Instant};

use pocketstation::{
    DeviceId, DeviceSelector, NativePlaybackReference, Session, SessionStartErrorCode, Source,
};
use wasapi::{DeviceEnumerator, Direction};

struct ComGuard;

impl ComGuard {
    fn enter() -> Result<Self, String> {
        wasapi::initialize_mta()
            .ok()
            .map_err(|error| format!("initialize Windows COM: {error}"))?;
        Ok(Self)
    }
}

impl Drop for ComGuard {
    fn drop(&mut self) {
        wasapi::deinitialize();
    }
}

fn first_active_id(
    enumerator: &DeviceEnumerator,
    direction: Direction,
) -> Result<Option<String>, String> {
    let devices = enumerator
        .get_device_collection(&direction)
        .map_err(|error| format!("enumerate {direction} endpoints: {error}"))?;
    if devices
        .get_nbr_devices()
        .map_err(|error| format!("count {direction} endpoints: {error}"))?
        == 0
    {
        return Ok(None);
    }
    let device = devices
        .get_device_at_index(0)
        .map_err(|error| format!("select {direction} endpoint: {error}"))?;
    device
        .get_id()
        .map(Some)
        .map_err(|error| format!("read {direction} endpoint ID: {error}"))
}

#[test]
fn given_real_windows_microphone_when_ordinary_capture_starts_then_single_stream_opens_and_stops() {
    let strict = std::env::var_os("PKS_REQUIRE_WINDOWS_NATIVE_AEC_PROBE").is_some();
    let microphone_id = (|| -> Result<Option<String>, String> {
        let _com = ComGuard::enter()?;
        let enumerator = DeviceEnumerator::new()
            .map_err(|error| format!("create Windows endpoint enumerator: {error}"))?;
        first_active_id(&enumerator, Direction::Capture)
    })();
    let microphone_id = match microphone_id {
        Ok(Some(id)) => id,
        Ok(None) => {
            eprintln!("W21_WINDOWS_ORDINARY_MIC_RESULT=NO_INPUT_ENDPOINT");
            assert!(!strict, "guest probe requires an active input endpoint");
            return;
        }
        Err(error) => {
            eprintln!("W21_WINDOWS_ORDINARY_MIC_RESULT=ENUMERATION_FAILED: {error}");
            assert!(
                !strict,
                "guest probe requires endpoint enumeration: {error}"
            );
            return;
        }
    };

    let session = Session::builder().build();
    let microphone = session
        .capture(Source::microphone(DeviceSelector::id(DeviceId::new(
            microphone_id,
        ))))
        .expect("declare exact microphone");
    let output = session.polled_audio().expect("declare polled audio output");
    microphone.send(output).expect("route microphone to output");
    let mut running = session.start().expect("ordinary microphone opens");
    let deadline = Instant::now() + Duration::from_millis(750);
    let mut delivered_frames = 0usize;
    while Instant::now() < deadline {
        if let Ok(batch) = running.try_poll_audio() {
            delivered_frames = delivered_frames.saturating_add(batch.len());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    eprintln!("W21_WINDOWS_ORDINARY_MIC_RESULT=OPENED delivered_frames={delivered_frames}");
    assert!(
        running.stop().is_success(),
        "ordinary Session stops cleanly"
    );
}

#[test]
fn given_enumerated_windows_endpoints_when_native_aec_requested_then_route_opens_or_rejects_without_raw_delivery(
) {
    let strict = std::env::var_os("PKS_REQUIRE_WINDOWS_NATIVE_AEC_PROBE").is_some();
    let endpoints = (|| -> Result<Option<(String, String)>, String> {
        let _com = ComGuard::enter()?;
        let enumerator = DeviceEnumerator::new()
            .map_err(|error| format!("create Windows endpoint enumerator: {error}"))?;
        let microphone = first_active_id(&enumerator, Direction::Capture)?;
        let render = first_active_id(&enumerator, Direction::Render)?;
        Ok(microphone.zip(render))
    })();
    let (microphone_id, render_id) = match endpoints {
        Ok(Some(pair)) => pair,
        Ok(None) => {
            eprintln!("W21_NATIVE_AEC_RESULT=NO_EXACT_INPUT_AND_RENDER_ENDPOINTS");
            assert!(
                !strict,
                "guest probe requires active input and render endpoints"
            );
            return;
        }
        Err(error) => {
            eprintln!("W21_NATIVE_AEC_RESULT=ENDPOINT_ENUMERATION_FAILED: {error}");
            assert!(
                !strict,
                "guest probe requires endpoint enumeration: {error}"
            );
            return;
        }
    };

    let session = Session::builder().build();
    let microphone = session
        .capture(Source::microphone(DeviceSelector::id(DeviceId::new(
            microphone_id,
        ))))
        .expect("declare exact microphone");
    session
        .native_aec(
            &microphone,
            NativePlaybackReference::output(DeviceId::new(render_id)),
        )
        .expect("declare exact native playback reference");
    let output = session.polled_audio().expect("declare polled audio output");
    microphone
        .send(output)
        .expect("route native microphone to output");

    match session.start() {
        Ok(mut running) => {
            let deadline = Instant::now() + Duration::from_millis(750);
            let mut delivered_frames = 0usize;
            while Instant::now() < deadline {
                if let Ok(batch) = running.try_poll_audio() {
                    delivered_frames = delivered_frames.saturating_add(batch.len());
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            eprintln!("W21_NATIVE_AEC_RESULT=ROUTE_OPENED delivered_frames={delivered_frames}");
            assert!(running.stop().is_success(), "native Session stops cleanly");
        }
        Err(error) => {
            // The virtual endpoint is allowed to lack a controllable AEC
            // effect. An unrelated startup failure must fail this probe;
            // otherwise broken capture setup could appear to pass admission.
            assert_eq!(
                error.code(),
                SessionStartErrorCode::CaptureBackendFailed,
                "native AEC failed before the expected effect check: {error:?}"
            );
            assert!(
                error.message().contains(
                    "selected Windows capture endpoint has no active controllable AEC effect"
                ),
                "native AEC failed for a reason other than an unavailable effect: {error:?}"
            );
            // Session did not hand the caller a RunningSession or a polling
            // receiver. Its gated startup may have opened a device briefly,
            // but no microphone frame was delivered to this application.
            eprintln!("W21_NATIVE_AEC_RESULT=ROUTE_REJECTED delivered_frames=0 error={error:?}");
        }
    }
}
