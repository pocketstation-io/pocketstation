use std::path::PathBuf;
use std::process::Command;

#[test]
fn given_versioned_process_set_c_contract_when_linked_then_declaration_is_selected_only() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let library_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let executable = std::env::temp_dir().join(format!("pks-process-set-c-{}", std::process::id()));
    let mut command = Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()));
    command
        .args(["-std=c11", "-Wall", "-Wextra", "-Werror"])
        .arg(root.join("tests/abi_process_set_c_conformance.c"))
        .arg("-I")
        .arg(root.join("include"))
        .arg("-L")
        .arg(&library_dir)
        .arg("-lpocketstation")
        .arg("-o")
        .arg(&executable);
    #[cfg(unix)]
    command.arg(format!("-Wl,-rpath,{}", library_dir.display()));
    let compile = command.output().unwrap();
    assert!(
        compile.status.success(),
        "C compile failed: {}",
        String::from_utf8_lossy(&compile.stderr)
    );
    let run = Command::new(&executable).output().unwrap();
    let _ = std::fs::remove_file(executable);
    assert!(
        run.status.success(),
        "C consumer failed {:?}: {}",
        run.status.code(),
        String::from_utf8_lossy(&run.stderr)
    );
}
