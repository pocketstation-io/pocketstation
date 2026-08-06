#![cfg(target_os = "macos")]

use std::path::PathBuf;
use std::process::Command;

#[test]
fn given_native_ring_contract_when_executed_then_visibility_and_drop_accounting_hold() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source = manifest_dir.join("tests/macos_native_ring_contract.c");
    let executable =
        std::env::temp_dir().join(format!("pks-native-ring-contract-{}", std::process::id()));
    let compiler = Command::new("xcrun")
        .args(["--find", "clang"])
        .output()
        .expect("xcrun must resolve the macOS C compiler");
    assert!(
        compiler.status.success(),
        "xcrun failed to resolve the macOS C compiler"
    );
    let compiler = String::from_utf8(compiler.stdout)
        .expect("xcrun compiler path must be UTF-8")
        .trim()
        .to_owned();
    let sdk = Command::new("xcrun")
        .arg("--show-sdk-path")
        .output()
        .expect("xcrun must resolve the macOS SDK");
    assert!(
        sdk.status.success(),
        "xcrun failed to resolve the macOS SDK"
    );
    let sdk = String::from_utf8(sdk.stdout)
        .expect("xcrun SDK path must be UTF-8")
        .trim()
        .to_owned();

    let compile = Command::new(compiler)
        .args([
            "-std=c11",
            "-O2",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pthread",
            "-isysroot",
        ])
        .arg(sdk)
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .output()
        .expect("native ring contract must compile");
    assert!(
        compile.status.success(),
        "native ring contract compilation failed: {}",
        String::from_utf8_lossy(&compile.stderr)
    );

    let result = Command::new(&executable)
        .output()
        .expect("native ring contract must execute");
    let _ = std::fs::remove_file(&executable);
    assert!(
        result.status.success(),
        "native ring contract failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}
