# External PCM input is saturated

<!-- claims: CLM-TRBL-018-CAP-001,CLM-TRBL-018-CAP-002,CLM-TRBL-018-CAP-003,CLM-TRBL-018-CAP-004,CLM-TRBL-018-SOURCE-001 -->

## Symptom

External PCM buffer acquisition or submission reports saturation, closure, cancellation, or invalid frame shape.

## Evidenced causes

- All finite input buffers are leased.
- The submitted sample count or format does not match `AudioInputConfig`.
- The input source is closed or its Session was cancelled.
- The downstream route cannot accept the frame.

## Distinguish the causes

Inspect acquisition failures, available slots, outstanding buffers, write error, cancellation state, and downstream route observations.

## Diagnostic signals

- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` (`error-fdc385a5c70e04dd3cdf`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` / `Cancelled` (`error-b7aa4ebc1860ffa7ee22`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` / `Closed` (`error-fb9a6f0c357fd99e74ea`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` / `Full` (`error-ada63bea703945e2d3a7`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` (`error-be4e946430b3a1e7ff29`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `Capacity` (`error-0865208765b6bdbf9f60`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `Empty` (`error-de475678e480c72f6785`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `MisalignedChannels` (`error-8a05309b88b80dfecd31`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `WrongFrameLength` (`error-ca5f261127aad9f5428b`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `WrongSource` (`error-3c9080bfef27a80c5bcb`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteError` (`error-bd1aefa9086d9f905e1b`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` (`error-c5fc409b75ad1482b6f4`)

## Executable evidence

- `given_bounded_audio_input_when_writes_are_invalid_or_saturated_then_ownership_is_explicit` exercises given bounded audio input when writes are invalid or saturated then ownership is explicit under its recorded setup (`test-795ca5a283c59dbf6066`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-2b664c22fd511e3c2f45`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` exercises given promptable or observable permission when opening input then native open decides under its recorded setup (`test-847d3fefe4665db8dd14`).
- `given_active_asp_when_required_then_sdk_accepts_external_provisioning` exercises given active asp when required then sdk accepts external provisioning under its recorded setup (`test-094ea52b81e34a03e0e1`).
- `given_stable_input_selector_when_wrapped_then_capture_mode_preserves_identity` exercises given stable input selector when wrapped then capture mode preserves identity under its recorded setup (`test-074f0bb3502ed6267879`).
- `given_full_source_input_when_more_frames_arrive_then_newest_rejects_and_counts` exercises given full source input when more frames arrive then newest rejects and counts under its recorded setup (`test-9884a85b98ea454bb6cf`).
- `given_full_input_branch_when_sent_then_overflow_is_counted_and_join_is_bounded` exercises given full input branch when sent then overflow is counted and join is bounded under its recorded setup (`test-95905f4e18a52f786a53`).
- `given_operator_composition_with_named_multi_input_output_manifest_then_each_declared_port_executes` exercises given operator composition with named multi input output manifest then each declared port executes under its recorded setup (`test-716956fd7ff21d2765ad`).
- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` exercises given operator composition with three external operators then derived output crosses each bounded edge under its recorded setup (`test-9ec51c75cedb5ffaef0f`).
- `given_two_inputs_when_processed_then_nonterminal_and_terminal_reach_each_branch` exercises given two inputs when processed then nonterminal and terminal reach each branch under its recorded setup (`test-23bf71cedc9a9fc7172c`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` exercises given derived stream chain when compiled then operator output feeds next named input under its recorded setup (`test-e9a24a392741b4dbe6e7`).
- `given_required_named_input_missing_when_compiled_then_failure_precedes_graph_runtime` exercises given required named input missing when compiled then failure precedes graph runtime under its recorded setup (`test-99e881c59d26f6126f74`).
- `given_duplicate_named_input_when_connected_then_declaration_fails_immediately` exercises given duplicate named input when connected then declaration fails immediately under its recorded setup (`test-f9a6ec4f71dbaf6d8083`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` exercises given foreign input handle when connected then declaration fails before freeze under its recorded setup (`test-766194f5939b3ddb896d`).
- `given_external_output_through_operator_when_compiled_then_normal_typed_edges_are_used` exercises given external output through operator when compiled then normal typed edges are used under its recorded setup (`test-1e9492347c366dc04946`).

## Corrective action

Release or submit outstanding buffers, correct frame shape, or create a new input after terminal closure.

## Retry and incomplete state

Retry acquisition only after capacity returns. Never replay a submitted frame unless application semantics permit duplication; accepted and rejected frames can leave a partial stream.

## Related reference

- [External Pcm](/docs/concepts/external-pcm.md)
- [Capture](/docs/reference/capture.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Configuration reference](/docs/reference/configuration.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)

## Evidence boundary

The claims on **External PCM input is saturated** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/extensions/audio_input/buffer.rs:1-367` (`DIRECT`)

For **External PCM input is saturated**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
