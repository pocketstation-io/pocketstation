use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use pocketstation::native_extension::{
    NativeExtensionKind, NativeExtensionLibraryErrorCode, EXTENSION_LIBRARY_ENTRYPOINT_V1,
};
use pocketstation::{
    EndpointDescriptor, NodeTypeId, Operator, OperatorConfiguration, OperatorId, Session,
    SourceConfiguration, SourceTypeId,
};
use tempfile::TempDir;

const SOURCE_ID: &str = "dev.pocketstation.source.fixture.v1";
const OPERATOR_ID: &str = "dev.pocketstation.fixture.operator.v1";
const ENDPOINT_ID: &str = "dev.pocketstation.fixture.endpoint.v1";

struct CompiledPlugin {
    _directory: TempDir,
    path: PathBuf,
    marker: PathBuf,
}

fn compile_plugin(name: &str, cfg: Option<&str>) -> CompiledPlugin {
    let directory = tempfile::tempdir().expect("fixture temp directory");
    let marker = directory.path().join("lifecycle.log");
    let path = directory.path().join(format!(
        "{}{}{}",
        std::env::consts::DLL_PREFIX,
        name,
        std::env::consts::DLL_SUFFIX
    ));
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/native_extension_plugin.rs");
    let mut command = Command::new("rustc");
    command
        .arg("--crate-type=cdylib")
        .arg("--edition=2021")
        .arg("-C")
        .arg("debuginfo=0")
        .arg(&source)
        .arg("-o")
        .arg(&path)
        .env("PKS_FIXTURE_MARKER", &marker);
    if let Some(cfg) = cfg {
        command.arg("--cfg").arg(cfg);
    }
    let output = command.output().expect("run rustc for fixture plugin");
    assert!(
        output.status.success(),
        "fixture plugin failed to compile:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    CompiledPlugin {
        _directory: directory,
        path,
        marker,
    }
}

fn marker_text(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

#[test]
fn given_relative_library_path_when_loaded_then_ambient_search_is_rejected() {
    // SAFETY: the invalid relative path is rejected before any native code is loaded.
    let error = unsafe { Session::new().load_native_extension_library("fixture-extension") }
        .expect_err("relative library path must be rejected");

    assert_eq!(
        error.code(),
        NativeExtensionLibraryErrorCode::PathNotAbsolute
    );
}

#[test]
fn given_library_without_entrypoint_when_loaded_then_typed_error_is_returned() {
    let plugin = compile_plugin("pks_missing_entrypoint", Some("no_entrypoint"));
    // SAFETY: the test compiled this local fixture and controls all of its code.
    let error = unsafe { Session::new().load_native_extension_library(&plugin.path) }
        .expect_err("missing extension entrypoint must fail");

    assert_eq!(
        error.code(),
        NativeExtensionLibraryErrorCode::EntrypointMissing
    );
    assert!(error.message().contains(EXTENSION_LIBRARY_ENTRYPOINT_V1));
}

#[test]
fn given_unsupported_library_abi_when_loaded_then_registration_never_mutates_session() {
    let plugin = compile_plugin("pks_unsupported_abi", Some("unsupported_abi"));
    let session = Session::new();
    // SAFETY: the test compiled this local fixture and controls all of its code.
    let error = unsafe { session.load_native_extension_library(&plugin.path) }
        .expect_err("unsupported ABI must fail before acquisition");

    assert_eq!(
        error.code(),
        NativeExtensionLibraryErrorCode::UnsupportedAbiMajor
    );
    assert!(marker_text(&plugin.marker).is_empty());
}

#[test]
fn given_acquired_malformed_registration_when_loaded_then_context_is_destroyed_once() {
    let plugin = compile_plugin("pks_invalid_registration", Some("invalid_registration"));
    // SAFETY: the test compiled this local fixture and deliberately controls
    // the malformed descriptor while keeping all referenced memory valid.
    let error = unsafe { Session::new().load_native_extension_library(&plugin.path) }
        .expect_err("malformed acquired registration must fail");

    assert_eq!(
        error.code(),
        NativeExtensionLibraryErrorCode::InvalidRegistration
    );
    let lifecycle = marker_text(&plugin.marker);
    assert_eq!(lifecycle.matches("destroy_instance:").count(), 0);
    assert_eq!(lifecycle.matches("destroy_registration:").count(), 1);
}

#[test]
fn given_valid_native_library_when_loaded_then_canonical_session_executes_complete_pipeline() {
    let plugin = compile_plugin("pks_valid_extension", None);
    let session = Session::new();
    // SAFETY: the test compiled this ABI-conformant local fixture and controls
    // its code, descriptors, callbacks, contexts, and lifetimes.
    let receipt = unsafe { session.load_native_extension_library(&plugin.path) }
        .expect("load native extension library");

    assert!(receipt.canonical_path().is_absolute());
    assert_eq!(receipt.registrations().len(), 3);
    assert_eq!(receipt.registrations()[0].id(), SOURCE_ID);
    assert_eq!(
        receipt.registrations()[0].kind(),
        NativeExtensionKind::Source
    );
    assert_eq!(receipt.registrations()[0].revision(), 1);
    assert_eq!(receipt.registrations()[0].generation(), 1);

    let source = session
        .source(
            SourceTypeId::new(SOURCE_ID).expect("source type id"),
            SourceConfiguration::default(),
        )
        .expect("declare acquired source");
    let operator = session
        .operator(Operator::new(
            OperatorId::new(OPERATOR_ID),
            OperatorConfiguration::new(),
        ))
        .expect("declare acquired operator");
    source
        .output("out")
        .expect("source output")
        .connect(operator.input("in").expect("operator input"))
        .expect("source to operator route");
    let endpoint = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(ENDPOINT_ID),
            OperatorId::new(ENDPOINT_ID),
        ))
        .expect("declare acquired endpoint");
    operator
        .output("out")
        .expect("operator output")
        .send_to(endpoint, Some("in".to_owned()))
        .expect("operator to endpoint route");

    let mut running = session.start().expect("start canonical Session");
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline && !marker_text(&plugin.marker).contains("consume:hello") {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(
        marker_text(&plugin.marker).contains("consume:hello"),
        "acquired endpoint did not consume the source/operator payload"
    );
    assert_eq!(running.external_source_metrics().len(), 1);
    assert_eq!(running.operator_metrics().len(), 1);
    assert!(running.stop().is_success());
    drop(running);

    let lifecycle = marker_text(&plugin.marker);
    assert_eq!(lifecycle.matches("destroy_instance:").count(), 3);
    assert_eq!(lifecycle.matches("destroy_registration:").count(), 3);
}

#[test]
fn given_duplicate_library_import_when_loaded_then_second_import_is_transactional() {
    let plugin = compile_plugin("pks_duplicate_import", None);
    let session = Session::new();
    // SAFETY: the test compiled this ABI-conformant local fixture and controls
    // its code, descriptors, callbacks, contexts, and lifetimes.
    unsafe { session.load_native_extension_library(&plugin.path) }.expect("first import");

    // SAFETY: this is the same controlled local fixture; the second import is
    // expected to fail during duplicate registration validation.
    let error = unsafe { session.load_native_extension_library(&plugin.path) }
        .expect_err("second import must fail atomically");
    assert_eq!(
        error.code(),
        NativeExtensionLibraryErrorCode::DuplicateRegistration
    );
    drop(session);

    let lifecycle = marker_text(&plugin.marker);
    assert_eq!(lifecycle.matches("destroy_instance:").count(), 6);
    assert_eq!(lifecycle.matches("destroy_registration:").count(), 6);
}
