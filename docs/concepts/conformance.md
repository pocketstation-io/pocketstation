# Conformance and qualification

<!-- claims: CLM-DOC-035-CAP-001,CLM-DOC-035-SOURCE-001 -->

## What it is

Conformance is executable evidence that an ABI, protocol, connector, or cross-language boundary matches a versioned fixture under recorded conditions. Qualification is evidence from a real provider, operating system, or device environment.

## Why it exists

A passing build or synthetic vector can prove contract compatibility without proving physical capture or live-provider behavior. Separate labels prevent one evidence scope from silently becoming another.

## Relationships

- Repository fixtures exercise deterministic Session and extension behavior.
- Portable connector vectors live at a versioned external prerequisite path.
- CI and release checks record commands and targets but do not replace physical qualification.

## Invariants and guarantees

- Missing fixtures are prerequisite failures, not passing evidence.
- Conformance results name their source revision and fixture revision.
- Qualification claims remain limited to the environment actually tested.

## When you encounter it

- **Author a connector** — Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.
- **Bind through C** — Create and operate a Session through ABI handles, status codes, and versioned callbacks.
- **Validate an integration** — Run protocol, ABI, connector, package, and example checks at the frozen source revision.

## Use it

- [Run protocol checks](/docs/how-to/run-protocol-checks.md)
- [Test connector conformance](/docs/how-to/test-connector-conformance.md)
- [Conformance fixtures are missing](/docs/troubleshooting/conformance-fixtures.md)

## Scope

- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

The scope of **Conformance and qualification** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

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
| `pocketstation::conformance::session_for_saturation` | function | Creates a finite fixture that produces enough frames to overflow a deliberately unconsumed canonical route. | `src/conformance.rs:204` |
| `pocketstation::conformance::session_with_recording` | function | Creates the deterministic canonical-engine fixture with multistem recording. | `src/conformance.rs:209` |

## Executable evidence

Executable evidence selected for **Conformance and qualification** is limited to each test's recorded setup and assertions:

- `given_pkss_v1_message_kinds_when_projected_then_values_remain_stable` — given pkss v1 message kinds when projected then values remain stable (`tests/protocol_compatibility.rs:35`; `test-3f19ed84be12761963bc`).
- `given_pkss_v1_signal_when_encoded_then_bytes_remain_stable` — given pkss v1 signal when encoded then bytes remain stable (`tests/protocol_compatibility.rs:12`; `test-96b5bb377993cd2e3876`).
- `abi_codec_cpp_conformance` — abi codec cpp conformance (`tests/abi_codec_cpp_conformance.cpp:1`; `test-6fbae5633a1419379cd7`).
- `abi_session_c_conformance` — abi session c conformance (`tests/abi_session_c_conformance.c:1`; `test-1ab6697ee6c783b1c41b`).
- `abi_session_c_success_conformance` — abi session c success conformance (`tests/abi_session_c_success_conformance.c:1`; `test-a2314a88cf28de25b331`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-041037b5b9482d79c8e2`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-d3686b94180b732c8001`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` — given panicking abi bodies when guarded then panics are contained (`src/abi/codec.rs:373`; `test-03d685383aaeadb55cad`).

## Related documentation

- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [Glossary](/docs/glossary.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Run the examples](/docs/getting-started/examples.md)
- [Author a connector](/docs/guides/connectors.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Run protocol checks](/docs/how-to/run-protocol-checks.md)

## Evidence boundary

The claims on **Conformance and qualification** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/conformance.rs:1-1252` (`DIRECT`)
- `tests/protocol_compatibility.rs:1-55` (`DIRECT`)

For **Conformance and qualification**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
