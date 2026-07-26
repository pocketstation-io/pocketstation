use std::error::Error;

use pks_audio as pks;

/// Starts the product Session with host-owned capture and endpoint resources.
///
/// A real embedding application constructs `SessionEngine` with its concrete
/// connector, browser, and recorder drivers, then passes the target capture
/// backends here. The example remains provider- and transport-neutral.
pub fn start_product_session(
    engine: &pks::SessionEngine,
    capture_backends: pks::CaptureBackendSet<'_>,
    application_name: String,
    connector_operator_id: String,
    browser_receiver_uri: String,
) -> Result<pks::RunningSession, Box<dyn Error>> {
    let session = pks::Session::new();
    let application = session.capture(pks::Source::application(pks::ApplicationSelector::name(
        application_name,
    )))?;
    let microphone = session.capture(pks::Source::microphone_default())?;

    let example_connector = session.connector(
        pks::OperatorId::new(connector_operator_id),
        pks::EndpointConfiguration::new(),
    )?;
    let browser = session.browser(browser_receiver_uri)?;

    application.send(example_connector)?;
    application.send(browser)?;
    application.record("application")?;

    microphone.send(example_connector)?;
    microphone.send(browser)?;
    microphone.record("microphone")?;

    engine.start(session, capture_backends).map_err(Into::into)
}

fn main() {}
