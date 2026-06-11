fn main() {
    // shm_reader.c is the Rust-side POSIX shared memory reader.
    // Plugin.cpp (the coreaudiod HAL plugin) is built separately via asp/Makefile.
    #[cfg(target_os = "macos")]
    {
        cc::Build::new()
            .file("asp/shm_reader.c")
            .include("asp")
            .compile("pks_asp_reader");
        println!("cargo:rerun-if-changed=asp/shm_reader.c");
        println!("cargo:rerun-if-changed=asp/bridge.h");
        println!("cargo:rerun-if-changed=asp/SharedRing.h");
    }
}
