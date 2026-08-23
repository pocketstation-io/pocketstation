# Endpoint API

<!-- claims: CLM-REF-007-CAP-001,CLM-REF-007-CAP-002,CLM-REF-007-SOURCE-001 -->

## Scope

- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `pocketstation::endpoint::contract::EndpointReceiver` | enum | Enumerates the supported endpoint receiver cases. | `src/endpoint/contract.rs:145` |
| `pocketstation::endpoint::identity::EndpointPreparationGroup` | enum | Factory-owned decision for batching endpoint inputs into one lifecycle. | `src/endpoint/identity.rs:23` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | Classifies failures reported as polled audio endpoint config error. | `src/endpoint/polled_audio_driver.rs:40` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | Classifies failures reported as polled audio poll error. | `src/endpoint/polled_audio_driver.rs:74` |
| `pocketstation::endpoint::registry::EndpointDriverRegistryError` | enum | Classifies failures reported as endpoint driver registry error. | `src/endpoint/registry.rs:16` |
| `pocketstation::endpoint::registry::EndpointPrepareError` | enum | Classifies failures reported as endpoint prepare error. | `src/endpoint/registry.rs:39` |
| `pocketstation::endpoint::runtime::EndpointFailureRetryability` | enum | Machine-readable recovery classification retained in Session outcomes. | `src/endpoint/runtime.rs:166` |
| `pocketstation::endpoint::runtime::EndpointFailureStage` | enum | Selects the endpoint failure stage used by PocketStation. | `src/endpoint/runtime.rs:156` |
| `pocketstation::endpoint::runtime::EndpointInputOrigin` | enum | Provenance of one endpoint input, independent of its physical receiver. | `src/endpoint/runtime.rs:31` |
| `pocketstation::endpoint::runtime::EndpointShutdownMode` | enum | Session shutdown intent delivered to an active endpoint. | `src/endpoint/runtime.rs:356` |
| `pocketstation::endpoint::runtime::EndpointStartFailureCause` | enum | Enumerates the supported endpoint start failure cause cases. | `src/endpoint/runtime.rs:438` |
| `EndpointDriverFactory::preparation_group` | function | Returns the preparation group associated with `EndpointDriverFactory`. | `src/endpoint/contract.rs:233` |
| `EndpointDriverFactory::prepare` | function | Prepares resources required by `EndpointDriverFactory`. | `src/endpoint/contract.rs:241` |
| `PreparedEndpointDriver::cancel_preparation` | function | Cancels preparation for `PreparedEndpointDriver`. | `src/endpoint/runtime.rs:328` |
| `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| `RunningEndpointDriver::join_and_finalize` | function | Joins `RunningEndpointDriver` and returns its finalization outcome. | `src/endpoint/runtime.rs:346` |
| `RunningEndpointDriver::observations` | function | Returns the observations exposed by `RunningEndpointDriver`. | `src/endpoint/runtime.rs:337` |
| `RunningEndpointDriver::request_shutdown` | function | Requests the selected shutdown mode from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:341` |
| `RunningEndpointDriver::request_stop` | function | Requests a graceful stop from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:339` |
| `as_str` | function | Returns the stable string representation of `EndpointGroupId`. | `src/endpoint/identity.rs:16` |
| `audio_stem_id` | function | Returns the audio stem identifier associated with `EndpointRouteContext`. | `src/endpoint/runtime.rs:88` |
| `channels` | function | Returns the channel count represented by `EndpointAudioFrame`. | `src/endpoint/contract.rs:47` |
| `channels` | function | Returns the channel count represented by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:239` |
| `code` | function | Returns the stable error or status code represented by `EndpointFailure`. | `src/endpoint/runtime.rs:212` |
| `connector_id` | function | Returns the connector identifier associated with `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:231` |
| `connector_id` | function | Returns the connector identifier associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:138` |
| `context` | function | Returns the context associated with `EndpointPortInput`. | `src/endpoint/contract.rs:219` |
| `drop` | function | Releases resources owned by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:195` |
| `edge_contract` | function | Returns the edge contract associated with `EndpointPortInput`. | `src/endpoint/contract.rs:215` |
| `endpoint_id` | function | Returns the endpoint identifier associated with `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:219` |
| `endpoint_id` | function | Returns the endpoint identifier associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:129` |
| `frame` | function | Returns the frame associated with `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:186` |
| `from_monotonic_timestamp_ns` | function | Creates `SessionTimelineOrigin` from monotonic timestamp nanoseconds. | `src/endpoint/runtime.rs:18` |
| `from_source` | function | Creates `EndpointRouteContext` from source. | `src/endpoint/runtime.rs:57` |
| `from_stem` | function | Creates `EndpointRouteContext` from stem. | `src/endpoint/runtime.rs:50` |
| `into_parts` | function | Consumes `EndpointPortInput` and returns its component values. | `src/endpoint/contract.rs:227` |
| `is_abandoned` | function | Returns whether abandoned applies to `EndpointAudioReceiver`. | `src/endpoint/contract.rs:92` |
| `is_abandoned` | function | Returns whether abandoned applies to `EndpointSignalReceiver`. | `src/endpoint/contract.rs:140` |
| `is_empty` | function | Returns whether `PolledAudioBatchLease` contains no values. | `src/endpoint/polled_audio_driver.rs:182` |
| `is_open` | function | Returns whether open applies to `EndpointStartGate`. | `src/endpoint/runtime.rs:376` |
| `len` | function | Returns the number of values held by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:178` |
| `lineage` | function | Returns the frame lineage associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:59` |
| `lineage` | function | Returns the frame lineage associated with `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:215` |
| `mark_discontinuity` | function | Marks the next value from `EndpointAudioReceiver` as discontinuous. | `src/endpoint/contract.rs:96` |
| `mark_worker_failure` | function | Returns the mark worker failure associated with `EndpointAudioReceiver`. | `src/endpoint/contract.rs:100` |
| `media` | function | Returns the media associated with `EndpointPortInput`. | `src/endpoint/contract.rs:211` |
| `message` | function | Returns the diagnostic message associated with `EndpointFailure`. | `src/endpoint/runtime.rs:208` |
| `monotonic_timestamp_ns` | function | Returns the monotonic timestamp nanoseconds associated with `SessionTimelineOrigin`. | `src/endpoint/runtime.rs:24` |
| `new` | function | Creates a new `EndpointGroupId`. | `src/endpoint/identity.rs:12` |
| `new` | function | Creates a new `EndpointPrepareContext`. | `src/endpoint/runtime.rs:108` |
| `new` | function | Creates a new `EndpointFailure`. | `src/endpoint/runtime.rs:182` |
| `node_configuration` | function | Returns the node configuration associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:150` |
| `observations` | function | Snapshots the bounded edge counters for this endpoint input. | `src/endpoint/contract.rs:108` |
| `origin` | function | Returns the origin associated with `EndpointRouteContext`. | `src/endpoint/runtime.rs:84` |
| `port_name` | function | Returns the port name associated with `EndpointPortInput`. | `src/endpoint/contract.rs:203` |
| `receiver` | function | Returns the receiver associated with `EndpointPortInput`. | `src/endpoint/contract.rs:223` |
| `recv` | function | Receives the next value from `EndpointSignalReceiver`. | `src/endpoint/contract.rs:136` |
| `retryability` | function | Returns the retryability associated with `EndpointFailure`. | `src/endpoint/runtime.rs:216` |
| `route_context` | function | Returns the route context associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:142` |
| `route_id` | function | Returns the route identifier associated with `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:223` |
| `route_id` | function | Returns the route identifier associated with `EndpointRouteContext`. | `src/endpoint/runtime.rs:80` |
| `sample_format` | function | Returns the sample format associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:51` |
| `sample_rate_hz` | function | Returns the sample rate hertz associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:43` |
| `sample_rate_hz` | function | Returns the sample rate hertz associated with `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:235` |
| `samples` | function | Returns the audio samples held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:55` |
| `samples` | function | Returns the audio samples held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:243` |
| `sequence_number` | function | Returns the sequence number associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:35` |
| `session_id` | function | Returns the session identifier associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:125` |
| `session_timeline_origin` | function | Returns the session timeline origin associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:146` |
| `signal` | function | Returns the signal associated with `EndpointRouteContext`. | `src/endpoint/runtime.rs:73` |
| `signal_spec` | function | Returns the signal spec associated with `EndpointPortInput`. | `src/endpoint/contract.rs:207` |
| `source_id` | function | Returns the source identifier associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:27` |
| `stage` | function | Returns the stage associated with `EndpointFailure`. | `src/endpoint/runtime.rs:204` |
| `stream_id` | function | Returns the stream identifier associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:31` |
| `stream_id` | function | Returns the stream identifier associated with `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:227` |
| `timestamp_ns` | function | Returns the timestamp nanoseconds associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:39` |
| `try_recv` | function | Attempts to receive the next value from `EndpointAudioReceiver` without waiting. | `src/endpoint/contract.rs:86` |
| `try_recv` | function | Attempts to receive the next value from `EndpointSignalReceiver` without waiting. | `src/endpoint/contract.rs:132` |
| `with_external_details` | function | Attaches stable external failure details without changing Endpoint's provider-neutral lifecycle authority. | `src/endpoint/runtime.rs:194` |
| `pocketstation::endpoint::contract::EndpointAudioFrame` | struct | Read-only audio frame delivered to an external endpoint. | `src/endpoint/contract.rs:18` |
| `pocketstation::endpoint::contract::EndpointAudioReceiver` | struct | Exclusive consumer for one bounded realtime-audio endpoint edge. | `src/endpoint/contract.rs:68` |
| `pocketstation::endpoint::contract::EndpointPortInput` | struct | Represents endpoint port input in the PocketStation API. | `src/endpoint/contract.rs:153` |
| `pocketstation::endpoint::contract::EndpointSignalReceiver` | struct | Exclusive consumer for one bounded asynchronous signal endpoint edge. | `src/endpoint/contract.rs:123` |
| `pocketstation::endpoint::identity::EndpointGroupId` | struct | Explicit Session-scoped grouping key for endpoints that share one lifecycle. | `src/endpoint/identity.rs:9` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioBatchLease` | struct | Owns bounded access to polled audio batch. | `src/endpoint/polled_audio_driver.rs:172` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioFrame` | struct | Represents polled audio frame in the PocketStation API. | `src/endpoint/polled_audio_driver.rs:210` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioObservations` | struct | Reports the polled audio observations collected at an observation boundary. | `src/endpoint/polled_audio_driver.rs:56` |
| `pocketstation::endpoint::runtime::EndpointCancellationOutcome` | struct | Reports the structured endpoint cancellation outcome. | `src/endpoint/runtime.rs:289` |
| `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | Represents endpoint driver finalization in the PocketStation API. | `src/endpoint/runtime.rs:295` |
| `pocketstation::endpoint::runtime::EndpointDriverObservations` | struct | Reports the endpoint driver observations collected at an observation boundary. | `src/endpoint/runtime.rs:228` |
| `pocketstation::endpoint::runtime::EndpointFailure` | struct | Reports a endpoint failure. | `src/endpoint/runtime.rs:174` |
| `pocketstation::endpoint::runtime::EndpointFinalizationOutcome` | struct | Reports the structured endpoint finalization outcome. | `src/endpoint/runtime.rs:301` |
| `pocketstation::endpoint::runtime::EndpointPrepareContext` | struct | Represents endpoint prepare context in the PocketStation API. | `src/endpoint/runtime.rs:98` |
| `pocketstation::endpoint::runtime::EndpointRouteContext` | struct | Typed Session route identity supplied to every endpoint input. | `src/endpoint/runtime.rs:44` |
| `pocketstation::endpoint::runtime::EndpointStartFailure` | struct | Reports a endpoint start failure. | `src/endpoint/runtime.rs:443` |
| `pocketstation::endpoint::runtime::EndpointStartGate` | struct | Read-only one-way start barrier shared by endpoint drivers in one startup. | `src/endpoint/runtime.rs:371` |
| `pocketstation::endpoint::runtime::PreparedEndpoint` | struct | Represents prepared endpoint in the PocketStation API. | `src/endpoint/runtime.rs:405` |
| `pocketstation::endpoint::runtime::RunningEndpoint` | struct | Represents running endpoint in the PocketStation API. | `src/endpoint/runtime.rs:481` |
| `pocketstation::endpoint::runtime::SessionTimelineOrigin` | struct | One Session-owned anchor in PocketStation's monotonic nanosecond clock. | `src/endpoint/runtime.rs:13` |
| `EndpointCancellationOutcome::observations` | struct_field | Carries the observations collected for `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:290` |
| `EndpointCancellationOutcome::result` | struct_field | Stores the result associated with `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:291` |
| `EndpointDriverFinalization::observations` | struct_field | Carries the observations collected for `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:296` |
| `EndpointDriverFinalization::result` | struct_field | Stores the result associated with `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:297` |
| `EndpointDriverObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:232` |
| `EndpointDriverObservations::failures_total` | struct_field | Counts the total number of failures observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:233` |
| `EndpointDriverObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:230` |
| `EndpointDriverObservations::frames_dropped_total` | struct_field | Counts the total number of frames dropped observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:231` |
| `EndpointDriverObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:229` |
| `EndpointDriverRegistryError::Duplicate::node_type_id` | struct_field | Identifies the node type associated with `Duplicate`. | `src/endpoint/registry.rs:26` |
| `EndpointDriverRegistryError::Duplicate::operator_id` | struct_field | Identifies the operator associated with `Duplicate`. | `src/endpoint/registry.rs:25` |
| `EndpointDriverRegistryError::OperatorNodeTypeConflict::operator_id` | struct_field | Identifies the operator associated with `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:32` |
| `EndpointDriverRegistryError::OperatorNodeTypeConflict::registered_node_type_id` | struct_field | Identifies the registered node type associated with `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:33` |
| `EndpointDriverRegistryError::OperatorNodeTypeConflict::requested_node_type_id` | struct_field | Identifies the requested node type associated with `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:34` |
| `EndpointFinalizationOutcome::join_finalize_result` | struct_field | Stores the join finalize result associated with `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:304` |
| `EndpointFinalizationOutcome::observations` | struct_field | Carries the observations collected for `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:302` |
| `EndpointFinalizationOutcome::request_stop_result` | struct_field | Stores the request stop result associated with `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:303` |
| `EndpointInputOrigin::Source::audio_stem_id` | struct_field | Identifies the audio stem associated with `Source`. | `src/endpoint/runtime.rs:38` |
| `EndpointInputOrigin::Source::source_id` | struct_field | Identifies the source associated with `Source`. | `src/endpoint/runtime.rs:36` |
| `EndpointInputOrigin::Source::stream_id` | struct_field | Identifies the stream associated with `Source`. | `src/endpoint/runtime.rs:37` |
| `EndpointPrepareError::NotRegistered::node_type_id` | struct_field | Identifies the node type associated with `NotRegistered`. | `src/endpoint/registry.rs:47` |
| `EndpointPrepareError::NotRegistered::operator_id` | struct_field | Identifies the operator associated with `NotRegistered`. | `src/endpoint/registry.rs:46` |
| `EndpointReceiver::Audio::receiver` | struct_field | Stores the receiver associated with `Audio`. | `src/endpoint/contract.rs:147` |
| `EndpointReceiver::Audio::sample_spec` | struct_field | Stores the sample spec associated with `Audio`. | `src/endpoint/contract.rs:148` |
| `PolledAudioObservations::batches_polled_total` | struct_field | Counts the total number of batches polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:69` |
| `PolledAudioObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:63` |
| `PolledAudioObservations::frames_polled_total` | struct_field | Counts the total number of frames polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:70` |
| `PolledAudioObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:62` |
| `PolledAudioObservations::invalid_ownership_drops_total` | struct_field | Counts the total number of invalid ownership drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:65` |
| `PolledAudioObservations::lease_capacity_count` | struct_field | Sets the lease capacity count available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:66` |
| `PolledAudioObservations::lease_exhausted_total` | struct_field | Counts the total number of lease exhausted observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:68` |
| `PolledAudioObservations::outstanding_leases` | struct_field | Stores the outstanding leases associated with `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:67` |
| `PolledAudioObservations::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:58` |
| `PolledAudioObservations::queue_depth_frames` | struct_field | Reports the queue depth frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:59` |
| `PolledAudioObservations::queue_depth_invariant_failures_total` | struct_field | Counts the total number of queue depth invariant failures observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:61` |
| `PolledAudioObservations::queue_full_drops_total` | struct_field | Counts the total number of queue full drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:64` |
| `PolledAudioObservations::queue_peak_frames` | struct_field | Reports the queue peak frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:60` |
| `PolledAudioObservations::registered_endpoints` | struct_field | Stores the registered endpoints associated with `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:57` |
| `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | Defines the implementation contract for endpoint. | `src/endpoint/contract.rs:232` |
| `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| `pocketstation::endpoint::contract::EndpointReceiver::Audio` | variant | Represents the audio case of `EndpointReceiver`. | `src/endpoint/contract.rs:146` |
| `pocketstation::endpoint::contract::EndpointReceiver::Signal` | variant | Represents the signal case of `EndpointReceiver`. | `src/endpoint/contract.rs:150` |
| `pocketstation::endpoint::identity::EndpointPreparationGroup::Route` | variant | Represents the route case of `EndpointPreparationGroup`. | `src/endpoint/identity.rs:24` |
| `pocketstation::endpoint::identity::EndpointPreparationGroup::Shared` | variant | Represents the shared case of `EndpointPreparationGroup`. | `src/endpoint/identity.rs:25` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::BatchCapacityTooLarge` | variant | Reports batch capacity too large. | `src/endpoint/polled_audio_driver.rs:50` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::LeaseCapacityTooLarge` | variant | Reports lease capacity too large. | `src/endpoint/polled_audio_driver.rs:52` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::QueueCapacityTooLarge` | variant | Reports queue capacity too large. | `src/endpoint/polled_audio_driver.rs:48` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroBatchCapacity` | variant | Reports zero batch capacity. | `src/endpoint/polled_audio_driver.rs:44` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroLeaseCapacity` | variant | Reports zero lease capacity. | `src/endpoint/polled_audio_driver.rs:46` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroQueueCapacity` | variant | Reports zero queue capacity. | `src/endpoint/polled_audio_driver.rs:42` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::Empty` | variant | Represents an empty value or collection. | `src/endpoint/polled_audio_driver.rs:76` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::LeaseCapacityExhausted` | variant | Reports lease capacity exhausted. | `src/endpoint/polled_audio_driver.rs:78` |
| `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::StatePoisoned` | variant | Reports state poisoned. | `src/endpoint/polled_audio_driver.rs:80` |
| `pocketstation::endpoint::registry::EndpointDriverRegistryError::Duplicate` | variant | Reports duplicate. | `src/endpoint/registry.rs:24` |
| `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyNodeTypeId` | variant | Reports empty node type identifier. | `src/endpoint/registry.rs:20` |
| `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyOperatorId` | variant | Reports empty operator identifier. | `src/endpoint/registry.rs:18` |
| `pocketstation::endpoint::registry::EndpointDriverRegistryError::OperatorNodeTypeConflict` | variant | Reports operator node type conflict. | `src/endpoint/registry.rs:31` |
| `pocketstation::endpoint::registry::EndpointPrepareError::Driver` | variant | Reports driver. | `src/endpoint/registry.rs:50` |
| `pocketstation::endpoint::registry::EndpointPrepareError::EmptyBatch` | variant | Reports empty batch. | `src/endpoint/registry.rs:41` |
| `pocketstation::endpoint::registry::EndpointPrepareError::NotRegistered` | variant | Reports not registered. | `src/endpoint/registry.rs:45` |
| `pocketstation::endpoint::runtime::EndpointFailureRetryability::Never` | variant | Reports never. | `src/endpoint/runtime.rs:167` |
| `pocketstation::endpoint::runtime::EndpointFailureRetryability::ReconfigurationRequired` | variant | Reports reconfiguration required. | `src/endpoint/runtime.rs:169` |
| `pocketstation::endpoint::runtime::EndpointFailureRetryability::Retryable` | variant | Reports retryable. | `src/endpoint/runtime.rs:168` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::CancelPreparation` | variant | Reports cancel preparation. | `src/endpoint/runtime.rs:158` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::JoinFinalize` | variant | Reports join finalize. | `src/endpoint/runtime.rs:161` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::Prepare` | variant | Reports prepare. | `src/endpoint/runtime.rs:157` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::RequestStop` | variant | Reports request stop. | `src/endpoint/runtime.rs:160` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::Start` | variant | Reports start. | `src/endpoint/runtime.rs:159` |
| `pocketstation::endpoint::runtime::EndpointInputOrigin::Signal` | variant | A typed signal whose detailed provenance is carried by `SignalLineage`. | `src/endpoint/runtime.rs:34` |
| `pocketstation::endpoint::runtime::EndpointInputOrigin::Source` | variant | Represents the source case of `EndpointInputOrigin`. | `src/endpoint/runtime.rs:35` |
| `pocketstation::endpoint::runtime::EndpointInputOrigin::Stem` | variant | Represents the stem case of `EndpointInputOrigin`. | `src/endpoint/runtime.rs:32` |
| `pocketstation::endpoint::runtime::EndpointShutdownMode::Abort` | variant | Selects abort behavior for `EndpointShutdownMode`. | `src/endpoint/runtime.rs:358` |
| `pocketstation::endpoint::runtime::EndpointShutdownMode::Drain` | variant | Selects drain behavior for `EndpointShutdownMode`. | `src/endpoint/runtime.rs:357` |
| `pocketstation::endpoint::runtime::EndpointStartFailureCause::Driver` | variant | Reports driver. | `src/endpoint/runtime.rs:440` |
| `pocketstation::endpoint::runtime::EndpointStartFailureCause::GateAlreadyOpen` | variant | Reports gate already open. | `src/endpoint/runtime.rs:439` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Endpoint failures](/docs/errors/endpoints.md)
- [Endpoint lifecycle](/docs/concepts/endpoints.md)
- [Polled audio](/docs/concepts/polled-audio.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/endpoint/mod.rs:1-38` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
