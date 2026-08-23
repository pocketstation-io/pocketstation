# Graph and route contracts

<!-- claims: CLM-REF-006-CAP-001,CLM-REF-006-CAP-002,CLM-REF-006-CAP-003,CLM-REF-006-CAP-004,CLM-REF-006-SOURCE-001 -->

## Scope

- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.
- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `pocketstation::graph::ports::MAX_ASYNC_PAYLOAD_BYTES` | constant | Sets the maximum supported async payload bytes. | `src/graph/ports.rs:13` |
| `pocketstation::graph::compile::resolve::CompileError` | enum | Classifies failures reported as compile error. | `src/graph/compile/resolve.rs:26` |
| `pocketstation::graph::node::ConfigError` | enum | Classifies failures reported as config error. | `src/graph/node.rs:141` |
| `pocketstation::graph::node::NodeDescriptorError` | enum | Classifies failures reported as node descriptor error. | `src/graph/node.rs:252` |
| `pocketstation::graph::node::NodeError` | enum | Classifies failures reported as node error. | `src/graph/node.rs:149` |
| `pocketstation::graph::partition::ExecutionPartition` | enum | WHERE an operator runs. | `src/graph/partition.rs:18` |
| `pocketstation::graph::partition::SafetyContract` | enum | WHAT an operator guarantees about its runtime behaviour. | `src/graph/partition.rs:82` |
| `pocketstation::graph::plan::PlanError` | enum | Classifies failures reported as plan error. | `src/graph/plan.rs:21` |
| `pocketstation::graph::ports::BackpressurePolicy` | enum | Selects the backpressure policy used by PocketStation. | `src/graph/ports.rs:265` |
| `pocketstation::graph::ports::ChannelLayout` | enum | Enumerates the supported channel layout cases. | `src/graph/ports.rs:27` |
| `pocketstation::graph::ports::ClockDomain` | enum | Enumerates the supported clock domain cases. | `src/graph/ports.rs:249` |
| `pocketstation::graph::ports::CopyPolicy` | enum | Selects the copy policy used by PocketStation. | `src/graph/ports.rs:280` |
| `pocketstation::graph::ports::DeliverySemantics` | enum | Selects the delivery semantics used by PocketStation. | `src/graph/ports.rs:273` |
| `pocketstation::graph::ports::EdgeObservabilityLevel` | enum | Selects the edge observability level used by PocketStation. | `src/graph/ports.rs:294` |
| `pocketstation::graph::ports::LossPolicy` | enum | Selects the loss policy used by PocketStation. | `src/graph/ports.rs:287` |
| `pocketstation::graph::ports::MediaCaps` | enum | Enumerates the supported media caps cases. | `src/graph/ports.rs:85` |
| `pocketstation::graph::ports::MediaKind` | enum | Selects the media kind used by PocketStation. | `src/graph/ports.rs:16` |
| `pocketstation::graph::ports::Multiplicity` | enum | Enumerates the supported multiplicity cases. | `src/graph/ports.rs:169` |
| `pocketstation::graph::ports::PortDirection` | enum | Selects the port direction used by PocketStation. | `src/graph/ports.rs:163` |
| `pocketstation::graph::ports::PortSpecError` | enum | Classifies failures reported as port spec error. | `src/graph/ports.rs:239` |
| `pocketstation::graph::registry::NodeDefinitionRef` | enum | Enumerates the supported node definition ref cases. | `src/graph/registry.rs:32` |
| `pocketstation::graph::registry::NodeRegistrationError` | enum | Classifies failures reported as node registration error. | `src/graph/registry.rs:57` |
| `pocketstation::graph::signal::continuity::SignalContinuityError` | enum | Classifies failures reported as signal continuity error. | `src/graph/signal/continuity.rs:89` |
| `pocketstation::graph::signal::envelope::SignalEnvelopeError` | enum | Classifies failures reported as signal envelope error. | `src/graph/signal/envelope.rs:137` |
| `pocketstation::graph::signal::lineage::SignalDerivationError` | enum | Classifies failures reported as signal derivation error. | `src/graph/signal/lineage.rs:159` |
| `pocketstation::graph::signal::lineage::SignalLineageError` | enum | Classifies failures reported as signal lineage error. | `src/graph/signal/lineage.rs:86` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | enum | Classifies failures reported as async operator manifest error. | `src/graph/signal/operator.rs:321` |
| `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | Selects the operator cancellation policy used by PocketStation. | `src/graph/signal/operator.rs:57` |
| `pocketstation::graph::signal::operator::OperatorFailurePolicy` | enum | Selects the operator failure policy used by PocketStation. | `src/graph/signal/operator.rs:63` |
| `pocketstation::graph::signal::payload::SignalPayload` | enum | Enumerates the supported signal payload cases. | `src/graph/signal/payload.rs:10` |
| `pocketstation::graph::signal::spec::BinaryFormat` | enum | Binary encoding hint for `SignalClass::Binary`. | `src/graph/signal/spec.rs:141` |
| `pocketstation::graph::signal::spec::Codec` | enum | Audio encoding format for `SignalClass::EncodedAudio`. | `src/graph/signal/spec.rs:113` |
| `pocketstation::graph::signal::spec::EventFormat` | enum | Event structure hint for `SignalClass::Event`. | `src/graph/signal/spec.rs:132` |
| `pocketstation::graph::signal::spec::SignalClass` | enum | The fundamental class of data flowing through a port. | `src/graph/signal/spec.rs:156` |
| `pocketstation::graph::signal::spec::SignalSpecError` | enum | Classifies failures reported as signal spec error. | `src/graph/signal/spec.rs:351` |
| `pocketstation::graph::signal::spec::TextFormat` | enum | Text encoding hint for `SignalClass::Text`. | `src/graph/signal/spec.rs:124` |
| `pocketstation::graph::signal::timing::SignalTimingError` | enum | Classifies failures reported as signal timing error. | `src/graph/signal/timing.rs:89` |
| `AsyncNode::cancel` | function | Requests cancellation of `AsyncNode`. | `src/graph/signal/operator.rs:36` |
| `AsyncNode::close` | function | Closes `AsyncNode` to further work. | `src/graph/signal/operator.rs:40` |
| `AsyncNode::flush` | function | Flushes pending output from `AsyncNode` at the end of a run. | `src/graph/signal/operator.rs:32` |
| `AsyncNode::prepare` | function | Prepares resources required by `AsyncNode`. | `src/graph/signal/operator.rs:14` |
| `AsyncNode::process` | function | Processes an input value through `AsyncNode`. | `src/graph/signal/operator.rs:19` |
| `AsyncNode::process_port` | function | Returns the process port associated with `AsyncNode`. | `src/graph/signal/operator.rs:24` |
| `AsyncOperatorFactory::create` | function | Creates the runtime implementation described by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:378` |
| `AsyncOperatorFactory::manifest` | function | Returns the manifest associated with `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:369` |
| `AsyncOperatorFactory::resolve_manifest` | function | Resolves manifest for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:371` |
| `AsyncOperatorFactory::validate_config` | function | Validates config for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:370` |
| `NodeDefinition::descriptor` | function | Returns the descriptor associated with `NodeDefinition`. | `src/graph/registry.rs:22` |
| `NodeDefinition::validate_config` | function | Validates config for `NodeDefinition`. | `src/graph/registry.rs:23` |
| `NodeFactory::descriptor` | function | Returns the descriptor associated with `NodeFactory`. | `src/graph/registry.rs:12` |
| `NodeFactory::instantiate` | function | Instantiates the runtime node described by `NodeFactory`. | `src/graph/registry.rs:14` |
| `NodeFactory::validate_config` | function | Validates config for `NodeFactory`. | `src/graph/registry.rs:13` |
| `RuntimeNode::prepare` | function | Prepares resources required by `RuntimeNode`. | `src/graph/runtime_node.rs:8` |
| `RuntimeNode::process` | function | Processes an input value through `RuntimeNode`. | `src/graph/runtime_node.rs:9` |
| `accepts` | function | Returns the accepts associated with `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:75` |
| `add_node` | function | Adds node for `Pipeline`. | `src/graph/dsl.rs:44` |
| `any` | function | Convenience constructor for a deliberately open boundary port. | `src/graph/signal/spec.rs:264` |
| `as_str` | function | Returns the stable string representation of `NodeTypeId`. | `src/graph/node.rs:16` |
| `as_str` | function | Returns the stable string representation of `OperatorId`. | `src/graph/operator.rs:23` |
| `as_str` | function | Returns the stable string representation of `SignalId`. | `src/graph/signal/spec.rs:29` |
| `as_str` | function | Returns the stable string representation of `SemanticRole`. | `src/graph/signal/spec.rs:64` |
| `as_str` | function | Returns the stable string representation of `SchemaRef`. | `src/graph/signal/spec.rs:94` |
| `async_factory` | function | Returns the async factory associated with `NodeRegistry`. | `src/graph/registry.rs:144` |
| `async_factory_by_operator` | function | Returns the async factory by operator associated with `NodeRegistry`. | `src/graph/registry.rs:151` |
| `audio` | function | Convenience constructor for PCM audio ports. | `src/graph/signal/spec.rs:269` |
| `backpressure` | function | Returns the backpressure associated with `EdgeContract`. | `src/graph/ports.rs:341` |
| `binary` | function | Convenience constructor for opaque or schema-backed binary ports. | `src/graph/signal/spec.rs:299` |
| `bounded_async` | function | Generic bounded asynchronous edge. Connected ports supply the payload representation and the envelope preserves its producer clock. | `src/graph/ports.rs:413` |
| `cancellation` | function | Returns the cancellation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:212` |
| `capacity_signals` | function | Returns the capacity signals associated with `PortPrepareContext`. | `src/graph/node.rs:361` |
| `channel_count` | function | Returns the channel count associated with `ChannelLayout`. | `src/graph/ports.rs:34` |
| `class` | function | Returns the class associated with `SignalSpec`. | `src/graph/signal/spec.rs:215` |
| `clock` | function | Returns the clock associated with `EdgeContract`. | `src/graph/ports.rs:329` |
| `clock_id` | function | Returns the clock identifier associated with `SignalLineage`. | `src/graph/signal/lineage.rs:68` |
| `connect` | function | Connects the requested ports through `Pipeline`. | `src/graph/dsl.rs:55` |
| `connect_with` | function | Connects pipeline ports using the supplied edge contract on `Pipeline`. | `src/graph/dsl.rs:59` |
| `connector_id` | function | Returns the connector identifier associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:153` |
| `contains` | function | Returns the contains associated with `NodeRegistry`. | `src/graph/registry.rs:164` |
| `control` | function | Convenience constructor for control ports. | `src/graph/signal/spec.rs:294` |
| `copy_policy` | function | Returns the copy policy associated with `EdgeContract`. | `src/graph/ports.rs:353` |
| `custom` | function | Convenience constructor for custom / vendor extension ports. | `src/graph/signal/spec.rs:304` |
| `deadline` | function | Returns the deadline associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:208` |
| `definition` | function | Returns the definition associated with `NodeRegistry`. | `src/graph/registry.rs:134` |
| `delivery` | function | Returns the delivery associated with `EdgeContract`. | `src/graph/ports.rs:345` |
| `derivation` | function | Returns the derivation associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:82` |
| `direction` | function | Returns the direction associated with `PortPrepareContext`. | `src/graph/node.rs:345` |
| `direction` | function | Returns the direction associated with `PortSpec`. | `src/graph/ports.rs:217` |
| `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:77` |
| `display_name` | function | Returns the display name associated with `NodeDescriptor`. | `src/graph/node.rs:226` |
| `duration_ns` | function | Returns the duration nanoseconds associated with `SignalTiming`. | `src/graph/signal/timing.rs:83` |
| `edge_contract` | function | Returns the edge contract associated with `PortPrepareContext`. | `src/graph/node.rs:357` |
| `edge_id` | function | Returns the edge identifier associated with `PortPrepareContext`. | `src/graph/node.rs:337` |
| `encoded_audio` | function | Convenience constructor for encoded audio ports. | `src/graph/signal/spec.rs:274` |
| `event` | function | Convenience constructor for event ports. | `src/graph/signal/spec.rs:284` |
| `execution` | function | Returns the execution associated with `NodeDescriptor`. | `src/graph/node.rs:238` |
| `execution_partition` | function | Returns the execution partition associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:58` |
| `failure` | function | Returns the failure associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:216` |
| `fmt` | function | Formats `NodeTypeId` with the requested formatter. | `src/graph/node.rs:31` |
| `fmt` | function | Formats `NodeConfig` with the requested formatter. | `src/graph/node.rs:116` |
| `from` | function | Converts the supplied value into `NodeTypeId`. | `src/graph/node.rs:37` |
| `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:41` |
| `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:47` |
| `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:70` |
| `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:76` |
| `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:100` |
| `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:106` |
| `from_audio` | function | Creates `SignalEnvelope` from audio. | `src/graph/signal/envelope.rs:27` |
| `from_frame` | function | Creates `SignalLineage` from frame. | `src/graph/signal/lineage.rs:46` |
| `from_frame` | function | Creates `SignalTiming` from frame. | `src/graph/signal/timing.rs:56` |
| `from_index` | function | Creates a stable runtime node identifier for externally assembled plans. | `src/graph/spec.rs:12` |
| `generation` | function | Returns the generation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:184` |
| `get` | function | Returns the value held by `NodeConfig`. | `src/graph/node.rs:92` |
| `get` | function | Returns the value held by `NodeRegistry`. | `src/graph/registry.rs:127` |
| `get_f32` | function | Returns the get f32 associated with `NodeConfig`. | `src/graph/node.rs:100` |
| `get_u32` | function | Returns the get u32 associated with `NodeConfig`. | `src/graph/node.rs:104` |
| `id` | function | Returns the id associated with `NodeHandle`. | `src/graph/dsl.rs:15` |
| `in_` | function | Selects a named input port from `NodeHandle`. | `src/graph/dsl.rs:24` |
| `index` | function | Returns the index associated with `NodeId`. | `src/graph/spec.rs:16` |
| `index` | function | Returns the index associated with `EdgeId`. | `src/graph/spec.rs:25` |
| `input_edge` | function | Returns the input edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:192` |
| `input_ports` | function | Returns the input ports associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:224` |
| `inputs` | function | Returns the inputs associated with `NodeDescriptor`. | `src/graph/node.rs:230` |
| `inputs` | function | Returns the inputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:62` |
| `into_payload` | function | Converts `SignalEnvelope` into payload. | `src/graph/signal/envelope.rs:86` |
| `into_spec` | function | Converts `Pipeline` into spec. | `src/graph/dsl.rs:90` |
| `is_audio` | function | Returns `true` for classes that carry real-time audio on the hot path. | `src/graph/signal/spec.rs:180` |
| `is_compatible_with` | function | Returns whether compatible with applies to `ChannelLayout`. | `src/graph/ports.rs:42` |
| `is_compatible_with` | function | Returns whether compatible with applies to `AudioCaps`. | `src/graph/ports.rs:56` |
| `is_compatible_with` | function | Returns whether compatible with applies to `MediaCaps`. | `src/graph/ports.rs:110` |
| `is_compatible_with` | function | Returns `true` if two signal classes are compatible for edge wiring. | `src/graph/signal/spec.rs:188` |
| `is_compatible_with` | function | Returns `true` if this spec can connect to `other` on an edge. | `src/graph/signal/spec.rs:324` |
| `is_portable` | function | Reports whether this value is a portable implementation contract ID. | `src/graph/operator.rs:31` |
| `is_portable` | function | Reports whether this custom signal ID is portable across packages, languages, and processes. | `src/graph/signal/spec.rs:35` |
| `is_realtime` | function | Returns whether realtime applies to `ClockDomain`. | `src/graph/ports.rs:259` |
| `is_sensitive` | function | Returns whether sensitive applies to `NodeConfig`. | `src/graph/node.rs:96` |
| `is_stateful` | function | Returns whether stateful applies to `NodeDescriptor`. | `src/graph/node.rs:246` |
| `is_terminal` | function | Returns whether terminal applies to `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:86` |
| `is_valid_for` | function | Returns `true` if this contract is compatible with the given partition. | `src/graph/partition.rs:107` |
| `is_well_formed` | function | Reports whether this value follows the portable node-type syntax. | `src/graph/node.rs:25` |
| `iter` | function | Iterates over the values held by `NodeConfig`. | `src/graph/node.rs:108` |
| `jitter_budget_ms` | function | Returns the jitter budget milliseconds associated with `EdgeContract`. | `src/graph/ports.rs:337` |
| `kind` | function | Returns the kind represented by `MediaCaps`. | `src/graph/ports.rs:97` |
| `latency_budget_ms` | function | Returns the latency budget milliseconds associated with `EdgeContract`. | `src/graph/ports.rs:333` |
| `lineage` | function | Returns the frame lineage associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:78` |
| `loss` | function | Returns the loss associated with `EdgeContract`. | `src/graph/ports.rs:349` |
| `map_payload` | function | Transforms the payload held by `SignalEnvelope` while preserving envelope metadata. | `src/graph/signal/envelope.rs:45` |
| `max_payload_bytes` | function | Returns the max payload bytes associated with `EdgeContract`. | `src/graph/ports.rs:361` |
| `media` | function | Returns the media associated with `PortPrepareContext`. | `src/graph/node.rs:353` |
| `media` | function | Returns the media associated with `PortSpec`. | `src/graph/ports.rs:225` |
| `media` | function | Returns the media associated with `EdgeContract`. | `src/graph/ports.rs:325` |
| `metrics` | function | Convenience constructor for metrics ports. | `src/graph/signal/spec.rs:289` |
| `multiplicity` | function | Returns the multiplicity associated with `PortSpec`. | `src/graph/ports.rs:229` |
| `name` | function | Returns the name associated with `PortSpec`. | `src/graph/ports.rs:213` |
| `needs_bridge_to` | function | Returns `true` if crossing from `self` to `other` requires a compiler-inserted Bridge. | `src/graph/partition.rs:71` |
| `negotiate` | function | Negotiates the compatible media capabilities shared by `MediaCaps` and its peer. | `src/graph/ports.rs:124` |
| `new` | function | Creates a new `Pipeline`. | `src/graph/dsl.rs:40` |
| `new` | function | Creates a new `NodeConfig`. | `src/graph/node.rs:62` |
| `new` | function | Creates a new `NodeDescriptor`. | `src/graph/node.rs:176` |
| `new` | function | Creates a new `PrepareContext`. | `src/graph/node.rs:271` |
| `new` | function | Creates a new `PortPrepareContext`. | `src/graph/node.rs:293` |
| `new` | function | Creates a new `OperatorId`. | `src/graph/operator.rs:19` |
| `new` | function | Creates a new `PortSpec`. | `src/graph/ports.rs:185` |
| `new` | function | Creates a new `NodeRegistry`. | `src/graph/registry.rs:73` |
| `new` | function | Creates a new `SignalDerivation`. | `src/graph/signal/lineage.rs:107` |
| `new` | function | Creates a new `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:144` |
| `new` | function | Creates a new `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:29` |
| `new` | function | Creates a new `SignalId`. | `src/graph/signal/spec.rs:25` |
| `new` | function | Creates a new `SemanticRole`. | `src/graph/signal/spec.rs:60` |
| `new` | function | Creates a new `SchemaRef`. | `src/graph/signal/spec.rs:90` |
| `new` | function | Creates a new `SignalSpec`. | `src/graph/signal/spec.rs:226` |
| `node` | function | Returns the node associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:188` |
| `observability` | function | Returns the observability associated with `EdgeContract`. | `src/graph/ports.rs:357` |
| `observe` | function | Returns the current observation exposed by `SignalContinuityTracker`. | `src/graph/signal/continuity.rs:18` |
| `observed` | function | Creates observed signal timing for `SignalTiming`. | `src/graph/signal/timing.rs:38` |
| `observed_timestamp_ns` | function | Returns the observed timestamp nanoseconds associated with `SignalTiming`. | `src/graph/signal/timing.rs:75` |
| `operator_generation` | function | Returns the operator generation associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:150` |
| `operator_id` | function | Returns the operator identifier associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:144` |
| `operator_id` | function | Returns the operator identifier associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:176` |
| `operator_revision` | function | Returns the operator revision associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:147` |
| `out` | function | Selects a named output port from `NodeHandle`. | `src/graph/dsl.rs:18` |
| `output_edge` | function | Returns the output edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:196` |
| `output_ports` | function | Returns the output ports associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:231` |
| `output_roles` | function | Returns the output roles associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:220` |
| `outputs` | function | Returns the outputs associated with `NodeDescriptor`. | `src/graph/node.rs:234` |
| `outputs` | function | Returns the outputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:66` |
| `payload` | function | Returns the payload associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:62` |
| `payload_size_bytes` | function | Returns the payload size bytes associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:66` |
| `permission` | function | Returns the permission associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:204` |
| `policy_epoch` | function | Returns the policy epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:80` |
| `port_name` | function | Returns the port name associated with `PortPrepareContext`. | `src/graph/node.rs:341` |
| `queue_capacity_frames` | function | Returns the queue capacity frames associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:200` |
| `rank` | function | Priority rank for scheduling: lower = higher priority. | `src/graph/partition.rs:60` |
| `rank` | function | Returns the rank associated with `EdgeObservabilityLevel`. | `src/graph/ports.rs:301` |
| `realtime_audio` | function | Generic realtime PCM edge. Concrete sample rate, frame size, and channel layout are negotiated from connected ports. | `src/graph/ports.rs:391` |
| `register` | function | Registers a node definition with `NodeRegistry` while preserving unique identities. | `src/graph/registry.rs:77` |
| `register_async` | function | Registers async for `NodeRegistry`. | `src/graph/registry.rs:89` |
| `register_definition` | function | Registers definition for `NodeRegistry`. | `src/graph/registry.rs:112` |
| `required` | function | Returns the required associated with `PortSpec`. | `src/graph/ports.rs:233` |
| `requires_realtime_safety` | function | Returns `true` if the partition requires strict real-time safety. | `src/graph/partition.rs:55` |
| `revision` | function | Returns the revision associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:180` |
| `role` | function | Returns the role associated with `SignalSpec`. | `src/graph/signal/spec.rs:219` |
| `safety` | function | Returns the safety associated with `NodeDescriptor`. | `src/graph/node.rs:242` |
| `schema` | function | Returns the schema associated with `SignalSpec`. | `src/graph/signal/spec.rs:223` |
| `sequence_number` | function | Returns the sequence number associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:90` |
| `sequence_number` | function | Returns the sequence number associated with `SignalLineage`. | `src/graph/signal/lineage.rs:71` |
| `session_id` | function | Returns the session identifier associated with `SignalLineage`. | `src/graph/signal/lineage.rs:59` |
| `session_timestamp_ns` | function | Returns the session timestamp nanoseconds associated with `SignalTiming`. | `src/graph/signal/timing.rs:79` |
| `signal` | function | Returns the signal associated with `PortPrepareContext`. | `src/graph/node.rs:349` |
| `signal` | function | Returns the signal associated with `PortSpec`. | `src/graph/ports.rs:221` |
| `signal_spec` | function | Returns the signal spec associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:70` |
| `size_bytes` | function | Owned media bytes represented by this payload. Envelope metadata and queue slot storage are fixed-size and accounted separately by the edge. | `src/graph/signal/payload.rs:37` |
| `source_generation` | function | Returns the source generation associated with `SignalLineage`. | `src/graph/signal/lineage.rs:74` |
| `source_id` | function | Returns the source identifier associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:100` |
| `source_id` | function | Returns the source identifier associated with `SignalLineage`. | `src/graph/signal/lineage.rs:65` |
| `source_timestamp_ns` | function | Returns the source timestamp nanoseconds associated with `SignalTiming`. | `src/graph/signal/timing.rs:71` |
| `stream_id` | function | Returns the stream identifier associated with `SignalLineage`. | `src/graph/signal/lineage.rs:62` |
| `supports` | function | Returns the supports associated with `SignalPayload`. | `src/graph/signal/payload.rs:17` |
| `supports_signal` | function | Returns whether supports signal applies to `MediaCaps`. | `src/graph/ports.rs:142` |
| `syntax_version` | function | Returns the syntax version associated with `OperatorId`. | `src/graph/operator.rs:35` |
| `text` | function | Convenience constructor for text ports. | `src/graph/signal/spec.rs:279` |
| `timestamp_end_ns` | function | Returns the timestamp end nanoseconds associated with `SignalTiming`. | `src/graph/signal/timing.rs:65` |
| `timestamp_ns` | function | Returns the timestamp nanoseconds associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:110` |
| `timing` | function | Returns the timing associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:74` |
| `try_new` | function | Creates a new `SignalLineage` after validating its inputs. | `src/graph/signal/lineage.rs:21` |
| `try_new` | function | Creates a new `SignalTiming` after validating its inputs. | `src/graph/signal/timing.rs:14` |
| `type_id` | function | Returns the type identifier associated with `NodeDescriptor`. | `src/graph/node.rs:222` |
| `untracked` | function | Creates an envelope for data that has not yet entered a source-aware Session. Session sources must attach lineage before routing it. | `src/graph/signal/envelope.rs:17` |
| `upstream_lineage` | function | Returns the upstream lineage associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:138` |
| `upstream_timing` | function | Returns the upstream timing associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:141` |
| `validate` | function | Validates `SignalEnvelope` against its declared contract. | `src/graph/signal/envelope.rs:117` |
| `validate` | function | Validates `AsyncOperatorManifest` against its declared contract. | `src/graph/signal/operator.rs:238` |
| `validate` | function | Validates `SignalSpec` against its declared contract. | `src/graph/signal/spec.rs:328` |
| `wire_id` | function | Stable language-neutral identifier for the fundamental wire class. Semantic role and schema remain separate fields. | `src/graph/signal/spec.rs:236` |
| `with` | function | Returns `NodeConfig` with the supplied entry applied. | `src/graph/node.rs:66` |
| `with_backpressure` | function | Sets the backpressure on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:370` |
| `with_copy_policy` | function | Sets the copy policy on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:375` |
| `with_derivation` | function | Sets the derivation on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:57` |
| `with_duration_ns` | function | Sets the duration nanoseconds on `SignalTiming` and returns the updated value. | `src/graph/signal/timing.rs:47` |
| `with_jitter_budget_ms` | function | Sets the jitter budget milliseconds on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:380` |
| `with_lineage` | function | Sets the lineage on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:51` |
| `with_max_payload_bytes` | function | Sets the max payload bytes on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:385` |
| `with_media` | function | Sets the media on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:365` |
| `with_role` | function | Attach a semantic role annotation. | `src/graph/signal/spec.rs:309` |
| `with_schema` | function | Attach a schema reference. | `src/graph/signal/spec.rs:315` |
| `with_sensitive` | function | Adds a setup-time value whose normal debug representation is redacted. | `src/graph/node.rs:81` |
| `pocketstation::graph` | module | Stable signal, port, capability, partition, and extension contracts. | `src/graph/mod.rs:1` |
| `pocketstation::graph::dsl::NodeHandle` | struct | Owns bounded access to node. | `src/graph/dsl.rs:10` |
| `pocketstation::graph::dsl::Pipeline` | struct | Represents pipeline in the PocketStation API. | `src/graph/dsl.rs:33` |
| `pocketstation::graph::node::NodeConfig` | struct | Configures node. | `src/graph/node.rs:43` |
| `pocketstation::graph::node::NodeDescriptor` | struct | Describes the node descriptor contract. | `src/graph/node.rs:165` |
| `pocketstation::graph::node::NodeTypeId` | struct | Uniquely identifies node type. | `src/graph/node.rs:13` |
| `pocketstation::graph::node::PortPrepareContext` | struct | Exact graph-owned contract for one prepared node port. | `src/graph/node.rs:282` |
| `pocketstation::graph::node::PrepareContext` | struct | Represents prepare context in the PocketStation API. | `src/graph/node.rs:266` |
| `pocketstation::graph::operator::OperatorId` | struct | Open identifier for a registered graph operator implementation. | `src/graph/operator.rs:16` |
| `pocketstation::graph::ports::AudioCaps` | struct | Represents audio caps in the PocketStation API. | `src/graph/ports.rs:48` |
| `pocketstation::graph::ports::EdgeContract` | struct | Represents edge contract in the PocketStation API. | `src/graph/ports.rs:311` |
| `pocketstation::graph::ports::PortSpec` | struct | Configures port. | `src/graph/ports.rs:175` |
| `pocketstation::graph::registry::NodeRegistry` | struct | Represents node registry in the PocketStation API. | `src/graph/registry.rs:67` |
| `pocketstation::graph::signal::continuity::SignalContinuityObservation` | struct | Represents signal continuity observation in the PocketStation API. | `src/graph/signal/continuity.rs:6` |
| `pocketstation::graph::signal::continuity::SignalContinuityTracker` | struct | Represents signal continuity tracker in the PocketStation API. | `src/graph/signal/continuity.rs:13` |
| `pocketstation::graph::signal::envelope::SignalEnvelope` | struct | Represents signal envelope in the PocketStation API. | `src/graph/signal/envelope.rs:6` |
| `pocketstation::graph::signal::lineage::SignalDerivation` | struct | Source-independent record of the signal consumed by an operator. | `src/graph/signal/lineage.rs:97` |
| `pocketstation::graph::signal::lineage::SignalLineage` | struct | Represents signal lineage in the PocketStation API. | `src/graph/signal/lineage.rs:8` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifest` | struct | Describes the async operator manifest contract. | `src/graph/signal/operator.rs:127` |
| `pocketstation::graph::signal::operator::OperatorDeadlinePolicy` | struct | Configures operator deadline. | `src/graph/signal/operator.rs:52` |
| `pocketstation::graph::signal::operator::OperatorOutputRolePolicy` | struct | Configures operator output role. | `src/graph/signal/operator.rs:69` |
| `pocketstation::graph::signal::operator::OperatorPermissionPolicy` | struct | Configures operator permission. | `src/graph/signal/operator.rs:46` |
| `pocketstation::graph::signal::preparation::AsyncOperatorPrepareContext` | struct | Complete graph-owned preparation contract for one asynchronous Operator. | `src/graph/signal/preparation.rs:22` |
| `pocketstation::graph::signal::spec::SchemaRef` | struct | Reference to an external schema document. | `src/graph/signal/spec.rs:87` |
| `pocketstation::graph::signal::spec::SemanticRole` | struct | Semantic role annotation on a port. | `src/graph/signal/spec.rs:57` |
| `pocketstation::graph::signal::spec::SignalId` | struct | Opaque identifier for a custom signal type. | `src/graph/signal/spec.rs:22` |
| `pocketstation::graph::signal::spec::SignalSpec` | struct | Full signal contract for a single port. | `src/graph/signal/spec.rs:205` |
| `pocketstation::graph::signal::timing::SignalTiming` | struct | Represents signal timing in the PocketStation API. | `src/graph/signal/timing.rs:6` |
| `pocketstation::graph::spec::EdgeId` | struct | Uniquely identifies edge. | `src/graph/spec.rs:22` |
| `pocketstation::graph::spec::EdgeSpec` | struct | Configures edge. | `src/graph/spec.rs:50` |
| `pocketstation::graph::spec::GraphSpec` | struct | Configures graph. | `src/graph/spec.rs:58` |
| `pocketstation::graph::spec::InputPortRef` | struct | Represents input port ref in the PocketStation API. | `src/graph/spec.rs:37` |
| `pocketstation::graph::spec::NodeId` | struct | Uniquely identifies node. | `src/graph/spec.rs:8` |
| `pocketstation::graph::spec::NodeSpec` | struct | Configures node. | `src/graph/spec.rs:43` |
| `pocketstation::graph::spec::OutputPortRef` | struct | Represents output port ref in the PocketStation API. | `src/graph/spec.rs:31` |
| `AudioCaps::channel_layout` | struct_field | Stores the channel layout associated with `AudioCaps`. | `src/graph/ports.rs:51` |
| `AudioCaps::format` | struct_field | Stores the format associated with `AudioCaps`. | `src/graph/ports.rs:52` |
| `AudioCaps::frame_samples` | struct_field | Stores the frame samples associated with `AudioCaps`. | `src/graph/ports.rs:50` |
| `AudioCaps::sample_rate_hz` | struct_field | Stores the sample rate value for `AudioCaps`, in hertz. | `src/graph/ports.rs:49` |
| `CompileError::AdapterUnavailable::edge` | struct_field | Stores the edge associated with `AdapterUnavailable`. | `src/graph/compile/resolve.rs:62` |
| `CompileError::AdapterUnavailable::type_id` | struct_field | Identifies the type associated with `AdapterUnavailable`. | `src/graph/compile/resolve.rs:62` |
| `CompileError::ClockDomainMismatch::expected` | struct_field | Records the value expected by `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:41` |
| `CompileError::ClockDomainMismatch::found` | struct_field | Stores the found associated with `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:42` |
| `CompileError::ClockDomainMismatch::node` | struct_field | Stores the node associated with `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:39` |
| `CompileError::ClockDomainMismatch::port` | struct_field | Stores the port associated with `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:40` |
| `CompileError::InvalidConfig::reason` | struct_field | Carries the reason reported by `InvalidConfig`. | `src/graph/compile/resolve.rs:30` |
| `CompileError::InvalidConfig::type_id` | struct_field | Identifies the type associated with `InvalidConfig`. | `src/graph/compile/resolve.rs:30` |
| `CompileError::InvalidRealtimeEdge::edge` | struct_field | Stores the edge associated with `InvalidRealtimeEdge`. | `src/graph/compile/resolve.rs:58` |
| `CompileError::InvalidRealtimeEdge::reason` | struct_field | Carries the reason reported by `InvalidRealtimeEdge`. | `src/graph/compile/resolve.rs:58` |
| `CompileError::InvalidSafetyContract::execution` | struct_field | Stores the execution associated with `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:54` |
| `CompileError::InvalidSafetyContract::node` | struct_field | Stores the node associated with `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:52` |
| `CompileError::InvalidSafetyContract::safety` | struct_field | Stores the safety associated with `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:55` |
| `CompileError::InvalidSafetyContract::type_id` | struct_field | Identifies the type associated with `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:53` |
| `CompileError::MediaMismatch::edge` | struct_field | Stores the edge associated with `MediaMismatch`. | `src/graph/compile/resolve.rs:45` |
| `CompileError::MediaMismatch::from` | struct_field | Identifies the origin represented by `MediaMismatch`. | `src/graph/compile/resolve.rs:45` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Graph and signal failures](/docs/errors/graph-and-signals.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Asynchronous signal lane](/docs/internals/async-signal-lane.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/mod.rs:1-67` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
