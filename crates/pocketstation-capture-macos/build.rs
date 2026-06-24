fn main() {
    #[cfg(target_os = "macos")]
    build_macos();
}

#[cfg(target_os = "macos")]
fn build_macos() {
    use std::path::PathBuf;

    let out_dir  = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let asp      = manifest.join("asp");

    // shm_reader.c — POSIX SHM reader for the ASP fallback path (macOS < 14.2).
    cc::Build::new()
        .file(asp.join("shm_reader.c"))
        .include(&asp)
        .compile("pks_asp_reader");

    println!("cargo:rerun-if-changed=asp/shm_reader.c");
    println!("cargo:rerun-if-changed=asp/bridge.h");
    println!("cargo:rerun-if-changed=asp/SharedRing.h");

    // source_discovery.m — CoreAudio process tap source enumeration and capture.
    // Objective-C, compiled with ARC enabled.  Requires macOS 14.2 SDK or later.
    cc::Build::new()
        .file(asp.join("source_discovery.m"))
        .flag("-fobjc-arc")
        .flag("-ObjC")
        .include(&asp)
        .compile("pks_tap_source");

    println!("cargo:rerun-if-changed=asp/source_discovery.m");
    println!("cargo:rerun-if-changed=asp/source_discovery.h");

    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=AppKit");

    // Plugin.cpp → HAL driver dylib → embedded in binary as the ASP fallback.
    let dylib  = out_dir.join("PocketStationLoopback.dylib");
    let bundle = out_dir.join("PocketStationLoopback.driver");
    let contents = bundle.join("Contents");
    let macos_dir = contents.join("MacOS");

    let sdk = std::env::var("SDKROOT").unwrap_or_else(|_| {
        let out = std::process::Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
            .expect("xcrun not found — install Xcode command-line tools");
        String::from_utf8(out.stdout)
            .expect("xcrun output is not UTF-8")
            .trim()
            .to_owned()
    });

    let coreaudio_headers = format!(
        "{sdk}/System/Library/Frameworks/CoreAudio.framework/Headers"
    );

    let status = std::process::Command::new("clang++")
        .args([
            "-std=c++17", "-O2",
            "-dynamiclib",
            "-isysroot", &sdk,
            "-I", &coreaudio_headers,
            "-framework", "CoreAudio",
            "-framework", "CoreFoundation",
            "-I", asp.to_str().unwrap(),
            asp.join("Plugin.cpp").to_str().unwrap(),
            "-o", dylib.to_str().unwrap(),
        ])
        .status()
        .expect("clang++ not found — install Xcode command-line tools");
    assert!(status.success(), "Plugin.cpp compilation failed");

    std::fs::create_dir_all(&macos_dir).unwrap();
    std::fs::copy(&dylib, macos_dir.join("PocketStationLoopback")).unwrap();
    std::fs::copy(asp.join("Info.plist"), contents.join("Info.plist")).unwrap();

    println!("cargo:rerun-if-changed=asp/Plugin.cpp");
    println!("cargo:rerun-if-changed=asp/Info.plist");

    println!(
        "cargo:rustc-env=PKS_DRIVER_DYLIB={}",
        macos_dir.join("PocketStationLoopback").display()
    );
    println!(
        "cargo:rustc-env=PKS_DRIVER_PLIST={}",
        contents.join("Info.plist").display()
    );
}
