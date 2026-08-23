# Endpoint API

<!-- claims: CLM-REF-007-CAP-001,CLM-REF-007-CAP-002,CLM-REF-007-SOURCE-001 -->

## Scope

- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.

The scope of **Endpoint API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Endpoint API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-adc64dc434868308ba4d | `pocketstation::endpoint::contract::EndpointReceiver` | enum | Enumerates the supported endpoint receiver cases. | `src/endpoint/contract.rs:145` |
| sym-280ef62d37b739c2237d | `pocketstation::endpoint::identity::EndpointPreparationGroup` | enum | Factory-owned decision for batching endpoint inputs into one lifecycle. | `src/endpoint/identity.rs:23` |
| sym-29ac19d5f350b7ef1b18 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | Classifies failures reported as polled audio endpoint config error. | `src/endpoint/polled_audio_driver.rs:40` |
| sym-8ca869922a8deea3a920 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | Classifies failures reported as polled audio poll error. | `src/endpoint/polled_audio_driver.rs:74` |
| sym-94ff39a7865d0ba80250 | `pocketstation::endpoint::registry::EndpointDriverRegistryError` | enum | Classifies failures reported as endpoint driver registry error. | `src/endpoint/registry.rs:16` |
| sym-10b2ad948f87c981bacf | `pocketstation::endpoint::registry::EndpointPrepareError` | enum | Classifies failures reported as endpoint prepare error. | `src/endpoint/registry.rs:39` |
| sym-d1ab12a403969a2601f7 | `pocketstation::endpoint::runtime::EndpointFailureRetryability` | enum | Machine-readable recovery classification retained in Session outcomes. | `src/endpoint/runtime.rs:166` |
| sym-9a3122f3d2a19e134d9c | `pocketstation::endpoint::runtime::EndpointFailureStage` | enum | Selects the endpoint failure stage used by PocketStation. | `src/endpoint/runtime.rs:156` |
| sym-b59835aff00d66ed49b4 | `pocketstation::endpoint::runtime::EndpointInputOrigin` | enum | Provenance of one endpoint input, independent of its physical receiver. | `src/endpoint/runtime.rs:31` |
| sym-71b48b21c17d0623f40a | `pocketstation::endpoint::runtime::EndpointShutdownMode` | enum | Session shutdown intent delivered to an active endpoint. | `src/endpoint/runtime.rs:356` |
| sym-4e3aaf03500efef1dfcc | `pocketstation::endpoint::runtime::EndpointStartFailureCause` | enum | Enumerates the supported endpoint start failure cause cases. | `src/endpoint/runtime.rs:438` |
| sym-4785d950f1deba565d3d | `EndpointDriverFactory::preparation_group` | function | Returns the preparation group associated with `EndpointDriverFactory`. | `src/endpoint/contract.rs:233` |
| sym-5858eb75b928c8f5a1d0 | `EndpointDriverFactory::prepare` | function | Prepares resources required by `EndpointDriverFactory`. | `src/endpoint/contract.rs:241` |
| sym-775076f9f636c432e69c | `PreparedEndpointDriver::cancel_preparation` | function | Cancels preparation for `PreparedEndpointDriver`. | `src/endpoint/runtime.rs:328` |
| sym-5c9f2e79390cbd3a6554 | `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| sym-f0e8cdf84d48aca086d6 | `RunningEndpointDriver::join_and_finalize` | function | Joins and finalize for `RunningEndpointDriver`. | `src/endpoint/runtime.rs:346` |
| sym-459a9a074e0856994405 | `RunningEndpointDriver::observations` | function | Returns the observations exposed by `RunningEndpointDriver`. | `src/endpoint/runtime.rs:337` |
| sym-4cb08eb58cfec6f939e5 | `RunningEndpointDriver::request_shutdown` | function | Requests the selected shutdown mode from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:341` |
| sym-d1f376b21fdab92dc979 | `RunningEndpointDriver::request_stop` | function | Requests a graceful stop from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:339` |
| sym-095f9600d9b7441f9d03 | `as_str` | function | Returns the stable string representation of `EndpointGroupId`. | `src/endpoint/identity.rs:16` |
| sym-7f4c17f9bbbf1584724e | `audio_stem_id` | function | Returns the audio stem identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:88` |
| sym-e0ccb37acd23f29eaeeb | `channels` | function | Returns the channel count represented by `EndpointAudioFrame`. | `src/endpoint/contract.rs:47` |
| sym-7114c597af9660b246ff | `channels` | function | Returns the channel count represented by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:239` |
| sym-91cb556f2fd4381a2927 | `code` | function | Returns the stable error or status code represented by `EndpointFailure`. | `src/endpoint/runtime.rs:212` |
| sym-1f4adc143d29ff96861c | `connector_id` | function | Returns the connector identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:231` |
| sym-411c5f107d46d687613f | `connector_id` | function | Returns the connector identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:138` |
| sym-765b19e5b736c4a1abb1 | `context` | function | Returns the context held by `EndpointPortInput`. | `src/endpoint/contract.rs:219` |
| sym-6a7daffc3610023cad91 | `default` | function | Returns the default `PolledAudioEndpointConfig` value. | `src/endpoint/polled_audio_driver.rs:30` |
| sym-4d610cca23cad69c4723 | `drop` | function | Releases resources owned by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:195` |
| sym-40b0c8b332b971a5a10f | `edge_contract` | function | Returns the edge contract held by `EndpointPortInput`. | `src/endpoint/contract.rs:215` |
| sym-db2683f156864f46467c | `endpoint_id` | function | Returns the endpoint identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:219` |
| sym-c641b13adaad8a6c02d4 | `endpoint_id` | function | Returns the endpoint identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:129` |
| sym-8ea70ec71b283e82bb5e | `frame` | function | Returns the frame held by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:186` |
| sym-a1d924cbc0478bd1398e | `from_monotonic_timestamp_ns` | function | Creates `SessionTimelineOrigin` from monotonic timestamp nanoseconds. | `src/endpoint/runtime.rs:18` |
| sym-37099899d4a6590b672f | `from_source` | function | Creates `EndpointRouteContext` from source. | `src/endpoint/runtime.rs:57` |
| sym-1aa6bb6281fa2a93bcb2 | `from_stem` | function | Creates `EndpointRouteContext` from stem. | `src/endpoint/runtime.rs:50` |
| sym-db4321629969fd8fe29f | `into_parts` | function | Consumes `EndpointPortInput` and returns its component values. | `src/endpoint/contract.rs:227` |
| sym-fed3cd219cf45a7c7ee6 | `into_plan_edge_receiver` | function | Converts `EndpointAudioReceiver` into plan edge receiver. | `src/endpoint/contract.rs:82` |
| sym-348e8a4240b8f282b928 | `is_abandoned` | function | Returns whether abandoned applies to `EndpointAudioReceiver`. | `src/endpoint/contract.rs:92` |
| sym-ea2545c4423c5ea821a4 | `is_abandoned` | function | Returns whether abandoned applies to `EndpointSignalReceiver`. | `src/endpoint/contract.rs:140` |
| sym-cdc39e098fa8525e68c5 | `is_empty` | function | Returns whether `PolledAudioBatchLease` contains no values. | `src/endpoint/polled_audio_driver.rs:182` |
| sym-c40844ae30b605f327e7 | `is_open` | function | Returns whether open applies to `EndpointStartGate`. | `src/endpoint/runtime.rs:376` |
| sym-9f927e55ff48c6179c6b | `len` | function | Returns the number of values held by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:178` |
| sym-83c3cf6e8b3e4e8859a4 | `lineage` | function | Returns the frame lineage carried by `EndpointAudioFrame`. | `src/endpoint/contract.rs:59` |
| sym-f9bbeca469782ea9a7c2 | `lineage` | function | Returns the frame lineage carried by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:215` |
| sym-8b50e50d6f02b38d6437 | `mark_discontinuity` | function | Marks the next value from `EndpointAudioReceiver` as discontinuous. | `src/endpoint/contract.rs:96` |
| sym-c3a37e4901efc1525c4b | `mark_worker_failure` | function | Returns the mark worker failure held by `EndpointAudioReceiver`. | `src/endpoint/contract.rs:100` |
| sym-a111f35155807eec9d8b | `media` | function | Returns the media held by `EndpointPortInput`. | `src/endpoint/contract.rs:211` |
| sym-947e217bd6ae8fa4abb0 | `message` | function | Returns the diagnostic message reported by `EndpointFailure`. | `src/endpoint/runtime.rs:208` |
| sym-de5267f80dc8a9bb728c | `monotonic_timestamp_ns` | function | Returns the monotonic timestamp nanoseconds held by `SessionTimelineOrigin`. | `src/endpoint/runtime.rs:24` |
| sym-0cf58a144f9a85982582 | `new` | function | Creates a new `EndpointGroupId`. | `src/endpoint/identity.rs:12` |
| sym-a1be858558e3a28f2626 | `new` | function | Creates a new `PolledAudioEndpoint`. | `src/endpoint/polled_audio.rs:22` |
| sym-ea665e1f03f6777240b5 | `new` | function | Creates a new `EndpointPrepareContext`. | `src/endpoint/runtime.rs:108` |
| sym-39ed52964be43aa64e82 | `new` | function | Creates a new `EndpointFailure`. | `src/endpoint/runtime.rs:182` |
| sym-e895b862c39b54dd1556 | `node_configuration` | function | Returns the node configuration held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:150` |
| sym-ff77c27aa7377b2bf9b0 | `observations` | function | Snapshots the bounded edge counters for this endpoint input. | `src/endpoint/contract.rs:108` |
| sym-e7c6edd26ced8a2c02a5 | `observations` | function | Returns the observations exposed by `PolledAudioReceipt`. | `src/endpoint/polled_audio_driver.rs:167` |
| sym-1d7d0d9de993ec919a6e | `origin` | function | Returns the origin held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:84` |
| sym-4d15a0b12b1aeb6d598e | `plan_edge_observation_handle` | function | Plans edge observation handle for `EndpointAudioReceiver`. | `src/endpoint/contract.rs:117` |
| sym-ae54f2ebf47973451b7a | `port_name` | function | Returns the port name held by `EndpointPortInput`. | `src/endpoint/contract.rs:203` |
| sym-9656d0a8e46811ec82bc | `receipt` | function | Returns the receipt associated with `PolledAudioEndpoint`. | `src/endpoint/polled_audio.rs:30` |
| sym-f2c69629ba9e7c7539a8 | `receiver` | function | Returns the receiver held by `EndpointPortInput`. | `src/endpoint/contract.rs:223` |
| sym-adc952ba5c898d4b9b1a | `recv` | function | Receives the next value from `EndpointSignalReceiver`. | `src/endpoint/contract.rs:136` |
| sym-97b6dcf0c3f93b8a5d14 | `retryability` | function | Returns the retryability associated with `EndpointFailure`. | `src/endpoint/runtime.rs:216` |
| sym-e14e4ccc1f945efe7625 | `route_context` | function | Returns the route context associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:142` |
| sym-35d7bf42cf22453bf0ba | `route_id` | function | Returns the route identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:223` |
| sym-d97c1fc6d21b1b6ee568 | `route_id` | function | Returns the route identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:80` |
| sym-033d971470da2af0a9d4 | `sample_format` | function | Returns the sample format associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:51` |
| sym-5d09422dd96053a2a69f | `sample_rate_hz` | function | Returns the sample rate hertz held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:43` |
| sym-f3d107283c5285a8d9e0 | `sample_rate_hz` | function | Returns the sample rate hertz held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:235` |
| sym-2f0d111a00ca5f86cf50 | `samples` | function | Returns the audio samples held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:55` |
| sym-f612888a97a6eb4f275c | `samples` | function | Returns the audio samples held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:243` |
| sym-a28a5a383f020397e255 | `sequence_number` | function | Returns the sequence number held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:35` |
| sym-6c7b927d2160ca21ee08 | `session_id` | function | Returns the session identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:125` |
| sym-3ea0cb1192354749380e | `session_timeline_origin` | function | Returns the session timeline origin associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:146` |
| sym-57abbe9a4e4f19ceae2b | `signal` | function | Returns the signal held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:73` |
| sym-cb7407e154a82f3e89b3 | `signal_spec` | function | Returns the signal spec held by `EndpointPortInput`. | `src/endpoint/contract.rs:207` |
| sym-4ef49f400751fadaa88b | `source_id` | function | Returns the source identifier held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:27` |
| sym-5c1728703e1eb4513514 | `stage` | function | Returns the stage held by `EndpointFailure`. | `src/endpoint/runtime.rs:204` |
| sym-40ec179be76127e8dcb3 | `stream_id` | function | Returns the stream identifier held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:31` |
| sym-83eabc1a32583616286a | `stream_id` | function | Returns the stream identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:227` |
| sym-d4bec64a2b4084678d0a | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:39` |
| sym-892b0eb29c32bd2e629a | `try_poll` | function | Attempts to poll through `PolledAudioReceipt`. | `src/endpoint/polled_audio_driver.rs:110` |
| sym-8ad0fb8f103b4dfa1c9d | `try_recv` | function | Attempts to receive the next value from `EndpointAudioReceiver` without waiting. | `src/endpoint/contract.rs:86` |
| sym-21ca6203bd69dfc90d24 | `try_recv` | function | Attempts to receive the next value from `EndpointSignalReceiver` without waiting. | `src/endpoint/contract.rs:132` |
| sym-16e1073102da090bb83a | `with_external_details` | function | Attaches stable external failure details without changing Endpoint's provider-neutral lifecycle authority. | `src/endpoint/runtime.rs:194` |
| sym-267e3ff822fee19f1ecd | `pocketstation::endpoint::contract::EndpointAudioFrame` | struct | Read-only audio frame delivered to an external endpoint. | `src/endpoint/contract.rs:18` |
| sym-477cdcbb1df07b07f639 | `pocketstation::endpoint::contract::EndpointAudioReceiver` | struct | Exclusive consumer for one bounded realtime-audio endpoint edge. | `src/endpoint/contract.rs:68` |
| sym-382e9fb3581d16a20ccc | `pocketstation::endpoint::contract::EndpointPortInput` | struct | Carries typed input for endpoint port. | `src/endpoint/contract.rs:153` |
| sym-627c94191cfef9a46728 | `pocketstation::endpoint::contract::EndpointSignalReceiver` | struct | Exclusive consumer for one bounded asynchronous signal endpoint edge. | `src/endpoint/contract.rs:123` |
| sym-5f2952d6944973cc1c88 | `pocketstation::endpoint::identity::EndpointGroupId` | struct | Explicit Session-scoped grouping key for endpoints that share one lifecycle. | `src/endpoint/identity.rs:9` |
| sym-fbcb718abdda5a26510c | `pocketstation::endpoint::polled_audio::PolledAudioEndpoint` | struct | Safe composition owner for application-polled audio. | `src/endpoint/polled_audio.rs:16` |
| sym-d40c1e0748d919e3234f | `pocketstation::endpoint::polled_audio_driver::PolledAudioBatchLease` | struct | Owns bounded access to polled audio batch. | `src/endpoint/polled_audio_driver.rs:172` |
| sym-a80ddf76df17c927b68e | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfig` | struct | Configures polled audio endpoint behavior at its owning API boundary. | `src/endpoint/polled_audio_driver.rs:23` |
| sym-434dbbbc531976bf9796 | `pocketstation::endpoint::polled_audio_driver::PolledAudioFrame` | struct | Carries one polled audio payload together with its declared metadata. | `src/endpoint/polled_audio_driver.rs:210` |
| sym-9b68fb7793859b5af3b3 | `pocketstation::endpoint::polled_audio_driver::PolledAudioObservations` | struct | Reports the polled audio observations collected at an observation boundary. | `src/endpoint/polled_audio_driver.rs:56` |
| sym-ffbe18ac1475853536f7 | `pocketstation::endpoint::polled_audio_driver::PolledAudioReceipt` | struct | Retains the identity and observation access returned for polled audio. | `src/endpoint/polled_audio_driver.rs:105` |
| sym-1acd6566beddec2b1ad1 | `pocketstation::endpoint::registry::EndpointDriverRegistry` | struct | Indexes registered endpoint driver implementations by their stable identities. | `src/endpoint/registry.rs:54` |
| sym-f4b8e6b72352b88ed6ad | `pocketstation::endpoint::runtime::EndpointCancellationOutcome` | struct | Reports the structured endpoint cancellation outcome. | `src/endpoint/runtime.rs:289` |
| sym-80686ff8b5ac0125a741 | `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | Reports an endpoint driver's terminal observations and any finalization failure. | `src/endpoint/runtime.rs:295` |
| sym-707209feeb55089f10f8 | `pocketstation::endpoint::runtime::EndpointDriverObservations` | struct | Reports the endpoint driver observations collected at an observation boundary. | `src/endpoint/runtime.rs:228` |
| sym-76f91d197c51f7d96af2 | `pocketstation::endpoint::runtime::EndpointFailure` | struct | Reports a endpoint failure. | `src/endpoint/runtime.rs:174` |
| sym-c22f8a7564d7a5b121a2 | `pocketstation::endpoint::runtime::EndpointFinalizationOutcome` | struct | Reports the structured endpoint finalization outcome. | `src/endpoint/runtime.rs:301` |
| sym-eef74ac3cc89e0fc9134 | `pocketstation::endpoint::runtime::EndpointPrepareContext` | struct | Carries the inputs and runtime context required to endpoint prepare. | `src/endpoint/runtime.rs:98` |
| sym-f4ca7bd5d5eea77667f1 | `pocketstation::endpoint::runtime::EndpointRouteContext` | struct | Typed Session route identity supplied to every endpoint input. | `src/endpoint/runtime.rs:44` |
| sym-d7394b735bd27ea67b1e | `pocketstation::endpoint::runtime::EndpointStartFailure` | struct | Reports a endpoint start failure. | `src/endpoint/runtime.rs:443` |
| sym-dd6611b464415b86d8bf | `pocketstation::endpoint::runtime::EndpointStartGate` | struct | Read-only one-way start barrier shared by endpoint drivers in one startup. | `src/endpoint/runtime.rs:371` |
| sym-8246b27fde41c7114b61 | `pocketstation::endpoint::runtime::PreparedEndpoint` | struct | Owns endpoint resources after preparation and before its runtime driver starts. | `src/endpoint/runtime.rs:405` |
| sym-0199ce6a3f14bc6410ee | `pocketstation::endpoint::runtime::RunningEndpoint` | struct | Owns a started endpoint driver until shutdown and finalization complete. | `src/endpoint/runtime.rs:481` |
| sym-9a2d09d1aa729ae9ad87 | `pocketstation::endpoint::runtime::SessionTimelineOrigin` | struct | One Session-owned anchor in PocketStation's monotonic nanosecond clock. | `src/endpoint/runtime.rs:13` |
| sym-9eafa855d0adb4757b7b | `EndpointCancellationOutcome::observations` | struct_field | Carries the observations collected for `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:290` |
| sym-b54f840f9c32f7ba9fc2 | `EndpointCancellationOutcome::result` | struct_field | Stores the result used by `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:291` |
| sym-cc3e93fe46141b5cadd6 | `EndpointDriverFinalization::observations` | struct_field | Carries the observations collected for `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:296` |
| sym-ae3c0e1cee9ba34d9710 | `EndpointDriverFinalization::result` | struct_field | Stores the result used by `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:297` |
| sym-f18f449a15a26fbedd15 | `EndpointDriverObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:232` |
| sym-3e8d7dc3ceadead997e3 | `EndpointDriverObservations::failures_total` | struct_field | Counts the total number of failures observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:233` |
| sym-93f41b49fc7737f0321b | `EndpointDriverObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:230` |
| sym-18239882504feb615200 | `EndpointDriverObservations::frames_dropped_total` | struct_field | Counts the total number of frames dropped observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:231` |
| sym-e6ef748486e51d8018eb | `EndpointDriverObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:229` |
| sym-67da87ab6ac014eea575 | `EndpointDriverRegistryError::Duplicate::node_type_id` | struct_field | Identifies the node type identifier recorded by `Duplicate`. | `src/endpoint/registry.rs:26` |
| sym-bb4e15a5c3c9d2517beb | `EndpointDriverRegistryError::Duplicate::operator_id` | struct_field | Identifies the operator identifier recorded by `Duplicate`. | `src/endpoint/registry.rs:25` |
| sym-2a1cc821592407f99fe0 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::operator_id` | struct_field | Identifies the operator identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:32` |
| sym-bb09ca0c12744042898c | `EndpointDriverRegistryError::OperatorNodeTypeConflict::registered_node_type_id` | struct_field | Identifies the registered node type identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:33` |
| sym-98e8637e19152ed24ef6 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::requested_node_type_id` | struct_field | Identifies the requested node type identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:34` |
| sym-897fec9d31bb50be35c5 | `EndpointFinalizationOutcome::join_finalize_result` | struct_field | Stores the join finalize result used by `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:304` |
| sym-cd9d7d3c0023b209e9a4 | `EndpointFinalizationOutcome::observations` | struct_field | Carries the observations collected for `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:302` |
| sym-d76d3bb026b7263f74a1 | `EndpointFinalizationOutcome::request_stop_result` | struct_field | Stores the request stop result used by `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:303` |
| sym-b315d98af68d13d169b2 | `EndpointInputOrigin::Source::audio_stem_id` | struct_field | Identifies the audio stem identifier recorded by `Source`. | `src/endpoint/runtime.rs:38` |
| sym-536711f3c786eb7932b4 | `EndpointInputOrigin::Source::source_id` | struct_field | Identifies the source identifier recorded by `Source`. | `src/endpoint/runtime.rs:36` |
| sym-5f1e40074f4ad284fe18 | `EndpointInputOrigin::Source::stream_id` | struct_field | Identifies the stream identifier recorded by `Source`. | `src/endpoint/runtime.rs:37` |
| sym-d630f30f329338ce4a5c | `EndpointPrepareError::NotRegistered::node_type_id` | struct_field | Identifies the node type identifier recorded by `NotRegistered`. | `src/endpoint/registry.rs:47` |
| sym-099bd3e73344dda7dd3e | `EndpointPrepareError::NotRegistered::operator_id` | struct_field | Identifies the operator identifier recorded by `NotRegistered`. | `src/endpoint/registry.rs:46` |
| sym-df6e5a09858ffdcc4441 | `EndpointReceiver::Audio::receiver` | struct_field | Stores the receiver used by `Audio`. | `src/endpoint/contract.rs:147` |
| sym-3ce0fa40a2cba322e866 | `EndpointReceiver::Audio::sample_spec` | struct_field | Stores the sample spec used by `Audio`. | `src/endpoint/contract.rs:148` |
| sym-21adcffcd2a34aa678ab | `PolledAudioEndpointConfig::max_batch_frames` | struct_field | Stores the max batch frames used by `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:25` |
| sym-5120060d28c6de64186d | `PolledAudioEndpointConfig::max_outstanding_leases` | struct_field | Stores the max outstanding leases used by `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:26` |
| sym-736e0245e5e31232c39f | `PolledAudioEndpointConfig::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:24` |
| sym-50983d32097e8f36d929 | `PolledAudioObservations::batches_polled_total` | struct_field | Counts the total number of batches polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:69` |
| sym-5cbaf29414a52c644cec | `PolledAudioObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:63` |
| sym-cb13211dc1085ed0e1c3 | `PolledAudioObservations::frames_polled_total` | struct_field | Counts the total number of frames polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:70` |
| sym-bb3bc37eb133eaa9f31c | `PolledAudioObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:62` |
| sym-b210aac23de5cf323bd9 | `PolledAudioObservations::invalid_ownership_drops_total` | struct_field | Counts the total number of invalid ownership drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:65` |
| sym-4bfd19b0b4c430a49114 | `PolledAudioObservations::lease_capacity_count` | struct_field | Sets the lease capacity count available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:66` |
| sym-eda9d5d63eb6e6be3b64 | `PolledAudioObservations::lease_exhausted_total` | struct_field | Counts the total number of lease exhausted observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:68` |
| sym-77b965237b400a2c6fd2 | `PolledAudioObservations::outstanding_leases` | struct_field | Stores the outstanding leases used by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:67` |
| sym-fb89adcd966167edc480 | `PolledAudioObservations::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:58` |
| sym-8cee7b51a4d6045b835b | `PolledAudioObservations::queue_depth_frames` | struct_field | Reports the queue depth frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:59` |
| sym-fcd3825abc7b443ccb32 | `PolledAudioObservations::queue_depth_invariant_failures_total` | struct_field | Counts the total number of queue depth invariant failures observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:61` |
| sym-4d3e7691869e45928567 | `PolledAudioObservations::queue_full_drops_total` | struct_field | Counts the total number of queue full drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:64` |
| sym-0322f69305ad1efb7f89 | `PolledAudioObservations::queue_peak_frames` | struct_field | Reports the queue peak frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:60` |
| sym-66d30bfd0461d14c1860 | `PolledAudioObservations::registered_endpoints` | struct_field | Stores the registered endpoints used by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:57` |
| sym-24a418620ed384787b21 | `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | Implement this trait to provide endpoint behavior to PocketStation; its methods define the preparation and runtime contract. | `src/endpoint/contract.rs:232` |
| sym-b1b5bdddc63ee20f95f0 | `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| sym-5693e1fc212774b1d075 | `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| sym-7f4aeca62674c8943a40 | `pocketstation::endpoint::contract::EndpointReceiver::Audio` | variant | Represents the audio alternative defined by `EndpointReceiver`. | `src/endpoint/contract.rs:146` |
| sym-8737f9eed43a76550ab1 | `pocketstation::endpoint::contract::EndpointReceiver::Signal` | variant | Represents the signal alternative defined by `EndpointReceiver`. | `src/endpoint/contract.rs:150` |
| sym-5599686d8ec27b98e6eb | `pocketstation::endpoint::identity::EndpointPreparationGroup::Route` | variant | Represents the route alternative defined by `EndpointPreparationGroup`. | `src/endpoint/identity.rs:24` |
| sym-54ca783c4fba6a4196ae | `pocketstation::endpoint::identity::EndpointPreparationGroup::Shared` | variant | Represents the shared alternative defined by `EndpointPreparationGroup`. | `src/endpoint/identity.rs:25` |
| sym-e4a04729c579cf3f97a7 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::BatchCapacityTooLarge` | variant | Reported when the owning operation encounters batch capacity too large. | `src/endpoint/polled_audio_driver.rs:50` |
| sym-5173a65b36933cba104d | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::LeaseCapacityTooLarge` | variant | Reported when the owning operation encounters lease capacity too large. | `src/endpoint/polled_audio_driver.rs:52` |
| sym-0f105d20b5b705c6e529 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::QueueCapacityTooLarge` | variant | Reported when the owning operation encounters queue capacity too large. | `src/endpoint/polled_audio_driver.rs:48` |
| sym-0eaa6c4838504decfc4b | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroBatchCapacity` | variant | Reported when the owning operation encounters zero batch capacity. | `src/endpoint/polled_audio_driver.rs:44` |
| sym-cd965e6c8bb2ad90f7c5 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroLeaseCapacity` | variant | Reported when the owning operation encounters zero lease capacity. | `src/endpoint/polled_audio_driver.rs:46` |
| sym-2d771b1fb2e321dffc44 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroQueueCapacity` | variant | Reported when the owning operation encounters zero queue capacity. | `src/endpoint/polled_audio_driver.rs:42` |
| sym-93cf3d8f2ccf7fc41ae1 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::Empty` | variant | Represents an empty value or collection. | `src/endpoint/polled_audio_driver.rs:76` |
| sym-e21a258537edfaf06b50 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::LeaseCapacityExhausted` | variant | Reported when the owning operation encounters lease capacity exhausted. | `src/endpoint/polled_audio_driver.rs:78` |
| sym-0143da2bc75c3b5a49fd | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::StatePoisoned` | variant | Reported when the owning operation encounters state poisoned. | `src/endpoint/polled_audio_driver.rs:80` |
| sym-5f1dc7ba77d614c8ad63 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::Duplicate` | variant | Reported when the owning operation encounters duplicate. | `src/endpoint/registry.rs:24` |
| sym-3b8dcd186b5884562b73 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyNodeTypeId` | variant | Reported when the owning operation encounters empty node type identifier. | `src/endpoint/registry.rs:20` |
| sym-64992d6a343255948ced | `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyOperatorId` | variant | Reported when the owning operation encounters empty operator identifier. | `src/endpoint/registry.rs:18` |
| sym-e0d277825d6d0b9cc68d | `pocketstation::endpoint::registry::EndpointDriverRegistryError::OperatorNodeTypeConflict` | variant | Reported when the owning operation encounters operator node type conflict. | `src/endpoint/registry.rs:31` |
| sym-ddb0f269a5665d3455c9 | `pocketstation::endpoint::registry::EndpointPrepareError::Driver` | variant | Reported when the owning operation encounters driver. | `src/endpoint/registry.rs:50` |
| sym-e291107299a9bbb2ca3a | `pocketstation::endpoint::registry::EndpointPrepareError::EmptyBatch` | variant | Reported when the owning operation encounters empty batch. | `src/endpoint/registry.rs:41` |
| sym-7d8561c8b8023fabebba | `pocketstation::endpoint::registry::EndpointPrepareError::NotRegistered` | variant | Reported when the owning operation encounters not registered. | `src/endpoint/registry.rs:45` |
| sym-6a6d38d4c601427330af | `pocketstation::endpoint::runtime::EndpointFailureRetryability::Never` | variant | Selects never behavior for `EndpointFailureRetryability`. | `src/endpoint/runtime.rs:167` |
| sym-27bebbab40ccd0f74b41 | `pocketstation::endpoint::runtime::EndpointFailureRetryability::ReconfigurationRequired` | variant | Selects reconfiguration required behavior for `EndpointFailureRetryability`. | `src/endpoint/runtime.rs:169` |
| sym-73adaff1abb76f2f712d | `pocketstation::endpoint::runtime::EndpointFailureRetryability::Retryable` | variant | Selects retryable behavior for `EndpointFailureRetryability`. | `src/endpoint/runtime.rs:168` |
| sym-4eedcbb89c8816aa1b07 | `pocketstation::endpoint::runtime::EndpointFailureStage::CancelPreparation` | variant | Identifies the cancel preparation state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:158` |
| sym-849a56a51ddca22ab870 | `pocketstation::endpoint::runtime::EndpointFailureStage::JoinFinalize` | variant | Identifies the join finalize state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:161` |
| sym-50657833fb0f41682c40 | `pocketstation::endpoint::runtime::EndpointFailureStage::Prepare` | variant | Identifies the prepare state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:157` |
| sym-a13f3acad67ef72304b0 | `pocketstation::endpoint::runtime::EndpointFailureStage::RequestStop` | variant | Identifies the request stop state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:160` |
| sym-26861b1620f8b9bde7a2 | `pocketstation::endpoint::runtime::EndpointFailureStage::Start` | variant | Identifies the start state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:159` |
| sym-02aa77e01bbd1b0c0b02 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Signal` | variant | A typed signal whose detailed provenance is carried by `SignalLineage`. | `src/endpoint/runtime.rs:34` |
| sym-692120f8084c94070506 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Source` | variant | Represents the source alternative defined by `EndpointInputOrigin`. | `src/endpoint/runtime.rs:35` |
| sym-ef6b2d7325398598ff93 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Stem` | variant | Represents the stem alternative defined by `EndpointInputOrigin`. | `src/endpoint/runtime.rs:32` |
| sym-0cc2735a1212157cc2f8 | `pocketstation::endpoint::runtime::EndpointShutdownMode::Abort` | variant | Selects abort behavior for `EndpointShutdownMode`. | `src/endpoint/runtime.rs:358` |
| sym-3b5c0f7180057e58da21 | `pocketstation::endpoint::runtime::EndpointShutdownMode::Drain` | variant | Selects drain behavior for `EndpointShutdownMode`. | `src/endpoint/runtime.rs:357` |
| sym-316c920624b4f5b2d208 | `pocketstation::endpoint::runtime::EndpointStartFailureCause::Driver` | variant | Reported when the owning operation encounters driver. | `src/endpoint/runtime.rs:440` |
| sym-2d45b4984613219b1c31 | `pocketstation::endpoint::runtime::EndpointStartFailureCause::GateAlreadyOpen` | variant | Reported when the owning operation encounters gate already open. | `src/endpoint/runtime.rs:439` |

## Interpretation

The **Endpoint API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

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

The claims on **Endpoint API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/endpoint/mod.rs:1-38` (`DIRECT`)

For **Endpoint API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
