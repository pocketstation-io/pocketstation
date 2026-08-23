# Session fails before start

<!-- claims: CLM-TRBL-001-CAP-001,CLM-TRBL-001-CAP-002,CLM-TRBL-001-CAP-003,CLM-TRBL-001-CAP-004,CLM-TRBL-001-CAP-005,CLM-TRBL-001-SOURCE-001 -->

Use this page when you observe **session fails before start**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Separate declaration and compile errors from resource-preparation and start errors. Preserve rollback failures as additional evidence rather than replacing the primary start failure.

## Diagnostic signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` (`error-00e5716261eba0f8cf3d`)
- `pocketstation::session::error::SessionError` / `UnknownStem` (`error-00f6e798d158df66c847`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` (`error-01d3fc855e2a00319076`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` (`error-023d6ab0b23a50a614ff`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` (`error-0279b2b6b0cb3b5801bc`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` (`error-037ddc3e193da74177f8`)
- `pocketstation::graph::node::NodeDescriptorError` / `InvalidSafetyContract` (`error-04b7031025a9b635fdbf`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` (`error-05c60389efcb84311921`)
- `pocketstation::session::prepare::error::SessionPrepareError` (`error-085082b521c14e5ecd1e`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` (`error-08a7536094bfb2242b17`)
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` (`error-09837185c7fca0f70618`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` (`error-0bc2f7c0b9f9dbf8ddd7`)
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `ZeroCapacity` (`error-0bd6f58be40ade9a01fe`)
- `pocketstation::graph::node::ConfigError` (`error-0be8ad81000b2924c24c`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `SequenceGap` (`error-0c04a3eedb823da29323`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingExternalAudioIngress` (`error-0cc0ae8a8cc4f1e05996`)
- `pocketstation::session::lifecycle::engine::SessionEngineBuildError` / `DuplicateSidecarId` (`error-0ce1015c73b65576cbeb`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `TimestampRegression` (`error-0d567cf627daa0adfee1`)
- `pocketstation::graph::compile::resolve::CompileError` (`error-0da3f91a5f274a27ab76`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEndpointDeclaration` (`error-0e46a3d13215bfc3898f`)
- `pocketstation::session::extensions::audio_input::AudioInputConfigError` (`error-108ece57ea443c789d81`)
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `ZeroProcessTimeout` (`error-10e3a522fa28fccdfc60`)
- `pocketstation::session::extensions::audio_input::source::AudioInputError` / `Manifest` (`error-11863b3a293345b0bb2d`)
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownEndpointInputPort` (`error-1281b697f9f4d62194b1`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingTypedEdgePlan` (`error-12fef698a1fbec823e7e`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingAsyncOperatorFactory` (`error-1310461ef521d30d4686`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEventReceiver` (`error-13dd584b4e2e8eaa490c`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `InvalidMagic` (`error-143cce14f0e71f68c4cf`)
- `pocketstation::graph::signal::operator::OperatorFailurePolicy` / `StopWorker` (`error-14ca51fa44623142d004`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `RecordAfterTerminal` (`error-16e269f1786471c2db63`)

## Executable evidence

- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` exercises given unrouted stem when session frozen then validation fails closed under its recorded setup (`test-1633b6167eec91db04e2`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` exercises given foreign input handle when connected then declaration fails before freeze under its recorded setup (`test-766194f5939b3ddb896d`).
- `start_prepared_session` exercises start prepared session under its recorded setup (`test-3c2e4e6902c7cc6b36a4`).
- `start_prepared_session_cancellable` exercises start prepared session cancellable under its recorded setup (`test-8ecdda012155cc965789`).
- `given_16khz_session_when_started_then_compiled_endpoint_contexts_preserve_declared_rate` exercises given 16khz session when started then compiled endpoint contexts preserve declared rate under its recorded setup (`test-036ad7eaba4f64ee56a4`).
- `given_capture_backlog_when_session_starts_then_no_destination_edge_overflows` exercises given capture backlog when session starts then no destination edge overflows under its recorded setup (`test-3c7b5a91848359ab3ef8`).
- `given_existing_output_when_started_then_recorder_fails_closed` exercises given existing output when started then recorder fails closed under its recorded setup (`test-3b4ddbb231cb8f0182e8`).
- `given_zero_capacity_when_started_then_recorder_fails_closed` exercises given zero capacity when started then recorder fails closed under its recorded setup (`test-fcc195708afeec03a930`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` exercises given stopped public session when new session starts then capture restarts cleanly under its recorded setup (`test-cbee0768bffa592adde2`).
- `given_linear_graph_when_compiled_then_topo_orders_source_before_sink` exercises given linear graph when compiled then topo orders source before sink under its recorded setup (`test-7ece727a2fa318f311df`).
- `given_echo_async_node_when_process_before_prepare_then_error_is_returned` exercises given echo async node when process before prepare then error is returned under its recorded setup (`test-bfea57e87a139988d3b9`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` exercises given receive before enqueue when observed then latency sample is rejected under its recorded setup (`test-904f86f2ad1e1369647c`).
- `given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed` exercises given core extension oversized sidecar payload when encoded then fails closed under its recorded setup (`test-f94228781a9717656566`).
- `given_capacity_above_global_bound_when_fanout_built_then_setup_fails` exercises given capacity above global bound when fanout built then setup fails under its recorded setup (`test-f0893eafe636572bd65e`).
- `given_missing_or_zero_payload_limit_when_fanout_built_then_setup_fails` exercises given missing or zero payload limit when fanout built then setup fails under its recorded setup (`test-6c3a749ca1eaac051f1f`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Session API](/docs/reference/session.md)
- [Session failures](/docs/errors/session.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/compile/error.rs:1-92` (`DIRECT`)
- `src/session/prepare/error.rs:1-95` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
