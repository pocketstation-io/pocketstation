#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use pks_capture::CaptureMode;
    use pks_capture_linux::DesktopCaptureSource;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    let application_id = std::env::args()
        .nth(1)
        .ok_or("usage: native_capture_probe <application-id>")?;
    let frame_count = Arc::new(AtomicU64::new(0));
    let sample_count = Arc::new(AtomicU64::new(0));
    let non_silent_sample_count = Arc::new(AtomicU64::new(0));

    let frame_count_callback = Arc::clone(&frame_count);
    let sample_count_callback = Arc::clone(&sample_count);
    let non_silent_sample_count_callback = Arc::clone(&non_silent_sample_count);
    let source = DesktopCaptureSource::capture_mode(
        CaptureMode::Application(application_id),
        move |frame| {
            let samples = frame.buffer.as_slice();
            frame_count_callback.fetch_add(1, Ordering::Relaxed);
            sample_count_callback.fetch_add(samples.len() as u64, Ordering::Relaxed);
            non_silent_sample_count_callback.fetch_add(
                samples
                    .iter()
                    .filter(|sample| sample.abs() > 0.000_001)
                    .count() as u64,
                Ordering::Relaxed,
            );
        },
    )?;

    std::thread::sleep(Duration::from_secs(2));
    drop(source);

    println!("frames={}", frame_count.load(Ordering::Relaxed));
    println!("samples={}", sample_count.load(Ordering::Relaxed));
    println!(
        "non_silent_samples={}",
        non_silent_sample_count.load(Ordering::Relaxed)
    );
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("native_capture_probe requires Linux");
    std::process::exit(1);
}
