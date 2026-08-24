# Endpoint API

<!-- claims: CLM-REF-007-SCOPE-001,CLM-REF-007-TEXT-001,CLM-REF-007-TEXT-002,CLM-REF-007-SOURCE-001 -->

## Scope

- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.

The scope of **Endpoint API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Endpoint API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-f41b220549324b1bc03c | `pocketstation::endpoint::contract::EndpointReceiver` | enum | Owns the bounded receiver for the media class accepted by an endpoint. | `src/endpoint/contract.rs:175` |
| sym-c2a87abcabc14ba01e3a | `pocketstation::endpoint::identity::EndpointPreparationGroup` | enum | Factory-owned decision for batching endpoint inputs into one lifecycle. | `src/endpoint/identity.rs:23` |
| sym-a89bc612a3b00ae6ae81 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | enum | Classifies failures surfaced by polled audio endpoint config operations. | `src/endpoint/polled_audio_driver.rs:40` |
| sym-573e7386b423f6e2c79a | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | enum | Classifies failures surfaced by polled audio poll operations. | `src/endpoint/polled_audio_driver.rs:74` |
| sym-23d150abd8808ea27637 | `pocketstation::endpoint::registry::EndpointDriverRegistryError` | enum | Classifies failures surfaced by endpoint driver registry operations. | `src/endpoint/registry.rs:16` |
| sym-dcf022bdce68f457ab12 | `pocketstation::endpoint::registry::EndpointPrepareError` | enum | Classifies failures produced during endpoint resource preparation. | `src/endpoint/registry.rs:39` |
| sym-04c73341db961b3a258d | `pocketstation::endpoint::runtime::EndpointFailureRetryability` | enum | Machine-readable recovery classification retained in Session outcomes. | `src/endpoint/runtime.rs:166` |
| sym-2ac4e71ae5a5c7b9e4d6 | `pocketstation::endpoint::runtime::EndpointFailureStage` | enum | Selects the endpoint failure stage used by PocketStation. | `src/endpoint/runtime.rs:156` |
| sym-f431fd7a79ebac5b019c | `pocketstation::endpoint::runtime::EndpointInputOrigin` | enum | Provenance of one endpoint input, independent of its physical receiver. | `src/endpoint/runtime.rs:31` |
| sym-a056d0a949c31542f472 | `pocketstation::endpoint::runtime::EndpointShutdownMode` | enum | Session shutdown intent delivered to an active endpoint. | `src/endpoint/runtime.rs:356` |
| sym-37b46431c1ff4468753a | `pocketstation::endpoint::runtime::EndpointStartFailureCause` | enum | Classifies the lifecycle stage responsible for an endpoint start failure. | `src/endpoint/runtime.rs:438` |
| sym-4b0ef43987b96b6f3540 | `EndpointDriverFactory::preparation_group` | function | Returns the preparation group associated with `EndpointDriverFactory`. | `src/endpoint/contract.rs:263` |
| sym-f529d30f10097964651a | `EndpointDriverFactory::prepare` | function | Prepares resources required by `EndpointDriverFactory`. | `src/endpoint/contract.rs:271` |
| sym-78c0c0868e0e6890db43 | `PreparedEndpointDriver::cancel_preparation` | function | Cancels resources created while preparing `PreparedEndpointDriver`. | `src/endpoint/runtime.rs:328` |
| sym-4616280bf6b55d26508d | `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| sym-10bdf73b1868933ea548 | `RunningEndpointDriver::join_and_finalize` | function | Joins and finalize for `RunningEndpointDriver`. | `src/endpoint/runtime.rs:346` |
| sym-cd04e67c105bddcbcec6 | `RunningEndpointDriver::observations` | function | Returns the observations exposed by `RunningEndpointDriver`. | `src/endpoint/runtime.rs:337` |
| sym-ad8b16eca0934003b206 | `RunningEndpointDriver::request_shutdown` | function | Requests the selected shutdown mode from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:341` |
| sym-6815ea1b0e23a3e58e2f | `RunningEndpointDriver::request_stop` | function | Requests a graceful stop from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:339` |
| sym-6acbd29b6724de4b584f | `as_str` | function | Returns the stable string representation of `EndpointGroupId`. | `src/endpoint/identity.rs:16` |
| sym-3545be6a5b39ea259c4d | `audio_stem_id` | function | Returns the audio stem identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:88` |
| sym-16a3ef48594b21bc3b2b | `channels` | function | Returns the channel count represented by `EndpointAudioFrame`. | `src/endpoint/contract.rs:61` |
| sym-4cbe7948fb0b2aa1dbb8 | `channels` | function | Returns the channel count represented by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:301` |
| sym-f5cae830f98ad619334b | `code` | function | Returns the stable error or status code represented by `EndpointFailure`. | `src/endpoint/runtime.rs:212` |
| sym-d97706ec8bcbc92af245 | `connector_id` | function | Returns the connector identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:293` |
| sym-a7836b3612121275e6ce | `connector_id` | function | Returns the connector identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:138` |
| sym-3a9c0d6cde74bac8980a | `context` | function | Returns the context held by `EndpointPortInput`. | `src/endpoint/contract.rs:249` |
| sym-567daa26a7dd7f60fe2f | `default` | function | Returns the default `PolledAudioEndpointConfig` value. | `src/endpoint/polled_audio_driver.rs:30` |
| sym-801da87612861d0e8d9a | `drop` | function | Releases resources owned by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:241` |
| sym-4d7a7ced948b546687bf | `edge_contract` | function | Returns the edge contract held by `EndpointPortInput`. | `src/endpoint/contract.rs:245` |
| sym-13d0c33d9b533c4ca958 | `endpoint_enqueued_at_ns` | function | Returns the endpoint enqueued at nanoseconds held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:281` |
| sym-c6b6e5f1e34176dcab49 | `endpoint_id` | function | Returns the endpoint identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:265` |
| sym-3429083d9425fded8a3f | `endpoint_id` | function | Returns the endpoint identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:129` |
| sym-2ef59b897969c3489269 | `frame` | function | Returns the frame held by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:232` |
| sym-2d85553d88646e996814 | `from_monotonic_timestamp_ns` | function | Creates `SessionTimelineOrigin` from monotonic timestamp nanoseconds. | `src/endpoint/runtime.rs:18` |
| sym-54144482285221055f1c | `from_source` | function | Creates `EndpointRouteContext` from source. | `src/endpoint/runtime.rs:57` |
| sym-efba1f597832298035b1 | `from_stem` | function | Creates `EndpointRouteContext` from stem. | `src/endpoint/runtime.rs:50` |
| sym-d75c6c2ea414e5af58f9 | `into_parts` | function | Consumes `EndpointPortInput` and returns its component values. | `src/endpoint/contract.rs:257` |
| sym-fe9469ed3661a3b13b96 | `into_plan_edge_receiver` | function | Converts `EndpointAudioReceiver` into plan edge receiver. | `src/endpoint/contract.rs:106` |
| sym-1bed125889cc7f8a2391 | `is_abandoned` | function | Reports whether abandoned is true for `EndpointAudioReceiver`. | `src/endpoint/contract.rs:122` |
| sym-adf717467b01bb7fab5e | `is_abandoned` | function | Reports whether abandoned is true for `EndpointSignalReceiver`. | `src/endpoint/contract.rs:170` |
| sym-88bb7e90d368d40d5266 | `is_empty` | function | Returns whether `PolledAudioBatchLease` contains no values. | `src/endpoint/polled_audio_driver.rs:228` |
| sym-3ae6167df33d7c121fb4 | `is_open` | function | Reports whether open is true for `EndpointStartGate`. | `src/endpoint/runtime.rs:376` |
| sym-3538665e71576de44f59 | `len` | function | Returns the number of values held by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:224` |
| sym-b64b3175821197baa243 | `lineage` | function | Returns the frame lineage carried by `EndpointAudioFrame`. | `src/endpoint/contract.rs:73` |
| sym-2d7ab65f4fc048dcf957 | `lineage` | function | Returns the frame lineage carried by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:261` |
| sym-f3819a7c5ca28bd21136 | `mark_discontinuity` | function | Marks the next value from `EndpointAudioReceiver` as discontinuous. | `src/endpoint/contract.rs:126` |
| sym-cb3538ab3a970277e126 | `mark_worker_failure` | function | Returns the mark worker failure held by `EndpointAudioReceiver`. | `src/endpoint/contract.rs:130` |
| sym-42f74cceaa2efbea8eb6 | `media` | function | Returns the media held by `EndpointPortInput`. | `src/endpoint/contract.rs:241` |
| sym-b919fbaaf3ee95bd524f | `message` | function | Returns the diagnostic message reported by `EndpointFailure`. | `src/endpoint/runtime.rs:208` |
| sym-0e94a9647c8615d0dc7a | `monotonic_timestamp_ns` | function | Returns the monotonic timestamp nanoseconds held by `SessionTimelineOrigin`. | `src/endpoint/runtime.rs:24` |
| sym-cfc6333600b6679e4a06 | `new` | function | Creates a new `EndpointGroupId`. | `src/endpoint/identity.rs:12` |
| sym-4de54826fa27a0e9d1c1 | `new` | function | Creates a new `PolledAudioEndpoint`. | `src/endpoint/polled_audio.rs:22` |
| sym-87d5185fa15ce8aee6f4 | `new` | function | Creates a new `EndpointPrepareContext`. | `src/endpoint/runtime.rs:108` |
| sym-70d2ba55ff9f6b881949 | `new` | function | Creates a new `EndpointFailure`. | `src/endpoint/runtime.rs:182` |
| sym-535c2950ab66dcde7852 | `node_configuration` | function | Returns the node configuration held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:150` |
| sym-9d0cf1b3fe4d1c6b600a | `observations` | function | Snapshots the bounded edge counters for this endpoint input. | `src/endpoint/contract.rs:138` |
| sym-adfa1bc7f0448364704c | `observations` | function | Returns the observations exposed by `PolledAudioReceipt`. | `src/endpoint/polled_audio_driver.rs:168` |
| sym-dfcacf7bacb5a454f532 | `origin` | function | Returns the origin held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:84` |
| sym-0021c29402fd0b2670af | `plan_edge_observation_handle` | function | Plans edge observation handle for `EndpointAudioReceiver`. | `src/endpoint/contract.rs:147` |
| sym-2a370db0288245b8728d | `polled_at_ns` | function | Returns the polled at nanoseconds held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:285` |
| sym-061986f6042d31f2d0cb | `port_name` | function | Returns the port name held by `EndpointPortInput`. | `src/endpoint/contract.rs:233` |
| sym-5063d668bb96d3521fd8 | `receipt` | function | Returns the receipt associated with `PolledAudioEndpoint`. | `src/endpoint/polled_audio.rs:30` |
| sym-15772bae169ea1388101 | `receiver` | function | Returns the receiver held by `EndpointPortInput`. | `src/endpoint/contract.rs:253` |
| sym-0ad1eada52eee6784c16 | `recv` | function | Receives the next value from `EndpointSignalReceiver`. | `src/endpoint/contract.rs:166` |
| sym-7a6657b985a5058c5383 | `retryability` | function | Returns the retryability associated with `EndpointFailure`. | `src/endpoint/runtime.rs:216` |
| sym-03b173a350c6a57a7269 | `route_context` | function | Returns the route context associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:142` |
| sym-1b20947938fd533901ed | `route_enqueued_at_ns` | function | Monotonic instant when the runtime accepted this frame into the route. | `src/endpoint/contract.rs:78` |
| sym-a077f8087bd1b0ea36cc | `route_enqueued_at_ns` | function | Returns the route enqueued at nanoseconds held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:273` |
| sym-869484fc3f9250429c55 | `route_id` | function | Returns the route identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:269` |
| sym-c5ba950a8b743723f401 | `route_id` | function | Returns the route identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:80` |
| sym-5a21c9e9cb6492af0861 | `route_received_at_ns` | function | Monotonic instant when this endpoint received the frame from the route. | `src/endpoint/contract.rs:83` |
| sym-25666093672968916805 | `route_received_at_ns` | function | Returns the route received at nanoseconds held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:277` |
| sym-fd3eb6acc1e5c5cd9dc4 | `sample_format` | function | Returns the sample format associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:65` |
| sym-17889b1aaf08c5787fe6 | `sample_rate_hz` | function | Returns the sample rate hertz held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:57` |
| sym-81dd0a065a56af223425 | `sample_rate_hz` | function | Returns the sample rate hertz held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:297` |
| sym-e9e52ba605e1fcf03748 | `samples` | function | Returns the audio samples held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:69` |
| sym-cdbeb7d9b5fd4ea1bebf | `samples` | function | Returns the audio samples held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:305` |
| sym-0822a99c4c9d5ae4d890 | `sequence_number` | function | Returns the sequence number held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:49` |
| sym-3d2f454d3fb300f11250 | `session_id` | function | Returns the session identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:125` |
| sym-501622870af6238258a3 | `session_timeline_origin` | function | Returns the session timeline origin associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:146` |
| sym-1233436237bf54848643 | `signal` | function | Returns the signal held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:73` |
| sym-1aaa28c6c59496229026 | `signal_spec` | function | Returns the signal spec held by `EndpointPortInput`. | `src/endpoint/contract.rs:237` |
| sym-0c795be22d819fc224e6 | `source_id` | function | Returns the source identifier held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:41` |
| sym-c50865f7d057cce6f672 | `stage` | function | Returns the stage held by `EndpointFailure`. | `src/endpoint/runtime.rs:204` |
| sym-569afd1361d7a8fabc78 | `stream_id` | function | Returns the stream identifier held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:45` |
| sym-3302bf8592ac13f92cde | `stream_id` | function | Returns the stream identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:289` |
| sym-01c9958af9a5b4195caa | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:53` |
| sym-85887ef875bea65803e6 | `try_poll` | function | Attempts to poll through `PolledAudioReceipt`. | `src/endpoint/polled_audio_driver.rs:110` |
| sym-00b7264beee8a8158b9d | `try_recv` | function | Attempts to receive the next value from `EndpointAudioReceiver` without waiting. | `src/endpoint/contract.rs:110` |
| sym-35d3830bf4f780585bae | `try_recv` | function | Attempts to receive the next value from `EndpointSignalReceiver` without waiting. | `src/endpoint/contract.rs:162` |
| sym-e453bdef6eaa09f39117 | `wait_poll` | function | Waits for a batch until the finite deadline expires. | `src/endpoint/polled_audio_driver.rs:176` |
| sym-7b240a56f2c38a14cb4d | `with_external_details` | function | Attaches stable external failure details without changing Endpoint's provider-neutral lifecycle authority. | `src/endpoint/runtime.rs:194` |
| sym-5d92cd6abbf10b538cae | `pocketstation::endpoint::contract::EndpointAudioFrame` | struct | Read-only audio frame delivered to an external endpoint. | `src/endpoint/contract.rs:18` |
| sym-cb8259906b02d741922e | `pocketstation::endpoint::contract::EndpointAudioReceiver` | struct | Exclusive consumer for one bounded realtime-audio endpoint edge. | `src/endpoint/contract.rs:92` |
| sym-ec411b1248474f059cb2 | `pocketstation::endpoint::contract::EndpointPortInput` | struct | Carries typed input for endpoint port. | `src/endpoint/contract.rs:183` |
| sym-f73348a54557eb092c7e | `pocketstation::endpoint::contract::EndpointSignalReceiver` | struct | Exclusive consumer for one bounded asynchronous signal endpoint edge. | `src/endpoint/contract.rs:153` |
| sym-4bb86447850f820d560d | `pocketstation::endpoint::identity::EndpointGroupId` | struct | Explicit Session-scoped grouping key for endpoints that share one lifecycle. | `src/endpoint/identity.rs:9` |
| sym-086112341efa8cc4d7a6 | `pocketstation::endpoint::polled_audio::PolledAudioEndpoint` | struct | Declares application-polled audio and retains its bounded receipt. | `src/endpoint/polled_audio.rs:16` |
| sym-c8c14c5956491c4ccbc8 | `pocketstation::endpoint::polled_audio_driver::PolledAudioBatchLease` | struct | Holds the ownership or bounded access represented by polled audio batch lease. | `src/endpoint/polled_audio_driver.rs:218` |
| sym-ce53392dc25d08a0be07 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfig` | struct | Configures polled audio endpoint behavior at its owning API boundary. | `src/endpoint/polled_audio_driver.rs:23` |
| sym-b48b465032534fae4941 | `pocketstation::endpoint::polled_audio_driver::PolledAudioFrame` | struct | Carries one polled audio payload together with its declared metadata. | `src/endpoint/polled_audio_driver.rs:256` |
| sym-e19b0ef994fca1143d6a | `pocketstation::endpoint::polled_audio_driver::PolledAudioObservations` | struct | Reports the polled audio observations collected at an observation boundary. | `src/endpoint/polled_audio_driver.rs:56` |
| sym-41e3b3d9082325e3e017 | `pocketstation::endpoint::polled_audio_driver::PolledAudioReceipt` | struct | Retains the identity and observation access returned for polled audio. | `src/endpoint/polled_audio_driver.rs:105` |
| sym-6c4c9e9d93ce90fd85ee | `pocketstation::endpoint::registry::EndpointDriverRegistry` | struct | Indexes registered endpoint driver implementations by their stable identities. | `src/endpoint/registry.rs:54` |
| sym-e967ffd7c9fe16ee8764 | `pocketstation::endpoint::runtime::EndpointCancellationOutcome` | struct | Reports the structured endpoint cancellation outcome. | `src/endpoint/runtime.rs:289` |
| sym-33d3d9274786c3d40f79 | `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | Reports an endpoint driver's terminal observations and any finalization failure. | `src/endpoint/runtime.rs:295` |
| sym-5f26c4f369af94e22c4b | `pocketstation::endpoint::runtime::EndpointDriverObservations` | struct | Reports the endpoint driver observations collected at an observation boundary. | `src/endpoint/runtime.rs:228` |
| sym-c5098d60dc65d940b524 | `pocketstation::endpoint::runtime::EndpointFailure` | struct | Classifies failures surfaced by endpoint failure operations. | `src/endpoint/runtime.rs:174` |
| sym-947b30b6d4b4443dc7af | `pocketstation::endpoint::runtime::EndpointFinalizationOutcome` | struct | Reports the structured endpoint finalization outcome. | `src/endpoint/runtime.rs:301` |
| sym-1398b98a6ac10d07b508 | `pocketstation::endpoint::runtime::EndpointPrepareContext` | struct | Carries the inputs and runtime context required to endpoint prepare. | `src/endpoint/runtime.rs:98` |
| sym-f0bba6959b4f6c86b37d | `pocketstation::endpoint::runtime::EndpointRouteContext` | struct | Typed Session route identity supplied to every endpoint input. | `src/endpoint/runtime.rs:44` |
| sym-37b086b66e8a7ff7a807 | `pocketstation::endpoint::runtime::EndpointStartFailure` | struct | Classifies failures surfaced by endpoint start failure operations. | `src/endpoint/runtime.rs:443` |
| sym-4aa43f188989d8622704 | `pocketstation::endpoint::runtime::EndpointStartGate` | struct | Read-only one-way start barrier shared by endpoint drivers in one startup. | `src/endpoint/runtime.rs:371` |
| sym-fc2716255038bebf5351 | `pocketstation::endpoint::runtime::PreparedEndpoint` | struct | Owns endpoint resources after preparation and before its runtime driver starts. | `src/endpoint/runtime.rs:405` |
| sym-8ce8ed6399db88e1031c | `pocketstation::endpoint::runtime::RunningEndpoint` | struct | Owns a started endpoint driver until shutdown and finalization complete. | `src/endpoint/runtime.rs:481` |
| sym-cbde7491554cf1458df0 | `pocketstation::endpoint::runtime::SessionTimelineOrigin` | struct | One Session-owned anchor in PocketStation's monotonic nanosecond clock. | `src/endpoint/runtime.rs:13` |
| sym-1dbd93f00685779469ae | `EndpointCancellationOutcome::observations` | struct_field | Carries the observations collected for `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:290` |
| sym-0bb31c638640c40d65ea | `EndpointCancellationOutcome::result` | struct_field | Records whether the result operation succeeded and preserves its typed failure for `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:291` |
| sym-e30b971fde8bafec2d7d | `EndpointDriverFinalization::observations` | struct_field | Carries the observations collected for `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:296` |
| sym-3f312e11c9c30f537c17 | `EndpointDriverFinalization::result` | struct_field | Records whether the result operation succeeded and preserves its typed failure for `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:297` |
| sym-26591ca38fc91f403f98 | `EndpointDriverObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:232` |
| sym-3042fb4688f1c3bc09f5 | `EndpointDriverObservations::failures_total` | struct_field | Counts the total number of failures observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:233` |
| sym-2914129c9eb77a6ac869 | `EndpointDriverObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:230` |
| sym-ba04d920f19dc9ef617b | `EndpointDriverObservations::frames_dropped_total` | struct_field | Counts the total number of frames dropped observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:231` |
| sym-fd2745ee2fc824c172eb | `EndpointDriverObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:229` |
| sym-5dbe8de137698daf52f5 | `EndpointDriverRegistryError::Duplicate::node_type_id` | struct_field | Identifies the node type identifier recorded by `Duplicate`. | `src/endpoint/registry.rs:26` |
| sym-853c1daeeb38e3466ec6 | `EndpointDriverRegistryError::Duplicate::operator_id` | struct_field | Identifies the operator identifier recorded by `Duplicate`. | `src/endpoint/registry.rs:25` |
| sym-c70b71d5a1909757f792 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::operator_id` | struct_field | Identifies the operator identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:32` |
| sym-a45719e51311db56ae19 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::registered_node_type_id` | struct_field | Identifies the registered node type identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:33` |
| sym-098ec2fcef156091c011 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::requested_node_type_id` | struct_field | Identifies the requested node type identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:34` |
| sym-d7450b886665f693c255 | `EndpointFinalizationOutcome::join_finalize_result` | struct_field | Records whether the join finalize result operation succeeded and preserves its typed failure for `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:304` |
| sym-f71a74d2c891690533eb | `EndpointFinalizationOutcome::observations` | struct_field | Carries the observations collected for `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:302` |
| sym-bc281d3d8b09873a9efa | `EndpointFinalizationOutcome::request_stop_result` | struct_field | Records whether the request stop result operation succeeded and preserves its typed failure for `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:303` |
| sym-71e2bdab0ab1062febe4 | `EndpointInputOrigin::Source::audio_stem_id` | struct_field | Identifies the audio stem identifier recorded by `Source`. | `src/endpoint/runtime.rs:38` |
| sym-36dd04c79f54df69253b | `EndpointInputOrigin::Source::source_id` | struct_field | Identifies the source identifier recorded by `Source`. | `src/endpoint/runtime.rs:36` |
| sym-c3aa2156b2e8ae48a9d5 | `EndpointInputOrigin::Source::stream_id` | struct_field | Identifies the stream identifier recorded by `Source`. | `src/endpoint/runtime.rs:37` |
| sym-a1bc17c54e0fd69fb009 | `EndpointPrepareError::NotRegistered::node_type_id` | struct_field | Identifies the node type identifier recorded by `NotRegistered`. | `src/endpoint/registry.rs:47` |
| sym-08ea7ae98668e4f410e8 | `EndpointPrepareError::NotRegistered::operator_id` | struct_field | Identifies the operator identifier recorded by `NotRegistered`. | `src/endpoint/registry.rs:46` |
| sym-0280bcbb7b36e1f9d20b | `EndpointReceiver::Audio::receiver` | struct_field | Owns the receiver endpoint through which `Audio` exchanges values. | `src/endpoint/contract.rs:177` |
| sym-7c15983f5471308539d8 | `EndpointReceiver::Audio::sample_spec` | struct_field | Declares the sample rate, channel layout, and format used by `Audio`. | `src/endpoint/contract.rs:178` |
| sym-8c85ed728f6ca1126122 | `PolledAudioEndpointConfig::max_batch_frames` | struct_field | Contains the max batch frames owned or reported by `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:25` |
| sym-1aec62a1a8a6db977ea2 | `PolledAudioEndpointConfig::max_outstanding_leases` | struct_field | Contains the max outstanding leases owned or reported by `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:26` |
| sym-ddb97ef8c35f40eb3ce0 | `PolledAudioEndpointConfig::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:24` |
| sym-f6373c492e3f40c10afe | `PolledAudioObservations::batches_polled_total` | struct_field | Counts the total number of batches polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:69` |
| sym-3d3c6f2c81509b6e7f8e | `PolledAudioObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:63` |
| sym-c32ee63eeee4beefe418 | `PolledAudioObservations::frames_polled_total` | struct_field | Counts the total number of frames polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:70` |
| sym-f6787b0d50c3053ccc88 | `PolledAudioObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:62` |
| sym-251f5c4e9b5c32858d01 | `PolledAudioObservations::invalid_ownership_drops_total` | struct_field | Counts the total number of invalid ownership drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:65` |
| sym-5b27eb7e661114bf1de2 | `PolledAudioObservations::lease_capacity_count` | struct_field | Sets the lease capacity count available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:66` |
| sym-db46e2482b9f0de59d9b | `PolledAudioObservations::lease_exhausted_total` | struct_field | Counts the total number of lease exhausted observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:68` |
| sym-60e6d205dc4532a6886d | `PolledAudioObservations::outstanding_leases` | struct_field | Contains the outstanding leases owned or reported by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:67` |
| sym-78b34b072e453fa9edf1 | `PolledAudioObservations::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:58` |
| sym-c2724347551ef46ad6d3 | `PolledAudioObservations::queue_depth_frames` | struct_field | Reports the queue depth frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:59` |
| sym-f6fb9f6920263449e1e2 | `PolledAudioObservations::queue_depth_invariant_failures_total` | struct_field | Counts the total number of queue depth invariant failures observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:61` |
| sym-55dad236cdb33cb2ee6a | `PolledAudioObservations::queue_full_drops_total` | struct_field | Counts the total number of queue full drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:64` |
| sym-265935d3610a950b210e | `PolledAudioObservations::queue_peak_frames` | struct_field | Reports the queue peak frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:60` |
| sym-cc5b4708a4dbd8bac663 | `PolledAudioObservations::registered_endpoints` | struct_field | References the registered endpoints participating in `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:57` |
| sym-c47143b8b73e85dd114d | `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | Implement this trait to provide endpoint behavior to PocketStation; its methods define the preparation and runtime contract. | `src/endpoint/contract.rs:262` |
| sym-5bb941fb228e70e5ee32 | `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| sym-d5be5ddfe876105c0106 | `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| sym-dba8e3e51898fdfc29a8 | `pocketstation::endpoint::contract::EndpointReceiver::Audio` | variant | Represents the audio alternative defined by `EndpointReceiver`. | `src/endpoint/contract.rs:176` |
| sym-ef61ae80d6d7068f5f8e | `pocketstation::endpoint::contract::EndpointReceiver::Signal` | variant | Represents the signal alternative defined by `EndpointReceiver`. | `src/endpoint/contract.rs:180` |
| sym-558f64d9bd04a5036345 | `pocketstation::endpoint::identity::EndpointPreparationGroup::Route` | variant | Represents the route alternative defined by `EndpointPreparationGroup`. | `src/endpoint/identity.rs:24` |
| sym-daa9085c5659242b0b72 | `pocketstation::endpoint::identity::EndpointPreparationGroup::Shared` | variant | Represents the shared alternative defined by `EndpointPreparationGroup`. | `src/endpoint/identity.rs:25` |
| sym-978375a0be59249f7f2c | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::BatchCapacityTooLarge` | variant | Reports that batch capacity exceeds the supported size limit. | `src/endpoint/polled_audio_driver.rs:50` |
| sym-63380bb9a43d9b88f8d6 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::LeaseCapacityTooLarge` | variant | Reports that lease capacity exceeds the supported size limit. | `src/endpoint/polled_audio_driver.rs:52` |
| sym-dddafdff62c6c8fc3ed7 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::QueueCapacityTooLarge` | variant | Reports that queue capacity exceeds the supported size limit. | `src/endpoint/polled_audio_driver.rs:48` |
| sym-a43421d697e769284905 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroBatchCapacity` | variant | Reports that batch capacity must be greater than zero. | `src/endpoint/polled_audio_driver.rs:44` |
| sym-af48981a7240618749f6 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroLeaseCapacity` | variant | Reports that lease capacity must be greater than zero. | `src/endpoint/polled_audio_driver.rs:46` |
| sym-4af8748ea496f1f89604 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroQueueCapacity` | variant | Reports that queue capacity must be greater than zero. | `src/endpoint/polled_audio_driver.rs:42` |
| sym-e5094729635ad491978b | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::Empty` | variant | Represents an empty value or collection. | `src/endpoint/polled_audio_driver.rs:76` |
| sym-504c5a64f147c1337468 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::LeaseCapacityExhausted` | variant | Reports that the available lease capacity range or capacity is exhausted. | `src/endpoint/polled_audio_driver.rs:78` |
| sym-abc5f1e44969a01e61ca | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::StatePoisoned` | variant | Reports that shared state became unavailable after a panic while locked. | `src/endpoint/polled_audio_driver.rs:80` |
| sym-44596104fc2b88bc781b | `pocketstation::endpoint::registry::EndpointDriverRegistryError::Duplicate` | variant | Reports that the supplied value duplicates an existing record. | `src/endpoint/registry.rs:24` |
| sym-3fa64548a35a9c949169 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyNodeTypeId` | variant | Reports that node type identifier is empty. | `src/endpoint/registry.rs:20` |
| sym-98aa3609aa51ffbe1420 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyOperatorId` | variant | Reports that operator identifier is empty. | `src/endpoint/registry.rs:18` |
| sym-8276bbd388b55f176a66 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::OperatorNodeTypeConflict` | variant | Reports that operator node type conflicts with an existing registration or declaration. | `src/endpoint/registry.rs:31` |
| sym-ed5d2de77317c48cb92e | `pocketstation::endpoint::registry::EndpointPrepareError::Driver` | variant | Classifies a failure at the driver stage or component of `EndpointPrepareError`. | `src/endpoint/registry.rs:50` |
| sym-e59935f9269e7cbc5a04 | `pocketstation::endpoint::registry::EndpointPrepareError::EmptyBatch` | variant | Reports that batch is empty. | `src/endpoint/registry.rs:41` |
| sym-ce86d37ee7b44ccbb6ce | `pocketstation::endpoint::registry::EndpointPrepareError::NotRegistered` | variant | Reports that no t registered is available. | `src/endpoint/registry.rs:45` |
| sym-dddb1010865603fbcb39 | `pocketstation::endpoint::runtime::EndpointFailureRetryability::Never` | variant | Declares an endpoint failure to be never. | `src/endpoint/runtime.rs:167` |
| sym-80a40b94cc9fcf9aeaf2 | `pocketstation::endpoint::runtime::EndpointFailureRetryability::ReconfigurationRequired` | variant | Declares an endpoint failure to be reconfiguration required. | `src/endpoint/runtime.rs:169` |
| sym-c1c3f9244f8f503af1c5 | `pocketstation::endpoint::runtime::EndpointFailureRetryability::Retryable` | variant | Declares an endpoint failure to be retryable. | `src/endpoint/runtime.rs:168` |
| sym-3cdfb3dc720339cd3977 | `pocketstation::endpoint::runtime::EndpointFailureStage::CancelPreparation` | variant | Identifies the cancel preparation state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:158` |
| sym-6914934ddf03a07ca48c | `pocketstation::endpoint::runtime::EndpointFailureStage::JoinFinalize` | variant | Identifies the join finalize state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:161` |
| sym-f05233c8ff710b0a20b1 | `pocketstation::endpoint::runtime::EndpointFailureStage::Prepare` | variant | Identifies the prepare state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:157` |
| sym-fb1fa7b7bce6ae748d4b | `pocketstation::endpoint::runtime::EndpointFailureStage::RequestStop` | variant | Identifies the request stop state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:160` |
| sym-d93e81069b2d5082a258 | `pocketstation::endpoint::runtime::EndpointFailureStage::Start` | variant | Identifies the start state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:159` |
| sym-128c9814777b16f53af6 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Signal` | variant | A typed signal whose detailed provenance is carried by `SignalLineage`. | `src/endpoint/runtime.rs:34` |
| sym-01b74597c85f85568c5d | `pocketstation::endpoint::runtime::EndpointInputOrigin::Source` | variant | Represents the source alternative defined by `EndpointInputOrigin`. | `src/endpoint/runtime.rs:35` |
| sym-c098bd91856062a35d55 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Stem` | variant | Represents the stem alternative defined by `EndpointInputOrigin`. | `src/endpoint/runtime.rs:32` |
| sym-f6aa89e96e4b00ae7ff3 | `pocketstation::endpoint::runtime::EndpointShutdownMode::Abort` | variant | Shuts an endpoint down using the abort mode. | `src/endpoint/runtime.rs:358` |
| sym-8cd975e76d6427c0529b | `pocketstation::endpoint::runtime::EndpointShutdownMode::Drain` | variant | Shuts an endpoint down using the drain mode. | `src/endpoint/runtime.rs:357` |
| sym-8a0c6873e9c295993f2d | `pocketstation::endpoint::runtime::EndpointStartFailureCause::Driver` | variant | Classifies a failure at the driver stage or component of `EndpointStartFailureCause`. | `src/endpoint/runtime.rs:440` |
| sym-ceddb4a392fd1ba1cf84 | `pocketstation::endpoint::runtime::EndpointStartFailureCause::GateAlreadyOpen` | variant | Classifies a failure at the gate already open stage or component of `EndpointStartFailureCause`. | `src/endpoint/runtime.rs:439` |

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

The claims on **Endpoint API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/endpoint/mod.rs:1-4` (`DECLARED`)

For **Endpoint API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
