use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let header_path = manifest_dir.join("include").join("pks_session.h");
    println!("cargo:rerun-if-changed={}", header_path.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let packaged_header_path = out_dir.join("pks_session.h");
    fs::copy(&header_path, &packaged_header_path).expect("copy packaged Session header");
}
