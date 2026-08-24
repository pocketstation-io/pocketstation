# Conformance and qualification

<!-- claims: CLM-DOC-035-SCOPE-001,CLM-DOC-035-TEXT-001,CLM-DOC-035-TEXT-002,CLM-DOC-035-TEXT-003,CLM-DOC-035-TEXT-004,CLM-DOC-035-TEXT-005,CLM-DOC-035-TEXT-006,CLM-DOC-035-SOURCE-001 -->

## What it is

Conformance is executable evidence that an ABI, protocol, connector, or cross-language boundary matches a versioned fixture under recorded conditions. Qualification is evidence from a real provider, operating system, or device environment.

## Why it exists

A passing build or synthetic vector can prove contract compatibility without proving physical capture or live-provider behavior. Separate labels prevent one evidence scope from silently becoming another.

## Relationships

- Repository fixtures exercise deterministic Session and extension behavior.
- The canonical connector vector corpus lives at `scripts/fixtures/connector-v1-vectors.json`; validation copies it to the versioned sibling path consumed by the portable-semantics test.
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

## Executable evidence

Executable evidence selected for **Conformance and qualification** is limited to each test's recorded setup and assertions:

- `given_pkss_v1_message_kinds_when_projected_then_values_remain_stable` — given pkss v1 message kinds when projected then values remain stable (`tests/protocol_compatibility.rs:35`; `test-be3fbcfc583f9a784846`).
- `given_pkss_v1_signal_when_encoded_then_bytes_remain_stable` — given pkss v1 signal when encoded then bytes remain stable (`tests/protocol_compatibility.rs:12`; `test-0185a2790c06762d1676`).
- `abi_codec_cpp_conformance` — abi codec cpp conformance (`tests/abi_codec_cpp_conformance.cpp:1`; `test-544e66e8f85e1ad9e055`).
- `abi_session_c_conformance` — abi session c conformance (`tests/abi_session_c_conformance.c:1`; `test-9e1beea6279253161031`).
- `abi_session_c_success_conformance` — abi session c success conformance (`tests/abi_session_c_success_conformance.c:1`; `test-fbd5f1d6e0ff13895c92`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-f2e13a28f1aa591f3c67`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-8ba7b5b19e9d7dfbc464`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-9fb4684ff29b5ab716fd`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-002ce44230f2b0ac6d7c`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-657d1e2cbdcbd70cf5fa`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-f10bfad1b583316ad6fb`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` — given panicking abi bodies when guarded then panics are contained (`src/abi/codec.rs:373`; `test-a807dc7f3aad831eda7a`).

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

The claims on **Conformance and qualification** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/conformance.rs:1-4` (`DECLARED`)
- `tests/protocol_compatibility.rs:12-35` (`TESTED`)
- `tests/protocol_compatibility.rs:35-54` (`TESTED`)

For **Conformance and qualification**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
