# Compatibility

PocketStation follows semantic versioning for its public Rust API. A compatible
minor or patch release keeps existing applications building against the normal
feature set. The `internal-testing` feature is not a public compatibility
promise.

The native library also preserves the versioned C ABI and PKSS sidecar protocol.
The repository stores their accepted layouts, symbols, and wire values in
[`c-abi-v1.baseline`](c-abi-v1.baseline).

The default feature set includes desktop capture and the Opus codec. A native
application that only captures PCM can omit the codec and its native library:

```toml
pocketstation = { version = "1.1.10", default-features = false, features = ["native-capture"] }
```

Use the `opus-codec` feature when the application calls the Rust Opus API or
links the C Opus functions declared in `pocketstation.h`.

An application that already owns part of the host audio stack can select one
PocketStation driver directly. For example, a Linux application that already
links an incompatible ALSA crate can enable only PipeWire:

```toml
pocketstation = {
  version = "1.1.10",
  default-features = false,
  features = ["pipewire-capture"]
}
```

The corresponding macOS and Windows features are `coreaudio-capture` and
`wasapi-capture`. Use the normal `native-capture` feature when PocketStation
should select the platform driver and include the Linux `snd-aloop` fallback.

Use the repository checks before changing a public API or protocol:

```bash
bash tools/check-api-compatibility.sh
bash tools/check-abi-compatibility.sh
```

The checks compare the current package with an earlier published release. Do not make a
breaking change appear compatible by editing the baseline. Additive changes
still require the matching API, ABI, or protocol version update.
