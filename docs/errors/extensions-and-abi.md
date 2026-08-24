# Extension and ABI failures

<!-- claims: CLM-ERR-007-SCOPE-001,CLM-ERR-007-TEXT-001,CLM-ERR-007-TEXT-002,CLM-ERR-007-SOURCE-001 -->

## Scope

- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.
- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Extension and ABI failures** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Extension and ABI failures**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Error inventory

| Evidence record | Type | Variant | Trigger | Developer action | Retryable | Retry basis | Recoverable | Recovery action | Test status | Tests | Defined |
|---|---|---|---|---|---|---|---|---|---|---|---|
| error-23644427ccfe2e45879e | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `InvalidLibraryDescriptor` | Classifies the invalid library descriptor failure case at the owning boundary. | Correct the invalid library descriptor condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the invalid library descriptor condition reported by the returned fields before repeating the operation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:88` |
| error-26b0a33c4952ffa51dc0 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `PathCanonicalizationFailed` | Classifies the path canonicalization failed failure case at the owning boundary. | Preserve `PathCanonicalizationFailed` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `PathCanonicalizationFailed` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:80` |
| error-30fb4ebaa151784596bc | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `InvalidRegistration` | Classifies the invalid registration failure case at the owning boundary. | Correct the invalid registration condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the invalid registration condition reported by the returned fields before repeating the operation. | linked | test-201890b5e1a3afb22ef5 | `src/native_extension/mod.rs:91` |
| error-34be8fc6e60997a8cd44 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `UnsupportedAbiMinor` | Classifies the unsupported abi minor failure case at the owning boundary. | Correct the unsupported abi minor condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the unsupported abi minor condition reported by the returned fields before repeating the operation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:87` |
| error-5417f71ea67cf7312a39 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `RegistrationStateUnavailable` | Classifies the registration state unavailable failure case at the owning boundary. | Preserve `RegistrationStateUnavailable` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `RegistrationStateUnavailable` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:93` |
| error-59bfaa1be8a652b3c7b6 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `PathNotAbsolute` | Classifies the path not absolute failure case at the owning boundary. | Preserve `PathNotAbsolute` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `PathNotAbsolute` and inspect the owning operation's typed fields before choosing recovery or presentation. | linked | test-de03d325dde793e9ec1b | `src/native_extension/mod.rs:79` |
| error-77bedea908d2b38f8807 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `EntrypointMissing` | Classifies the entrypoint missing failure case at the owning boundary. | Preserve `EntrypointMissing` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `EntrypointMissing` and inspect the owning operation's typed fields before choosing recovery or presentation. | linked | test-4678c1b527bd72fa5830 | `src/native_extension/mod.rs:83` |
| error-8c56cdf18bee2fc21bae | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | type | Returned by operations whose signature names pocketstation::native_extension::NativeExtensionLibraryErrorCode. | Preserve `NativeExtensionLibraryErrorCode` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `NativeExtensionLibraryErrorCode` and inspect the owning operation's typed fields before choosing recovery or presentation. | linked | test-201890b5e1a3afb22ef5, test-4678c1b527bd72fa5830, test-527acebb13bb37c6f430, test-5d5feb4cbedf509d8146, test-de03d325dde793e9ec1b | `src/native_extension/mod.rs:78` |
| error-971353c5ff9dde1692e9 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `DuplicateRegistration` | Classifies the duplicate registration failure case at the owning boundary. | Correct the duplicate registration condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the duplicate registration condition reported by the returned fields before repeating the operation. | linked | test-5d5feb4cbedf509d8146 | `src/native_extension/mod.rs:92` |
| error-b6f16e992a8c85677bc8 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `EntrypointPanicked` | Classifies the entrypoint panicked failure case at the owning boundary. | Preserve `EntrypointPanicked` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `EntrypointPanicked` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:84` |
| error-c40c70d459d226d51a4a | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `PathNotFile` | Classifies the path not file failure case at the owning boundary. | Preserve `PathNotFile` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `PathNotFile` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:81` |
| error-c790f0651b2af49daa0d | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `RegistrationAcquisitionFailed` | Classifies the registration acquisition failed failure case at the owning boundary. | Preserve `RegistrationAcquisitionFailed` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `RegistrationAcquisitionFailed` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:90` |
| error-d97432d4b6bf9216a606 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `LibraryLoadFailed` | Classifies the library load failed failure case at the owning boundary. | Preserve `LibraryLoadFailed` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `LibraryLoadFailed` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:82` |
| error-da5a536d4d262a1e066a | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `UnsupportedAbiMajor` | Classifies the unsupported abi major failure case at the owning boundary. | Correct the unsupported abi major condition reported by the returned fields before repeating the operation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Correct the unsupported abi major condition reported by the returned fields before repeating the operation. | linked | test-527acebb13bb37c6f430, test-ee6a03f0db459d8f2e98 | `src/native_extension/mod.rs:86` |
| error-ecf350aa6c55c67611e2 | `pocketstation::native_extension::NativeExtensionLibraryError` | type | Returned by operations whose signature names pocketstation::native_extension::NativeExtensionLibraryError. | Preserve `NativeExtensionLibraryError` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `NativeExtensionLibraryError` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:124` |
| error-ef9532d3aedcc9d33105 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `RegistrationAcquisitionPanicked` | Classifies the registration acquisition panicked failure case at the owning boundary. | Preserve `RegistrationAcquisitionPanicked` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `RegistrationAcquisitionPanicked` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:89` |
| error-f77a66c0a6491320325a | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | `EntrypointFailed` | Classifies the entrypoint failed failure case at the owning boundary. | Preserve `EntrypointFailed` and inspect the owning operation's typed fields before choosing recovery or presentation. | not_declared | No retry guarantee is inferred from an error or variant name. | not_declared | Preserve `EntrypointFailed` and inspect the owning operation's typed fields before choosing recovery or presentation. | no_direct_test_link_extracted | none linked | `src/native_extension/mod.rs:85` |

## Interpretation

The **Extension and ABI failures** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

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

The claims on **Extension and ABI failures** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/native_extension/library.rs:19-19` (`DIRECT`)
- `src/native_extension/library.rs:21-24` (`DIRECT`)
- `src/native_extension/library.rs:22-22` (`DIRECT`)
- `src/native_extension/library.rs:23-23` (`DIRECT`)
- `src/native_extension/library.rs:33-230` (`DIRECT`)
- `src/native_extension/library.rs:232-263` (`DIRECT`)
- `src/native_extension/library.rs:265-271` (`DIRECT`)
- `src/abi/session/error.rs:3-3` (`DIRECT`)
- `src/abi/session/error.rs:3-3` (`DIRECT`)
- `src/abi/session/error.rs:3-3` (`DIRECT`)
- `src/abi/session/error.rs:3-3` (`DIRECT`)
- `src/abi/session/error.rs:4-35` (`DIRECT`)
- `src/abi/session/error.rs:6-6` (`DIRECT`)
- `src/abi/session/error.rs:8-8` (`DIRECT`)
- `src/abi/session/error.rs:10-10` (`DIRECT`)
- `src/abi/session/error.rs:12-12` (`DIRECT`)
- `src/abi/session/error.rs:14-14` (`DIRECT`)
- `src/abi/session/error.rs:16-16` (`DIRECT`)
- `src/abi/session/error.rs:18-18` (`DIRECT`)
- `src/abi/session/error.rs:20-20` (`DIRECT`)
- `src/abi/session/error.rs:22-22` (`DIRECT`)
- `src/abi/session/error.rs:24-24` (`DIRECT`)
- `src/abi/session/error.rs:26-26` (`DIRECT`)
- `src/abi/session/error.rs:28-28` (`DIRECT`)
- `src/abi/session/error.rs:30-30` (`DIRECT`)
- `src/abi/session/error.rs:32-32` (`DIRECT`)
- `src/abi/session/error.rs:34-34` (`DIRECT`)
- `src/abi/session/error.rs:38-70` (`DIRECT`)

For **Extension and ABI failures**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
