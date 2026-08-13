fn main() {
    #[cfg(target_os = "macos")]
    {
        build_macos_capture_bridge();
        if std::env::var_os("CARGO_FEATURE_MACOS_ASP_DRIVER_ARTIFACT").is_some() {
            build_macos_asp_driver_artifact();
        }
    }
}

#[cfg(target_os = "macos")]
fn build_macos_capture_bridge() {
    use std::path::PathBuf;

    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let asp = manifest.join("native/macos/asp");

    cc::Build::new()
        .file(asp.join("shm_reader.c"))
        .include(&asp)
        .compile("pks_asp_reader");
    cc::Build::new()
        .file(asp.join("source_discovery.m"))
        .flag("-fobjc-arc")
        .flag("-ObjC")
        .include(&asp)
        .compile("pks_tap_source");
    cc::Build::new()
        .file(asp.join("authorization.m"))
        .flag("-fobjc-arc")
        .include(&asp)
        .compile("pks_capture_authorization");

    for file in [
        "shm_reader.c",
        "bridge.h",
        "SharedRing.h",
        "source_discovery.m",
        "source_discovery.h",
        "authorization.m",
        "authorization.h",
    ] {
        println!("cargo:rerun-if-changed={}", asp.join(file).display());
    }

    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=AppKit");
}

#[cfg(target_os = "macos")]
fn build_macos_asp_driver_artifact() {
    use std::path::PathBuf;

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let asp = manifest.join("native/macos/asp");
    for file in ["Plugin.cpp", "Info.plist", "SharedRing.h"] {
        println!("cargo:rerun-if-changed={}", asp.join(file).display());
    }

    let dylib = out_dir.join("PocketStationLoopback.dylib");
    let contents = out_dir.join("PocketStationLoopback.driver/Contents");
    let macos_dir = contents.join("MacOS");
    let sdk = std::env::var("SDKROOT").unwrap_or_else(|_| {
        let output = std::process::Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
            .expect("xcrun not found — install Xcode command-line tools");
        String::from_utf8(output.stdout)
            .expect("xcrun output is not UTF-8")
            .trim()
            .to_owned()
    });
    let coreaudio_headers = format!("{sdk}/System/Library/Frameworks/CoreAudio.framework/Headers");
    let status = std::process::Command::new("clang++")
        .args([
            "-std=c++17",
            "-O2",
            "-dynamiclib",
            "-isysroot",
            &sdk,
            "-I",
            &coreaudio_headers,
            "-framework",
            "CoreAudio",
            "-framework",
            "CoreFoundation",
            "-I",
            asp.to_str().expect("native path is UTF-8"),
            asp.join("Plugin.cpp")
                .to_str()
                .expect("plugin path is UTF-8"),
            "-o",
            dylib.to_str().expect("output path is UTF-8"),
        ])
        .status()
        .expect("clang++ not found — install Xcode command-line tools");
    assert!(status.success(), "Plugin.cpp compilation failed");

    std::fs::create_dir_all(&macos_dir).expect("create driver bundle");
    std::fs::copy(&dylib, macos_dir.join("PocketStationLoopback")).expect("copy driver binary");
    std::fs::copy(asp.join("Info.plist"), contents.join("Info.plist")).expect("copy driver plist");

    println!(
        "cargo:rustc-env=PKS_DRIVER_DYLIB={}",
        macos_dir.join("PocketStationLoopback").display()
    );
    println!(
        "cargo:rustc-env=PKS_DRIVER_PLIST={}",
        contents.join("Info.plist").display()
    );
}
