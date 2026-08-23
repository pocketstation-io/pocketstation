# Graph and signal failures

<!-- claims: CLM-ERR-003-CAP-001,CLM-ERR-003-CAP-002,CLM-ERR-003-CAP-003,CLM-ERR-003-CAP-004,CLM-ERR-003-CAP-005,CLM-ERR-003-SOURCE-001,CLM-ERR-003-ERROR-0001,CLM-ERR-003-ERROR-0002,CLM-ERR-003-ERROR-0003,CLM-ERR-003-ERROR-0004,CLM-ERR-003-ERROR-0005,CLM-ERR-003-ERROR-0006,CLM-ERR-003-ERROR-0007,CLM-ERR-003-ERROR-0008,CLM-ERR-003-ERROR-0009,CLM-ERR-003-ERROR-0010,CLM-ERR-003-ERROR-0011,CLM-ERR-003-ERROR-0012,CLM-ERR-003-ERROR-0013,CLM-ERR-003-ERROR-0014,CLM-ERR-003-ERROR-0015,CLM-ERR-003-ERROR-0016,CLM-ERR-003-ERROR-0017,CLM-ERR-003-ERROR-0018,CLM-ERR-003-ERROR-0019,CLM-ERR-003-ERROR-0020,CLM-ERR-003-ERROR-0021,CLM-ERR-003-ERROR-0022,CLM-ERR-003-ERROR-0023,CLM-ERR-003-ERROR-0024,CLM-ERR-003-ERROR-0025,CLM-ERR-003-ERROR-0026,CLM-ERR-003-ERROR-0027,CLM-ERR-003-ERROR-0028,CLM-ERR-003-ERROR-0029,CLM-ERR-003-ERROR-0030,CLM-ERR-003-ERROR-0031,CLM-ERR-003-ERROR-0032,CLM-ERR-003-ERROR-0033,CLM-ERR-003-ERROR-0034,CLM-ERR-003-ERROR-0035,CLM-ERR-003-ERROR-0036,CLM-ERR-003-ERROR-0037,CLM-ERR-003-ERROR-0038,CLM-ERR-003-ERROR-0039,CLM-ERR-003-ERROR-0040,CLM-ERR-003-ERROR-0041,CLM-ERR-003-ERROR-0042,CLM-ERR-003-ERROR-0043,CLM-ERR-003-ERROR-0044,CLM-ERR-003-ERROR-0045,CLM-ERR-003-ERROR-0046,CLM-ERR-003-ERROR-0047,CLM-ERR-003-ERROR-0048,CLM-ERR-003-ERROR-0049,CLM-ERR-003-ERROR-0050,CLM-ERR-003-ERROR-0051,CLM-ERR-003-ERROR-0052,CLM-ERR-003-ERROR-0053,CLM-ERR-003-ERROR-0054,CLM-ERR-003-ERROR-0055,CLM-ERR-003-ERROR-0056,CLM-ERR-003-ERROR-0057,CLM-ERR-003-ERROR-0058,CLM-ERR-003-ERROR-0059,CLM-ERR-003-ERROR-0060,CLM-ERR-003-ERROR-0061,CLM-ERR-003-ERROR-0062,CLM-ERR-003-ERROR-0063,CLM-ERR-003-ERROR-0064,CLM-ERR-003-ERROR-0065,CLM-ERR-003-ERROR-0066,CLM-ERR-003-ERROR-0067,CLM-ERR-003-ERROR-0068,CLM-ERR-003-ERROR-0069,CLM-ERR-003-ERROR-0070,CLM-ERR-003-ERROR-0071,CLM-ERR-003-ERROR-0072,CLM-ERR-003-ERROR-0073,CLM-ERR-003-ERROR-0074,CLM-ERR-003-ERROR-0075,CLM-ERR-003-ERROR-0076,CLM-ERR-003-ERROR-0077,CLM-ERR-003-ERROR-0078,CLM-ERR-003-ERROR-0079,CLM-ERR-003-ERROR-0080,CLM-ERR-003-ERROR-0081,CLM-ERR-003-ERROR-0082,CLM-ERR-003-ERROR-0083,CLM-ERR-003-ERROR-0084,CLM-ERR-003-ERROR-0085,CLM-ERR-003-ERROR-0086,CLM-ERR-003-ERROR-0087,CLM-ERR-003-ERROR-0088,CLM-ERR-003-ERROR-0089,CLM-ERR-003-ERROR-0090,CLM-ERR-003-ERROR-0091,CLM-ERR-003-ERROR-0092,CLM-ERR-003-ERROR-0093,CLM-ERR-003-ERROR-0094,CLM-ERR-003-ERROR-0095,CLM-ERR-003-ERROR-0096,CLM-ERR-003-ERROR-0097,CLM-ERR-003-ERROR-0098,CLM-ERR-003-ERROR-0099,CLM-ERR-003-ERROR-0100,CLM-ERR-003-ERROR-0101,CLM-ERR-003-ERROR-0102,CLM-ERR-003-ERROR-0103,CLM-ERR-003-ERROR-0104,CLM-ERR-003-ERROR-0105,CLM-ERR-003-ERROR-0106,CLM-ERR-003-ERROR-0107,CLM-ERR-003-ERROR-0108,CLM-ERR-003-ERROR-0109,CLM-ERR-003-ERROR-0110,CLM-ERR-003-ERROR-0111,CLM-ERR-003-ERROR-0112,CLM-ERR-003-ERROR-0113,CLM-ERR-003-ERROR-0114,CLM-ERR-003-ERROR-0115,CLM-ERR-003-ERROR-0116,CLM-ERR-003-ERROR-0117,CLM-ERR-003-ERROR-0118,CLM-ERR-003-ERROR-0119,CLM-ERR-003-ERROR-0120,CLM-ERR-003-ERROR-0121,CLM-ERR-003-ERROR-0122,CLM-ERR-003-ERROR-0123,CLM-ERR-003-ERROR-0124,CLM-ERR-003-ERROR-0125,CLM-ERR-003-ERROR-0126,CLM-ERR-003-ERROR-0127,CLM-ERR-003-ERROR-0128,CLM-ERR-003-ERROR-0129,CLM-ERR-003-ERROR-0130,CLM-ERR-003-ERROR-0131,CLM-ERR-003-ERROR-0132,CLM-ERR-003-ERROR-0133,CLM-ERR-003-ERROR-0134,CLM-ERR-003-ERROR-0135,CLM-ERR-003-ERROR-0136,CLM-ERR-003-ERROR-0137,CLM-ERR-003-ERROR-0138 -->

## Scope

- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.
- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Error inventory

| Evidence record | Type | Variant | Retryable | Recoverable | Defined |
|---|---|---|---|---|---|
| error-04b7031025a9b635fdbf | `pocketstation::graph::node::NodeDescriptorError` | `InvalidSafetyContract` | unknown | unknown | `src/graph/node.rs:258` |
| error-0be8ad81000b2924c24c | `pocketstation::graph::node::ConfigError` | type | unknown | unknown | `src/graph/node.rs:141` |
| error-0da3f91a5f274a27ab76 | `pocketstation::graph::compile::resolve::CompileError` | type | unknown | unknown | `src/graph/compile/resolve.rs:26` |
| error-10e3a522fa28fccdfc60 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `ZeroProcessTimeout` | unknown | unknown | `src/graph/signal/operator.rs:331` |
| error-143cce14f0e71f68c4cf | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `InvalidMagic` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:298` |
| error-14ca51fa44623142d004 | `pocketstation::graph::signal::operator::OperatorFailurePolicy` | `StopWorker` | unknown | unknown | `src/graph/signal/operator.rs:65` |
| error-170066b0b40a26e0e33d | `pocketstation::graph::node::NodeError` | `Process` | unknown | unknown | `src/graph/node.rs:153` |
| error-18565faf820bbf8e2650 | `pocketstation::graph::signal::continuity::SignalContinuityError` | `SequenceGapWithoutDiscontinuity` | unknown | unknown | `src/graph/signal/continuity.rs:97` |
| error-1877b4a7bdffa5d7ed88 | `pocketstation::graph::compile::resolve::CompileError` | `MediaMismatch` | unknown | unknown | `src/graph/compile/resolve.rs:45` |
| error-1897c7da4711d75eb14d | `pocketstation::graph::signal::continuity::SignalContinuityError` | `InvalidEnvelope` | unknown | unknown | `src/graph/signal/continuity.rs:91` |
| error-18d1485abaf31198b6d8 | `pocketstation::graph::plan::PlanError` | `MoveExclusiveFanOut` | unknown | unknown | `src/graph/plan.rs:25` |
| error-1981cbd27763ca5ffcbe | `pocketstation::graph::node::NodeDescriptorError` | `EmptyDisplayName` | unknown | unknown | `src/graph/node.rs:256` |
| error-19eabd878a9188bf94ce | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Wait` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:726` |
| error-1be7a5d9b8d5cbceab93 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `InputEdgeMediaMismatch` | unknown | unknown | `src/graph/signal/operator.rs:349` |
| error-1d9b879cab06d8598907 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `ReservedFieldSet` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:308` |
| error-1fb4b2d84a6cf23abbd9 | `pocketstation::graph::node::ConfigError` | `Missing` | unknown | unknown | `src/graph/node.rs:143` |
| error-201cc7749bdbbd671d69 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `InvalidTerminal` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:306` |
| error-23eba8b87dea81473095 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `FrameLengthOverflow` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:320` |
| error-2431503c1bc613dbc5c4 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `OutputEdgeMediaMismatch` | unknown | unknown | `src/graph/signal/operator.rs:351` |
| error-243f5b367fb16b38fdea | `pocketstation::graph::registry::NodeRegistrationError` | `DuplicateNodeType` | unknown | unknown | `src/graph/registry.rs:61` |
| error-2652e90b3fd931c3b8db | `pocketstation::graph::signal::continuity::SignalContinuityError` | `IdentityChanged` | unknown | unknown | `src/graph/signal/continuity.rs:95` |
| error-28b1fb124ed036dbd23a | `pocketstation::graph::signal::continuity::SignalContinuityError` | `RecoveryWithoutDiscontinuity` | unknown | unknown | `src/graph/signal/continuity.rs:105` |
| error-28d03b0e892f47f2b948 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `NetworkPermissionMismatch` | unknown | unknown | `src/graph/signal/operator.rs:337` |
| error-2b03bbb58bb17d9482da | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `UnknownMessageKind` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:304` |
| error-2db114f654fd6a04b5c2 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `InvalidOutputSignal` | unknown | unknown | `src/graph/signal/operator.rs:355` |
| error-2eeea9820e7888cc5de4 | `pocketstation::graph::compile::resolve::CompileError` | `SignalMismatch` | unknown | unknown | `src/graph/compile/resolve.rs:47` |
| error-306338233ac4dcdb29af | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `RealtimePartition` | unknown | unknown | `src/graph/signal/operator.rs:333` |
| error-34e4957a736a942f80e3 | `pocketstation::graph::compile::resolve::CompileError` | `UnknownNodeType` | unknown | unknown | `src/graph/compile/resolve.rs:28` |
| error-3636f110b3c505b0fc87 | `pocketstation::runtime::audio::executor::ExecError` | `Node` | unknown | unknown | `src/runtime/audio/executor.rs:22` |
| error-3a3e737bfe0585596712 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `ProcessingTimeout` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:717` |
| error-3cd909d07c85a305113b | `pocketstation::graph::signal::spec::SignalSpecError` | `EmptyRole` | unknown | unknown | `src/graph/signal/spec.rs:355` |
| error-3dbf0292e22bf7695a5b | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `UnexpectedMessage` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:710` |
| error-3e94a7d48aef8ed4220b | `pocketstation::graph::node::NodeError` | `ProcessTimeout` | unknown | unknown | `src/graph/node.rs:155` |
| error-3f7304dbb0de0fe37726 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `InvalidDataKind` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:724` |
| error-3fbce6034564f1a51e83 | `pocketstation::runtime::audio::runner::PlanRunnerError` | `ZeroSourceCapacity` | unknown | unknown | `src/runtime/audio/runner.rs:258` |
| error-42bdc2f2f6593846eeb6 | `pocketstation::graph::ports::PortSpecError` | type | unknown | unknown | `src/graph/ports.rs:239` |
| error-4c396f1ad9633a15e4c4 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `UnknownSidecar` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:732` |
| error-4cd7a5440dde80383b2e | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | type | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:686` |
| error-4ea4cd0f2c783fb588f7 | `pocketstation::graph::signal::envelope::SignalEnvelopeError` | `SourceMismatch` | unknown | unknown | `src/graph/signal/envelope.rs:145` |
| error-4f7cc30b74223a1354c0 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Protocol` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:698` |
| error-51f341e5e95d92745cc7 | `pocketstation::runtime::audio::runner::PlanRunnerError` | `DuplicateSource` | unknown | unknown | `src/runtime/audio/runner.rs:260` |
| error-5286121f9c3fbfe5e9ac | `pocketstation::graph::signal::continuity::SignalContinuityError` | `GenerationRegressed` | unknown | unknown | `src/graph/signal/continuity.rs:103` |
| error-54f4584702054e08e6ae | `pocketstation::graph::node::NodeDescriptorError` | type | unknown | unknown | `src/graph/node.rs:252` |
| error-56b7234805197b00e9a8 | `pocketstation::graph::signal::continuity::SignalContinuityError` | `PolicyRegressed` | unknown | unknown | `src/graph/signal/continuity.rs:107` |
| error-56b76ee666f183f18d1c | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `InvalidConfiguration` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:688` |
| error-58f0c73713233a07f939 | `pocketstation::graph::node::NodeError` | `Config` | unknown | unknown | `src/graph/node.rs:161` |
| error-59c0b276f329f504019c | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `ControlQueueFull` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:704` |
| error-5b268064d2e22e35a7c0 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `OutputSignalMediaMismatch` | unknown | unknown | `src/graph/signal/operator.rs:359` |
| error-603c220a0e3fb059e46a | `pocketstation::graph::ports::PortSpecError` | `EmptyName` | unknown | unknown | `src/graph/ports.rs:241` |
| error-61c6e209cbf4397817b0 | `pocketstation::graph::node::NodeDescriptorError` | `PortDirectionMismatch` | unknown | unknown | `src/graph/node.rs:260` |
| error-626edd9c657f8ffd0e25 | `pocketstation::graph::node::NodeError` | `Prepare` | unknown | unknown | `src/graph/node.rs:151` |
| error-63a6455a10f1d6924948 | `pocketstation::graph::signal::spec::SignalSpecError` | `EmptySchema` | unknown | unknown | `src/graph/signal/spec.rs:357` |
| error-64b40c037850805370f3 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `UnexpectedEof` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:708` |
| error-653bd7812d7fe83c3da8 | `pocketstation::graph::plan::PlanError` | `MissingEdgeContract` | unknown | unknown | `src/graph/plan.rs:27` |
| error-6b989a5c2a96463f3bee | `pocketstation::runtime::audio::runner::PlanRunnerError` | `AlreadyFinished` | unknown | unknown | `src/runtime/audio/runner.rs:264` |
| error-6d1ec1127908a9ea0c38 | `pocketstation::graph::signal::lineage::SignalLineageError` | `ZeroSourceGeneration` | unknown | unknown | `src/graph/signal/lineage.rs:88` |
| error-7185d783b565af22876f | `pocketstation::graph::registry::NodeRegistrationError` | type | unknown | unknown | `src/graph/registry.rs:57` |
| error-74ad5949b81a0dc23687 | `pocketstation::graph::compile::resolve::CompileError` | `CycleDetected` | unknown | unknown | `src/graph/compile/resolve.rs:60` |
| error-799377810d3c5799f2f8 | `pocketstation::graph::node::NodeDescriptorError` | `DuplicatePort` | unknown | unknown | `src/graph/node.rs:262` |
| error-79e0585ab28e7200b4bb | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `EmptyOperatorId` | unknown | unknown | `src/graph/signal/operator.rs:323` |
| error-7af39dc8190c4a6e3b46 | `pocketstation::graph::signal::continuity::SignalContinuityError` | `TimestampRegression` | unknown | unknown | `src/graph/signal/continuity.rs:99` |
| error-7c61de949e6f7c062440 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | type | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:292` |
| error-7f0c0aa2821caae1334f | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `DuplicateOutputRole` | unknown | unknown | `src/graph/signal/operator.rs:363` |
| error-7fd6796c326d3ebfb732 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `UnsupportedBackpressure` | unknown | unknown | `src/graph/signal/operator.rs:343` |
| error-8455c52b7405c60c7c37 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `MissingInputPort` | unknown | unknown | `src/graph/signal/operator.rs:339` |
| error-859b642b9c4c40e0c7b0 | `pocketstation::graph::signal::spec::SignalSpecError` | `EmptyCustomId` | unknown | unknown | `src/graph/signal/spec.rs:353` |
| error-85a163b5d3dbc22b4866 | `pocketstation::graph::compile::resolve::CompileError` | `InvalidConfig` | unknown | unknown | `src/graph/compile/resolve.rs:30` |
| error-86166fdee8a4d41c5609 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | type | unknown | unknown | `src/graph/signal/operator.rs:321` |
| error-8c217ffd4dacafb9276f | `pocketstation::graph::compile::resolve::CompileError` | `ClockDomainMismatch` | unknown | unknown | `src/graph/compile/resolve.rs:38` |
| error-8c284a4a0efd542eb004 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `FrameTooLarge` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:700` |
| error-8c706102b34fa25f3815 | `pocketstation::graph::signal::lineage::SignalDerivationError` | type | unknown | unknown | `src/graph/signal/lineage.rs:159` |
| error-8d44c749b736491e2485 | `pocketstation::runtime::audio::runner::PlanRunnerError` | `ZeroWorkBudget` | unknown | unknown | `src/runtime/audio/runner.rs:262` |
| error-8e93353612bc514e4e07 | `pocketstation::graph::signal::envelope::SignalEnvelopeError` | `PayloadSpecMismatch` | unknown | unknown | `src/graph/signal/envelope.rs:141` |
| error-9039c359ed36e4f9c562 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `ZeroQueueCapacity` | unknown | unknown | `src/graph/signal/operator.rs:329` |
| error-94160f6a05c87ae3be16 | `pocketstation::graph::signal::continuity::SignalContinuityError` | type | unknown | unknown | `src/graph/signal/continuity.rs:89` |
| error-94cb93e0cfde0d31ea6b | `pocketstation::graph::ports::PortSpecError` | `SignalMediaMismatch` | unknown | unknown | `src/graph/ports.rs:245` |
| error-967638890231d868a30e | `pocketstation::graph::registry::NodeRegistrationError` | `InvalidAsyncManifest` | unknown | unknown | `src/graph/registry.rs:59` |
| error-9a16cba937d081d7dbc5 | `pocketstation::graph::compile::resolve::CompileError` | `InvalidRealtimeEdge` | unknown | unknown | `src/graph/compile/resolve.rs:58` |
| error-9ba468b27c25e5eb7e82 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `AlreadyReaped` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:730` |
| error-9d1d28353aeed3c35e56 | `pocketstation::graph::signal::envelope::SignalEnvelopeError` | `SequenceMismatch` | unknown | unknown | `src/graph/signal/envelope.rs:143` |
| error-9d624a622d7040a1fe0f | `pocketstation::runtime::audio::runner::PlanRunnerError` | type | unknown | unknown | `src/runtime/audio/runner.rs:256` |
| error-a13edd0adb172f70699e | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Spawn` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:690` |
| error-a40a6a70ccfabb71722d | `pocketstation::graph::node::NodeError` | type | unknown | unknown | `src/graph/node.rs:149` |
| error-a40cc696fa1557b8b562 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `MissingPipe` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:694` |
| error-a437958ed452bc7066fb | `pocketstation::graph::signal::operator::OperatorFailurePolicy` | type | unknown | unknown | `src/graph/signal/operator.rs:63` |
| error-a984218b068597ea76b4 | `pocketstation::runtime::audio::runner::PlanRunnerError` | `Execution` | unknown | unknown | `src/runtime/audio/runner.rs:266` |
| error-aa1faf9d48551be8e857 | `pocketstation::graph::signal::operator::OperatorFailurePolicy` | `Continue` | unknown | unknown | `src/graph/signal/operator.rs:64` |
| error-acee01765fe764bfd55c | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `InvalidState` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:719` |
| error-ad10e5c05c58ef45163e | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `InputSignalMediaMismatch` | unknown | unknown | `src/graph/signal/operator.rs:357` |
| error-ae3ce028fb5d1b747b4f | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Timeout` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:715` |
| error-ae9c108957e87a432388 | `pocketstation::graph::plan::PlanError` | `FanInOnSinglePort` | unknown | unknown | `src/graph/plan.rs:23` |
| error-b0bb2cce36e6f3a9aa85 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `UnsupportedMajor` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:300` |
| error-b36b236eb4981b0ab39d | `pocketstation::graph::signal::envelope::SignalEnvelopeError` | `InvalidSignalSpec` | unknown | unknown | `src/graph/signal/envelope.rs:139` |
| error-b50659f2a5cf60658994 | `pocketstation::graph::compile::resolve::CompileError` | `UnknownNode` | unknown | unknown | `src/graph/compile/resolve.rs:32` |
| error-b52af948f021ce508485 | `pocketstation::graph::signal::lineage::SignalDerivationError` | `ZeroOperatorVersion` | unknown | unknown | `src/graph/signal/lineage.rs:165` |
| error-b69082720fc106b6e49f | `pocketstation::graph::signal::envelope::SignalEnvelopeError` | type | unknown | unknown | `src/graph/signal/envelope.rs:137` |
| error-c11beeb8e6aa548db6bf | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `EmptySignalId` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:310` |
| error-c2615c3982620c9f1187 | `pocketstation::graph::ports::PortSpecError` | `InvalidSignal` | unknown | unknown | `src/graph/ports.rs:243` |
| error-c8cd17c7fd72e321724b | `pocketstation::graph::compile::resolve::CompileError` | `InvalidSafetyContract` | unknown | unknown | `src/graph/compile/resolve.rs:51` |
| error-c8da30f10b161f3331fa | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `Truncated` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:294` |
| error-c9af3416556986aeaae3 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `TerminalOutputRoleNotAllowed` | unknown | unknown | `src/graph/signal/operator.rs:365` |
| error-cf7b83db4172d742f393 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `InvalidSafetyContract` | unknown | unknown | `src/graph/signal/operator.rs:335` |
| error-d1ab504acc86e7199ea2 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `ZeroGeneration` | unknown | unknown | `src/graph/signal/operator.rs:327` |
| error-d1c44816ab5c6bbb5dd3 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `TrailingBytes` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:296` |
| error-d2b53989cb5e8e167f9c | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `FrameTooLarge` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:322` |
| error-d2dd9157fa8f288e23b3 | `pocketstation::graph::signal::timing::SignalTimingError` | `TimestampOverflow` | unknown | unknown | `src/graph/signal/timing.rs:93` |
| error-d2de5e77aafafa864140 | `pocketstation::graph::node::NodeError` | `ExternalBoundaryExecution` | unknown | unknown | `src/graph/node.rs:159` |
| error-d62f3706b721c2ef88e3 | `pocketstation::graph::signal::lineage::SignalLineageError` | type | unknown | unknown | `src/graph/signal/lineage.rs:86` |
| error-d86392ce579eb45b3db1 | `pocketstation::graph::signal::spec::SignalSpecError` | type | unknown | unknown | `src/graph/signal/spec.rs:351` |
| error-dac9e65b096b9b1d3663 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `UnsupportedOutputBackpressure` | unknown | unknown | `src/graph/signal/operator.rs:347` |
| error-dafcbf188eb0f57e0c65 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Closed` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:706` |
| error-db489334cd6e93d6e506 | `pocketstation::graph::signal::timing::SignalTimingError` | type | unknown | unknown | `src/graph/signal/timing.rs:89` |
| error-dc43dd381c51c5f53ba3 | `pocketstation::graph::compile::resolve::CompileError` | `UnknownPort` | unknown | unknown | `src/graph/compile/resolve.rs:34` |
| error-ddecdc2a0f765f18a9a6 | `pocketstation::graph::node::ConfigError` | `Invalid` | unknown | unknown | `src/graph/node.rs:145` |
| error-def6f087edab3cbe65d3 | `pocketstation::graph::plan::PlanError` | type | unknown | unknown | `src/graph/plan.rs:21` |
| error-dfb29737d96f7524858d | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `UnsupportedInputCopyPolicy` | unknown | unknown | `src/graph/signal/operator.rs:345` |
| error-e0093a3c3b8e27256dd2 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `UnsupportedMinor` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:302` |
| error-e2660b145ff9db8787dc | `pocketstation::graph::compile::resolve::CompileError` | `AdapterUnavailable` | unknown | unknown | `src/graph/compile/resolve.rs:62` |
| error-e2b62436b7a240898e91 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `MissingOutputPort` | unknown | unknown | `src/graph/signal/operator.rs:341` |
| error-e4672d7e92fa5cb1d206 | `pocketstation::runtime::audio::executor::ExecError` | type | unknown | unknown | `src/runtime/audio/executor.rs:20` |
| error-e5d0b8eacb45ae5e003a | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Kill` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:728` |
| error-e5e6f48e4c021e878369 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `ZeroRevision` | unknown | unknown | `src/graph/signal/operator.rs:325` |
| error-e6deb2150f219ce5b4b1 | `pocketstation::graph::signal::lineage::SignalDerivationError` | `EmptyOperatorId` | unknown | unknown | `src/graph/signal/lineage.rs:163` |
| error-e8fc6628b0533a37e582 | `pocketstation::graph::plan::PlanError` | `MissingOutputSignal` | unknown | unknown | `src/graph/plan.rs:29` |
| error-e946903c6c6cbce7e693 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `EmptyOutputRole` | unknown | unknown | `src/graph/signal/operator.rs:361` |
| error-ebf00c87f66861e59d16 | `pocketstation::graph::compile::resolve::CompileError` | `WrongPortDirection` | unknown | unknown | `src/graph/compile/resolve.rs:36` |
| error-eed6152ba9747eb89a61 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | `InvalidInputSignal` | unknown | unknown | `src/graph/signal/operator.rs:353` |
| error-eee3fd27c0031552e3e9 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Io` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:696` |
| error-efbb8501a0345d5c733e | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `FieldTooLarge` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:312` |
| error-f1dd5dd27bc97a5148cf | `pocketstation::graph::signal::continuity::SignalContinuityError` | `DiscontinuityRegressed` | unknown | unknown | `src/graph/signal/continuity.rs:101` |
| error-f257d5d9cd7c4c2da0a3 | `pocketstation::graph::node::NodeDescriptorError` | `EmptyTypeId` | unknown | unknown | `src/graph/node.rs:254` |
| error-f662eed9eaef230d5e97 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `ThreadSpawn` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:692` |
| error-f7a063c20f98bd14eb84 | `pocketstation::graph::registry::NodeRegistrationError` | `DuplicateOperatorId` | unknown | unknown | `src/graph/registry.rs:63` |
| error-f905d948900c2917309f | `pocketstation::graph::signal::continuity::SignalContinuityError` | `MissingLineage` | unknown | unknown | `src/graph/signal/continuity.rs:93` |
| error-fbecdc2096c9e93a9462 | `pocketstation::graph::signal::lineage::SignalDerivationError` | `InvalidTimestampRange` | unknown | unknown | `src/graph/signal/lineage.rs:161` |
| error-fd6557db957f52eb4959 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `DataQueueFull` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:702` |
| error-ff0c5d9bc0446db9f66b | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `InvalidUtf8` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:318` |
| error-ff20230354eeef68eea7 | `pocketstation::graph::signal::timing::SignalTimingError` | `ZeroDuration` | unknown | unknown | `src/graph/signal/timing.rs:91` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Asynchronous signal lane](/docs/internals/async-signal-lane.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/node.rs:1-411` (`DIRECT`)
- `src/runtime/signal/error.rs:1-56` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
