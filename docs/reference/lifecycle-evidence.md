# Lifecycle evidence index

<!-- claims: CLM-REF-022-CAP-001,CLM-REF-022-CAP-002,CLM-REF-022-CAP-003,CLM-REF-022-CAP-004,CLM-REF-022-SOURCE-001,CLM-REF-022-LIFECYCLE-0001,CLM-REF-022-LIFECYCLE-0002,CLM-REF-022-LIFECYCLE-0003,CLM-REF-022-LIFECYCLE-0004,CLM-REF-022-LIFECYCLE-0005,CLM-REF-022-LIFECYCLE-0006,CLM-REF-022-LIFECYCLE-0007,CLM-REF-022-LIFECYCLE-0008,CLM-REF-022-LIFECYCLE-0009,CLM-REF-022-LIFECYCLE-0010,CLM-REF-022-LIFECYCLE-0011,CLM-REF-022-LIFECYCLE-0012,CLM-REF-022-LIFECYCLE-0013,CLM-REF-022-LIFECYCLE-0014,CLM-REF-022-LIFECYCLE-0015,CLM-REF-022-LIFECYCLE-0016,CLM-REF-022-LIFECYCLE-0017,CLM-REF-022-LIFECYCLE-0018,CLM-REF-022-LIFECYCLE-0019,CLM-REF-022-LIFECYCLE-0020,CLM-REF-022-LIFECYCLE-0021,CLM-REF-022-LIFECYCLE-0022,CLM-REF-022-LIFECYCLE-0023,CLM-REF-022-LIFECYCLE-0024,CLM-REF-022-LIFECYCLE-0025,CLM-REF-022-LIFECYCLE-0026,CLM-REF-022-LIFECYCLE-0027,CLM-REF-022-LIFECYCLE-0028,CLM-REF-022-LIFECYCLE-0029,CLM-REF-022-LIFECYCLE-0030,CLM-REF-022-LIFECYCLE-0031,CLM-REF-022-LIFECYCLE-0032,CLM-REF-022-LIFECYCLE-0033,CLM-REF-022-LIFECYCLE-0034,CLM-REF-022-LIFECYCLE-0035,CLM-REF-022-LIFECYCLE-0036,CLM-REF-022-LIFECYCLE-0037,CLM-REF-022-LIFECYCLE-0038,CLM-REF-022-LIFECYCLE-0039,CLM-REF-022-LIFECYCLE-0040,CLM-REF-022-LIFECYCLE-0041,CLM-REF-022-LIFECYCLE-0042,CLM-REF-022-LIFECYCLE-0043,CLM-REF-022-LIFECYCLE-0044,CLM-REF-022-LIFECYCLE-0045,CLM-REF-022-LIFECYCLE-0046,CLM-REF-022-LIFECYCLE-0047,CLM-REF-022-LIFECYCLE-0048,CLM-REF-022-LIFECYCLE-0049,CLM-REF-022-LIFECYCLE-0050,CLM-REF-022-LIFECYCLE-0051,CLM-REF-022-LIFECYCLE-0052,CLM-REF-022-LIFECYCLE-0053,CLM-REF-022-LIFECYCLE-0054,CLM-REF-022-LIFECYCLE-0055,CLM-REF-022-LIFECYCLE-0056,CLM-REF-022-LIFECYCLE-0057,CLM-REF-022-LIFECYCLE-0058,CLM-REF-022-LIFECYCLE-0059,CLM-REF-022-LIFECYCLE-0060,CLM-REF-022-LIFECYCLE-0061 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.
- **Run connector workers.** Supervise connector delivery, acknowledgement, retry budgets, readiness, cancellation, drain, and abort.
- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Inventory

| Evidence record | Operation | Trigger | From | To | Source |
|---|---|---|---|---|---|
| life-01cc5d2b80f8e24f1fdc | `ConnectorWorker::cancel_preparation` | cancel_preparation | unknown | unknown | `src/connector/worker/mod.rs:35` |
| life-033f82e656e133a23c4c | `drop_observations` | drop_observations | unknown | unknown | `src/session/lifecycle/observations.rs:206` |
| life-05e66cfb59672fcf3e47 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Cancel` | Cancel | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:13` |
| life-1e6e9f452ca83bcd4874 | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | Prepare | unknown | unknown | `src/session/lifecycle/engine.rs:321` |
| life-1fdc6a18491085eeb80b | `AsyncNode::close` | close | unknown | unknown | `src/graph/signal/operator.rs:40` |
| life-23ee10c2333375238483 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | Cancelled | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:277` |
| life-2c74a6091c9cde4045cb | `drop` | drop | unknown | unknown | `src/session/lifecycle/trace.rs:249` |
| life-2ceded8ff4d093c4470f | `start` | start | unknown | unknown | `src/lib.rs:617` |
| life-31d39bc66ea3e22cdbb4 | `cancel` | cancel | unknown | unknown | `src/lib.rs:903` |
| life-326d10a69e8bf7fdb781 | `pocketstation::connector::error::ConnectorErrorStage::Join` | Join | unknown | unknown | `src/connector/error.rs:68` |
| life-37b462ed94c0e7d7bcaa | `pocketstation::endpoint::runtime::EndpointShutdownMode::Drain` | Drain | unknown | unknown | `src/endpoint/runtime.rs:357` |
| life-396be01d86a31314ead0 | `pocketstation::session::lifecycle::start_contract::SessionStartError::Cancelled` | Cancelled | unknown | unknown | `src/session/lifecycle/start_contract.rs:194` |
| life-3ac69529ebf1ff2919b5 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Running` | Running | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:27` |
| life-3b6b26099edb78d498dd | `drop` | drop | unknown | unknown | `src/frame/pool.rs:322` |
| life-3bd045de883d59fb612b | `shutdown_mode` | shutdown_mode | unknown | unknown | `src/connector/worker/coordination.rs:32` |
| life-4ca80c498146346ca2ad | `drop_rate_pct` | drop_rate_pct | unknown | unknown | `src/session/lifecycle/observations.rs:171` |
| life-4f6084523023bad934b0 | `ActiveCaptureBackend::stop_and_join` | stop_and_join | unknown | unknown | `src/capture/capture_owner.rs:111` |
| life-51677582d6bfbf19bf36 | `SourceDriver::close` | close | unknown | unknown | `src/session/extensions/source.rs:273` |
| life-591e104c350f112363df | `EndpointDriverFactory::prepare` | prepare | unknown | unknown | `src/endpoint/contract.rs:241` |
| life-5b07b94601e40546dc60 | `pocketstation::connector::error::ConnectorErrorStage::Prepare` | Prepare | unknown | unknown | `src/connector/error.rs:62` |
| life-6127631a15d75622b3a3 | `drop_rate_pct` | drop_rate_pct | unknown | unknown | `src/runtime/audio/router.rs:165` |
| life-626edd9c657f8ffd0e25 | `pocketstation::graph::node::NodeError::Prepare` | Prepare | unknown | unknown | `src/graph/node.rs:151` |
| life-6838c7db0d54daff94be | `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | Stopped | unknown | unknown | `src/session/lifecycle/events.rs:211` |
| life-6b5d8c54e0f2147b52d5 | `ConnectorFactory::prepare` | prepare | unknown | unknown | `src/connector/worker/mod.rs:26` |
| life-6f6114d034e4edd4755a | `close` | close | unknown | unknown | `src/session/extensions/audio_input/mod.rs:127` |
| life-7030b8447dc3db092c92 | `prepare` | prepare | unknown | unknown | `src/connector/sidecar.rs:49` |
| life-71b5805cedb692ebf400 | `start_cancellable` | start_cancellable | unknown | unknown | `src/lib.rs:621` |
| life-7a6ae3567d5c4cae3ab8 | `start` | start | unknown | unknown | `src/session/lifecycle/trace.rs:160` |
| life-83664090921ee3a1c28f | `pocketstation::SessionStartErrorKind::Cancelled` | Cancelled | unknown | unknown | `src/lib.rs:1066` |
| life-8389e526c053c0f4878c | `pocketstation::endpoint::runtime::EndpointFailureStage::Start` | Start | unknown | unknown | `src/endpoint/runtime.rs:159` |
| life-85e8c03a97502ae9709d | `pocketstation::SessionCancelDisposition::Cancelled` | Cancelled | unknown | unknown | `src/lib.rs:1086` |
| life-88f209a3c7bc2ba137fb | `ConnectorWorker::run` | run | unknown | unknown | `src/connector/worker/mod.rs:33` |
| life-8ab6ff40055ef9e6b1e4 | `SourceDriver::prepare` | prepare | unknown | unknown | `src/session/extensions/source.rs:268` |
| life-9cf382d0e5a6816d4c71 | `pocketstation::endpoint::runtime::EndpointShutdownMode::Abort` | Abort | unknown | unknown | `src/endpoint/runtime.rs:358` |
| life-9d1d446ad1f2873e395b | `RuntimeNode::prepare` | prepare | unknown | unknown | `src/graph/runtime_node.rs:8` |
| life-9dedf8edf94ac1e55756 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | Cancelled | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:301` |
| life-a103fa50f0a44e41e441 | `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | Stopped | unknown | unknown | `src/session/lifecycle/events.rs:23` |
| life-a125b7d6bdf384df6d6f | `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | Finalized | unknown | unknown | `src/session/lifecycle/observations.rs:444` |
| life-a41d9c931f7f68bd3076 | `RunningEndpointDriver::join_and_finalize` | join_and_finalize | unknown | unknown | `src/endpoint/runtime.rs:346` |
| life-a77333ff603f041c3363 | `PreparedEndpointDriver::start` | start | unknown | unknown | `src/endpoint/runtime.rs:323` |
| life-a77494b6f212277d3533 | `ConnectorDriverFactory::prepare` | prepare | unknown | unknown | `src/connector/worker/driver.rs:132` |
| life-a7cf93ad46ee7edbf114 | `ConnectorDriver::cancel_preparation` | cancel_preparation | unknown | unknown | `src/connector/worker/driver.rs:116` |
| life-a967213e86e6aa71aa55 | `CallbackCaptureBackend::prepare` | prepare | unknown | unknown | `src/capture/capture_owner.rs:84` |
| life-b190ec77abf54ff75844 | `pocketstation::endpoint::runtime::EndpointFailureStage::Prepare` | Prepare | unknown | unknown | `src/endpoint/runtime.rs:157` |
| life-b56488eda9c3ea6b7b3e | `pocketstation::session::error_code::SessionStopCode::Stopped` | Stopped | unknown | unknown | `src/session/error_code.rs:151` |
| life-b86a51a255f9938c1308 | `close` | close | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:242` |
| life-c2ba2f7ef3b906bb0d57 | `ConnectorDriver::shutdown` | shutdown | unknown | unknown | `src/connector/worker/driver.rs:108` |
| life-c85ae5780d9d74272fa7 | `ConnectorDriver::start` | start | unknown | unknown | `src/connector/worker/driver.rs:93` |
| life-ca5172d134b2b5db799e | `drop` | drop | unknown | unknown | `src/connector/configuration.rs:49` |
| life-cb920d0c8e017093dbb3 | `stop` | stop | unknown | unknown | `src/lib.rs:886` |
| life-ce971b431224b00409e2 | `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | Running | unknown | unknown | `src/session/lifecycle/events.rs:21` |
| life-cfbf77d2f976c5aed1ae | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Start` | Start | unknown | unknown | `src/session/lifecycle/engine.rs:323` |
| life-d21c7012be5a41c1150c | `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | Cancelled | unknown | unknown | `src/abi/session/abi.rs:94` |
| life-d2e8d472ee35c4189976 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Close` | Close | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:14` |
| life-d2fad2d6121e658194a2 | `AsyncNode::cancel` | cancel | unknown | unknown | `src/graph/signal/operator.rs:36` |
| life-d8c08562ef859bb25c31 | `PreparedEndpointDriver::cancel_preparation` | cancel_preparation | unknown | unknown | `src/endpoint/runtime.rs:328` |
| life-dd8c856052da7cc9b967 | `pocketstation::SessionStopDisposition::Stopped` | Stopped | unknown | unknown | `src/lib.rs:1080` |
| life-ec902207cd2a525b3786 | `drop` | drop | unknown | unknown | `src/endpoint/polled_audio_driver.rs:195` |
| life-f3df86859aef858ed448 | `AsyncNode::prepare` | prepare | unknown | unknown | `src/graph/signal/operator.rs:14` |
| life-f648d9f62ff6bcc1ebe5 | `pocketstation::connector::error::ConnectorErrorStage::Shutdown` | Shutdown | unknown | unknown | `src/connector/error.rs:67` |
| life-fe366e66ab10b0e67afd | `drop` | drop | unknown | unknown | `src/frame/pool.rs:265` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Treat stop outcomes as data](/docs/best-practices/terminal-outcomes.md)
- [Host a managed-process sidecar](/docs/how-to/host-sidecar.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/mod.rs:1-58` (`DIRECT`)
- `src/endpoint/runtime.rs:1-531` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
