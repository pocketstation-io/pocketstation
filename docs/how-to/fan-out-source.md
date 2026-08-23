# Fan out one source

<!-- claims: CLM-GUIDE-007-CAP-001,CLM-GUIDE-007-CAP-002,CLM-GUIDE-007-CAP-003,CLM-GUIDE-007-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.
- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Declare the source once.
2. Create each consumer endpoint independently.
3. Connect the same source output to each endpoint.
4. Set explicit edge policy where the default is unsuitable.
5. Observe each route separately so saturation remains attributable.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::extensions::audio_input::source::PcmSource` | struct | Low-level PCM source ownership for integrations that separately retain the Session handles and producer writer. | `src/session/extensions/audio_input/source.rs:33` |
| `pocketstation::session::lifecycle::events::SessionSourceFailure` | struct | Source failure associated with one stable session stem. | `src/session/lifecycle/events.rs:104` |
| `from_source_output` | function | Wraps a public external-source output in the same typed Rust façade. Runtime identity remains the output's stable `SignalSpec` and schema. | `src/session/declaration/typed_stream.rs:118` |
| `into_pcm_source` | function | Converts the convenience façade into explicit source, output, and producer ownership. | `src/session/extensions/audio_input/mod.rs:137` |
| `pocketstation::session::extensions::audio_input::PCM_SOURCE_TYPE_ID` | constant | Stable runtime identity of the underlying PCM source implementation. | `src/session/extensions/audio_input/mod.rs:19` |
| `pocketstation::session::extensions::source::SourceDriver` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/source.rs:267` |
| `pocketstation::session::extensions::source::SourceFactory` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/source.rs:276` |
| `pocketstation::runtime::audio::runner::PlanSourceInputObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/audio/runner.rs:22` |
| `pocketstation::session::declaration::draft::SourceInstanceHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/declaration/draft.rs:846` |
| `pocketstation::session::declaration::draft::SourceOutputHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/declaration/draft.rs:922` |
| `pocketstation::session::declaration::spec::SourceInstanceId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/declaration/spec.rs:14` |
| `pocketstation::session::declaration::spec::SourceInstanceSpec` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/declaration/spec.rs:68` |
| `pocketstation::session::declaration::spec::SourceOutputSpec` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/declaration/spec.rs:94` |
| `pocketstation::session::extensions::source::SourceCancellation` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/source.rs:250` |
| `pocketstation::session::extensions::source::SourceConfiguration` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/source.rs:87` |
| `pocketstation::session::extensions::source::SourceEmission` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/source.rs:261` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_many_input_port_with_multiple_sources_when_planned_then_one_fan_in_group` — given many input port with multiple sources when planned then one fan in group (`src/graph/compile/plan.rs:664`; `test-e8c1159b6f21b80587d2`).
- `given_single_input_port_with_multiple_sources_when_planned_then_fan_in_on_single_port_error` — given single input port with multiple sources when planned then fan in on single port error (`src/graph/compile/plan.rs:683`; `test-a9d800fafb2562c90bd3`).
- `given_linear_graph_when_compiled_then_topo_orders_source_before_sink` — given linear graph when compiled then topo orders source before sink (`src/graph/compile/resolve.rs:973`; `test-7ece727a2fa318f311df`).
- `given_stereo_source_into_mono_only_sink_when_compiled_then_mono_mix_adapter_inserted` — given stereo source into mono only sink when compiled then mono mix adapter inserted (`src/graph/compile/resolve.rs:1211`; `test-65a2b9b9fd5cc9674c26`).
- `given_stereo_source_into_stereo_sink_when_compiled_then_stereo_survives_no_adapter` — given stereo source into stereo sink when compiled then stereo survives no adapter (`src/graph/compile/resolve.rs:1232`; `test-6dba13dc9eb08d12b1a1`).
- `given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated` — given foreign clock timestamp when delivered then source latency is not fabricated (`src/runtime/audio/router.rs:1182`; `test-133d3a4b4c11520b3884`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` — given lineaged source fan out when branch frames are copied then exact lineage is preserved (`src/runtime/audio/router.rs:1076`; `test-d798548d6c8b059ba1a8`).
- `given_one_source_with_three_edges_when_dispatched_then_every_edge_receives_identified_frame` — given one source with three edges when dispatched then every edge receives identified frame (`src/runtime/audio/router.rs:1044`; `test-413eb70a225c14f5ec09`).
- `given_two_sources_with_six_edges_when_dispatched_then_source_identity_stays_separate` — given two sources with six edges when dispatched then source identity stays separate (`src/runtime/audio/router.rs:1281`; `test-c9cc919bb7901f88dbe8`).
- `given_full_source_input_when_more_frames_arrive_then_newest_rejects_and_counts` — given full source input when more frames arrive then newest rejects and counts (`src/runtime/audio/runner.rs:704`; `test-9884a85b98ea454bb6cf`).
- `given_queued_sources_when_cancelled_with_budget_then_drain_is_bounded_and_rest_discards` — given queued sources when cancelled with budget then drain is bounded and rest discards (`src/runtime/audio/runner.rs:636`; `test-31632b8eb3f0b3c90934`).
- `given_queued_sources_when_cancelled_with_discard_then_no_frame_executes` — given queued sources when cancelled with discard then no frame executes (`src/runtime/audio/runner.rs:682`; `test-e8ced072a71ada7c3d25`).

## Failure signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` — `error-00e5716261eba0f8cf3d`
- `pocketstation::session::error::SessionError` / `UnknownStem` — `error-00f6e798d158df66c847`
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` — `error-01d3fc855e2a00319076`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` — `error-023d6ab0b23a50a614ff`
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` — `error-0279b2b6b0cb3b5801bc`
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` — `error-037ddc3e193da74177f8`
- `pocketstation::graph::node::NodeDescriptorError` / `InvalidSafetyContract` — `error-04b7031025a9b635fdbf`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` — `error-05c60389efcb84311921`
- `pocketstation::session::prepare::error::SessionPrepareError` — `error-085082b521c14e5ecd1e`
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` — `error-08a7536094bfb2242b17`
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` — `error-09837185c7fca0f70618`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` — `error-0bc2f7c0b9f9dbf8ddd7`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Choose route capacity and loss policy](/docs/how-to/configure-route-policy.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/declaration/draft.rs:1-1417` (`DIRECT`)
- `src/runtime/audio/router.rs:1-1615` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
