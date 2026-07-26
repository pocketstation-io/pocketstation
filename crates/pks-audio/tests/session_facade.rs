use pks_audio::{Session, SessionEngine};

fn require_canonical_session(_session: pks_session::Session) {}
fn require_canonical_engine(_engine: &pks_session::SessionEngine) {}

#[test]
fn given_public_facade_when_session_types_used_then_canonical_types_are_exported() {
    require_canonical_session(Session::new());

    let require_engine: fn(&SessionEngine) = require_canonical_engine;
    let _ = require_engine;
}
