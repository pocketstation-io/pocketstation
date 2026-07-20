use std::env;
use std::error::Error;
use std::io;

use pks_audio as pks;

fn required_environment(name: &str) -> Result<String, io::Error> {
    env::var(name).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("required environment variable {name} is missing"),
        )
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let application_name = required_environment("PKS_APPLICATION_NAME")?;
    let connector_key = required_environment("PKS_CONNECTOR_KEY")?;
    let browser_receiver_uri = required_environment("PKS_BROWSER_RECEIVER_URI")?;

    let session = pks::Session::new();
    let application = session.capture(pks::Source::application(pks::ApplicationSelector::name(
        application_name,
    )));
    let microphone = session.capture(pks::Source::microphone_default());

    let example_connector = session.connector(pks::ConnectorKey::new(connector_key));
    let browser = session.browser(browser_receiver_uri);

    application.send(example_connector);
    application.send(browser);
    application.record("application");

    microphone.send(example_connector);
    microphone.send(browser);
    microphone.record("microphone");

    session.run().await?;
    Ok(())
}
