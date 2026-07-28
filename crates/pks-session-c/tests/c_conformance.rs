use std::path::{Path, PathBuf};
use std::process::Command;

fn compile_and_run_c_harness(source_path: &Path, executable_name: &str) {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let include_path = manifest_dir.join("include");
    let executable_dir = std::env::current_exe()
        .expect("resolve Rust conformance executable")
        .parent()
        .expect("resolve Cargo dependency directory")
        .to_path_buf();
    let output_path =
        std::env::temp_dir().join(format!("{executable_name}-{}", std::process::id()));
    let compiler = std::env::var("CC").unwrap_or_else(|_| "cc".to_owned());
    let mut command = Command::new(&compiler);
    command
        .arg(source_path)
        .arg("-I")
        .arg(&include_path)
        .arg("-L")
        .arg(&executable_dir)
        .arg("-lpks_session_c")
        .arg("-Werror")
        .arg("-o")
        .arg(&output_path);
    #[cfg(unix)]
    command.arg(format!("-Wl,-rpath,{}", executable_dir.to_string_lossy()));
    let output = command
        .output()
        .expect("compile and link Session C conformance");

    if !output.status.success() {
        panic!(
            "link Session C conformance failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let run = Command::new(&output_path)
        .output()
        .expect("execute Session C conformance");
    if !run.status.success() {
        panic!(
            "Session C conformance failed with status {:?}:\nstdout:\n{}\nstderr:\n{}",
            run.status.code(),
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr)
        );
    }
}

#[test]
fn given_public_c_harness_when_linked_and_executed_then_real_engine_compile_passes() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    compile_and_run_c_harness(
        &manifest_dir.join("tests").join("c_conformance.c"),
        "pks-session-c-conformance",
    );
}

#[test]
fn given_abi_1_0_metrics_buffer_when_polled_then_tail_canary_is_unchanged() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    compile_and_run_c_harness(
        &manifest_dir
            .join("tests")
            .join("c_abi_1_0_metrics_canary.c"),
        "pks-session-c-abi-1-0-metrics-canary",
    );
}
