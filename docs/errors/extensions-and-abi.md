# Extension and ABI failures

<!-- claims: CLM-ERR-007-CAP-001,CLM-ERR-007-CAP-002,CLM-ERR-007-CAP-003,CLM-ERR-007-SOURCE-001,CLM-ERR-007-ERROR-0001,CLM-ERR-007-ERROR-0002,CLM-ERR-007-ERROR-0003,CLM-ERR-007-ERROR-0004,CLM-ERR-007-ERROR-0005,CLM-ERR-007-ERROR-0006,CLM-ERR-007-ERROR-0007,CLM-ERR-007-ERROR-0008,CLM-ERR-007-ERROR-0009,CLM-ERR-007-ERROR-0010,CLM-ERR-007-ERROR-0011,CLM-ERR-007-ERROR-0012,CLM-ERR-007-ERROR-0013,CLM-ERR-007-ERROR-0014,CLM-ERR-007-ERROR-0015,CLM-ERR-007-ERROR-0016,CLM-ERR-007-ERROR-0017 -->

## Scope

- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.
- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Error inventory

| Evidence record | Type | Variant | Retryable | Recoverable | Defined |
|---|---|---|---|---|---|
| error-1a38891e490c637fd1f2 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `UnsupportedAbiMinor` | unknown | unknown | `src/native_extension/mod.rs:87` |
| error-1a6a79718688c3bd3715 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | type | unknown | unknown | `src/native_extension/mod.rs:78` |
| error-1f5abab34214f5b63b01 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `InvalidLibraryDescriptor` | unknown | unknown | `src/native_extension/mod.rs:88` |
| error-1fc53703fe6fb00d8fab | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `InvalidRegistration` | unknown | unknown | `src/native_extension/mod.rs:91` |
| error-21780b72885c9e29a71c | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `RegistrationAcquisitionFailed` | unknown | unknown | `src/native_extension/mod.rs:90` |
| error-3ea568577af25a2fc5e6 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `DuplicateRegistration` | unknown | unknown | `src/native_extension/mod.rs:92` |
| error-480de969fd202f51c549 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `EntrypointFailed` | unknown | unknown | `src/native_extension/mod.rs:85` |
| error-527bf9a9b68ccbda6c90 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `EntrypointPanicked` | unknown | unknown | `src/native_extension/mod.rs:84` |
| error-6baafa30dcc94f68cad3 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `PathNotAbsolute` | unknown | unknown | `src/native_extension/mod.rs:79` |
| error-735365ce5d41572f06f2 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `PathCanonicalizationFailed` | unknown | unknown | `src/native_extension/mod.rs:80` |
| error-76f6466e465a0d7a52d9 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `PathNotFile` | unknown | unknown | `src/native_extension/mod.rs:81` |
| error-86e449bc7fc67119dab5 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `RegistrationAcquisitionPanicked` | unknown | unknown | `src/native_extension/mod.rs:89` |
| error-b09bbf6a2e20bddb6588 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `EntrypointMissing` | unknown | unknown | `src/native_extension/mod.rs:83` |
| error-bbda4884c178e8e9698d | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `UnsupportedAbiMajor` | unknown | unknown | `src/native_extension/mod.rs:86` |
| error-c7729f534adabf1f25cb | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `RegistrationStateUnavailable` | unknown | unknown | `src/native_extension/mod.rs:93` |
| error-c8625ea4d5c0a2384c45 | `pocketstation::native_extension::NativeExtensionLibraryError` | type | unknown | unknown | `src/native_extension/mod.rs:124` |
| error-fda081be8f949890171b | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `LibraryLoadFailed` | unknown | unknown | `src/native_extension/mod.rs:82` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Operate a Session through C](/docs/how-to/use-c-session-api.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/native_extension/library.rs:1-272` (`DIRECT`)
- `src/abi/session/error.rs:1-72` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
