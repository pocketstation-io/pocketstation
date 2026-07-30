use pocketstation::{Session, Source};

#[test]
fn given_public_facade_when_session_declared_then_canonical_types_are_used() {
    let require_source: fn(Source) -> Source = |source| source;
    let _ = require_source(Source::microphone_default());

    let session_constructor = Session::new;
    let _ = session_constructor;

    let configured = Session::builder().recording_root("recordings").build();
    let _ = configured.id();
}
