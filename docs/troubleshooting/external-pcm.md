# External PCM input is saturated

<!-- claims: CLM-TRBL-018-CAP-001,CLM-TRBL-018-CAP-002,CLM-TRBL-018-CAP-003,CLM-TRBL-018-CAP-004,CLM-TRBL-018-SOURCE-001 -->

Use this page when you observe **external pcm input is saturated**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Distinguish buffer acquisition exhaustion from write format or capacity errors and cancellation. Use observations before changing bounded capacity.

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
- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-11b972ad42d5de880e06`)
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownEndpointInputPort` (`error-1281b697f9f4d62194b1`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingTypedEdgePlan` (`error-12fef698a1fbec823e7e`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingAsyncOperatorFactory` (`error-1310461ef521d30d4686`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEventReceiver` (`error-13dd584b4e2e8eaa490c`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `InvalidMagic` (`error-143cce14f0e71f68c4cf`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `RecordAfterTerminal` (`error-16e269f1786471c2db63`)
- `pocketstation::session::error::SessionError` / `UnknownSourceOutput` (`error-16edb8f15b75c471db64`)
- `pocketstation::session::compile::error::SessionCompileError` / `AmbiguousEndpointInput` (`error-17674f66426c713d90a2`)
- `pocketstation::session::lifecycle::events::SessionRollbackFailure` (`error-1955a522796dc25c325d`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Wait` (`error-19eabd878a9188bf94ce`)

## Executable evidence

- `given_bounded_audio_input_when_writes_are_invalid_or_saturated_then_ownership_is_explicit` exercises given bounded audio input when writes are invalid or saturated then ownership is explicit under its recorded setup (`test-795ca5a283c59dbf6066`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-2b664c22fd511e3c2f45`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` exercises given promptable or observable permission when opening input then native open decides under its recorded setup (`test-847d3fefe4665db8dd14`).
- `given_active_asp_when_required_then_sdk_accepts_external_provisioning` exercises given active asp when required then sdk accepts external provisioning under its recorded setup (`test-094ea52b81e34a03e0e1`).
- `given_stable_input_selector_when_wrapped_then_capture_mode_preserves_identity` exercises given stable input selector when wrapped then capture mode preserves identity under its recorded setup (`test-074f0bb3502ed6267879`).
- `given_full_source_input_when_more_frames_arrive_then_newest_rejects_and_counts` exercises given full source input when more frames arrive then newest rejects and counts under its recorded setup (`test-9884a85b98ea454bb6cf`).
- `given_full_input_branch_when_sent_then_overflow_is_counted_and_join_is_bounded` exercises given full input branch when sent then overflow is counted and join is bounded under its recorded setup (`test-95905f4e18a52f786a53`).
- `given_operator_composition_with_named_multi_input_output_manifest_then_each_declared_port_executes` exercises given operator composition with named multi input output manifest then each declared port executes under its recorded setup (`test-716956fd7ff21d2765ad`).
- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` exercises given operator composition with three external operators then derived output crosses each bounded edge under its recorded setup (`test-9ec51c75cedb5ffaef0f`).
- `given_two_inputs_when_processed_then_nonterminal_and_terminal_reach_each_branch` exercises given two inputs when processed then nonterminal and terminal reach each branch under its recorded setup (`test-23bf71cedc9a9fc7172c`).
- `input_mut` exercises input mut under its recorded setup (`test-c2f81d47835d0ed9aa78`).
- `external_source_declarations` exercises external source declarations under its recorded setup (`test-067a9c1179f9fa65bb67`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` exercises given derived stream chain when compiled then operator output feeds next named input under its recorded setup (`test-e9a24a392741b4dbe6e7`).
- `given_required_named_input_missing_when_compiled_then_failure_precedes_graph_runtime` exercises given required named input missing when compiled then failure precedes graph runtime under its recorded setup (`test-99e881c59d26f6126f74`).
- `given_duplicate_named_input_when_connected_then_declaration_fails_immediately` exercises given duplicate named input when connected then declaration fails immediately under its recorded setup (`test-f9a6ec4f71dbaf6d8083`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Configuration reference](/docs/reference/configuration.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/extensions/audio_input/buffer.rs:1-367` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
