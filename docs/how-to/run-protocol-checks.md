# Run protocol checks

<!-- claims: CLM-GUIDE-025-CAP-001,CLM-GUIDE-025-CAP-002,CLM-GUIDE-025-SOURCE-001 -->

## Scope

- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.
- **Build and publish repository artifacts.** Run architecture, protocol, package, platform, and release checks used by the repository publication workflow.

The scope of **Run protocol checks** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

The repository toolchain plus every public, sibling, or private fixture named by the selected check.

## Procedure

1. Run the repository protocol script.
2. Run C ABI and compatibility tests selected by CI.
3. Provide required sibling or private fixtures.
4. Distinguish absent prerequisites from assertion failures.
5. Record command, target, and fixture revision.

## Important consequence

Separate a missing prerequisite, compilation error, assertion failure, and unqualified physical path.

## Verify the outcome

Each command exits successfully and its target, feature set, fixture revision, and evidence scope are recorded.

Executable evidence selected for **Run protocol checks** is limited to each test's recorded setup and assertions:

- `test-publish-recovery` — test publish recovery (`scripts/test-publish-recovery.sh:1`; `test-adb1dc831caefa6dcbaa`).
- `test-session-c-conformance` — test session c conformance (`scripts/test-session-c-conformance.sh:1`; `test-ff28fea505d22371ce41`).
- `test-single-package-publish` — test single package publish (`scripts/test-single-package-publish.sh:1`; `test-2cb6c90c68d9362041c6`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-041037b5b9482d79c8e2`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-d3686b94180b732c8001`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` — given panicking abi bodies when guarded then panics are contained (`src/abi/codec.rs:373`; `test-03d685383aaeadb55cad`).
- `given_rejected_capacity_when_retried_then_encoder_state_is_unchanged` — given rejected capacity when retried then encoder state is unchanged (`src/abi/codec.rs:323`; `test-d02294e14bc1e7d6bfd2`).
- `given_sine_440hz_when_round_trip_then_decoded_has_energy` — given sine 440hz when round trip then decoded has energy (`src/abi/codec.rs:435`; `test-3e20a259ad1a0f55a8c8`).

## Failure signals

No task-specific public error was resolved for run protocol checks; preserve the owning API's returned error.

## API reference

- [Conformance](/docs/concepts/conformance.md)
- [Test Evidence](/docs/reference/test-evidence.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::conformance` | module | Deterministic canonical-engine fixture for external conformance harnesses. | `src/conformance.rs:1` |
| `pocketstation::conformance::ExtensionConformanceReport` | struct | Language-neutral outcome returned by the W20 fixture. | `src/conformance.rs:573` |
| `pocketstation::conformance::ExtensionSignal` | struct | Owns one signal payload used by the native-extension conformance fixtures. | `src/conformance.rs:1181` |
| `pocketstation::conformance::ObservedEndpointError` | enum | Classifies failures reported as observed endpoint error. | `src/conformance.rs:345` |
| `pocketstation::conformance::observed_browser` | function | Declares and registers a deterministic native browser boundary used only by cross-language conformance harnesses. | `src/conformance.rs:335` |
| `pocketstation::conformance::observed_connector` | function | Declares and registers a deterministic native connector used only by cross-language conformance harnesses. | `src/conformance.rs:274` |
| `pocketstation::conformance::run_extension_vector` | function | Executes the neutral typed Source -> `Stream<T>` -> Operator -> Endpoint vector through the canonical public Session. | `src/conformance.rs:1006` |
| `pocketstation::conformance::session` | function | Runs the conformance assertions for the Session contract. | `src/conformance.rs:198` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Compatibility and evidence](/docs/compatibility/README.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Test evidence index](/docs/reference/test-evidence.md)
- [A conformance check cannot find external fixtures](/docs/troubleshooting/conformance-fixtures.md)
- [Keep qualification claims scoped](/docs/best-practices/evidence-boundaries.md)

## Evidence boundary

The claims on **Run protocol checks** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `scripts/check_protocol.sh:1-132` (`DIRECT`)

For **Run protocol checks**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
