# Graph and route contracts

<!-- claims: CLM-REF-006-CAP-001,CLM-REF-006-CAP-002,CLM-REF-006-CAP-003,CLM-REF-006-CAP-004,CLM-REF-006-SOURCE-001 -->

## Scope

- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.
- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.

The scope of **Graph and route contracts** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Graph and route contracts**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-39765f9cd0c0fb34c75a | `pocketstation::graph::operator::OPERATOR_ID_SYNTAX_VERSION` | constant | Version of the serialized operator-identifier syntax. | `src/graph/operator.rs:6` |
| sym-c3da8a156b60fde775c7 | `pocketstation::graph::plan::EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES` | constant | A sequential edge receiver may retain the frame it just popped while it processes that frame. Copy-pool sizing must cover that owned frame in addition to every frame that can still be queued. | `src/graph/plan.rs:16` |
| sym-06754e50ae4dbb11aece | `pocketstation::graph::plan::EDGE_RING_CAPACITY_FRAMES` | constant | Defines the public edge ring capacity frames value. | `src/graph/plan.rs:12` |
| sym-bfcff1df5a0c4e946955 | `pocketstation::graph::plan::FRAME_BYTES_MONO_48K` | constant | Defines the public frame bytes mono 48 k value. | `src/graph/plan.rs:11` |
| sym-45f8a54a67e90ca0e98a | `pocketstation::graph::plan::MAX_EDGE_RING_CAPACITY_FRAMES` | constant | Sets the maximum supported edge ring capacity frames. | `src/graph/plan.rs:17` |
| sym-8965f2238e75bd29390b | `pocketstation::graph::ports::MAX_ASYNC_PAYLOAD_BYTES` | constant | Sets the maximum supported async payload bytes. | `src/graph/ports.rs:13` |
| sym-4fcb9723ef545df20ac8 | `pocketstation::graph::compile::resolve::CompileError` | enum | Classifies failures reported as compile error. | `src/graph/compile/resolve.rs:26` |
| sym-c3a1ecffd346ee433f7d | `pocketstation::graph::node::ConfigError` | enum | Classifies failures reported as config error. | `src/graph/node.rs:141` |
| sym-f025a37eca498e64758e | `pocketstation::graph::node::NodeDescriptorError` | enum | Classifies failures reported as node descriptor error. | `src/graph/node.rs:252` |
| sym-e77de4e7bba3768a4f6c | `pocketstation::graph::node::NodeError` | enum | Classifies failures reported as node error. | `src/graph/node.rs:149` |
| sym-cc20f40d48d0b1cb1f05 | `pocketstation::graph::partition::ExecutionPartition` | enum | WHERE an operator runs. | `src/graph/partition.rs:18` |
| sym-029e7dbe36cf7d064f9a | `pocketstation::graph::partition::SafetyContract` | enum | WHAT an operator guarantees about its runtime behaviour. | `src/graph/partition.rs:82` |
| sym-af7ad85c4fbca3400638 | `pocketstation::graph::plan::PlanError` | enum | Classifies failures reported as plan error. | `src/graph/plan.rs:21` |
| sym-483b55a8b79b880f182d | `pocketstation::graph::ports::BackpressurePolicy` | enum | Selects the backpressure policy used by PocketStation. | `src/graph/ports.rs:265` |
| sym-4c00608939c517ac97d7 | `pocketstation::graph::ports::ChannelLayout` | enum | Enumerates the supported channel layout cases. | `src/graph/ports.rs:27` |
| sym-6b73e2a8a016f8e04982 | `pocketstation::graph::ports::ClockDomain` | enum | Enumerates the supported clock domain cases. | `src/graph/ports.rs:249` |
| sym-eab5bcb627544f2e4f84 | `pocketstation::graph::ports::CopyPolicy` | enum | Selects the copy policy used by PocketStation. | `src/graph/ports.rs:280` |
| sym-ff6064aef4e4fce40483 | `pocketstation::graph::ports::DeliverySemantics` | enum | Selects the delivery semantics used by PocketStation. | `src/graph/ports.rs:273` |
| sym-d66bbe8dc359d2f3928f | `pocketstation::graph::ports::EdgeObservabilityLevel` | enum | Selects the edge observability level used by PocketStation. | `src/graph/ports.rs:294` |
| sym-eb1f585bd7270ee1a32c | `pocketstation::graph::ports::LossPolicy` | enum | Selects the loss policy used by PocketStation. | `src/graph/ports.rs:287` |
| sym-3c90a46c18851e763795 | `pocketstation::graph::ports::MediaCaps` | enum | Enumerates the supported media caps cases. | `src/graph/ports.rs:85` |
| sym-d7f179dd53d4e0981b38 | `pocketstation::graph::ports::MediaKind` | enum | Selects the media kind used by PocketStation. | `src/graph/ports.rs:16` |
| sym-2ceeae7585bdacd3fc7a | `pocketstation::graph::ports::Multiplicity` | enum | Enumerates the supported multiplicity cases. | `src/graph/ports.rs:169` |
| sym-137cad195a6ea3af43e0 | `pocketstation::graph::ports::PortDirection` | enum | Selects the port direction used by PocketStation. | `src/graph/ports.rs:163` |
| sym-48116bf4757dd0ea87d6 | `pocketstation::graph::ports::PortSpecError` | enum | Classifies failures reported as port spec error. | `src/graph/ports.rs:239` |
| sym-439a0bf5cb33e6b94cb5 | `pocketstation::graph::registry::NodeDefinitionRef` | enum | Enumerates the supported node definition ref cases. | `src/graph/registry.rs:32` |
| sym-b6d81f6ae57aa0d62bfa | `pocketstation::graph::registry::NodeRegistrationError` | enum | Classifies failures reported as node registration error. | `src/graph/registry.rs:57` |
| sym-d955a14f909addfc1475 | `pocketstation::graph::signal::continuity::SignalContinuityError` | enum | Classifies failures reported as signal continuity error. | `src/graph/signal/continuity.rs:89` |
| sym-bcc991272a84381d89b1 | `pocketstation::graph::signal::envelope::SignalEnvelopeError` | enum | Classifies failures reported as signal envelope error. | `src/graph/signal/envelope.rs:137` |
| sym-6970d81cb09c75a72210 | `pocketstation::graph::signal::lineage::SignalDerivationError` | enum | Classifies failures reported as signal derivation error. | `src/graph/signal/lineage.rs:159` |
| sym-aafc34922d43197db130 | `pocketstation::graph::signal::lineage::SignalLineageError` | enum | Classifies failures reported as signal lineage error. | `src/graph/signal/lineage.rs:86` |
| sym-f7159cc45b5de1e0732c | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | enum | Classifies failures reported as async operator manifest error. | `src/graph/signal/operator.rs:321` |
| sym-26d6430f6471c39c45bb | `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | Selects the operator cancellation policy used by PocketStation. | `src/graph/signal/operator.rs:57` |
| sym-1c4c408d47a3d6396cfb | `pocketstation::graph::signal::operator::OperatorFailurePolicy` | enum | Selects the operator failure policy used by PocketStation. | `src/graph/signal/operator.rs:63` |
| sym-9868bde57bc5fc380680 | `pocketstation::graph::signal::payload::SignalPayload` | enum | Enumerates the supported signal payload cases. | `src/graph/signal/payload.rs:10` |
| sym-3aaa0287f72b8a594c51 | `pocketstation::graph::signal::spec::BinaryFormat` | enum | Binary encoding hint for `SignalClass::Binary`. | `src/graph/signal/spec.rs:141` |
| sym-66ab9aa90b84727d686a | `pocketstation::graph::signal::spec::Codec` | enum | Audio encoding format for `SignalClass::EncodedAudio`. | `src/graph/signal/spec.rs:113` |
| sym-d1913dcde8b63010c8c6 | `pocketstation::graph::signal::spec::EventFormat` | enum | Event structure hint for `SignalClass::Event`. | `src/graph/signal/spec.rs:132` |
| sym-e97c39ab79fc17dd35dd | `pocketstation::graph::signal::spec::SignalClass` | enum | The fundamental class of data flowing through a port. | `src/graph/signal/spec.rs:156` |
| sym-3c1758bf6bba6308e9df | `pocketstation::graph::signal::spec::SignalSpecError` | enum | Classifies failures reported as signal spec error. | `src/graph/signal/spec.rs:351` |
| sym-f83afadac94e75998e53 | `pocketstation::graph::signal::spec::TextFormat` | enum | Text encoding hint for `SignalClass::Text`. | `src/graph/signal/spec.rs:124` |
| sym-5818bdec6a1b05fc739b | `pocketstation::graph::signal::timing::SignalTimingError` | enum | Classifies failures reported as signal timing error. | `src/graph/signal/timing.rs:89` |
| sym-5aae2cabada09c6ba8a6 | `AsyncNode::cancel` | function | Requests cancellation of `AsyncNode`. | `src/graph/signal/operator.rs:36` |
| sym-e6e78ffac15439249c55 | `AsyncNode::close` | function | Closes `AsyncNode` to further work. | `src/graph/signal/operator.rs:40` |
| sym-60c43b986b92ec3bfda3 | `AsyncNode::flush` | function | Flushes pending output from `AsyncNode` at the end of a run. | `src/graph/signal/operator.rs:32` |
| sym-4c83f61648e2a06176df | `AsyncNode::prepare` | function | Prepares resources required by `AsyncNode`. | `src/graph/signal/operator.rs:14` |
| sym-21b01f5c6ed3a11f1a20 | `AsyncNode::process` | function | Processes an input value through `AsyncNode`. | `src/graph/signal/operator.rs:19` |
| sym-0942eba5d5fceac41988 | `AsyncNode::process_port` | function | Returns the process port held by `AsyncNode`. | `src/graph/signal/operator.rs:24` |
| sym-708ec8c1d50ce7123227 | `AsyncOperatorFactory::create` | function | Creates the runtime implementation described by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:378` |
| sym-bcfefc27332935ccca94 | `AsyncOperatorFactory::manifest` | function | Returns the manifest held by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:369` |
| sym-5e256a2c9ff41b5bdcfd | `AsyncOperatorFactory::resolve_manifest` | function | Resolves manifest for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:371` |
| sym-7c0f476638ec6b056d69 | `AsyncOperatorFactory::validate_config` | function | Validates config for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:370` |
| sym-92eb8f203e1ebecb6f82 | `NodeDefinition::descriptor` | function | Returns the descriptor associated with `NodeDefinition`. | `src/graph/registry.rs:22` |
| sym-9d7949ea0b416de4d1cb | `NodeDefinition::validate_config` | function | Validates config for `NodeDefinition`. | `src/graph/registry.rs:23` |
| sym-eaddd3481c0f9e30d8c0 | `NodeFactory::descriptor` | function | Returns the descriptor associated with `NodeFactory`. | `src/graph/registry.rs:12` |
| sym-dcd8abe9dd507babe210 | `NodeFactory::instantiate` | function | Instantiates the runtime node described by `NodeFactory`. | `src/graph/registry.rs:14` |
| sym-f94849632be857a7354a | `NodeFactory::validate_config` | function | Validates config for `NodeFactory`. | `src/graph/registry.rs:13` |
| sym-f2fa4350e7a859527850 | `RuntimeNode::prepare` | function | Prepares resources required by `RuntimeNode`. | `src/graph/runtime_node.rs:8` |
| sym-2ef7f17b8694f3d96cbd | `RuntimeNode::process` | function | Processes an input value through `RuntimeNode`. | `src/graph/runtime_node.rs:9` |
| sym-c1c61c7d9b4bef0b1508 | `accepts` | function | Returns whether accepts is true for `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:75` |
| sym-87003503ff089a9dd863 | `add_node` | function | Adds node for `Pipeline`. | `src/graph/dsl.rs:44` |
| sym-6ce1a2cb5ee1f86493cf | `any` | function | Convenience constructor for a deliberately open boundary port. | `src/graph/signal/spec.rs:264` |
| sym-d65e4bd95d21a2527dcc | `as_str` | function | Returns the stable string representation of `NodeTypeId`. | `src/graph/node.rs:16` |
| sym-f50d405ba82ebeb40541 | `as_str` | function | Returns the stable string representation of `OperatorId`. | `src/graph/operator.rs:23` |
| sym-cbd818fcc6341db1dea3 | `as_str` | function | Returns the stable string representation of `SignalId`. | `src/graph/signal/spec.rs:29` |
| sym-c19bc7d1b15405d7ef23 | `as_str` | function | Returns the stable string representation of `SemanticRole`. | `src/graph/signal/spec.rs:64` |
| sym-7089f618e899e9dab49b | `as_str` | function | Returns the stable string representation of `SchemaRef`. | `src/graph/signal/spec.rs:94` |
| sym-53240e78e7e5e31d64d1 | `async_factory` | function | Returns the async factory associated with `NodeRegistry`. | `src/graph/registry.rs:144` |
| sym-4aa59105bd7b79996be5 | `async_factory_by_operator` | function | Returns the async factory by operator associated with `NodeRegistry`. | `src/graph/registry.rs:151` |
| sym-f10e1d3bd85962bda1c4 | `async_node_type_id` | function | Returns the async node type identifier held by `NodeRegistry`. | `src/graph/registry.rs:160` |
| sym-b8325e913de8e0179332 | `audio` | function | Convenience constructor for PCM audio ports. | `src/graph/signal/spec.rs:269` |
| sym-63f1aac4dd3f9eaa49fd | `backpressure` | function | Returns the backpressure associated with `EdgeContract`. | `src/graph/ports.rs:341` |
| sym-5cf5030b216c16fe810e | `binary` | function | Convenience constructor for opaque or schema-backed binary ports. | `src/graph/signal/spec.rs:299` |
| sym-0d4a11b9d5ef2e26e62f | `bounded_async` | function | Generic bounded asynchronous edge. Connected ports supply the payload representation and the envelope preserves its producer clock. | `src/graph/ports.rs:413` |
| sym-16bd4425261fffd04ba8 | `branch_copy_pool_bytes` | function | Returns the branch copy pool bytes held by `EdgeBufferPlan`. | `src/graph/plan.rs:57` |
| sym-9d712fad464358ad5323 | `branch_copy_pool_capacity_frames` | function | Returns the branch copy pool capacity frames held by `EdgeBufferPlan`. | `src/graph/plan.rs:48` |
| sym-60ff45fe284ba7b6ebb7 | `cancellation` | function | Returns the cancellation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:212` |
| sym-e1727a00565651dc8f64 | `capacity_signals` | function | Returns the capacity signals associated with `PortPrepareContext`. | `src/graph/node.rs:361` |
| sym-70771741ca5628c66f0f | `channel_count` | function | Returns the channel count held by `ChannelLayout`. | `src/graph/ports.rs:34` |
| sym-c7f680c12d1b3a72ec22 | `class` | function | Returns the class associated with `SignalSpec`. | `src/graph/signal/spec.rs:215` |
| sym-da095c09dd3f9a6c970b | `clock` | function | Returns the clock associated with `EdgeContract`. | `src/graph/ports.rs:329` |
| sym-dc5fff3ffd689363e387 | `clock_id` | function | Returns the clock identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:68` |
| sym-910af977649e51eb0d87 | `compile` | function | Compiles its owned operation for `Compiler`. | `src/graph/compile/resolve.rs:464` |
| sym-255c1250509b1e464cbc | `connect` | function | Connects the requested ports through `Pipeline`. | `src/graph/dsl.rs:55` |
| sym-bbad897aeb7a5e08149e | `connect_with` | function | Connects pipeline ports using the supplied edge contract on `Pipeline`. | `src/graph/dsl.rs:59` |
| sym-e1d54b23cb99c6c6b7a7 | `connector_id` | function | Returns the connector identifier held by `SignalDerivation`. | `src/graph/signal/lineage.rs:153` |
| sym-28f290acaf29fc919a23 | `contains` | function | Returns whether contains is true for `NodeRegistry`. | `src/graph/registry.rs:164` |
| sym-616a7202b6ff0092c073 | `control` | function | Convenience constructor for control ports. | `src/graph/signal/spec.rs:294` |
| sym-be7fa54e9ea9043544f7 | `copy_policy` | function | Returns the copy policy held by `EdgeContract`. | `src/graph/ports.rs:353` |
| sym-9368d177ae3e8cbdd0aa | `custom` | function | Convenience constructor for custom / vendor extension ports. | `src/graph/signal/spec.rs:304` |
| sym-7f3c3aecd675c182eddd | `deadline` | function | Returns the deadline associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:208` |
| sym-81cb0c79d1667584fca9 | `default` | function | Returns the default `RuntimePlanner` value. | `src/graph/compile/plan.rs:349` |
| sym-ae5d332bf994af7d868e | `default` | function | Returns the default `Compiler` value. | `src/graph/compile/resolve.rs:513` |
| sym-73080b2b6d7efead95e0 | `definition` | function | Returns the definition associated with `NodeRegistry`. | `src/graph/registry.rs:134` |
| sym-471995bd2d8dd6e45a0a | `delivery` | function | Returns the delivery associated with `EdgeContract`. | `src/graph/ports.rs:345` |
| sym-3e9506472d6a607ff82f | `derivation` | function | Returns the derivation associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:82` |
| sym-8277e57458dee8c11f29 | `descriptor` | function | Returns the descriptor associated with `PassthroughFactory`. | `src/graph/builtins.rs:70` |
| sym-476fbda9bf1e13a7449f | `descriptor` | function | Returns the descriptor associated with `GainFactory`. | `src/graph/builtins.rs:110` |
| sym-cd9a178a1ded665bb67e | `descriptor` | function | Returns the descriptor associated with `MonoMixFactory`. | `src/graph/builtins.rs:169` |
| sym-165eb2ce7542087de402 | `descriptor` | function | Returns the descriptor associated with `NodeDefinitionRef`. | `src/graph/registry.rs:39` |
| sym-ddbb7babdefa2c49ba99 | `direction` | function | Returns the direction associated with `PortPrepareContext`. | `src/graph/node.rs:345` |
| sym-93a75b6d781d2c33cac2 | `direction` | function | Returns the direction associated with `PortSpec`. | `src/graph/ports.rs:217` |
| sym-f8b730c83e4adefb9406 | `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:77` |
| sym-7737822215bedc45c9e3 | `display_name` | function | Returns the display name held by `NodeDescriptor`. | `src/graph/node.rs:226` |
| sym-cf1f8ff43a349e09aa72 | `duration_ns` | function | Returns the duration nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:83` |
| sym-60d8cf973db65857a614 | `edge_buffer` | function | Returns the edge buffer associated with `MemoryPlan`. | `src/graph/plan.rs:71` |
| sym-205f20aeeb83d05bf2fb | `edge_contract` | function | Returns the edge contract held by `PortPrepareContext`. | `src/graph/node.rs:357` |
| sym-d19f73749fda9ced227b | `edge_count` | function | Returns the edge count held by `GraphIr`. | `src/graph/ir.rs:43` |
| sym-609b3b1ad4daabfeee08 | `edge_count` | function | Returns the edge count held by `GraphSpec`. | `src/graph/spec.rs:69` |
| sym-72fa5deb567e0fd70a5e | `edge_id` | function | Returns the edge identifier held by `PortPrepareContext`. | `src/graph/node.rs:337` |
| sym-e5bb12d7a197ff37dba3 | `encoded_audio` | function | Convenience constructor for encoded audio ports. | `src/graph/signal/spec.rs:274` |
| sym-ed7b6e0d91dc8a84ae4d | `event` | function | Convenience constructor for event ports. | `src/graph/signal/spec.rs:284` |
| sym-7205f6787d2ee1b7b007 | `execution` | function | Returns the execution held by `NodeDescriptor`. | `src/graph/node.rs:238` |
| sym-dcc0672d33b86f0c40dd | `execution_partition` | function | Returns the execution partition associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:58` |
| sym-1a0f975ff6202dab6437 | `failure` | function | Returns the failure held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:216` |
| sym-52a6fbc698f955b2cd96 | `fmt` | function | Formats `NodeTypeId` with the requested formatter. | `src/graph/node.rs:31` |
| sym-3312c0e9fada600ccfb1 | `fmt` | function | Formats `NodeConfig` with the requested formatter. | `src/graph/node.rs:116` |
| sym-712dd0ed5d758d0259fc | `from` | function | Converts the supplied value into `NodeTypeId`. | `src/graph/node.rs:37` |
| sym-0260a0359d6a61e4abd7 | `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:41` |
| sym-6a1a06a410e59afb704f | `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:47` |
| sym-57adf6fa9c960a194659 | `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:70` |
| sym-d21313b85a5a4f9b112b | `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:76` |
| sym-63c44e108dc20375eb36 | `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:100` |
| sym-01078514b0053e0d48e4 | `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:106` |
| sym-3b0b482d47e67dd82b8d | `from_audio` | function | Creates `SignalEnvelope` from audio. | `src/graph/signal/envelope.rs:27` |
| sym-c8e8aac676d5eebd09b3 | `from_frame` | function | Creates `SignalLineage` from frame. | `src/graph/signal/lineage.rs:46` |
| sym-23617123d6583636fd64 | `from_frame` | function | Creates `SignalTiming` from frame. | `src/graph/signal/timing.rs:56` |
| sym-9ff05aa034f6fd7eb466 | `from_index` | function | Creates a stable runtime node identifier for externally assembled plans. | `src/graph/spec.rs:12` |
| sym-abb31d14a22ef594f1e0 | `generation` | function | Returns the generation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:184` |
| sym-6cca7b42b19a7963f00d | `get` | function | Returns the value held by `NodeConfig`. | `src/graph/node.rs:92` |
| sym-4ae7e2e7746961ddcb78 | `get` | function | Returns the value held by `NodeRegistry`. | `src/graph/registry.rs:127` |
| sym-13fa24385bb0481aa0e4 | `get_f32` | function | Returns the get f32 associated with `NodeConfig`. | `src/graph/node.rs:100` |
| sym-ffb635f4663ee56ddfa8 | `get_u32` | function | Returns the get u32 associated with `NodeConfig`. | `src/graph/node.rs:104` |
| sym-45d23c64ab95d9d99186 | `id` | function | Returns the id held by `NodeHandle`. | `src/graph/dsl.rs:15` |
| sym-46e559b1c0bd1721a52c | `id` | function | Returns the id held by `ResolvedNode`. | `src/graph/ir.rs:16` |
| sym-8e17aa73649cd14f1cc5 | `in_` | function | Selects a named input port from `NodeHandle`. | `src/graph/dsl.rs:24` |
| sym-040f459afdad39ad3886 | `index` | function | Returns the index held by `NodeId`. | `src/graph/spec.rs:16` |
| sym-a8af8d2eab0172a0d7a6 | `index` | function | Returns the index held by `EdgeId`. | `src/graph/spec.rs:25` |
| sym-c97cfd4baf922e230a94 | `input_edge` | function | Returns the input edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:192` |
| sym-2498228c5bbd13648656 | `input_ports` | function | Returns the input ports held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:224` |
| sym-60352d699b6436aa0952 | `inputs` | function | Returns the inputs associated with `NodeDescriptor`. | `src/graph/node.rs:230` |
| sym-de549fe8cfa5d7f7b830 | `inputs` | function | Returns the inputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:62` |
| sym-571a00a0e0dad50368ff | `instantiate` | function | Instantiates the runtime node described by `PassthroughFactory`. | `src/graph/builtins.rs:86` |
| sym-9fdce2b6e76eecf4e15f | `instantiate` | function | Instantiates the runtime node described by `GainFactory`. | `src/graph/builtins.rs:135` |
| sym-df4c461fa6eaa9f738b2 | `instantiate` | function | Instantiates the runtime node described by `MonoMixFactory`. | `src/graph/builtins.rs:185` |
| sym-6d6b21a9f385a15c2b4d | `into_payload` | function | Converts `SignalEnvelope` into payload. | `src/graph/signal/envelope.rs:86` |
| sym-8423f122d3f100838ddb | `into_spec` | function | Converts `Pipeline` into spec. | `src/graph/dsl.rs:90` |
| sym-fd7c2353ef0ba142e77a | `is_audio` | function | Returns `true` for classes that carry real-time audio on the hot path. | `src/graph/signal/spec.rs:180` |
| sym-6957b53fa20c046cd164 | `is_compatible_with` | function | Returns whether compatible with applies to `ChannelLayout`. | `src/graph/ports.rs:42` |
| sym-4a48d42a2bb469618c1a | `is_compatible_with` | function | Returns whether compatible with applies to `AudioCaps`. | `src/graph/ports.rs:56` |
| sym-b66c66799f3942868330 | `is_compatible_with` | function | Returns whether compatible with applies to `MediaCaps`. | `src/graph/ports.rs:110` |
| sym-60d37dec7979b459d3fd | `is_compatible_with` | function | Returns `true` if two signal classes are compatible for edge wiring. | `src/graph/signal/spec.rs:188` |
| sym-6d2f696c2f0bcf134b2a | `is_compatible_with` | function | Returns `true` if this spec can connect to `other` on an edge. | `src/graph/signal/spec.rs:324` |
| sym-ced2c36e825774462916 | `is_empty` | function | Returns whether `NodeRegistry` contains no values. | `src/graph/registry.rs:174` |
| sym-c4242678455f12a2a79b | `is_portable` | function | Reports whether this value is a portable implementation contract ID. | `src/graph/operator.rs:31` |
| sym-53f9b768dede3d88ded9 | `is_portable` | function | Reports whether this custom signal ID is portable across packages, languages, and processes. | `src/graph/signal/spec.rs:35` |
| sym-3067364c756e1dbf3c95 | `is_realtime` | function | Returns whether realtime applies to `ClockDomain`. | `src/graph/ports.rs:259` |
| sym-25c1de9f07a865e0fe97 | `is_sensitive` | function | Returns whether sensitive applies to `NodeConfig`. | `src/graph/node.rs:96` |
| sym-0f4929d08758870a7625 | `is_stateful` | function | Returns whether stateful applies to `NodeDescriptor`. | `src/graph/node.rs:246` |
| sym-62cadbfa63ba01f06a9c | `is_terminal` | function | Returns whether terminal applies to `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:86` |
| sym-ba34c52c37d7de4461cc | `is_valid_for` | function | Returns `true` if this contract is compatible with the given partition. | `src/graph/partition.rs:107` |
| sym-95cdf3ba054aa12d8cdd | `is_well_formed` | function | Reports whether this value follows the portable node-type syntax. | `src/graph/node.rs:25` |
| sym-1b878040951ec109e51e | `iter` | function | Iterates over the values held by `NodeConfig`. | `src/graph/node.rs:108` |
| sym-43b9a3ee7a683e657bf1 | `jitter_budget_ms` | function | Returns the jitter budget milliseconds held by `EdgeContract`. | `src/graph/ports.rs:337` |
| sym-6b625be43fd3c89865e4 | `kind` | function | Returns the kind represented by `MediaCaps`. | `src/graph/ports.rs:97` |
| sym-38f0dccf4d9126f4c381 | `latency_budget_ms` | function | Returns the latency budget milliseconds held by `EdgeContract`. | `src/graph/ports.rs:333` |
| sym-774089c61834270d3508 | `len` | function | Returns the number of values held by `NodeRegistry`. | `src/graph/registry.rs:169` |
| sym-1dd2284550c9c2085802 | `lineage` | function | Returns the frame lineage carried by `SignalEnvelope`. | `src/graph/signal/envelope.rs:78` |
| sym-f7144091d8bfca40c650 | `loss` | function | Returns the loss associated with `EdgeContract`. | `src/graph/ports.rs:349` |
| sym-e68e640f091d011d8061 | `map_payload` | function | Transforms the payload held by `SignalEnvelope` while preserving envelope metadata. | `src/graph/signal/envelope.rs:45` |
| sym-c6fa198f9907fdec7957 | `max_payload_bytes` | function | Returns the max payload bytes held by `EdgeContract`. | `src/graph/ports.rs:361` |
| sym-59c84b5114d9ccab7207 | `media` | function | Returns the media held by `PortPrepareContext`. | `src/graph/node.rs:353` |
| sym-4b2290f3212fc6e167e0 | `media` | function | Returns the media held by `PortSpec`. | `src/graph/ports.rs:225` |
| sym-5ed77a87b9fecdd34b49 | `media` | function | Returns the media held by `EdgeContract`. | `src/graph/ports.rs:325` |
| sym-35144345756d03ecc4c7 | `metric_id` | function | Returns the metric identifier held by `RuntimePlan`. | `src/graph/plan.rs:145` |
| sym-58e27492a8bef2ad0845 | `metrics` | function | Convenience constructor for metrics ports. | `src/graph/signal/spec.rs:289` |
| sym-141e9c5891dcd0c4ff69 | `multiplicity` | function | Returns the multiplicity associated with `PortSpec`. | `src/graph/ports.rs:229` |
| sym-6bbf728d87e74976df2a | `name` | function | Returns the name associated with `PortSpec`. | `src/graph/ports.rs:213` |
| sym-6e55e01faf43e8a23cea | `needs_bridge_to` | function | Returns `true` if crossing from `self` to `other` requires a compiler-inserted Bridge. | `src/graph/partition.rs:71` |
| sym-336637cc5b0aa7486ec8 | `negotiate` | function | Negotiates the compatible media capabilities shared by `MediaCaps` and its peer. | `src/graph/ports.rs:124` |
| sym-4800fa93f446b821656f | `new` | function | Creates a new `RuntimePlanner`. | `src/graph/compile/plan.rs:14` |
| sym-8e635350ed4e8cc94836 | `new` | function | Creates a new `Compiler`. | `src/graph/compile/resolve.rs:449` |
| sym-501b7f4f5836ab935975 | `new` | function | Creates a new `Pipeline`. | `src/graph/dsl.rs:40` |
| sym-632f4284e84cd93b03eb | `new` | function | Creates a new `NodeConfig`. | `src/graph/node.rs:62` |
| sym-baccec434c0630b5b128 | `new` | function | Creates a new `NodeDescriptor`. | `src/graph/node.rs:176` |
| sym-cb61f3c847b318c0fb5a | `new` | function | Creates a new `PrepareContext`. | `src/graph/node.rs:271` |
| sym-84744a4dec72dad90121 | `new` | function | Creates a new `PortPrepareContext`. | `src/graph/node.rs:293` |
| sym-d89cdb51ac7756fa856d | `new` | function | Creates a new `OperatorId`. | `src/graph/operator.rs:19` |
| sym-a20e080faf9223fb614b | `new` | function | Creates a new `PortSpec`. | `src/graph/ports.rs:185` |
| sym-1c6dd1dfe38f33c22990 | `new` | function | Creates a new `NodeRegistry`. | `src/graph/registry.rs:73` |
| sym-836ce3931737b05f2d33 | `new` | function | Creates a new `SignalDerivation`. | `src/graph/signal/lineage.rs:107` |
| sym-7193b901d461c0e9ca43 | `new` | function | Creates a new `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:144` |
| sym-48818d161810b5399bc4 | `new` | function | Creates a new `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:29` |
| sym-ccffe09a19245796e996 | `new` | function | Creates a new `SignalId`. | `src/graph/signal/spec.rs:25` |
| sym-bac810f703773ab52c40 | `new` | function | Creates a new `SemanticRole`. | `src/graph/signal/spec.rs:60` |
| sym-a80eb1b0c92a1368eb47 | `new` | function | Creates a new `SchemaRef`. | `src/graph/signal/spec.rs:90` |
| sym-08fa3bcb2f21834f932b | `new` | function | Creates a new `SignalSpec`. | `src/graph/signal/spec.rs:226` |
| sym-75ff7c3f6b094d8e0794 | `node` | function | Returns the node held by `GraphIr`. | `src/graph/ir.rs:47` |
| sym-b5113f12ad418b171a05 | `node` | function | Returns the node held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:188` |
| sym-d21d97866bc7d452a58d | `node` | function | Returns the node held by `GraphSpec`. | `src/graph/spec.rs:73` |
| sym-8da2711f4954700b3c0d | `node_count` | function | Returns the node count held by `GraphIr`. | `src/graph/ir.rs:40` |
| sym-9b737f5f193aeecd746b | `node_count` | function | Returns the node count held by `RuntimePlan`. | `src/graph/plan.rs:135` |
| sym-c3af1cdaa9a78f5da605 | `node_count` | function | Returns the node count held by `GraphSpec`. | `src/graph/spec.rs:65` |
| sym-4b5b8d12d566894093ee | `observability` | function | Returns the observability associated with `EdgeContract`. | `src/graph/ports.rs:357` |
| sym-f72be4adaaf6e63df4cb | `observe` | function | Returns the current observation exposed by `SignalContinuityTracker`. | `src/graph/signal/continuity.rs:18` |
| sym-d0eaceec9c05753154b5 | `observed` | function | Creates observed signal timing for `SignalTiming`. | `src/graph/signal/timing.rs:38` |
| sym-0d01bd0ea3304e47aa21 | `observed_timestamp_ns` | function | Returns the observed timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:75` |
| sym-a1a03603e13732dc10c5 | `operator_generation` | function | Returns the operator generation associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:150` |
| sym-0d93214a11f553678570 | `operator_id` | function | Returns the operator identifier held by `SignalDerivation`. | `src/graph/signal/lineage.rs:144` |
| sym-6de09da4d4990dde0757 | `operator_id` | function | Returns the operator identifier held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:176` |
| sym-dad9510c0489ee5ea571 | `operator_revision` | function | Returns the operator revision held by `SignalDerivation`. | `src/graph/signal/lineage.rs:147` |
| sym-99b366ee077deb6f8637 | `out` | function | Selects a named output port from `NodeHandle`. | `src/graph/dsl.rs:18` |
| sym-165350c7eb4d01df0d2a | `output_edge` | function | Returns the output edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:196` |
| sym-a4a36c332782d868d32e | `output_ports` | function | Returns the output ports held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:231` |
| sym-3ff74d3a91ed55ee819b | `output_roles` | function | Returns the output roles associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:220` |
| sym-99af61c6fad5797e92de | `outputs` | function | Returns the outputs associated with `NodeDescriptor`. | `src/graph/node.rs:234` |
| sym-582f4c0d4006e5ae3813 | `outputs` | function | Returns the outputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:66` |
| sym-0c160b19fd4474a1371d | `partition` | function | Returns the partition associated with `RuntimePlan`. | `src/graph/plan.rs:140` |
| sym-f090e15638035881e524 | `payload` | function | Returns the payload associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:62` |
| sym-89321d5d3ca7f9614d2d | `payload_size_bytes` | function | Returns the payload size bytes held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:66` |
| sym-4e58cb471f6015a6e16c | `permission` | function | Returns the permission associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:204` |
| sym-9812c802447ad91a03e1 | `plan` | function | Lower a verified IR into an execution-ready plan. | `src/graph/compile/plan.rs:24` |
| sym-edd92efe7a54ca69ced8 | `pocketstation::graph::builtins::register_builtins` | function | Registers builtins for `builtins`. | `src/graph/builtins.rs:220` |
| sym-b4724928e196c76bee2b | `policy_epoch` | function | Returns the policy epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:80` |
| sym-d34d02730f4091d78665 | `port_name` | function | Returns the port name held by `PortPrepareContext`. | `src/graph/node.rs:341` |
| sym-0403aa53b47c8ce6a126 | `prepare` | function | Prepares resources required by `PassthroughNode`. | `src/graph/builtins.rs:98` |
| sym-1555ba585793ec03dee4 | `prepare` | function | Prepares resources required by `GainNode`. | `src/graph/builtins.rs:154` |
| sym-8a35a9fe22759b2b9c89 | `prepare` | function | Prepares resources required by `MonoMixNode`. | `src/graph/builtins.rs:197` |
| sym-258a8ce93055beafae2a | `process` | function | Processes an input value through `PassthroughNode`. | `src/graph/builtins.rs:102` |
| sym-e0e41303eb7700e25d8b | `process` | function | Processes an input value through `GainNode`. | `src/graph/builtins.rs:158` |
| sym-959aabff687a70d5aaca | `process` | function | Processes an input value through `MonoMixNode`. | `src/graph/builtins.rs:201` |
| sym-996cf2843f20c56c51e1 | `queue_capacity_frames` | function | Returns the queue capacity frames held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:200` |
| sym-e0d3138e8702f4735a5a | `rank` | function | Priority rank for scheduling: lower = higher priority. | `src/graph/partition.rs:60` |
| sym-23421c43505fe755b0c7 | `rank` | function | Returns the rank associated with `EdgeObservabilityLevel`. | `src/graph/ports.rs:301` |
| sym-91cd07cd903200167e85 | `realtime_audio` | function | Generic realtime PCM edge. Concrete sample rate, frame size, and channel layout are negotiated from connected ports. | `src/graph/ports.rs:391` |
| sym-919d7c5825d7c899beec | `register` | function | Registers a node definition with `NodeRegistry` while preserving unique identities. | `src/graph/registry.rs:77` |
| sym-faa2afdc3209724248db | `register_async` | function | Registers async for `NodeRegistry`. | `src/graph/registry.rs:89` |
| sym-4d44ee21069d7426704e | `register_definition` | function | Registers definition for `NodeRegistry`. | `src/graph/registry.rs:112` |
| sym-03479efd2b40d0c96fec | `required` | function | Returns the required held by `PortSpec`. | `src/graph/ports.rs:233` |
| sym-62c608a4c5be55750a40 | `requires_realtime_safety` | function | Returns `true` if the partition requires strict real-time safety. | `src/graph/partition.rs:55` |
| sym-f26a138e6883327137e1 | `revision` | function | Returns the revision held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:180` |
| sym-b36ef5f001b9cab9bfd2 | `role` | function | Returns the role associated with `SignalSpec`. | `src/graph/signal/spec.rs:219` |
| sym-776bcc49344645af6806 | `safety` | function | Returns the safety held by `NodeDescriptor`. | `src/graph/node.rs:242` |
| sym-c1f16585dc839e939307 | `schema` | function | Returns the schema held by `SignalSpec`. | `src/graph/signal/spec.rs:223` |
| sym-47790cb72d92e2c9a65d | `sequence_number` | function | Returns the sequence number held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:90` |
| sym-f61d79d29127e976d995 | `sequence_number` | function | Returns the sequence number held by `SignalLineage`. | `src/graph/signal/lineage.rs:71` |
| sym-620cbbbe1801a55c105f | `session_id` | function | Returns the session identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:59` |
| sym-e60011769c414b552d0d | `session_timestamp_ns` | function | Returns the session timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:79` |
| sym-eaec6bfdad1634d716cf | `signal` | function | Returns the signal held by `PortPrepareContext`. | `src/graph/node.rs:349` |
| sym-90d7d530db87c660af44 | `signal` | function | Returns the signal held by `PortSpec`. | `src/graph/ports.rs:221` |
| sym-4e7e8a1fa845699a0f26 | `signal_spec` | function | Returns the signal spec held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:70` |
| sym-3e13e9e872b74e22420a | `size_bytes` | function | Owned media bytes represented by this payload. Envelope metadata and queue slot storage are fixed-size and accounted separately by the edge. | `src/graph/signal/payload.rs:37` |
| sym-14f98766e0b55e2653b5 | `source_generation` | function | Returns the source generation associated with `SignalLineage`. | `src/graph/signal/lineage.rs:74` |
| sym-710524a75f88e53a3c41 | `source_id` | function | Returns the source identifier held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:100` |
| sym-43488b47bd041888b96a | `source_id` | function | Returns the source identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:65` |
| sym-e84693afeae9bdc19d62 | `source_timestamp_ns` | function | Returns the source timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:71` |
| sym-7a45cca7a39083387a5d | `spec` | function | Returns the spec associated with `Pipeline`. | `src/graph/dsl.rs:86` |
| sym-4b63e7102cfdd7d4aa31 | `stream_id` | function | Returns the stream identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:62` |
| sym-99ae82bf9dcffcfb428d | `supports` | function | Returns whether supports is true for `SignalPayload`. | `src/graph/signal/payload.rs:17` |
| sym-791a8c729906d51aff5b | `supports_signal` | function | Returns whether supports signal applies to `MediaCaps`. | `src/graph/ports.rs:142` |
| sym-9d40cdc6e238ae01433f | `syntax_version` | function | Returns the syntax version held by `OperatorId`. | `src/graph/operator.rs:35` |
| sym-b413552451c02630cb6b | `text` | function | Convenience constructor for text ports. | `src/graph/signal/spec.rs:279` |
| sym-af2b07027a70cb678472 | `timestamp_end_ns` | function | Returns the timestamp end nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:65` |
| sym-c119046c1ba5cd1f6ab7 | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:110` |
| sym-374b52ec82d70bd29706 | `timing` | function | Returns the timing associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:74` |
| sym-c3b32da3a661cb4eee80 | `topo_order` | function | Returns the topo order associated with `GraphIr`. | `src/graph/ir.rs:51` |
| sym-0dcd8f08977269d04741 | `total_bytes` | function | Returns the total bytes held by `EdgeBufferPlan`. | `src/graph/plan.rs:44` |
| sym-6c4621ccecc3cd2bf2fa | `try_new` | function | Creates a new `SignalLineage` after validating its inputs. | `src/graph/signal/lineage.rs:21` |
| sym-f63ba85284550d4f57f1 | `try_new` | function | Creates a new `SignalTiming` after validating its inputs. | `src/graph/signal/timing.rs:14` |
| sym-76b01e53be77e448ec97 | `type_id` | function | Returns the type identifier held by `NodeDescriptor`. | `src/graph/node.rs:222` |
| sym-01f8a81e39a7b2ee2fdb | `type_ids` | function | Returns the type identifiers held by `NodeRegistry`. | `src/graph/registry.rs:179` |
| sym-85ee7c5f479a400c6639 | `type_str` | function | Returns the type str associated with `ResolvedNode`. | `src/graph/ir.rs:19` |
| sym-2a8626d1442fe79ec687 | `typed_edge` | function | Returns the typed edge associated with `RuntimePlan`. | `src/graph/plan.rs:152` |
| sym-339c8509b7a1e07e55ad | `untracked` | function | Creates an envelope for data that has not yet entered a source-aware Session. Session sources must attach lineage before routing it. | `src/graph/signal/envelope.rs:17` |
| sym-9ed9b9e32359e86016cf | `upstream_lineage` | function | Returns the upstream lineage held by `SignalDerivation`. | `src/graph/signal/lineage.rs:138` |
| sym-66b72d654eedc1e47b15 | `upstream_timing` | function | Returns the upstream timing associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:141` |
| sym-fa0b35e1926f0614794b | `validate` | function | Validates `SignalEnvelope` against its declared contract. | `src/graph/signal/envelope.rs:117` |
| sym-64bc9b02618ad57280b7 | `validate` | function | Validates `AsyncOperatorManifest` against its declared contract. | `src/graph/signal/operator.rs:238` |
| sym-060ff447535f4d8819a5 | `validate` | function | Validates `SignalSpec` against its declared contract. | `src/graph/signal/spec.rs:328` |
| sym-715a37328717b030183b | `validate_config` | function | Validates config for `PassthroughFactory`. | `src/graph/builtins.rs:82` |
| sym-a03fc8ba878c311a2716 | `validate_config` | function | Validates config for `GainFactory`. | `src/graph/builtins.rs:122` |
| sym-317697f91f22bb20b85c | `validate_config` | function | Validates config for `MonoMixFactory`. | `src/graph/builtins.rs:181` |
| sym-f5bd3a902608d86dc6e7 | `validate_config` | function | Validates config for `NodeDefinitionRef`. | `src/graph/registry.rs:47` |
| sym-6ef496a1b40f7398314c | `wire_id` | function | Stable language-neutral identifier for the fundamental wire class. Semantic role and schema remain separate fields. | `src/graph/signal/spec.rs:236` |
| sym-8ae3534590506ed86fb5 | `with` | function | Returns `NodeConfig` with the supplied entry applied. | `src/graph/node.rs:66` |
| sym-627db520310059eca7ec | `with_backpressure` | function | Sets the backpressure on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:370` |
| sym-da6d4ffe77b7e932d2d7 | `with_copy_policy` | function | Sets the copy policy on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:375` |
| sym-687cece7a09ec0fb114d | `with_derivation` | function | Sets the derivation on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:57` |
| sym-26d27b862403373c9c03 | `with_duration_ns` | function | Sets the duration nanoseconds on `SignalTiming` and returns the updated value. | `src/graph/signal/timing.rs:47` |
| sym-323eff431ca9de287a74 | `with_jitter_budget_ms` | function | Sets the jitter budget milliseconds on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:380` |
| sym-70767f46b65e3b7ee5a7 | `with_lineage` | function | Sets the lineage on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:51` |
| sym-c943f133ee9a45f550df | `with_max_payload_bytes` | function | Sets the max payload bytes on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:385` |
| sym-66545fc4ae5804efe857 | `with_media` | function | Sets the media on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:365` |
| sym-6e2035240b0754c5d84d | `with_role` | function | Attach a semantic role annotation. | `src/graph/signal/spec.rs:309` |
| sym-d19dc48d1ebe26b02c2e | `with_schema` | function | Attach a schema reference. | `src/graph/signal/spec.rs:315` |
| sym-2dd0dc96d616dce7df18 | `with_sensitive` | function | Adds a setup-time value whose normal debug representation is redacted. | `src/graph/node.rs:81` |
| sym-2fc137327957b96be890 | `pocketstation::graph` | module | Stable signal, port, capability, partition, and extension contracts. | `src/graph/mod.rs:1` |
| sym-e9b4ed91b9dd9e9693d5 | `pocketstation::graph::builtins::GainFactory` | struct | Constructs gain implementations from validated declarations. | `src/graph/builtins.rs:107` |
| sym-9138021fd69def65c431 | `pocketstation::graph::builtins::GainNode` | struct | Executes the graph-node behavior defined for gain. | `src/graph/builtins.rs:149` |
| sym-abf1724908e924cd339e | `pocketstation::graph::builtins::MonoMixFactory` | struct | Constructs mono mix implementations from validated declarations. | `src/graph/builtins.rs:166` |

## Interpretation

The **Graph and route contracts** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

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

The claims on **Graph and route contracts** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/mod.rs:1-67` (`DIRECT`)

For **Graph and route contracts**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
