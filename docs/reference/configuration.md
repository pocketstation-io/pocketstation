# Configuration reference

<!-- claims: CLM-REF-013-CAP-001,CLM-REF-013-CAP-002,CLM-REF-013-CAP-003,CLM-REF-013-CAP-004,CLM-REF-013-CAP-005,CLM-REF-013-CAP-006,CLM-REF-013-CAP-007,CLM-REF-013-CAP-008,CLM-REF-013-CAP-009,CLM-REF-013-SOURCE-001,CLM-REF-013-CONFIG-0001,CLM-REF-013-CONFIG-0002,CLM-REF-013-CONFIG-0003,CLM-REF-013-CONFIG-0004,CLM-REF-013-CONFIG-0005,CLM-REF-013-CONFIG-0006,CLM-REF-013-CONFIG-0007,CLM-REF-013-CONFIG-0008,CLM-REF-013-CONFIG-0009,CLM-REF-013-CONFIG-0010,CLM-REF-013-CONFIG-0011,CLM-REF-013-CONFIG-0012,CLM-REF-013-CONFIG-0013,CLM-REF-013-CONFIG-0014,CLM-REF-013-CONFIG-0015,CLM-REF-013-CONFIG-0016,CLM-REF-013-CONFIG-0017,CLM-REF-013-CONFIG-0018,CLM-REF-013-CONFIG-0019,CLM-REF-013-CONFIG-0020,CLM-REF-013-CONFIG-0021,CLM-REF-013-CONFIG-0022,CLM-REF-013-CONFIG-0023,CLM-REF-013-CONFIG-0024,CLM-REF-013-CONFIG-0025,CLM-REF-013-CONFIG-0026,CLM-REF-013-CONFIG-0027,CLM-REF-013-CONFIG-0028,CLM-REF-013-CONFIG-0029,CLM-REF-013-CONFIG-0030,CLM-REF-013-CONFIG-0031,CLM-REF-013-CONFIG-0032,CLM-REF-013-CONFIG-0033,CLM-REF-013-CONFIG-0034,CLM-REF-013-CONFIG-0035,CLM-REF-013-CONFIG-0036,CLM-REF-013-CONFIG-0037,CLM-REF-013-CONFIG-0038,CLM-REF-013-CONFIG-0039,CLM-REF-013-CONFIG-0040,CLM-REF-013-CONFIG-0041,CLM-REF-013-CONFIG-0042,CLM-REF-013-CONFIG-0043,CLM-REF-013-CONFIG-0044,CLM-REF-013-CONFIG-0045,CLM-REF-013-CONFIG-0046,CLM-REF-013-CONFIG-0047,CLM-REF-013-CONFIG-0048,CLM-REF-013-CONFIG-0049,CLM-REF-013-CONFIG-0050,CLM-REF-013-CONFIG-0051,CLM-REF-013-CONFIG-0052,CLM-REF-013-CONFIG-0053,CLM-REF-013-CONFIG-0054,CLM-REF-013-CONFIG-0055,CLM-REF-013-CONFIG-0056,CLM-REF-013-CONFIG-0057,CLM-REF-013-CONFIG-0058,CLM-REF-013-CONFIG-0059,CLM-REF-013-CONFIG-0060,CLM-REF-013-CONFIG-0061,CLM-REF-013-CONFIG-0062,CLM-REF-013-CONFIG-0063,CLM-REF-013-CONFIG-0064,CLM-REF-013-CONFIG-0065,CLM-REF-013-CONFIG-0066,CLM-REF-013-CONFIG-0067,CLM-REF-013-CONFIG-0068,CLM-REF-013-CONFIG-0069,CLM-REF-013-CONFIG-0070,CLM-REF-013-CONFIG-0071,CLM-REF-013-CONFIG-0072,CLM-REF-013-CONFIG-0073,CLM-REF-013-CONFIG-0074,CLM-REF-013-CONFIG-0075,CLM-REF-013-CONFIG-0076,CLM-REF-013-CONFIG-0077,CLM-REF-013-CONFIG-0078,CLM-REF-013-CONFIG-0079,CLM-REF-013-CONFIG-0080,CLM-REF-013-CONFIG-0081,CLM-REF-013-CONFIG-0082,CLM-REF-013-CONFIG-0083,CLM-REF-013-CONFIG-0084,CLM-REF-013-CONFIG-0085,CLM-REF-013-CONFIG-0086,CLM-REF-013-CONFIG-0087,CLM-REF-013-CONFIG-0088,CLM-REF-013-CONFIG-0089,CLM-REF-013-CONFIG-0090,CLM-REF-013-CONFIG-0091,CLM-REF-013-CONFIG-0092,CLM-REF-013-CONFIG-0093,CLM-REF-013-CONFIG-0094,CLM-REF-013-CONFIG-0095,CLM-REF-013-CONFIG-0096,CLM-REF-013-CONFIG-0097,CLM-REF-013-CONFIG-0098,CLM-REF-013-CONFIG-0099,CLM-REF-013-CONFIG-0100,CLM-REF-013-CONFIG-0101,CLM-REF-013-CONFIG-0102,CLM-REF-013-CONFIG-0103,CLM-REF-013-CONFIG-0104,CLM-REF-013-CONFIG-0105,CLM-REF-013-CONFIG-0106,CLM-REF-013-CONFIG-0107,CLM-REF-013-CONFIG-0108,CLM-REF-013-CONFIG-0109,CLM-REF-013-CONFIG-0110,CLM-REF-013-CONFIG-0111,CLM-REF-013-CONFIG-0112,CLM-REF-013-CONFIG-0113,CLM-REF-013-CONFIG-0114,CLM-REF-013-CONFIG-0115,CLM-REF-013-CONFIG-0116,CLM-REF-013-CONFIG-0117,CLM-REF-013-CONFIG-0118,CLM-REF-013-CONFIG-0119,CLM-REF-013-CONFIG-0120,CLM-REF-013-CONFIG-0121,CLM-REF-013-CONFIG-0122,CLM-REF-013-CONFIG-0123,CLM-REF-013-CONFIG-0124,CLM-REF-013-CONFIG-0125,CLM-REF-013-CONFIG-0126,CLM-REF-013-CONFIG-0127,CLM-REF-013-CONFIG-0128,CLM-REF-013-CONFIG-0129,CLM-REF-013-CONFIG-0130,CLM-REF-013-CONFIG-0131,CLM-REF-013-CONFIG-0132,CLM-REF-013-CONFIG-0133,CLM-REF-013-CONFIG-0134,CLM-REF-013-CONFIG-0135 -->

## Scope

- **Install and feature-select the crate.** Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.
- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.
- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Run connector workers.** Supervise connector delivery, acknowledgement, retry budgets, readiness, cancellation, drain, and abort.
- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.
- **Inject external PCM.** Acquire bounded input buffers and write externally produced PCM through the source extension lifecycle.
- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Configuration inventory

| Evidence record | Kind | Name | Parent | Default | When read | Source |
|---|---|---|---|---|---|---|
| cfg-010150fb6e2edcc18ac7 | variant | `MoveExclusive` | `pocketstation::graph::ports::CopyPolicy` | unknown | unknown | `src/graph/ports.rs:281` |
| cfg-0370b7ecbdf2b9d6fbdb | variant | `ZeroLeaseCapacity` | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:46` |
| cfg-05708ca7c54f6f84a3c7 | struct_field | `process_timeout_ms` | `pocketstation::graph::signal::operator::OperatorDeadlinePolicy` | unknown | unknown | `src/graph/signal/operator.rs:53` |
| cfg-09064704b028bca0e197 | variant | `NonEmpty` | `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | unknown | unknown | `src/connector/configuration.rs:160` |
| cfg-0937f629a131470e2b2a | variant | `BoundedQueue` | `pocketstation::graph::ports::BackpressurePolicy` | unknown | unknown | `src/graph/ports.rs:268` |
| cfg-0bed26cd5cd9ccfe0b20 | variant | `QueueCapacityTooLarge` | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:48` |
| cfg-0c83ebde568152ad3edf | variant | `TooManyFields` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:571` |
| cfg-0debc438cdb0c0980588 | variant | `ConcealForAudio` | `pocketstation::graph::ports::LossPolicy` | unknown | unknown | `src/graph/ports.rs:288` |
| cfg-0fa3fff5510ff5006c01 | variant | `Allowed` | `pocketstation::capture::authorization::ApplicationPolicyObservation` | unknown | unknown | `src/capture/authorization.rs:232` |
| cfg-1190f8cd716a82590696 | variant | `ApplicationIdentity` | `pocketstation::capture::selection::SelectorPersistenceScope` | unknown | unknown | `src/capture/selection.rs:75` |
| cfg-13aec77d5ad5fbec7cd1 | variant | `UnsignedInteger` | `pocketstation::connector::configuration::ConnectorConfigurationValue` | unknown | unknown | `src/connector/configuration.rs:70` |
| cfg-14ca51fa44623142d004 | variant | `StopWorker` | `pocketstation::graph::signal::operator::OperatorFailurePolicy` | unknown | unknown | `src/graph/signal/operator.rs:65` |
| cfg-1555cc3d0e570c98edd4 | cargo_feature | `internal-testing` | `Cargo.toml [features]` | False | Cargo feature resolution | `Cargo.toml:1` |
| cfg-16e1a7cde0e4081ed34c | environment_variable | `PKS_FIXTURE_MARKER` | `None` | unknown | See source evidence. | `tests/fixtures/native_extension_plugin.rs:230` |
| cfg-16fe034657303e4973f8 | variant | `InvalidValue` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:575` |
| cfg-1b61cae5d42097d0bbe9 | variant | `CopyToBranchPool` | `pocketstation::graph::ports::CopyPolicy` | unknown | unknown | `src/graph/ports.rs:283` |
| cfg-1d6a796d4c71e9531054 | variant | `ProcessId` | `pocketstation::session::declaration::selector::ApplicationSelector` | unknown | unknown | `src/session/declaration/selector.rs:34` |
| cfg-1f87349ce093ac3486f9 | struct_field | `maximum_samples_per_channel` | `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | unknown | unknown | `src/codec/decoder.rs:31` |
| cfg-1fb4b2d84a6cf23abbd9 | variant | `Missing` | `pocketstation::graph::node::ConfigError` | unknown | unknown | `src/graph/node.rs:143` |
| cfg-20e58c6bbc3ac729a8e8 | variant | `ValueTooLarge` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:577` |
| cfg-22ab1e3ede7ae88d56fb | variant | `NotApplicable` | `pocketstation::capture::authorization::ApplicationPolicyObservation` | unknown | unknown | `src/capture/authorization.rs:235` |
| cfg-277a12ba2783a1157b60 | variant | `DurationMilliseconds` | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | unknown | unknown | `src/connector/configuration.rs:60` |
| cfg-293d1af27f9bef772cfd | struct_field | `fec` | `pocketstation::codec::encoder::OpusConfig` | unknown | unknown | `src/codec/encoder.rs:88` |
| cfg-2f295a051ff6d0366ead | variant | `WrongType` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:574` |
| cfg-31f95125213e6a48d677 | variant | `UnsignedRange` | `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | unknown | unknown | `src/connector/configuration.rs:163` |
| cfg-32d2c24be1978d72f328 | variant | `Name` | `pocketstation::session::declaration::selector::ApplicationSelector` | unknown | unknown | `src/session/declaration/selector.rs:40` |
| cfg-33641f2a5bd566f77719 | variant | `Required` | `pocketstation::connector::configuration::ConnectorConfigurationRequirement` | unknown | unknown | `src/connector/configuration.rs:153` |
| cfg-340d86a0676ab674a6ba | struct_field | `network_allowed` | `pocketstation::graph::signal::operator::OperatorPermissionPolicy` | unknown | unknown | `src/graph/signal/operator.rs:47` |
| cfg-36317bb0c5b73c9f66a7 | variant | `UnsignedInteger` | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | unknown | unknown | `src/connector/configuration.rs:59` |
| cfg-37775f819a84416494a5 | variant | `UnknownField` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:572` |
| cfg-3a6293f360bae614e017 | variant | `ByteCount` | `pocketstation::connector::configuration::ConnectorConfigurationValue` | unknown | unknown | `src/connector/configuration.rs:72` |
| cfg-3c8c0ec79438df066bec | environment_variable | `CARGO_FEATURE_NATIVE_CAPTURE` | `None` | unknown | See source evidence. | `build.rs:4` |
| cfg-3eab56c3c6b9ed63c514 | variant | `Default` | `pocketstation::capture::selection::InputDeviceSelector` | unknown | unknown | `src/capture/selection.rs:11` |
| cfg-3f3d02bb69a74d3a07ef | variant | `UnsupportedMajor` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:259` |
| cfg-4055838a830f20f7900a | variant | `InvalidDeadline` | `pocketstation::connector::readiness::ConnectorReadinessPolicyError` | unknown | unknown | `src/connector/readiness.rs:63` |
| cfg-4188b0e503f060db1c19 | variant | `ZeroFrameSamples` | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:87` |
| cfg-4196c60a023b5f4847c7 | struct_field | `complexity` | `pocketstation::codec::encoder::OpusConfig` | unknown | unknown | `src/codec/encoder.rs:84` |
| cfg-4224a0310fdf71575265 | environment_variable | `CARGO_MANIFEST_DIR` | `None` | unknown | See source evidence. | `build.rs:17` |
| cfg-477559f268cb84014843 | variant | `NotObservable` | `pocketstation::capture::authorization::ApplicationPolicyObservation` | unknown | unknown | `src/capture/authorization.rs:234` |
| cfg-47f70a6abf2181acf9af | variant | `PlatformIdentity` | `pocketstation::capture::selection::SelectorPersistenceScope` | unknown | unknown | `src/capture/selection.rs:78` |
| cfg-48b73c8939c692658460 | variant | `SignedRange` | `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | unknown | unknown | `src/connector/configuration.rs:162` |
| cfg-48e2a245a9a12165631c | environment_variable | `CXX` | `None` | unknown | See source evidence. | `tests/abi_codec_cpp_conformance.rs:20` |
| cfg-4a046ab28843a5b0e7da | variant | `MissingRequiredField` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:573` |
| cfg-4a51fb66fefffbe50611 | variant | `TrailingBytes` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:255` |
| cfg-4a59953777f27b46a3a1 | cargo_feature | `default` | `Cargo.toml [features]` | False | Cargo feature resolution | `Cargo.toml:1` |
| cfg-4cdf00648ce1c4ceed37 | variant | `DiscardQueued` | `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | unknown | unknown | `src/graph/signal/operator.rs:58` |
| cfg-501a697b4d9bbef1cee1 | variant | `Id` | `pocketstation::session::declaration::selector::DeviceSelector` | unknown | unknown | `src/session/declaration/selector.rs:109` |
| cfg-5175539560395b69dd7b | variant | `BatchCapacityTooLarge` | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:50` |
| cfg-51f1055ca3a98170c561 | variant | `OneOf` | `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | unknown | unknown | `src/connector/configuration.rs:164` |
| cfg-52b5f991baa9ec6301d9 | struct_field | `channels` | `pocketstation::codec::encoder::OpusConfig` | unknown | unknown | `src/codec/encoder.rs:76` |
| cfg-54d4db6cef98faa6c28b | environment_variable | `OUT_DIR` | `None` | unknown | See source evidence. | `build.rs:59` |
| cfg-571a8dd79ebc75a2a9d4 | variant | `UnsupportedSampleFormat` | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:83` |
| cfg-5b102543d9499772995c | variant | `LengthOverflow` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:277` |
| cfg-5dc3b2a2eb52adf55084 | variant | `Optional` | `pocketstation::connector::configuration::ConnectorConfigurationRequirement` | unknown | unknown | `src/connector/configuration.rs:154` |
| cfg-5dffe906591cdbdacc27 | variant | `BlockForbidden` | `pocketstation::graph::ports::BackpressurePolicy` | unknown | unknown | `src/graph/ports.rs:269` |
| cfg-5e6d785c8002aad9fce3 | variant | `Text` | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | unknown | unknown | `src/connector/configuration.rs:56` |
| cfg-5ffb124216b1e484ea21 | struct_field | `dtx` | `pocketstation::codec::encoder::OpusConfig` | unknown | unknown | `src/codec/encoder.rs:86` |
| cfg-602ce05411bc22202de7 | variant | `ShareReadOnly` | `pocketstation::graph::ports::CopyPolicy` | unknown | unknown | `src/graph/ports.rs:282` |
| cfg-6254ee70cb9d96a46039 | struct_field | `requested_samples_per_channel` | `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | unknown | unknown | `src/codec/decoder.rs:30` |
| cfg-6a12319cb2ab6134781d | variant | `InvalidValue` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:273` |
| cfg-7312c7b8cdcf2cdc6882 | cargo_feature | `native-capture` | `Cargo.toml [features]` | True | Cargo feature resolution | `Cargo.toml:1` |
| cfg-75f560f2ded0bc32418e | struct_field | `terminal` | `pocketstation::graph::signal::operator::OperatorOutputRolePolicy` | unknown | unknown | `src/graph/signal/operator.rs:71` |
| cfg-7b77c29136d86b5835f6 | variant | `ValueTooLarge` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:271` |
| cfg-7bc8e4b962270019df55 | struct_field | `reason` | `pocketstation::session::lifecycle::engine::SessionEngineBuildError::InvalidConfiguration` | unknown | unknown | `src/session/lifecycle/engine.rs:299` |
| cfg-7c441b312ac457daa601 | variant | `SessionDefaultDevice` | `pocketstation::capture::selection::SelectorPersistenceScope` | unknown | unknown | `src/capture/selection.rs:77` |
| cfg-7e08e2a1669705da5b62 | variant | `InvalidFieldName` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:267` |
| cfg-7f4f2c63d7d9f232f4f5 | variant | `UnknownValueKind` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:275` |
| cfg-803a13e7634e2376715b | variant | `Denied` | `pocketstation::capture::authorization::ApplicationPolicyObservation` | unknown | unknown | `src/capture/authorization.rs:233` |
| cfg-804ebd8972ec6dc0a6d8 | variant | `DeviceIdentity` | `pocketstation::capture::selection::SelectorPersistenceScope` | unknown | unknown | `src/capture/selection.rs:76` |
| cfg-8a220d80818a0480df24 | variant | `DropNewest` | `pocketstation::graph::ports::BackpressurePolicy` | unknown | unknown | `src/graph/ports.rs:266` |
| cfg-8ce3127c1900c78e04d3 | environment_variable | `CC` | `None` | unknown | See source evidence. | `tests/abi_session_c_conformance.rs:14` |
| cfg-8d128d781276e9bbfef8 | variant | `InvalidCapacity` | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:85` |
| cfg-9004498127de07b8f6f2 | variant | `MustDeliverOrFail` | `pocketstation::graph::ports::LossPolicy` | unknown | unknown | `src/graph/ports.rs:289` |
| cfg-923552100e4677c87ee9 | variant | `SignedInteger` | `pocketstation::connector::configuration::ConnectorConfigurationValue` | unknown | unknown | `src/connector/configuration.rs:69` |
| cfg-938eedc4e68f764a4008 | environment_variable | `PKS_WRITE_AUDIO_ARTIFACTS` | `None` | unknown | See source evidence. | `src/codec/encoder.rs:818` |
| cfg-9a6ebc28dfef4225fe4d | environment_variable | `SDKROOT` | `None` | unknown | See source evidence. | `build.rs:69` |
| cfg-9c43e47d49a4ea92c64d | environment_variable | `CARGO_PKG_VERSION` | `None` | unknown | See source evidence. | `examples/connector_authoring.rs:127` |
| cfg-9db01b1164f402cb50a1 | variant | `ZeroQueueCapacity` | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:42` |
| cfg-9dcaf60d25aa54bbf58d | struct_field | `reason` | `pocketstation::session::error::SessionError::InvalidSelector` | unknown | unknown | `src/session/error.rs:23` |
| cfg-9ebf2d2509e9920eb42d | variant | `ZeroBatchCapacity` | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:44` |
| cfg-a39d88fad5eeb0f5d3f1 | variant | `StableId` | `pocketstation::session::declaration::selector::ApplicationSelector` | unknown | unknown | `src/session/declaration/selector.rs:39` |
| cfg-a3a1c8010d05ad11da72 | variant | `UnsupportedMinor` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:261` |
| cfg-a40b6282bc45f876422f | variant | `Secret` | `pocketstation::connector::configuration::ConnectorConfigurationValue` | unknown | unknown | `src/connector/configuration.rs:73` |
| cfg-a4f1ae1c02a4ffc4a140 | struct_field | `bitrate_kbps` | `pocketstation::codec::encoder::OpusConfig` | unknown | unknown | `src/codec/encoder.rs:82` |
| cfg-a730e20559890b14a1c2 | variant | `Truncated` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:253` |
| cfg-a753ff62a72a421ed184 | variant | `SecretDefaultForbidden` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:579` |
| cfg-a792b5bd178e4ad992af | struct_field | `source_type_id` | `pocketstation::session::compile::error::SessionCompileError::InvalidExternalSourceConfiguration` | unknown | unknown | `src/session/compile/error.rs:79` |
| cfg-a8e458a7b1123416885c | variant | `ConstraintViolation` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:576` |
| cfg-a943a35874778e86e36e | struct_field | `type_id` | `pocketstation::graph::compile::resolve::CompileError::InvalidConfig` | unknown | unknown | `src/graph/compile/resolve.rs:30` |
| cfg-aa1faf9d48551be8e857 | variant | `Continue` | `pocketstation::graph::signal::operator::OperatorFailurePolicy` | unknown | unknown | `src/graph/signal/operator.rs:64` |
| cfg-aa6b719e4e844095490a | variant | `BundleId` | `pocketstation::session::declaration::selector::ApplicationSelector` | unknown | unknown | `src/session/declaration/selector.rs:33` |
| cfg-ab3c5b3a4bd8a6f33fa8 | variant | `Secret` | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | unknown | unknown | `src/connector/configuration.rs:62` |
| cfg-abe4efbaa3a7aaea13b3 | variant | `TooManyFields` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:265` |
| cfg-adf4eaf1e287f1a53b74 | struct_field | `reason` | `pocketstation::session::compile::error::SessionCompileError::InvalidExternalSourceConfiguration` | unknown | unknown | `src/session/compile/error.rs:80` |
| cfg-b070c65b72329d774615 | cargo_feature | `macos-asp-driver-artifact` | `Cargo.toml [features]` | False | Cargo feature resolution | `Cargo.toml:1` |
| cfg-b1bf3a6dd0e8d07b938b | variant | `DurationMilliseconds` | `pocketstation::connector::configuration::ConnectorConfigurationValue` | unknown | unknown | `src/connector/configuration.rs:71` |
| cfg-b1f1d73ecb66ba0745eb | environment_variable | `XDG_RUNTIME_DIR` | `None` | unknown | See source evidence. | `src/capture/platform/linux/pipewire.rs:717` |
| cfg-b81ae01de00a5851018f | variant | `SecretClassificationMismatch` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:580` |
| cfg-b8dada58f97f816869c6 | cargo_feature | `conformance-fixtures` | `Cargo.toml [features]` | False | Cargo feature resolution | `Cargo.toml:1` |
| cfg-ba09fe4bdf521831dbb1 | variant | `DropOldest` | `pocketstation::graph::ports::BackpressurePolicy` | unknown | unknown | `src/graph/ports.rs:267` |
| cfg-ba41b381b4b42f24bf3e | struct_field | `allowed` | `pocketstation::graph::signal::operator::OperatorOutputRolePolicy` | unknown | unknown | `src/graph/signal/operator.rs:70` |
| cfg-bbe53920f4b4f9c2657d | variant | `DrainQueued` | `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | unknown | unknown | `src/graph/signal/operator.rs:59` |
| cfg-bf765fb671636dde32be | variant | `Text` | `pocketstation::connector::configuration::ConnectorConfigurationValue` | unknown | unknown | `src/connector/configuration.rs:67` |
| cfg-bfbdf83d404d15ee43f1 | variant | `Boolean` | `pocketstation::connector::configuration::ConnectorConfigurationValue` | unknown | unknown | `src/connector/configuration.rs:68` |
| cfg-c0121ae98b5fa0e7b031 | variant | `InvalidThreshold` | `pocketstation::connector::readiness::ConnectorReadinessPolicyError` | unknown | unknown | `src/connector/readiness.rs:65` |
| cfg-c13721b6444212b813d5 | struct_field | `application` | `pocketstation::codec::encoder::OpusConfig` | unknown | unknown | `src/codec/encoder.rs:80` |
| cfg-c1a2e0af956fcc7f7c3d | variant | `DuplicateField` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:570` |
| cfg-c3f90f97088db1a8a500 | variant | `InvalidMagic` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:257` |
| cfg-c43e7d721f296ccf4c97 | variant | `UnexpectedSensitiveValue` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:581` |
| cfg-cb5c9b24f5b2d6b9785a | variant | `Default` | `pocketstation::connector::configuration::ConnectorConfigurationRequirement` | unknown | unknown | `src/connector/configuration.rs:155` |
| cfg-cba6815cc4b01927c405 | variant | `SignedInteger` | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | unknown | unknown | `src/connector/configuration.rs:58` |
| cfg-d2c5c2fa400122f5c196 | variant | `ProcessInstance` | `pocketstation::session::declaration::selector::ApplicationSelector` | unknown | unknown | `src/session/declaration/selector.rs:35` |
| cfg-d4964459637bee30cfb6 | variant | `Boolean` | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | unknown | unknown | `src/connector/configuration.rs:57` |
| cfg-d4bfc077ad48eafb7b3d | variant | `StableId` | `pocketstation::capture::selection::InputDeviceSelector` | unknown | unknown | `src/capture/selection.rs:12` |
| cfg-d75924fc2fc0dab06d82 | variant | `DuplicateField` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:269` |
| cfg-d7788e2978fa3f444f84 | struct_field | `reason` | `pocketstation::graph::compile::resolve::CompileError::InvalidConfig` | unknown | unknown | `src/graph/compile/resolve.rs:30` |
| cfg-d8ba815f1c349040eb3c | variant | `FrameSampleCountOverflow` | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:89` |
| cfg-d91f7ed6aee22ea7e388 | variant | `TextLengthBytes` | `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | unknown | unknown | `src/connector/configuration.rs:161` |
| cfg-dae7706a7ab13e3b6838 | variant | `ByteCount` | `pocketstation::connector::configuration::ConnectorConfigurationValueKind` | unknown | unknown | `src/connector/configuration.rs:61` |
| cfg-dd9b0944fab115622a9e | variant | `ZeroSampleRate` | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:79` |
| cfg-ddecdc2a0f765f18a9a6 | variant | `Invalid` | `pocketstation::graph::node::ConfigError` | unknown | unknown | `src/graph/node.rs:145` |
| cfg-de470ce9d79bea914069 | struct_field | `frame_duration` | `pocketstation::codec::encoder::OpusConfig` | unknown | unknown | `src/codec/encoder.rs:78` |
| cfg-dec298f9c52c3cd85e8e | struct_field | `filesystem_allowed` | `pocketstation::graph::signal::operator::OperatorPermissionPolicy` | unknown | unknown | `src/graph/signal/operator.rs:48` |
| cfg-e1d1aef99be9c81bebdf | variant | `ReservedFieldSet` | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | unknown | unknown | `src/connector/transport.rs:263` |
| cfg-e504c315c84e8e9f5177 | variant | `LeaseCapacityTooLarge` | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:52` |
| cfg-e842b9317dd44c0affc4 | variant | `ProcessLifetime` | `pocketstation::capture::selection::SelectorPersistenceScope` | unknown | unknown | `src/capture/selection.rs:74` |
| cfg-eea403e4ffb890f6ba2e | variant | `InvalidSchema` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:569` |
| cfg-eef69b49fe108985c4d4 | variant | `EmptySecret` | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | unknown | unknown | `src/connector/configuration.rs:578` |
| cfg-f2c584f5bb5758a9b87d | environment_variable | `CARGO_FEATURE_MACOS_ASP_DRIVER_ARTIFACT` | `None` | unknown | See source evidence. | `build.rs:7` |
| cfg-f418d25c2781ccf865ba | struct_field | `reason` | `pocketstation::session::lifecycle::start_contract::SessionStartError::InvalidOptions` | unknown | unknown | `src/session/lifecycle/start_contract.rs:115` |
| cfg-f6b776b6d163375cbd02 | variant | `Default` | `pocketstation::session::declaration::selector::DeviceSelector` | unknown | unknown | `src/session/declaration/selector.rs:108` |
| cfg-f859505bd445f3f17654 | variant | `DropAllowed` | `pocketstation::graph::ports::LossPolicy` | unknown | unknown | `src/graph/ports.rs:290` |
| cfg-f8a2b87b7ff8ad7b7f41 | environment_variable | `PKS_TAP_DIAG` | `None` | unknown | See source evidence. | `src/capture/platform/macos/macos_tap.rs:480` |
| cfg-fa274108f6d7414cba40 | struct_field | `sample_rate` | `pocketstation::codec::encoder::OpusConfig` | unknown | unknown | `src/codec/encoder.rs:74` |
| cfg-ffbf1295f7ee96519d1c | variant | `UnsupportedChannelCount` | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:81` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)
- [PocketStation documentation](/docs/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)
- `src/connector/configuration.rs:1-673` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
