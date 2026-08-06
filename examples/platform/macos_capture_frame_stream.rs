#[cfg(target_os = "macos")]
mod macos {
    use std::time::{Duration, Instant};

    use pks_capture::{captured_frame_stream, CaptureMode};
    use pks_capture_macos::SystemLoopbackSource;

    fn usage() -> ! {
        eprintln!("usage: capture_frame_stream <pid> [duration-seconds]");
        std::process::exit(2);
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut arguments = std::env::args().skip(1);
        let pid: u32 = arguments.next().unwrap_or_else(|| usage()).parse()?;
        let duration_s: u64 = arguments.next().unwrap_or_else(|| "3".to_owned()).parse()?;
        if arguments.next().is_some() {
            usage();
        }

        let (sender, mut frames) = captured_frame_stream(4)?;
        let capture =
            SystemLoopbackSource::capture_mode(CaptureMode::Process(pid), sender.into_callback())?;
        let deadline = Instant::now() + Duration::from_secs(duration_s);
        let mut frame_count = 0_u64;
        let mut sample_count = 0_u64;
        let mut sum_squares = 0.0_f64;

        while Instant::now() < deadline {
            if let Some(frame) = frames.try_next() {
                frame_count += 1;
                sample_count += frame.buffer.len() as u64;
                sum_squares += frame
                    .buffer
                    .as_slice()
                    .iter()
                    .map(|sample| f64::from(*sample) * f64::from(*sample))
                    .sum::<f64>();
            } else {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        drop(capture);

        if frame_count == 0 || sample_count == 0 {
            return Err("capture stream produced no frames".into());
        }
        let rms = (sum_squares / sample_count as f64).sqrt();
        let stats = frames.stats();
        println!(
            "frames={} samples={} rms={:.6} delivered={} dropped_newest={}",
            frame_count, sample_count, rms, stats.delivered_frames, stats.dropped_newest_frames
        );
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    macos::run()
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("capture_frame_stream is available only on macOS");
}
