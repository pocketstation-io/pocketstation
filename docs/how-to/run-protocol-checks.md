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

- `test-publish-recovery` — test publish recovery (`scripts/test-publish-recovery.sh:1`; `test-ed1499f4e962b2a908d0`).
- `test-session-c-conformance` — test session c conformance (`scripts/test-session-c-conformance.sh:1`; `test-9ccda9c28ea127ec4fe9`).
- `test-single-package-publish` — test single package publish (`scripts/test-single-package-publish.sh:1`; `test-042a4aa4fca762147921`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-f2e13a28f1aa591f3c67`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-8ba7b5b19e9d7dfbc464`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-9fb4684ff29b5ab716fd`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-002ce44230f2b0ac6d7c`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-657d1e2cbdcbd70cf5fa`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-f10bfad1b583316ad6fb`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` — given panicking abi bodies when guarded then panics are contained (`src/abi/codec.rs:373`; `test-a807dc7f3aad831eda7a`).
- `given_rejected_capacity_when_retried_then_encoder_state_is_unchanged` — given rejected capacity when retried then encoder state is unchanged (`src/abi/codec.rs:323`; `test-2f90a0a4149901522180`).
- `given_sine_440hz_when_round_trip_then_decoded_has_energy` — given sine 440hz when round trip then decoded has energy (`src/abi/codec.rs:435`; `test-073998688232a109f6e7`).

## Failure signals

No task-specific public error was resolved for run protocol checks; preserve the owning API's returned error.

## API reference

- [Conformance](/docs/concepts/conformance.md)
- [Test Evidence](/docs/reference/test-evidence.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::conformance` | module | Deterministic Session fixture for external conformance harnesses. | `src/conformance.rs:1` |
| `pocketstation::conformance::ExtensionConformanceReport` | struct | Language-neutral outcome returned by the W20 fixture. | `src/conformance.rs:572` |
| `pocketstation::conformance::ExtensionSignal` | struct | Owns one signal payload used by the native-extension conformance fixtures. | `src/conformance.rs:1180` |
| `pocketstation::conformance::ObservedEndpointError` | enum | Classifies failures reported as observed endpoint error. | `src/conformance.rs:344` |
| `pocketstation::conformance::observed_browser` | function | Declares and registers a deterministic native browser boundary used only by cross-language conformance harnesses. | `src/conformance.rs:334` |
| `pocketstation::conformance::observed_connector` | function | Declares and registers a deterministic native connector used only by cross-language conformance harnesses. | `src/conformance.rs:273` |
| `pocketstation::conformance::run_extension_vector` | function | Executes the neutral typed Source -> `Stream<T>` -> Operator -> Endpoint vector through the public Session. | `src/conformance.rs:1005` |
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

The claims on **Run protocol checks** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `scripts/check_protocol.sh:1-132` (`DIRECT`)

For **Run protocol checks**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
