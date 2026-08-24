# Compatibility and evidence

<!-- claims: CLM-REF-015-SCOPE-001,CLM-REF-015-TEXT-001,CLM-REF-015-TEXT-002,CLM-REF-015-SOURCE-001 -->

## Scope

- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.
- **Build and publish repository artifacts.** Run architecture, protocol, package, platform, and release checks used by the repository publication workflow.

The scope of **Compatibility and evidence** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Implemented boundary

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::conformance` | module | Deterministic Session fixture for external conformance harnesses. | `src/conformance.rs:1` |
| `pocketstation::conformance::ExtensionConformanceReport` | struct | Language-neutral outcome returned by the W20 fixture. | `src/conformance.rs:572` |
| `pocketstation::conformance::ExtensionSignal` | struct | Owns one signal payload used by the native-extension conformance fixtures. | `src/conformance.rs:1180` |
| `pocketstation::conformance::ObservedEndpointError` | enum | Classifies failures surfaced by observed endpoint operations. | `src/conformance.rs:344` |
| `pocketstation::conformance::observed_browser` | function | Declares and registers a deterministic native browser boundary used only by cross-language conformance harnesses. | `src/conformance.rs:334` |
| `pocketstation::conformance::observed_connector` | function | Declares and registers a deterministic native connector used only by cross-language conformance harnesses. | `src/conformance.rs:273` |
| `pocketstation::conformance::run_extension_vector` | function | Executes the neutral typed Source -> `Stream<T>` -> Operator -> Endpoint vector through the public Session. | `src/conformance.rs:1005` |
| `pocketstation::conformance::session` | function | Runs the conformance assertions for the Session contract. | `src/conformance.rs:198` |
| `pocketstation::conformance::session_for_saturation` | function | Creates a finite fixture that produces enough frames to overflow a deliberately unconsumed route. | `src/conformance.rs:204` |
| `pocketstation::conformance::session_with_recording` | function | Creates the deterministic Session fixture with multistem recording. | `src/conformance.rs:209` |
| `pocketstation::conformance::session_with_recording_and_trace` | function | Creates the deterministic Session fixture with both aligned multistem recording and a bounded Session diagnostic trace. | `src/conformance.rs:230` |
| `pocketstation::conformance::session_with_trace` | function | Creates the deterministic Session fixture with a bounded diagnostic trace. | `src/conformance.rs:216` |
| `signal_spec` | function | Returns the signal spec held by `ExtensionSignal`. | `src/conformance.rs:1183` |
| `pocketstation::conformance::EXTENSION_ENDPOINT_ID` | constant | Defines extension endpoint identifier as `"org.pocketstation.conformance.endpoint.v1"` for the owning public contract. | `src/conformance.rs:558` |
| `pocketstation::conformance::EXTENSION_ENDPOINT_INPUT_PORT` | constant | Defines extension endpoint input port as `"in"` for the owning public contract. | `src/conformance.rs:563` |
| `pocketstation::conformance::EXTENSION_ENDPOINT_NODE_ID` | constant | Defines extension endpoint node identifier as `"org.pocketstation.conformance.endpoint-node.v1"` for the owning public contract. | `src/conformance.rs:559` |
| `pocketstation::conformance::EXTENSION_INPUT_PAYLOAD` | constant | Defines extension input payload as `b"seed"` for the owning public contract. | `src/conformance.rs:564` |
| `pocketstation::conformance::EXTENSION_OPERATOR_ID` | constant | Defines extension operator identifier as `"org.pocketstation.conformance.operator.v1"` for the owning public contract. | `src/conformance.rs:556` |
| `pocketstation::conformance::EXTENSION_OPERATOR_INPUT_PORT` | constant | Defines extension operator input port as `"in"` for the owning public contract. | `src/conformance.rs:561` |
| `pocketstation::conformance::EXTENSION_OPERATOR_NODE_ID` | constant | Defines extension operator node identifier as `"org.pocketstation.conformance.operator-node.v1"` for the owning public contract. | `src/conformance.rs:557` |

## Permission and source opening

For **Compatibility and evidence**, permission observation and source opening remain separate. The host application owns prompts and selection UI. A non-prompting observation is advisory where implemented; preparation or open returns the authoritative result.

## Qualification boundary

The target-specific files, Cargo dependencies, and CI cited by **Compatibility and evidence** establish implementation or build evidence only. They do not qualify every device, operating-system revision, packaging context, permission state, or physical path.

## Executable evidence

Executable evidence selected for **Compatibility and evidence** is limited to each test's recorded setup and assertions:

- `given_process_evidence_when_provider_succeeds_then_actual_invocation_is_persisted` — given process evidence when provider succeeds then actual invocation is persisted (`examples/whisper-transcribe/src/lib.rs:1129`; `test-004f9f3662355f6c02cc`).
- `given_process_evidence_when_provider_times_out_then_kill_and_reap_are_persisted` — given process evidence when provider times out then kill and reap are persisted (`examples/whisper-transcribe/src/lib.rs:1180`; `test-2ed5fdd4ba19977c8dc9`).
- `given_wrong_major_when_checked_then_compatibility_fails` — given wrong major when checked then compatibility fails (`src/abi/session/mod.rs:998`; `test-ee6a03f0db459d8f2e98`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-f98e0a98874ff7dfbdf8`).
- `given_discontinuity_change_inside_window_when_processed_then_window_is_rejected` — given discontinuity change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1400`; `test-2a5f4a8f1e04f5b220c4`).
- `given_hung_provider_when_deadline_expires_then_child_is_killed_and_reaped` — given hung provider when deadline expires then child is killed and reaped (`examples/whisper-transcribe/src/lib.rs:1108`; `test-aa794f0809b00c2d3394`).
- `given_instance_timeout_when_manifest_resolves_then_deadline_matches_configuration` — given instance timeout when manifest resolves then deadline matches configuration (`examples/whisper-transcribe/src/lib.rs:1055`; `test-899bb5750fda98d0832b`).
- `given_lineaged_window_when_transcribed_then_derived_range_covers_every_frame` — given lineaged window when transcribed then derived range covers every frame (`examples/whisper-transcribe/src/lib.rs:1311`; `test-5978528a8ea570fad70d`).
- `given_missing_binary_when_prepare_runs_then_connector_fails_closed` — given missing binary when prepare runs then connector fails closed (`examples/whisper-transcribe/src/lib.rs:1098`; `test-bd5ed751c752083c7711`).
- `given_outer_cancellation_when_process_is_active_then_child_receipt_is_finalized` — given outer cancellation when process is active then child receipt is finalized (`examples/whisper-transcribe/src/lib.rs:1220`; `test-841a6b80171cfb0f55e8`).
- `given_permission_change_inside_window_when_processed_then_window_is_rejected` — given permission change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1419`; `test-1601ba20883aee1ac630`).
- `given_source_change_inside_window_when_processed_then_window_is_rejected_and_reset` — given source change inside window when processed then window is rejected and reset (`examples/whisper-transcribe/src/lib.rs:1379`; `test-384d43cad3cb43576f09`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Run protocol checks](/docs/how-to/run-protocol-checks.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Test evidence index](/docs/reference/test-evidence.md)
- [A conformance check cannot find external fixtures](/docs/troubleshooting/conformance-fixtures.md)
- [Keep qualification claims scoped](/docs/best-practices/evidence-boundaries.md)

## Evidence boundary

The claims on **Compatibility and evidence** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `docs/compatibility/c-abi-v1.baseline:1-21` (`DIRECT`)
- `.github/workflows/ci.yml:1-21` (`DIRECT`)

For **Compatibility and evidence**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
