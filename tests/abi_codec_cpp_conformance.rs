use std::path::PathBuf;
use std::process::Command;

#[test]
fn given_cpp_consumer_when_linked_then_codec_symbols_keep_c_linkage() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source_path = manifest_dir
        .join("tests")
        .join("abi_codec_cpp_conformance.cpp");
    let include_path = manifest_dir.join("include");
    let executable_dir = std::env::current_exe()
        .expect("resolve Rust conformance executable")
        .parent()
        .expect("resolve Cargo dependency directory")
        .to_path_buf();
    let output_path = std::env::temp_dir().join(format!(
        "pocketstation-abi-cpp-conformance-{}",
        std::process::id()
    ));
    let compiler = std::env::var("CXX").unwrap_or_else(|_| "c++".to_owned());
    let mut command = Command::new(&compiler);
    command
        .arg("-std=c++17")
        .arg(&source_path)
        .arg("-I")
        .arg(&include_path)
        .arg("-L")
        .arg(&executable_dir)
        .arg("-lpocketstation")
        .arg("-Werror")
        .arg("-o")
        .arg(&output_path);
    #[cfg(unix)]
    command.arg(format!("-Wl,-rpath,{}", executable_dir.to_string_lossy()));
    let output = command
        .output()
        .expect("compile and link codec C++ fixture");

    if !output.status.success() {
        panic!(
            "link codec C++ fixture failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let run = Command::new(&output_path)
        .output()
        .expect("execute codec C++ fixture");
    if !run.status.success() {
        panic!(
            "codec C++ fixture failed with status {:?}:\nstdout:\n{}\nstderr:\n{}",
            run.status.code(),
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr)
        );
    }
}
