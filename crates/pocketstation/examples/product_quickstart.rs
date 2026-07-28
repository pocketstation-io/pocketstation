use std::error::Error;

use pocketstation as pks;

fn main() -> Result<(), Box<dyn Error>> {
    let session = pks::Session::new();
    let app = session.capture(pks::Source::application(pks::ApplicationSelector::name(
        "PocketStation Demo",
    )))?;
    let mic = session.capture(pks::Source::microphone_default())?;
    let app_audio = session.polled_audio()?;
    let mic_audio = session.polled_audio()?;

    app.send(app_audio)?;
    mic.send(mic_audio)?;

    let mut running = session.start()?;
    let outcome = running.stop();
    if !outcome.is_success() {
        return Err("PocketStation Session did not finalize cleanly".into());
    }
    Ok(())
}
