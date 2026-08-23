# Session trace validation fails

<!-- claims: CLM-TRBL-017-CAP-001,CLM-TRBL-017-CAP-002,CLM-TRBL-017-CAP-003,CLM-TRBL-017-SOURCE-001 -->

Use this page when you observe **session trace validation fails**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Use the trace validation error and record index to distinguish ordering, identity, and terminal-record failures. Do not rewrite a trace to make validation pass.

## Diagnostic signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` (`error-00e5716261eba0f8cf3d`)
- `pocketstation::session::error::SessionError` / `UnknownStem` (`error-00f6e798d158df66c847`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` (`error-01d3fc855e2a00319076`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` (`error-023d6ab0b23a50a614ff`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` (`error-0279b2b6b0cb3b5801bc`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` (`error-037ddc3e193da74177f8`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` (`error-05c60389efcb84311921`)
- `pocketstation::session::prepare::error::SessionPrepareError` (`error-085082b521c14e5ecd1e`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` (`error-08a7536094bfb2242b17`)
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` (`error-09837185c7fca0f70618`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` (`error-0bc2f7c0b9f9dbf8ddd7`)
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `ZeroCapacity` (`error-0bd6f58be40ade9a01fe`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `SequenceGap` (`error-0c04a3eedb823da29323`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingExternalAudioIngress` (`error-0cc0ae8a8cc4f1e05996`)
- `pocketstation::session::lifecycle::engine::SessionEngineBuildError` / `DuplicateSidecarId` (`error-0ce1015c73b65576cbeb`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `TimestampRegression` (`error-0d567cf627daa0adfee1`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEndpointDeclaration` (`error-0e46a3d13215bfc3898f`)
- `pocketstation::session::extensions::audio_input::AudioInputConfigError` (`error-108ece57ea443c789d81`)
- `pocketstation::session::extensions::audio_input::source::AudioInputError` / `Manifest` (`error-11863b3a293345b0bb2d`)
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownEndpointInputPort` (`error-1281b697f9f4d62194b1`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingTypedEdgePlan` (`error-12fef698a1fbec823e7e`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingAsyncOperatorFactory` (`error-1310461ef521d30d4686`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEventReceiver` (`error-13dd584b4e2e8eaa490c`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `RecordAfterTerminal` (`error-16e269f1786471c2db63`)
- `pocketstation::session::error::SessionError` / `UnknownSourceOutput` (`error-16edb8f15b75c471db64`)
- `pocketstation::session::compile::error::SessionCompileError` / `AmbiguousEndpointInput` (`error-17674f66426c713d90a2`)
- `pocketstation::session::lifecycle::events::SessionRollbackFailure` (`error-1955a522796dc25c325d`)
- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `MissingPort` (`error-1bd7ae7942029f778071`)
- `pocketstation::session::error_code::SessionStartErrorCode` (`error-1c7816652dd061fb1141`)
- `pocketstation::session::extensions::audio_input::source::AudioInputError` (`error-1de3680efda0db59054d`)

## Executable evidence

- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` exercises given unrouted stem when session frozen then validation fails closed under its recorded setup (`test-1633b6167eec91db04e2`).
- `given_derived_stream_without_destination_when_frozen_then_validation_fails_closed` exercises given derived stream without destination when frozen then validation fails closed under its recorded setup (`test-94e7fa143670693acd86`).
- `given_invalid_lifecycle_when_validated_then_validation_fails_closed` exercises given invalid lifecycle when validated then validation fails closed under its recorded setup (`test-ad4fc9ea8d172cd4b678`).
- `given_public_facade_when_session_trace_enabled_then_trace_replays_complete_lifecycle` exercises given public facade when session trace enabled then trace replays complete lifecycle under its recorded setup (`test-4f8b8179e33a1ceba291`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` exercises given cloned stem when session frozen then mutation is rejected under its recorded setup (`test-1682e00b3166c4846a92`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` exercises given derived stream when through called again then chain is preserved in session spec under its recorded setup (`test-837dd73be7d5c552ef15`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` exercises given operator when declared then session scoped instance and routes are preserved under its recorded setup (`test-69203660038a41959c14`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` exercises given endpoint operator id when imported from session then endpoint contract type is reexported under its recorded setup (`test-c1047cbdeb5a7bf9bc3b`).
- `given_newer_minor_version_when_validated_then_schema_fails_closed` exercises given newer minor version when validated then schema fails closed under its recorded setup (`test-7fe4fdd3769bd70886b6`).
- `given_duplicate_named_input_when_connected_then_declaration_fails_immediately` exercises given duplicate named input when connected then declaration fails immediately under its recorded setup (`test-f9a6ec4f71dbaf6d8083`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` exercises given foreign input handle when connected then declaration fails before freeze under its recorded setup (`test-766194f5939b3ddb896d`).
- `given_duplicate_type_when_session_nodes_registered_then_registry_is_unchanged` exercises given duplicate type when session nodes registered then registry is unchanged under its recorded setup (`test-417c35b0251dee5fe0b7`).
- `given_structural_ingress_when_validated_then_session_metadata_is_not_required` exercises given structural ingress when validated then session metadata is not required under its recorded setup (`test-7406cc23117530680012`).
- `register_session_graph_nodes` exercises register session graph nodes under its recorded setup (`test-3d5f82ddbefbd9cd1a57`).
- `given_nonportable_source_identities_when_constructed_then_each_fails_typed` exercises given nonportable source identities when constructed then each fails typed under its recorded setup (`test-49c22efdf198e8e31c91`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Instrument a Session](/docs/how-to/instrument-session.md)
- [Observation API](/docs/reference/observations.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/trace.rs:1-1179` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
