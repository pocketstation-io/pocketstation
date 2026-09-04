use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use pocketstation as pks;

struct Options {
    application: Option<String>,
    system_audio: bool,
    microphone: bool,
    recording_root: Option<PathBuf>,
    duration: Option<Duration>,
}

fn options() -> Result<Option<Options>, Box<dyn Error>> {
    let mut application = None;
    let mut system_audio = false;
    let mut microphone = false;
    let mut recording_root = None;
    let mut duration = None;
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--application" => {
                application = Some(
                    arguments
                        .next()
                        .ok_or("--application requires a name, identifier, or process ID")?,
                );
            }
            "--system-audio" => system_audio = true,
            "--microphone" => microphone = true,
            "--record" => {
                recording_root = Some(PathBuf::from(
                    arguments
                        .next()
                        .ok_or("--record requires an output directory")?,
                ));
            }
            "--duration" => {
                let seconds = arguments
                    .next()
                    .ok_or("--duration requires a positive number of seconds")?
                    .parse::<f64>()?;
                if !seconds.is_finite() || seconds <= 0.0 {
                    return Err("--duration must be a positive number of seconds".into());
                }
                duration = Some(Duration::from_secs_f64(seconds));
            }
            "--help" | "-h" => {
                println!(
                    "Usage: quickstart [--application <name-or-id>] \
                     [--system-audio] [--microphone] [--record <directory>] \
                     [--duration <seconds>]"
                );
                return Ok(None);
            }
            _ => return Err(format!("unknown argument: {argument}").into()),
        }
    }
    if system_audio && application.is_some() {
        return Err("--system-audio and --application cannot be used together".into());
    }
    Ok(Some(Options {
        application,
        system_audio,
        microphone,
        recording_root,
        duration,
    }))
}

fn choose_application(query: Option<&str>) -> Result<pks::ApplicationSelector, Box<dyn Error>> {
    let mut sources: Vec<_> = pks::discover_sources()
        .into_iter()
        .filter(|source| source.stable_id.kind == pks::SourceKind::Application)
        .collect();
    sources.sort_by(|left, right| left.name.cmp(&right.name));
    if sources.is_empty() {
        return Err("no running desktop applications were discovered".into());
    }

    let selected = if let Some(query) = query {
        let process_id = query.parse::<u32>().ok();
        let mut matches: Vec<_> = sources
            .into_iter()
            .filter(|source| {
                process_id.is_some_and(|value| source.process_id == Some(value))
                    || source.name.eq_ignore_ascii_case(query)
                    || source
                        .app_id
                        .as_deref()
                        .is_some_and(|value| value.eq_ignore_ascii_case(query))
            })
            .collect();
        if matches.len() != 1 {
            return Err(format!(
                "--application must match exactly one running application; found {}",
                matches.len()
            )
            .into());
        }
        matches.remove(0)
    } else {
        println!("Choose a running application:");
        for (index, source) in sources.iter().enumerate() {
            println!("  {}. {}", index + 1, source.name);
        }
        print!("Application [1-{}]: ", sources.len());
        io::stdout().flush()?;
        let mut selection = String::new();
        io::stdin().read_line(&mut selection)?;
        let index = selection
            .trim()
            .parse::<usize>()
            .map_err(|_| "enter the number shown beside the application")?;
        if !(1..=sources.len()).contains(&index) {
            return Err("the selected application number is outside the displayed range".into());
        }
        sources.remove(index - 1)
    };

    Ok(match selected.process_id {
        Some(process_id) => pks::ApplicationSelector::process_instance(
            pks::ProcessId::new(process_id),
            selected.stable_id,
        ),
        None => pks::ApplicationSelector::stable_id(selected.stable_id),
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let Some(options) = options()? else {
        return Ok(());
    };
    let mut builder = pks::Session::builder();
    if let Some(root) = options.recording_root.as_ref() {
        builder = builder.recording_root(root);
    }
    let session = builder.build();
    let primary = if options.system_audio {
        session.capture(pks::Source::system_audio())?
    } else {
        session.capture(pks::Source::application(choose_application(
            options.application.as_deref(),
        )?))?
    };
    primary.send(session.polled_audio()?)?;
    if options.recording_root.is_some() {
        primary.record(if options.system_audio {
            "system"
        } else {
            "application"
        })?;
    }

    let expected_stems = if options.microphone {
        let microphone = session.capture(pks::Source::microphone_default())?;
        microphone.send(session.polled_audio()?)?;
        if options.recording_root.is_some() {
            microphone.record("microphone")?;
        }
        2
    } else {
        1
    };

    let mut running = session.start()?;
    let deadline = Instant::now() + options.duration.unwrap_or(Duration::from_secs(10));
    let mut frames_by_stem = BTreeMap::<u64, usize>::new();
    while Instant::now() < deadline
        && (options.duration.is_some()
            || frames_by_stem.values().filter(|count| **count >= 2).count() < expected_stems)
    {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let frame = batch
                    .frame(index)
                    .ok_or("bounded audio batch returned an invalid frame index")?;
                *frames_by_stem
                    .entry(frame.lineage().stem_id().get())
                    .or_default() += 1;
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    if frames_by_stem.values().filter(|count| **count >= 2).count() != expected_stems {
        return Err("the selected sources did not produce media before the deadline".into());
    }

    let outcome = running.stop();
    if !outcome.is_success() {
        return Err("PocketStation Session did not finalize cleanly".into());
    }
    if options.recording_root.is_some() {
        let recording = running
            .recording_outcome()
            .ok_or("PocketStation Session did not expose a recording outcome")?;
        if recording.state != pks::SessionRecordingState::Complete
            || recording.completed_stems != expected_stems
            || recording.failed_stems != 0
        {
            return Err("PocketStation multistem recording did not complete".into());
        }
    }
    Ok(())
}
