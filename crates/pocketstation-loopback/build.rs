fn main() {
    // ASP bridge is macOS-only.
    #[cfg(target_os = "macos")]
    compile_asp_bridge();
}

#[cfg(target_os = "macos")]
fn compile_asp_bridge() {
    let asp_enabled = std::env::var("CARGO_FEATURE_ASP").is_ok();
    let mut build = cc::Build::new();

    if asp_enabled {
        build
            .cpp(true)
            .std("c++17")
            .include("asp")
            .include("vendor/libASPL/include")
            .define("POCKETSTATION_ASP_ENABLED", None)
            .file("asp/Plugin.cpp")
            .flag("-framework CoreAudio")
            .flag("-framework CoreFoundation");
    } else {
        build.file("asp/bridge_stub.c");
    }

    build.compile("pks_asp");

    println!("cargo:rerun-if-changed=asp/");
    println!("cargo:rerun-if-changed=vendor/libASPL/");
}
