fn main() {
    #[cfg(target_os = "macos")]
    build_macos();
}

#[cfg(target_os = "macos")]
fn build_macos() {
    use std::path::PathBuf;

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let asp = manifest.join("asp");

    // shm_reader.c → static lib linked into the Rust binary (always needed).
    cc::Build::new()
        .file(asp.join("shm_reader.c"))
        .include(&asp)
        .compile("pks_asp_reader");

    println!("cargo:rerun-if-changed=asp/shm_reader.c");
    println!("cargo:rerun-if-changed=asp/bridge.h");
    println!("cargo:rerun-if-changed=asp/SharedRing.h");

    // Plugin.cpp → .dylib → .driver bundle (always on macOS).
    // The bundle bytes are embedded in the binary via include_bytes! in macos.rs.
    let dylib = out_dir.join("PocketStationLoopback.dylib");
    let bundle = out_dir.join("PocketStationLoopback.driver");
    let contents = bundle.join("Contents");
    let macos_dir = contents.join("MacOS");

    // Resolve the SDK root: prefer SDKROOT env (set by Xcode), fall back to xcrun.
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

    // AudioServerPlugIn.h lives inside CoreAudio.framework/Headers — add it
    // explicitly so the flat #include <CoreAudio/AudioServerPlugIn.h> resolves.
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
            asp.to_str().unwrap(),
            asp.join("Plugin.cpp").to_str().unwrap(),
            "-o",
            dylib.to_str().unwrap(),
        ])
        .status()
        .expect("clang++ not found — install Xcode command-line tools");
    assert!(status.success(), "Plugin.cpp compilation failed");

    std::fs::create_dir_all(&macos_dir).unwrap();
    std::fs::copy(&dylib, macos_dir.join("PocketStationLoopback")).unwrap();
    std::fs::copy(asp.join("Info.plist"), contents.join("Info.plist")).unwrap();

    println!("cargo:rerun-if-changed=asp/Plugin.cpp");
    println!("cargo:rerun-if-changed=asp/Info.plist");

    // Expose bundle file paths for include_bytes! in macos.rs.
    println!(
        "cargo:rustc-env=PKS_DRIVER_DYLIB={}",
        macos_dir.join("PocketStationLoopback").display()
    );
    println!(
        "cargo:rustc-env=PKS_DRIVER_PLIST={}",
        contents.join("Info.plist").display()
    );
}
