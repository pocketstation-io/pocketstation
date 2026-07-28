fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let config_path = std::path::PathBuf::from(&crate_dir).join("cbindgen.toml");
    println!("cargo:rerun-if-changed={}", config_path.display());
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-env-changed=PKS_CODEC_C_HEADER_OUTPUT");

    let bindings = cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(cbindgen::Config::from_file(config_path).unwrap_or_default())
        .generate()
        .expect("generate codec C bindings");
    bindings.write_to_file(out_dir.join("pks_codec.h"));

    if let Some(explicit_output) = std::env::var_os("PKS_CODEC_C_HEADER_OUTPUT") {
        bindings.write_to_file(std::path::PathBuf::from(explicit_output));
    }
}
