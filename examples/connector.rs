use std::error::Error;
use std::time::Duration;

use pocketstation::connector::Connector;
use pocketstation::{Session, Source};

fn main() -> Result<(), Box<dyn Error>> {
    let application = std::env::args()
        .nth(1)
        .ok_or("provide the running application name or identifier")?;
    let session = Session::new();
    let destination = session.destination(Connector::from_audio_fn(|frame| {
        println!(
            "source={} sequence={} samples={}",
            frame.source_id().get(),
            frame.sequence_number(),
            frame.samples().len()
        );
        Ok(())
    })?)?;

    session
        .capture(Source::application(application))?
        .send(destination)?;
    let mut running = session.start()?;
    std::thread::sleep(Duration::from_secs(5));
    let outcome = running.stop();
    if !outcome.is_success() {
        return Err("Session did not stop successfully".into());
    }
    Ok(())
}
