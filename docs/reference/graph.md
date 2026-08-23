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
| sym-7e71261c61f78cefd417 | `pocketstation::graph::operator::OPERATOR_ID_SYNTAX_VERSION` | constant | Version of the serialized operator-identifier syntax. | `src/graph/operator.rs:6` |
| sym-3cc63fdcaf2e3841e729 | `pocketstation::graph::plan::EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES` | constant | A sequential edge receiver may retain the frame it just popped while it processes that frame. Copy-pool sizing must cover that owned frame in addition to every frame that can still be queued. | `src/graph/plan.rs:16` |
| sym-a7b4280132651539cc89 | `pocketstation::graph::plan::EDGE_RING_CAPACITY_FRAMES` | constant | Defines the public edge ring capacity frames value. | `src/graph/plan.rs:12` |
| sym-202840de95069aac7815 | `pocketstation::graph::plan::FRAME_BYTES_MONO_48K` | constant | Defines the public frame bytes mono 48 k value. | `src/graph/plan.rs:11` |
| sym-eec261911eeb3838d12f | `pocketstation::graph::plan::MAX_EDGE_RING_CAPACITY_FRAMES` | constant | Sets the maximum supported edge ring capacity frames. | `src/graph/plan.rs:17` |
| sym-6104a324170be362b4e2 | `pocketstation::graph::ports::MAX_ASYNC_PAYLOAD_BYTES` | constant | Sets the maximum supported async payload bytes. | `src/graph/ports.rs:13` |
| sym-ef308cb6434678c35687 | `pocketstation::graph::compile::resolve::CompileError` | enum | Classifies failures reported as compile error. | `src/graph/compile/resolve.rs:26` |
| sym-f23fbf485a1abdca3ff2 | `pocketstation::graph::node::ConfigError` | enum | Classifies failures reported as config error. | `src/graph/node.rs:141` |
| sym-63eacff5c07a2f53dc8d | `pocketstation::graph::node::NodeDescriptorError` | enum | Classifies failures reported as node descriptor error. | `src/graph/node.rs:252` |
| sym-8839c0168f2c08f2c049 | `pocketstation::graph::node::NodeError` | enum | Classifies failures reported as node error. | `src/graph/node.rs:149` |
| sym-62ce3a27efa7fea45df0 | `pocketstation::graph::partition::ExecutionPartition` | enum | WHERE an operator runs. | `src/graph/partition.rs:18` |
| sym-142ebf4cccf857875822 | `pocketstation::graph::partition::SafetyContract` | enum | WHAT an operator guarantees about its runtime behaviour. | `src/graph/partition.rs:82` |
| sym-04dd5abde97700d6bea8 | `pocketstation::graph::plan::PlanError` | enum | Classifies failures reported as plan error. | `src/graph/plan.rs:21` |
| sym-bb29d2e9bc5ef5790a5b | `pocketstation::graph::ports::BackpressurePolicy` | enum | Selects the backpressure policy used by PocketStation. | `src/graph/ports.rs:265` |
| sym-8c71eb7bf77b533a3950 | `pocketstation::graph::ports::ChannelLayout` | enum | Enumerates the supported channel layout cases. | `src/graph/ports.rs:27` |
| sym-c994bd3c6236b085ea39 | `pocketstation::graph::ports::ClockDomain` | enum | Enumerates the supported clock domain cases. | `src/graph/ports.rs:249` |
| sym-b708b27c76144c20fa7b | `pocketstation::graph::ports::CopyPolicy` | enum | Selects the copy policy used by PocketStation. | `src/graph/ports.rs:280` |
| sym-780123bab167f93fc9d2 | `pocketstation::graph::ports::DeliverySemantics` | enum | Selects the delivery semantics used by PocketStation. | `src/graph/ports.rs:273` |
| sym-7cb8e2639c16f6f287d7 | `pocketstation::graph::ports::EdgeObservabilityLevel` | enum | Selects the edge observability level used by PocketStation. | `src/graph/ports.rs:294` |
| sym-0940509ee3453cf34ddf | `pocketstation::graph::ports::LossPolicy` | enum | Selects the loss policy used by PocketStation. | `src/graph/ports.rs:287` |
| sym-822c3c845e91e2ffb25a | `pocketstation::graph::ports::MediaCaps` | enum | Enumerates the supported media caps cases. | `src/graph/ports.rs:85` |
| sym-74305f5acfae6a732e2b | `pocketstation::graph::ports::MediaKind` | enum | Selects the media kind used by PocketStation. | `src/graph/ports.rs:16` |
| sym-1b42b2cf2aa6f3bb4d1a | `pocketstation::graph::ports::Multiplicity` | enum | Enumerates the supported multiplicity cases. | `src/graph/ports.rs:169` |
| sym-c3e0d763e39512ba85d5 | `pocketstation::graph::ports::PortDirection` | enum | Selects the port direction used by PocketStation. | `src/graph/ports.rs:163` |
| sym-b9f6cf6d84646f2f54cd | `pocketstation::graph::ports::PortSpecError` | enum | Classifies failures reported as port spec error. | `src/graph/ports.rs:239` |
| sym-9f1d225abf797e856b20 | `pocketstation::graph::registry::NodeDefinitionRef` | enum | Enumerates the supported node definition ref cases. | `src/graph/registry.rs:32` |
| sym-08a016336466041e5717 | `pocketstation::graph::registry::NodeRegistrationError` | enum | Classifies failures reported as node registration error. | `src/graph/registry.rs:57` |
| sym-578043ee573a0f9b99e6 | `pocketstation::graph::signal::continuity::SignalContinuityError` | enum | Classifies failures reported as signal continuity error. | `src/graph/signal/continuity.rs:89` |
| sym-a4afff76933f8071989c | `pocketstation::graph::signal::envelope::SignalEnvelopeError` | enum | Classifies failures reported as signal envelope error. | `src/graph/signal/envelope.rs:137` |
| sym-5b1af0913743b3dc15fe | `pocketstation::graph::signal::lineage::SignalDerivationError` | enum | Classifies failures reported as signal derivation error. | `src/graph/signal/lineage.rs:159` |
| sym-321d364165d7c21bacec | `pocketstation::graph::signal::lineage::SignalLineageError` | enum | Classifies failures reported as signal lineage error. | `src/graph/signal/lineage.rs:86` |
| sym-7c4acf8b5348e2b02362 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | enum | Classifies failures reported as async operator manifest error. | `src/graph/signal/operator.rs:321` |
| sym-de8d9bf729d99cd1b90c | `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | Selects the operator cancellation policy used by PocketStation. | `src/graph/signal/operator.rs:57` |
| sym-c2d8be1a7356019be8c9 | `pocketstation::graph::signal::operator::OperatorFailurePolicy` | enum | Selects the operator failure policy used by PocketStation. | `src/graph/signal/operator.rs:63` |
| sym-d1b44af5c9c63b1683e8 | `pocketstation::graph::signal::payload::SignalPayload` | enum | Enumerates the supported signal payload cases. | `src/graph/signal/payload.rs:10` |
| sym-5ee081d3cfce2f617af2 | `pocketstation::graph::signal::spec::BinaryFormat` | enum | Binary encoding hint for `SignalClass::Binary`. | `src/graph/signal/spec.rs:141` |
| sym-1b3317725d97bbefcd4d | `pocketstation::graph::signal::spec::Codec` | enum | Audio encoding format for `SignalClass::EncodedAudio`. | `src/graph/signal/spec.rs:113` |
| sym-1effef32701817ca9bef | `pocketstation::graph::signal::spec::EventFormat` | enum | Event structure hint for `SignalClass::Event`. | `src/graph/signal/spec.rs:132` |
| sym-24b831fc21a3c9d7c3d3 | `pocketstation::graph::signal::spec::SignalClass` | enum | The fundamental class of data flowing through a port. | `src/graph/signal/spec.rs:156` |
| sym-b386818365cee36fb88f | `pocketstation::graph::signal::spec::SignalSpecError` | enum | Classifies failures reported as signal spec error. | `src/graph/signal/spec.rs:351` |
| sym-30151b96c6d0821dc2db | `pocketstation::graph::signal::spec::TextFormat` | enum | Text encoding hint for `SignalClass::Text`. | `src/graph/signal/spec.rs:124` |
| sym-d6d356025cf7255edba3 | `pocketstation::graph::signal::timing::SignalTimingError` | enum | Classifies failures reported as signal timing error. | `src/graph/signal/timing.rs:89` |
| sym-9642176d7ce0fae2cfdb | `AsyncNode::cancel` | function | Requests cancellation of `AsyncNode`. | `src/graph/signal/operator.rs:36` |
| sym-5c9e1eb4c40189e4507c | `AsyncNode::close` | function | Closes `AsyncNode` to further work. | `src/graph/signal/operator.rs:40` |
| sym-d19b1106016b42c888a3 | `AsyncNode::flush` | function | Flushes pending output from `AsyncNode` at the end of a run. | `src/graph/signal/operator.rs:32` |
| sym-810a3c6d136ea253d9c1 | `AsyncNode::prepare` | function | Prepares resources required by `AsyncNode`. | `src/graph/signal/operator.rs:14` |
| sym-60579117d36ce3ae29e0 | `AsyncNode::process` | function | Processes an input value through `AsyncNode`. | `src/graph/signal/operator.rs:19` |
| sym-f7b2dd8f4291dd8d5924 | `AsyncNode::process_port` | function | Returns the process port held by `AsyncNode`. | `src/graph/signal/operator.rs:24` |
| sym-ab50a788828394fe1a21 | `AsyncOperatorFactory::create` | function | Creates the runtime implementation described by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:378` |
| sym-0a1384e3fd04b4d39049 | `AsyncOperatorFactory::manifest` | function | Returns the manifest held by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:369` |
| sym-807e19db4b5ec22d82c8 | `AsyncOperatorFactory::resolve_manifest` | function | Resolves manifest for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:371` |
| sym-058eb6025ed335fcad46 | `AsyncOperatorFactory::validate_config` | function | Validates config for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:370` |
| sym-95d186f05bc1b9a0e1fc | `NodeDefinition::descriptor` | function | Returns the descriptor associated with `NodeDefinition`. | `src/graph/registry.rs:22` |
| sym-756730521855d6410b74 | `NodeDefinition::validate_config` | function | Validates config for `NodeDefinition`. | `src/graph/registry.rs:23` |
| sym-5f3b142ed649e97a8a16 | `NodeFactory::descriptor` | function | Returns the descriptor associated with `NodeFactory`. | `src/graph/registry.rs:12` |
| sym-308f05bdc22e7d8614b8 | `NodeFactory::instantiate` | function | Instantiates the runtime node described by `NodeFactory`. | `src/graph/registry.rs:14` |
| sym-0c7ecbd3f919b7ef72eb | `NodeFactory::validate_config` | function | Validates config for `NodeFactory`. | `src/graph/registry.rs:13` |
| sym-910d45960a7a0e969a3b | `RuntimeNode::prepare` | function | Prepares resources required by `RuntimeNode`. | `src/graph/runtime_node.rs:8` |
| sym-f6799f84a742f4bbb94e | `RuntimeNode::process` | function | Processes an input value through `RuntimeNode`. | `src/graph/runtime_node.rs:9` |
| sym-1dc345935cb016e9ada7 | `accepts` | function | Returns whether accepts is true for `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:75` |
| sym-33c855e1e7e0d665cbfb | `add_node` | function | Adds node for `Pipeline`. | `src/graph/dsl.rs:44` |
| sym-2e9bb7ccca392ce72b7f | `any` | function | Convenience constructor for a deliberately open boundary port. | `src/graph/signal/spec.rs:264` |
| sym-c4d691ed75e3107f9e7d | `as_str` | function | Returns the stable string representation of `NodeTypeId`. | `src/graph/node.rs:16` |
| sym-c1514acd18e430e01b9c | `as_str` | function | Returns the stable string representation of `OperatorId`. | `src/graph/operator.rs:23` |
| sym-e7833d1e66813f75c4fa | `as_str` | function | Returns the stable string representation of `SignalId`. | `src/graph/signal/spec.rs:29` |
| sym-afb07b88012cf5d9f435 | `as_str` | function | Returns the stable string representation of `SemanticRole`. | `src/graph/signal/spec.rs:64` |
| sym-705795c35cd0de970c84 | `as_str` | function | Returns the stable string representation of `SchemaRef`. | `src/graph/signal/spec.rs:94` |
| sym-e18051e89c18feace5e6 | `async_factory` | function | Returns the async factory associated with `NodeRegistry`. | `src/graph/registry.rs:144` |
| sym-fe6e57f9a3d6642b2913 | `async_factory_by_operator` | function | Returns the async factory by operator associated with `NodeRegistry`. | `src/graph/registry.rs:151` |
| sym-575f25f9e00122ebe881 | `async_node_type_id` | function | Returns the async node type identifier held by `NodeRegistry`. | `src/graph/registry.rs:160` |
| sym-eda77a12b588bd3de5b1 | `audio` | function | Convenience constructor for PCM audio ports. | `src/graph/signal/spec.rs:269` |
| sym-556d991164d8862d47a0 | `backpressure` | function | Returns the backpressure associated with `EdgeContract`. | `src/graph/ports.rs:341` |
| sym-6451c64a209cd4c0b3e4 | `binary` | function | Convenience constructor for opaque or schema-backed binary ports. | `src/graph/signal/spec.rs:299` |
| sym-22e3046e14d9c5d31b22 | `bounded_async` | function | Generic bounded asynchronous edge. Connected ports supply the payload representation and the envelope preserves its producer clock. | `src/graph/ports.rs:413` |
| sym-f4f70bce71bd42b44339 | `branch_copy_pool_bytes` | function | Returns the branch copy pool bytes held by `EdgeBufferPlan`. | `src/graph/plan.rs:57` |
| sym-765df13202cdcd582c2a | `branch_copy_pool_capacity_frames` | function | Returns the branch copy pool capacity frames held by `EdgeBufferPlan`. | `src/graph/plan.rs:48` |
| sym-2cd53fcb9d9466bb699e | `cancellation` | function | Returns the cancellation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:212` |
| sym-6989e7d604357fb2446c | `capacity_signals` | function | Returns the capacity signals associated with `PortPrepareContext`. | `src/graph/node.rs:361` |
| sym-9e49b4a8ad90ee1c7d4e | `channel_count` | function | Returns the channel count held by `ChannelLayout`. | `src/graph/ports.rs:34` |
| sym-1a857f3aa63ddf26c1b2 | `class` | function | Returns the class associated with `SignalSpec`. | `src/graph/signal/spec.rs:215` |
| sym-053c78f7e0d2836eaed8 | `clock` | function | Returns the clock associated with `EdgeContract`. | `src/graph/ports.rs:329` |
| sym-44a7d16dea5bbbda4d6d | `clock_id` | function | Returns the clock identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:68` |
| sym-f772dacab759a8e9e66b | `compile` | function | Compiles its owned operation for `Compiler`. | `src/graph/compile/resolve.rs:464` |
| sym-ec1e34e47737b444a4ae | `connect` | function | Connects the requested ports through `Pipeline`. | `src/graph/dsl.rs:55` |
| sym-806b172733c210276185 | `connect_with` | function | Connects pipeline ports using the supplied edge contract on `Pipeline`. | `src/graph/dsl.rs:59` |
| sym-6929300e9be787b99234 | `connector_id` | function | Returns the connector identifier held by `SignalDerivation`. | `src/graph/signal/lineage.rs:153` |
| sym-d2ff6deafe0c2e7e26c8 | `contains` | function | Returns whether contains is true for `NodeRegistry`. | `src/graph/registry.rs:164` |
| sym-580a1dc419b01bf7c5af | `control` | function | Convenience constructor for control ports. | `src/graph/signal/spec.rs:294` |
| sym-494f99c0b5d652874f70 | `copy_policy` | function | Returns the copy policy held by `EdgeContract`. | `src/graph/ports.rs:353` |
| sym-93251a13ab5d4a5e5072 | `custom` | function | Convenience constructor for custom / vendor extension ports. | `src/graph/signal/spec.rs:304` |
| sym-d702ca116c48c799c774 | `deadline` | function | Returns the deadline associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:208` |
| sym-de350093988e7d14c0ac | `default` | function | Returns the default `RuntimePlanner` value. | `src/graph/compile/plan.rs:349` |
| sym-b1902764a29198ef1867 | `default` | function | Returns the default `Compiler` value. | `src/graph/compile/resolve.rs:513` |
| sym-dcb5e0588fab365a19c7 | `definition` | function | Returns the definition associated with `NodeRegistry`. | `src/graph/registry.rs:134` |
| sym-e916d03fa72a650b78b3 | `delivery` | function | Returns the delivery associated with `EdgeContract`. | `src/graph/ports.rs:345` |
| sym-28451209252058476fb8 | `derivation` | function | Returns the derivation associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:82` |
| sym-4d22610034cf756ca6ea | `descriptor` | function | Returns the descriptor associated with `PassthroughFactory`. | `src/graph/builtins.rs:70` |
| sym-45c32ecad1ac42cbd26d | `descriptor` | function | Returns the descriptor associated with `GainFactory`. | `src/graph/builtins.rs:110` |
| sym-06610d9f4ca3e6efac10 | `descriptor` | function | Returns the descriptor associated with `MonoMixFactory`. | `src/graph/builtins.rs:169` |
| sym-d8a211489124a08425d2 | `descriptor` | function | Returns the descriptor associated with `NodeDefinitionRef`. | `src/graph/registry.rs:39` |
| sym-13032dd31eb4cc492fdb | `direction` | function | Returns the direction associated with `PortPrepareContext`. | `src/graph/node.rs:345` |
| sym-893b8f9f841f5d379b68 | `direction` | function | Returns the direction associated with `PortSpec`. | `src/graph/ports.rs:217` |
| sym-9a0ea2064c79ae612aca | `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:77` |
| sym-6231de0f2ff63f025a56 | `display_name` | function | Returns the display name held by `NodeDescriptor`. | `src/graph/node.rs:226` |
| sym-db7739acf4a049ac817e | `duration_ns` | function | Returns the duration nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:83` |
| sym-b25bfeb7fc80ca6ae6ee | `edge_buffer` | function | Returns the edge buffer associated with `MemoryPlan`. | `src/graph/plan.rs:71` |
| sym-c8da66f370729379cea7 | `edge_contract` | function | Returns the edge contract held by `PortPrepareContext`. | `src/graph/node.rs:357` |
| sym-74e190b45549845b8de4 | `edge_count` | function | Returns the edge count held by `GraphIr`. | `src/graph/ir.rs:43` |
| sym-a9bd0817b7a3e0d0a511 | `edge_count` | function | Returns the edge count held by `GraphSpec`. | `src/graph/spec.rs:69` |
| sym-ce4d5aa9e601fb810c11 | `edge_id` | function | Returns the edge identifier held by `PortPrepareContext`. | `src/graph/node.rs:337` |
| sym-7a8bbbedb1dada1a1e63 | `encoded_audio` | function | Convenience constructor for encoded audio ports. | `src/graph/signal/spec.rs:274` |
| sym-1ed93660c3142fc23078 | `event` | function | Convenience constructor for event ports. | `src/graph/signal/spec.rs:284` |
| sym-109839bcc0f547d406a9 | `execution` | function | Returns the execution held by `NodeDescriptor`. | `src/graph/node.rs:238` |
| sym-8d74fcdee897cc6b2612 | `execution_partition` | function | Returns the execution partition associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:58` |
| sym-ce1d5048575f2b7df65a | `failure` | function | Returns the failure held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:216` |
| sym-394bf065ef645171da20 | `fmt` | function | Formats `NodeTypeId` with the requested formatter. | `src/graph/node.rs:31` |
| sym-7a52b52dec52c8241c9b | `fmt` | function | Formats `NodeConfig` with the requested formatter. | `src/graph/node.rs:116` |
| sym-860c89400cb03842aff4 | `from` | function | Converts the supplied value into `NodeTypeId`. | `src/graph/node.rs:37` |
| sym-3d7f6422ad7c37c6fba4 | `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:41` |
| sym-c845451343259baf21d7 | `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:47` |
| sym-10f672fb4fd9ea0353b5 | `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:70` |
| sym-16ae1fc4ab19f08b73f9 | `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:76` |
| sym-ebc447f732c6b6f2bc2c | `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:100` |
| sym-2bbd9d7852288cc41867 | `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:106` |
| sym-45f4c0cdf51bcdc4938c | `from_audio` | function | Creates `SignalEnvelope` from audio. | `src/graph/signal/envelope.rs:27` |
| sym-ab62f3af0b2b53f135a3 | `from_frame` | function | Creates `SignalLineage` from frame. | `src/graph/signal/lineage.rs:46` |
| sym-afed6d749b1c8f4c3fe2 | `from_frame` | function | Creates `SignalTiming` from frame. | `src/graph/signal/timing.rs:56` |
| sym-925a2e0b934e56ec63a6 | `from_index` | function | Creates a stable runtime node identifier for externally assembled plans. | `src/graph/spec.rs:12` |
| sym-1b8d79e615e1fe954f3c | `generation` | function | Returns the generation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:184` |
| sym-3a76f39dc504291bffd6 | `get` | function | Returns the value held by `NodeConfig`. | `src/graph/node.rs:92` |
| sym-bdcdf4508ad2f85d3973 | `get` | function | Returns the value held by `NodeRegistry`. | `src/graph/registry.rs:127` |
| sym-43ea790b9b2e11b1f367 | `get_f32` | function | Returns the get f32 associated with `NodeConfig`. | `src/graph/node.rs:100` |
| sym-a60956ec58426d610a4a | `get_u32` | function | Returns the get u32 associated with `NodeConfig`. | `src/graph/node.rs:104` |
| sym-e0cb77466f989d46496c | `id` | function | Returns the id held by `NodeHandle`. | `src/graph/dsl.rs:15` |
| sym-a96cb8990200bc9574fc | `id` | function | Returns the id held by `ResolvedNode`. | `src/graph/ir.rs:16` |
| sym-5c1cf0d28403174ca276 | `in_` | function | Selects a named input port from `NodeHandle`. | `src/graph/dsl.rs:24` |
| sym-924609a46165ae2cfe6e | `index` | function | Returns the index held by `NodeId`. | `src/graph/spec.rs:16` |
| sym-c69d054b4be929761cf1 | `index` | function | Returns the index held by `EdgeId`. | `src/graph/spec.rs:25` |
| sym-dd96ef2aeb6ef1eb26f8 | `input_edge` | function | Returns the input edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:192` |
| sym-d9b0585f544c51598a14 | `input_ports` | function | Returns the input ports held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:224` |
| sym-1b14d521019c46c6df5f | `inputs` | function | Returns the inputs associated with `NodeDescriptor`. | `src/graph/node.rs:230` |
| sym-8f5ae7c7298283723995 | `inputs` | function | Returns the inputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:62` |
| sym-4f321ed7a203a3f5cf9c | `instantiate` | function | Instantiates the runtime node described by `PassthroughFactory`. | `src/graph/builtins.rs:86` |
| sym-66e194664cbbd9404707 | `instantiate` | function | Instantiates the runtime node described by `GainFactory`. | `src/graph/builtins.rs:135` |
| sym-c7f732c51c86e2736226 | `instantiate` | function | Instantiates the runtime node described by `MonoMixFactory`. | `src/graph/builtins.rs:185` |
| sym-c64fa9fb69d22a43a170 | `into_payload` | function | Converts `SignalEnvelope` into payload. | `src/graph/signal/envelope.rs:86` |
| sym-577f70ac8cddb985786a | `into_spec` | function | Converts `Pipeline` into spec. | `src/graph/dsl.rs:90` |
| sym-b906c1bced2bdb6a5317 | `is_audio` | function | Returns `true` for classes that carry real-time audio on the hot path. | `src/graph/signal/spec.rs:180` |
| sym-13ba64b8902a67ddb451 | `is_compatible_with` | function | Returns whether compatible with applies to `ChannelLayout`. | `src/graph/ports.rs:42` |
| sym-ba6f1a5ae34c37391c97 | `is_compatible_with` | function | Returns whether compatible with applies to `AudioCaps`. | `src/graph/ports.rs:56` |
| sym-1660e53891c50aea7f58 | `is_compatible_with` | function | Returns whether compatible with applies to `MediaCaps`. | `src/graph/ports.rs:110` |
| sym-53a017982fc527349282 | `is_compatible_with` | function | Returns `true` if two signal classes are compatible for edge wiring. | `src/graph/signal/spec.rs:188` |
| sym-5762003566d065479b7e | `is_compatible_with` | function | Returns `true` if this spec can connect to `other` on an edge. | `src/graph/signal/spec.rs:324` |
| sym-b2fd5f431d9ee97ce804 | `is_empty` | function | Returns whether `NodeRegistry` contains no values. | `src/graph/registry.rs:174` |
| sym-6ece39cf3776addca714 | `is_portable` | function | Reports whether this value is a portable implementation contract ID. | `src/graph/operator.rs:31` |
| sym-93b640de9e9b9e6dae8c | `is_portable` | function | Reports whether this custom signal ID is portable across packages, languages, and processes. | `src/graph/signal/spec.rs:35` |
| sym-0d8267066ef89ab05c61 | `is_realtime` | function | Returns whether realtime applies to `ClockDomain`. | `src/graph/ports.rs:259` |
| sym-963f249bd1aea6af2b47 | `is_sensitive` | function | Returns whether sensitive applies to `NodeConfig`. | `src/graph/node.rs:96` |
| sym-2afbd1ea7c3f124dfc31 | `is_stateful` | function | Returns whether stateful applies to `NodeDescriptor`. | `src/graph/node.rs:246` |
| sym-aade164ba212f0ffdcd9 | `is_terminal` | function | Returns whether terminal applies to `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:86` |
| sym-62d0ddd2396f4158a7c8 | `is_valid_for` | function | Returns `true` if this contract is compatible with the given partition. | `src/graph/partition.rs:107` |
| sym-5c57b5f3925d88b78690 | `is_well_formed` | function | Reports whether this value follows the portable node-type syntax. | `src/graph/node.rs:25` |
| sym-e800a22871e2d39d4a92 | `iter` | function | Iterates over the values held by `NodeConfig`. | `src/graph/node.rs:108` |
| sym-737471186be91f70d361 | `jitter_budget_ms` | function | Returns the jitter budget milliseconds held by `EdgeContract`. | `src/graph/ports.rs:337` |
| sym-57621d660613d124bc7d | `kind` | function | Returns the kind represented by `MediaCaps`. | `src/graph/ports.rs:97` |
| sym-88ac630e6853f144d88b | `latency_budget_ms` | function | Returns the latency budget milliseconds held by `EdgeContract`. | `src/graph/ports.rs:333` |
| sym-5584661d81e492674fbc | `len` | function | Returns the number of values held by `NodeRegistry`. | `src/graph/registry.rs:169` |
| sym-d928dfcd4acd982941e6 | `lineage` | function | Returns the frame lineage carried by `SignalEnvelope`. | `src/graph/signal/envelope.rs:78` |
| sym-0678413cd713daedd00b | `loss` | function | Returns the loss associated with `EdgeContract`. | `src/graph/ports.rs:349` |
| sym-3e7d253933f0017b44ba | `map_payload` | function | Transforms the payload held by `SignalEnvelope` while preserving envelope metadata. | `src/graph/signal/envelope.rs:45` |
| sym-ef33765f60bd132eadb2 | `max_payload_bytes` | function | Returns the max payload bytes held by `EdgeContract`. | `src/graph/ports.rs:361` |
| sym-772e938f00772a8503ef | `media` | function | Returns the media held by `PortPrepareContext`. | `src/graph/node.rs:353` |
| sym-7e8ce0544afa645581b4 | `media` | function | Returns the media held by `PortSpec`. | `src/graph/ports.rs:225` |
| sym-22676867b0d15bab67f4 | `media` | function | Returns the media held by `EdgeContract`. | `src/graph/ports.rs:325` |
| sym-113e05ae5fbaa2c0f028 | `metric_id` | function | Returns the metric identifier held by `RuntimePlan`. | `src/graph/plan.rs:145` |
| sym-c2c639401b42645bb6ba | `metrics` | function | Convenience constructor for metrics ports. | `src/graph/signal/spec.rs:289` |
| sym-fc477cd677af53c92c6b | `multiplicity` | function | Returns the multiplicity associated with `PortSpec`. | `src/graph/ports.rs:229` |
| sym-788106bf95ab2bdd7f7c | `name` | function | Returns the name associated with `PortSpec`. | `src/graph/ports.rs:213` |
| sym-fb87a273b6f988c4aa46 | `needs_bridge_to` | function | Returns `true` if crossing from `self` to `other` requires a compiler-inserted Bridge. | `src/graph/partition.rs:71` |
| sym-4b477dab963879323344 | `negotiate` | function | Negotiates the compatible media capabilities shared by `MediaCaps` and its peer. | `src/graph/ports.rs:124` |
| sym-b68c3e616aaa58b68339 | `new` | function | Creates a new `RuntimePlanner`. | `src/graph/compile/plan.rs:14` |
| sym-7e5f2e28b6331376d63a | `new` | function | Creates a new `Compiler`. | `src/graph/compile/resolve.rs:449` |
| sym-bd3a861e9622497aecca | `new` | function | Creates a new `Pipeline`. | `src/graph/dsl.rs:40` |
| sym-b5064e4af7ce624453f8 | `new` | function | Creates a new `NodeConfig`. | `src/graph/node.rs:62` |
| sym-687e670038b6609a360f | `new` | function | Creates a new `NodeDescriptor`. | `src/graph/node.rs:176` |
| sym-39e31561ba3bfa2ed241 | `new` | function | Creates a new `PrepareContext`. | `src/graph/node.rs:271` |
| sym-b053bd428778a9cd31eb | `new` | function | Creates a new `PortPrepareContext`. | `src/graph/node.rs:293` |
| sym-f923baa2343ca7973bb0 | `new` | function | Creates a new `OperatorId`. | `src/graph/operator.rs:19` |
| sym-94e3d1cec3698575733a | `new` | function | Creates a new `PortSpec`. | `src/graph/ports.rs:185` |
| sym-d7732458296b48f768a6 | `new` | function | Creates a new `NodeRegistry`. | `src/graph/registry.rs:73` |
| sym-fe437b9fb0aaf12d51c8 | `new` | function | Creates a new `SignalDerivation`. | `src/graph/signal/lineage.rs:107` |
| sym-c8580f82ceed08e7c92b | `new` | function | Creates a new `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:144` |
| sym-371f2a20d5a64c7f8939 | `new` | function | Creates a new `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:29` |
| sym-7bcc18095a8907bf9b15 | `new` | function | Creates a new `SignalId`. | `src/graph/signal/spec.rs:25` |
| sym-46acac4d46fcc2cdad86 | `new` | function | Creates a new `SemanticRole`. | `src/graph/signal/spec.rs:60` |
| sym-54c64d2b00dfed01239b | `new` | function | Creates a new `SchemaRef`. | `src/graph/signal/spec.rs:90` |
| sym-f469a7085bd1b4e5dfbe | `new` | function | Creates a new `SignalSpec`. | `src/graph/signal/spec.rs:226` |
| sym-326b76705931c8f8480d | `node` | function | Returns the node held by `GraphIr`. | `src/graph/ir.rs:47` |
| sym-b0afa83546e4319231fa | `node` | function | Returns the node held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:188` |
| sym-192c1a9561eb55f5cadd | `node` | function | Returns the node held by `GraphSpec`. | `src/graph/spec.rs:73` |
| sym-520efa0c85a457ce848a | `node_count` | function | Returns the node count held by `GraphIr`. | `src/graph/ir.rs:40` |
| sym-4fda4173b78ebd43a378 | `node_count` | function | Returns the node count held by `RuntimePlan`. | `src/graph/plan.rs:135` |
| sym-8f6bfd5768e658b5f70a | `node_count` | function | Returns the node count held by `GraphSpec`. | `src/graph/spec.rs:65` |
| sym-9a415f567ef1471d0a8e | `observability` | function | Returns the observability associated with `EdgeContract`. | `src/graph/ports.rs:357` |
| sym-992b257f18f1c4cc5298 | `observe` | function | Returns the current observation exposed by `SignalContinuityTracker`. | `src/graph/signal/continuity.rs:18` |
| sym-12dc62d0bd0885d8fc00 | `observed` | function | Creates observed signal timing for `SignalTiming`. | `src/graph/signal/timing.rs:38` |
| sym-074b05e12d2a3ea9963e | `observed_timestamp_ns` | function | Returns the observed timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:75` |
| sym-24b00937acc05b71f6db | `operator_generation` | function | Returns the operator generation associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:150` |
| sym-8ff73184ea1f85f0b753 | `operator_id` | function | Returns the operator identifier held by `SignalDerivation`. | `src/graph/signal/lineage.rs:144` |
| sym-9fdae1b4561a92b38e0c | `operator_id` | function | Returns the operator identifier held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:176` |
| sym-a4a72750d0738fe4fc9b | `operator_revision` | function | Returns the operator revision held by `SignalDerivation`. | `src/graph/signal/lineage.rs:147` |
| sym-e3c6eb1de0ca96234dfc | `out` | function | Selects a named output port from `NodeHandle`. | `src/graph/dsl.rs:18` |
| sym-e68babbafd6f6b97672c | `output_edge` | function | Returns the output edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:196` |
| sym-5f7590d23635def1696e | `output_ports` | function | Returns the output ports held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:231` |
| sym-ee093c39da623635f4ef | `output_roles` | function | Returns the output roles associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:220` |
| sym-8417dc284f756364cbd0 | `outputs` | function | Returns the outputs associated with `NodeDescriptor`. | `src/graph/node.rs:234` |
| sym-48cad7900de712e74f64 | `outputs` | function | Returns the outputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:66` |
| sym-bcde5bc10f8c38ef38b0 | `partition` | function | Returns the partition associated with `RuntimePlan`. | `src/graph/plan.rs:140` |
| sym-a2d755aefd3247e4069b | `payload` | function | Returns the payload associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:62` |
| sym-5fa61a47369bd239fced | `payload_size_bytes` | function | Returns the payload size bytes held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:66` |
| sym-c3632754280e719cb8a0 | `permission` | function | Returns the permission associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:204` |
| sym-221910aa2a407a20b2a6 | `plan` | function | Lower a verified IR into an execution-ready plan. | `src/graph/compile/plan.rs:24` |
| sym-874507cffde971fe579a | `pocketstation::graph::builtins::register_builtins` | function | Registers builtins for `builtins`. | `src/graph/builtins.rs:220` |
| sym-7db5666d2dff33bc81b5 | `policy_epoch` | function | Returns the policy epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:80` |
| sym-0321107d9ee385b6a2d9 | `port_name` | function | Returns the port name held by `PortPrepareContext`. | `src/graph/node.rs:341` |
| sym-1b072df2e8c199b88812 | `prepare` | function | Prepares resources required by `PassthroughNode`. | `src/graph/builtins.rs:98` |
| sym-45fe39b372136f053d22 | `prepare` | function | Prepares resources required by `GainNode`. | `src/graph/builtins.rs:154` |
| sym-f4df6d2fb38479a041f3 | `prepare` | function | Prepares resources required by `MonoMixNode`. | `src/graph/builtins.rs:197` |
| sym-cb61f77e7c88e858fa6f | `process` | function | Processes an input value through `PassthroughNode`. | `src/graph/builtins.rs:102` |
| sym-23912e56afc7e14ab0f3 | `process` | function | Processes an input value through `GainNode`. | `src/graph/builtins.rs:158` |
| sym-af499cf346452c91c428 | `process` | function | Processes an input value through `MonoMixNode`. | `src/graph/builtins.rs:201` |
| sym-4e088039148f4ee6cbcb | `queue_capacity_frames` | function | Returns the queue capacity frames held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:200` |
| sym-d39bc3b5e4947ef45350 | `rank` | function | Priority rank for scheduling: lower = higher priority. | `src/graph/partition.rs:60` |
| sym-b1f93c6ba89bb907ab5e | `rank` | function | Returns the rank associated with `EdgeObservabilityLevel`. | `src/graph/ports.rs:301` |
| sym-f2823e099d189909ba5f | `realtime_audio` | function | Generic realtime PCM edge. Concrete sample rate, frame size, and channel layout are negotiated from connected ports. | `src/graph/ports.rs:391` |
| sym-127d4e8eac621aa2cd73 | `register` | function | Registers a node definition with `NodeRegistry` while preserving unique identities. | `src/graph/registry.rs:77` |
| sym-d26f92546b597ee8eef7 | `register_async` | function | Registers async for `NodeRegistry`. | `src/graph/registry.rs:89` |
| sym-38ceb9e02ee487d3eb18 | `register_definition` | function | Registers definition for `NodeRegistry`. | `src/graph/registry.rs:112` |
| sym-7a0c4d40c5310e826b12 | `required` | function | Returns the required held by `PortSpec`. | `src/graph/ports.rs:233` |
| sym-6127dbc8a3fa24244d1a | `requires_realtime_safety` | function | Returns `true` if the partition requires strict real-time safety. | `src/graph/partition.rs:55` |
| sym-8a92331d70fc48323ebd | `revision` | function | Returns the revision held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:180` |
| sym-6cbb5a9702d66bb9acab | `role` | function | Returns the role associated with `SignalSpec`. | `src/graph/signal/spec.rs:219` |
| sym-2a11bb5d1823a37a93d1 | `safety` | function | Returns the safety held by `NodeDescriptor`. | `src/graph/node.rs:242` |
| sym-87959113d8c029a0e5b5 | `schema` | function | Returns the schema held by `SignalSpec`. | `src/graph/signal/spec.rs:223` |
| sym-3d646b762682ed19138f | `sequence_number` | function | Returns the sequence number held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:90` |
| sym-bb0dc9b29e51083e200b | `sequence_number` | function | Returns the sequence number held by `SignalLineage`. | `src/graph/signal/lineage.rs:71` |
| sym-27466d271df564e717ec | `session_id` | function | Returns the session identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:59` |
| sym-0267117466ccdec4ecb8 | `session_timestamp_ns` | function | Returns the session timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:79` |
| sym-7dd55d12dd6380c1deeb | `signal` | function | Returns the signal held by `PortPrepareContext`. | `src/graph/node.rs:349` |
| sym-dd99cd344d21bf7dbb2a | `signal` | function | Returns the signal held by `PortSpec`. | `src/graph/ports.rs:221` |
| sym-08f37b5d5044dd0831e3 | `signal_spec` | function | Returns the signal spec held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:70` |
| sym-822f8306016599aabc4b | `size_bytes` | function | Owned media bytes represented by this payload. Envelope metadata and queue slot storage are fixed-size and accounted separately by the edge. | `src/graph/signal/payload.rs:37` |
| sym-e86a63738361451935e5 | `source_generation` | function | Returns the source generation associated with `SignalLineage`. | `src/graph/signal/lineage.rs:74` |
| sym-a6ff5b3bbd5b6dda6463 | `source_id` | function | Returns the source identifier held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:100` |
| sym-b0efd6d304ca0b3dd9d1 | `source_id` | function | Returns the source identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:65` |
| sym-89d57a64f65f1ba17140 | `source_timestamp_ns` | function | Returns the source timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:71` |
| sym-67a67719d3a5121b84ba | `spec` | function | Returns the spec associated with `Pipeline`. | `src/graph/dsl.rs:86` |
| sym-428715befe67be9cea63 | `stream_id` | function | Returns the stream identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:62` |
| sym-ba4f5e56d43535277f59 | `supports` | function | Returns whether supports is true for `SignalPayload`. | `src/graph/signal/payload.rs:17` |
| sym-e855bceefb46d8582818 | `supports_signal` | function | Returns whether supports signal applies to `MediaCaps`. | `src/graph/ports.rs:142` |
| sym-e8e668a7e4cace617ce0 | `syntax_version` | function | Returns the syntax version held by `OperatorId`. | `src/graph/operator.rs:35` |
| sym-a455ae640500e774203c | `text` | function | Convenience constructor for text ports. | `src/graph/signal/spec.rs:279` |
| sym-50d2b8c719e7733cec6b | `timestamp_end_ns` | function | Returns the timestamp end nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:65` |
| sym-c1b6f572ff5263b5a11b | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:110` |
| sym-0ab1b64bcc0cbfe5043a | `timing` | function | Returns the timing associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:74` |
| sym-166cb316032972045648 | `topo_order` | function | Returns the topo order associated with `GraphIr`. | `src/graph/ir.rs:51` |
| sym-7ca2c113e686021a5382 | `total_bytes` | function | Returns the total bytes held by `EdgeBufferPlan`. | `src/graph/plan.rs:44` |
| sym-35bfe2e5904a7f79c1a3 | `try_new` | function | Creates a new `SignalLineage` after validating its inputs. | `src/graph/signal/lineage.rs:21` |
| sym-9296b749e3314413d49b | `try_new` | function | Creates a new `SignalTiming` after validating its inputs. | `src/graph/signal/timing.rs:14` |
| sym-0bd81d1b03195e37a8c3 | `type_id` | function | Returns the type identifier held by `NodeDescriptor`. | `src/graph/node.rs:222` |
| sym-4acc73f21d0ef46c885f | `type_ids` | function | Returns the type identifiers held by `NodeRegistry`. | `src/graph/registry.rs:179` |
| sym-1baffffec7b4283a4fb3 | `type_str` | function | Returns the type str associated with `ResolvedNode`. | `src/graph/ir.rs:19` |
| sym-28db26cc562bb969a6a2 | `typed_edge` | function | Returns the typed edge associated with `RuntimePlan`. | `src/graph/plan.rs:152` |
| sym-f993040e646ca6b9454e | `untracked` | function | Creates an envelope for data that has not yet entered a source-aware Session. Session sources must attach lineage before routing it. | `src/graph/signal/envelope.rs:17` |
| sym-4b70b02be3b032f7dc81 | `upstream_lineage` | function | Returns the upstream lineage held by `SignalDerivation`. | `src/graph/signal/lineage.rs:138` |
| sym-0c7b7dd6fd5c06082796 | `upstream_timing` | function | Returns the upstream timing associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:141` |
| sym-1a9b138a5be7f705864e | `validate` | function | Validates `SignalEnvelope` against its declared contract. | `src/graph/signal/envelope.rs:117` |
| sym-de5e159b7b99c2a19739 | `validate` | function | Validates `AsyncOperatorManifest` against its declared contract. | `src/graph/signal/operator.rs:238` |
| sym-9f7dee1de2898f998aab | `validate` | function | Validates `SignalSpec` against its declared contract. | `src/graph/signal/spec.rs:328` |
| sym-f633d73f182b59133294 | `validate_config` | function | Validates config for `PassthroughFactory`. | `src/graph/builtins.rs:82` |
| sym-c4442f34c129c1d217e7 | `validate_config` | function | Validates config for `GainFactory`. | `src/graph/builtins.rs:122` |
| sym-fd6807d3d194959e8584 | `validate_config` | function | Validates config for `MonoMixFactory`. | `src/graph/builtins.rs:181` |
| sym-cf0afdda5671d8271d09 | `validate_config` | function | Validates config for `NodeDefinitionRef`. | `src/graph/registry.rs:47` |
| sym-b8f00f5913d4dc0b8ea1 | `wire_id` | function | Stable language-neutral identifier for the fundamental wire class. Semantic role and schema remain separate fields. | `src/graph/signal/spec.rs:236` |
| sym-5c3a13ef33fc1607a0a2 | `with` | function | Returns `NodeConfig` with the supplied entry applied. | `src/graph/node.rs:66` |
| sym-4e45baf748617df48a70 | `with_backpressure` | function | Sets the backpressure on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:370` |
| sym-a9ae595a7b53ac1d3a85 | `with_copy_policy` | function | Sets the copy policy on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:375` |
| sym-e30c7ea084f435683404 | `with_derivation` | function | Sets the derivation on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:57` |
| sym-1b243564c06db150e470 | `with_duration_ns` | function | Sets the duration nanoseconds on `SignalTiming` and returns the updated value. | `src/graph/signal/timing.rs:47` |
| sym-2af95d23d18cceb763aa | `with_jitter_budget_ms` | function | Sets the jitter budget milliseconds on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:380` |
| sym-1e65e79dd7b779a051a4 | `with_lineage` | function | Sets the lineage on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:51` |
| sym-555b47b19472a4dc3f3d | `with_max_payload_bytes` | function | Sets the max payload bytes on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:385` |
| sym-63bcba5c6bc3f69d83f4 | `with_media` | function | Sets the media on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:365` |
| sym-791f270dedc9b5e1803d | `with_role` | function | Attach a semantic role annotation. | `src/graph/signal/spec.rs:309` |
| sym-5de4cecef3f1cd823fd9 | `with_schema` | function | Attach a schema reference. | `src/graph/signal/spec.rs:315` |
| sym-887474f5adecb94d44db | `with_sensitive` | function | Adds a setup-time value whose normal debug representation is redacted. | `src/graph/node.rs:81` |
| sym-85c70119164eb3ec742a | `pocketstation::graph` | module | Stable signal, port, capability, partition, and extension contracts. | `src/graph/mod.rs:1` |
| sym-c29c3e128aaa67f89e86 | `pocketstation::graph::builtins::GainFactory` | struct | Constructs gain implementations from validated declarations. | `src/graph/builtins.rs:107` |
| sym-8434e3c12de20a019e55 | `pocketstation::graph::builtins::GainNode` | struct | Executes the graph-node behavior defined for gain. | `src/graph/builtins.rs:149` |
| sym-b9cafd7e4da90ab989df | `pocketstation::graph::builtins::MonoMixFactory` | struct | Constructs mono mix implementations from validated declarations. | `src/graph/builtins.rs:166` |

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

The claims on **Graph and route contracts** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/graph/mod.rs:1-67` (`DIRECT`)

For **Graph and route contracts**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
