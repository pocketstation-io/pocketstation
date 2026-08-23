# Frames or signals are dropped

<!-- claims: CLM-TRBL-006-CAP-001,CLM-TRBL-006-CAP-002,CLM-TRBL-006-CAP-003,CLM-TRBL-006-CAP-004,CLM-TRBL-006-CAP-005,CLM-TRBL-006-CAP-006,CLM-TRBL-006-CAP-007,CLM-TRBL-006-SOURCE-001 -->

Use this page when you observe **frames or signals are dropped**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Compare queue depth, saturation, and drop observations by route. Change capacity only after identifying the constrained consumer and its declared loss and backpressure policy.

## Diagnostic signals

- `pocketstation::endpoint::runtime::EndpointFailureStage` / `CancelPreparation` (`error-0265bb447764629fa47b`)
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroLeaseCapacity` (`error-0370b7ecbdf2b9d6fbdb`)
- `pocketstation::graph::node::NodeDescriptorError` / `InvalidSafetyContract` (`error-04b7031025a9b635fdbf`)
- `pocketstation::graph::node::ConfigError` (`error-0be8ad81000b2924c24c`)
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `QueueCapacityTooLarge` (`error-0bed26cd5cd9ccfe0b20`)
- `pocketstation::graph::compile::resolve::CompileError` (`error-0da3f91a5f274a27ab76`)
- `pocketstation::endpoint::registry::EndpointDriverRegistryError` / `OperatorNodeTypeConflict` (`error-0db6114718e1d213362f`)
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `ZeroProcessTimeout` (`error-10e3a522fa28fccdfc60`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `InvalidMagic` (`error-143cce14f0e71f68c4cf`)
- `pocketstation::graph::signal::operator::OperatorFailurePolicy` / `StopWorker` (`error-14ca51fa44623142d004`)
- `pocketstation::graph::node::NodeError` / `Process` (`error-170066b0b40a26e0e33d`)
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `SequenceGapWithoutDiscontinuity` (`error-18565faf820bbf8e2650`)
- `pocketstation::graph::compile::resolve::CompileError` / `MediaMismatch` (`error-1877b4a7bdffa5d7ed88`)
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `InvalidEnvelope` (`error-1897c7da4711d75eb14d`)
- `pocketstation::graph::plan::PlanError` / `MoveExclusiveFanOut` (`error-18d1485abaf31198b6d8`)
- `pocketstation::graph::node::NodeDescriptorError` / `EmptyDisplayName` (`error-1981cbd27763ca5ffcbe`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Wait` (`error-19eabd878a9188bf94ce`)
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `InputEdgeMediaMismatch` (`error-1be7a5d9b8d5cbceab93`)
- `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` / `LeaseCapacityExhausted` (`error-1d54a56031f21d638e8a`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `ReservedFieldSet` (`error-1d9b879cab06d8598907`)
- `pocketstation::graph::node::ConfigError` / `Missing` (`error-1fb4b2d84a6cf23abbd9`)
- `pocketstation::endpoint::runtime::EndpointFailureStage` (`error-1fdd7e0417ea75e9688a`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `InvalidTerminal` (`error-201cc7749bdbbd671d69`)
- `pocketstation::endpoint::runtime::EndpointFailure` (`error-21860e8a08d6660b2cd4`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `FrameLengthOverflow` (`error-23eba8b87dea81473095`)
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `OutputEdgeMediaMismatch` (`error-2431503c1bc613dbc5c4`)
- `pocketstation::graph::registry::NodeRegistrationError` / `DuplicateNodeType` (`error-243f5b367fb16b38fdea`)
- `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` / `Empty` (`error-25cba0c2435c181a17c1`)
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `IdentityChanged` (`error-2652e90b3fd931c3b8db`)
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `RecoveryWithoutDiscontinuity` (`error-28b1fb124ed036dbd23a`)

## Executable evidence

- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` exercises given enqueued and dropped frames when observed then drop rate uses all attempts under its recorded setup (`test-9a0bb689d2371b66a92f`).
- `given_supported_non_audio_signals_when_checked_then_media_is_symmetric` exercises given supported non audio signals when checked then media is symmetric under its recorded setup (`test-d97a306ad6dc3558e082`).
- `given_contiguous_signals_when_replayed_then_continuity_is_deterministic` exercises given contiguous signals when replayed then continuity is deterministic under its recorded setup (`test-d0dc80cc2da279b6a618`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` exercises given lineaged source fan out when branch frames are copied then exact lineage is preserved under its recorded setup (`test-d798548d6c8b059ba1a8`).
- `given_shutdown_with_queued_shared_frames_when_receivers_drop_then_pool_slots_are_released` exercises given shutdown with queued shared frames when receivers drop then pool slots are released under its recorded setup (`test-f8946231a23a1c5d14de`).
- `given_slow_full_branch_when_more_frames_dispatched_then_other_branch_continues` exercises given slow full branch when more frames dispatched then other branch continues under its recorded setup (`test-98b1ad304d1d5b646f6a`).
- `given_full_source_input_when_more_frames_arrive_then_newest_rejects_and_counts` exercises given full source input when more frames arrive then newest rejects and counts under its recorded setup (`test-9884a85b98ea454bb6cf`).
- `dropped_count` exercises dropped count under its recorded setup (`test-701b8246fe39c342f6ed`).
- `given_prepared_multi_source_runner_when_ready_frames_process_then_no_heap_allocation_occurs` exercises given prepared multi source runner when ready frames process then no heap allocation occurs under its recorded setup (`test-85537f6d9bc6ada5654e`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` exercises given external consumer when declared then provider and typed endpoint use public api under its recorded setup (`test-ace9b7d11da2036ce899`).
- `into_plan_edge_receiver` exercises into plan edge receiver under its recorded setup (`test-0eec7a35869b3b94ca49`).
- `plan_edge_observation_handle` exercises plan edge observation handle under its recorded setup (`test-f8b5136b41672df7481a`).
- `given_concurrent_publish_and_poll_when_observed_then_depth_stays_bounded_and_returns_to_zero` exercises given concurrent publish and poll when observed then depth stays bounded and returns to zero under its recorded setup (`test-53df4ef903e72a1de69c`).
- `given_held_batch_when_polled_then_samples_stay_stable_and_lease_exhaustion_is_counted` exercises given held batch when polled then samples stay stable and lease exhaustion is counted under its recorded setup (`test-1c0e175d584cd2910a77`).
- `given_impossible_dequeue_when_observed_then_depth_saturates_and_failure_is_explicit` exercises given impossible dequeue when observed then depth saturates and failure is explicit under its recorded setup (`test-7760a60bd2740c15c01d`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Graph and signal failures](/docs/errors/graph-and-signals.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Size bounded routes from observations](/docs/best-practices/route-sizing.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/audio/router.rs:1-1615` (`DIRECT`)
- `src/runtime/signal/edge.rs:1-651` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
