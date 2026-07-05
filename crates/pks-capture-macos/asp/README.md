# asp/ — AudioServerPlugin (ASP) directory

## Status

Wave A stub: the `asp` Cargo feature is **off by default**.
`bridge_stub.c` is compiled unconditionally — it makes `asp_is_installed()`
always return `false`.

## Enabling the real ASP (human operator step)

1. Add the libASPL submodule:
   ```
   git submodule add https://github.com/appleasp/libASPL vendor/libASPL
   git submodule update --init --recursive
   ```
2. Build with the `asp` feature:
   ```
   cargo build -p pocketstation-loopback --features asp
   ```
3. Sign and install the built plugin:
   ```
   sudo cp target/release/libpks_asp.dylib \
        /Library/Audio/Plug-Ins/HAL/PocketStation.driver
   sudo killall coreaudiod
   ```
4. Verify: `asp_is_installed()` should return `true`.

## Files

| File | Purpose |
|------|---------|
| `bridge.h` | C header shared by stub and full implementation |
| `bridge_stub.c` | Compiled when `asp` feature is absent; returns 0 |
| `Plugin.cpp` | Compiled when `asp` feature is present; requires libASPL |
| `README.md` | This file |
