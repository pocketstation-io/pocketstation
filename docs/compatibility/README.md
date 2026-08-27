# Compatibility

PocketStation follows semantic versioning for its public Rust API. A compatible
minor or patch release keeps existing applications building against the normal
feature set. The `internal-testing` feature is not a public compatibility
promise.

The native library also preserves the versioned C ABI and PKSS sidecar protocol.
The repository stores their accepted layouts, symbols, and wire values in
[`c-abi-v1.baseline`](c-abi-v1.baseline).

Use the repository checks before changing a public boundary:

```bash
bash tools/check-api-compatibility.sh
bash tools/check-abi-compatibility.sh
```

The checks compare the candidate with a packaged earlier release. Do not make a
breaking change appear compatible by editing the baseline. Additive changes
still require the matching API, ABI, or protocol version update.
