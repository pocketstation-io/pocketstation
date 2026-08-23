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

- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` (`error-dcce5878d5029e48c12c`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` / `Cancelled` (`error-277b4455fde06fca7361`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` / `Closed` (`error-61958c54d6cb6f8952cf`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` / `Full` (`error-c222fd2773c7c5c55794`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` (`error-37b6891ffb137bc2279d`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `Capacity` (`error-85dcb1c364d4167e34c8`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `Empty` (`error-41bc9ec6df2a0eb3bf43`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `MisalignedChannels` (`error-cccf197068b4774b1611`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `WrongFrameLength` (`error-14214888fdd0a8616ecb`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` / `WrongSource` (`error-c0b3efa4cd29860dcd57`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteError` (`error-2f91639b7baf4444f54a`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` (`error-97f4b00411fc44b24799`)

## Executable evidence

- `given_bounded_audio_input_when_writes_are_invalid_or_saturated_then_ownership_is_explicit` exercises given bounded audio input when writes are invalid or saturated then ownership is explicit under its recorded setup (`test-5a764dda823599c553f3`).
- `given_denied_permission_when_opening_input_then_capture_fails_closed` exercises given denied permission when opening input then capture fails closed under its recorded setup (`test-93f56a3510497f49f523`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` exercises given promptable or observable permission when opening input then native open decides under its recorded setup (`test-136298dd50a44f77d3ac`).
- `given_active_asp_when_required_then_sdk_accepts_external_provisioning` exercises given active asp when required then sdk accepts external provisioning under its recorded setup (`test-0f83918e778e3285908b`).
- `given_stable_input_selector_when_wrapped_then_capture_mode_preserves_identity` exercises given stable input selector when wrapped then capture mode preserves identity under its recorded setup (`test-a213feaa87257702636d`).
- `given_full_source_input_when_more_frames_arrive_then_newest_rejects_and_counts` exercises given full source input when more frames arrive then newest rejects and counts under its recorded setup (`test-3f76115476879df63de7`).
- `given_full_input_branch_when_sent_then_overflow_is_counted_and_join_is_bounded` exercises given full input branch when sent then overflow is counted and join is bounded under its recorded setup (`test-4ea56b723e02f622ad3d`).
- `given_operator_composition_with_named_multi_input_output_manifest_then_each_declared_port_executes` exercises given operator composition with named multi input output manifest then each declared port executes under its recorded setup (`test-bc69e1a774892a686b9f`).
- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` exercises given operator composition with three external operators then derived output crosses each bounded edge under its recorded setup (`test-eab3e581e210e0e82882`).
- `given_two_inputs_when_processed_then_nonterminal_and_terminal_reach_each_branch` exercises given two inputs when processed then nonterminal and terminal reach each branch under its recorded setup (`test-f35227f30fb90260d27a`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` exercises given derived stream chain when compiled then operator output feeds next named input under its recorded setup (`test-081f9254eabd3bfeaad1`).
- `given_required_named_input_missing_when_compiled_then_failure_precedes_graph_runtime` exercises given required named input missing when compiled then failure precedes graph runtime under its recorded setup (`test-e868470f819453421dd7`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` exercises given foreign input handle when connected then declaration fails before freeze under its recorded setup (`test-0098e9bf5859cd4840f9`).
- `given_repeated_named_input_when_declared_then_compiler_retains_multiplicity_authority` exercises given repeated named input when declared then compiler retains multiplicity authority under its recorded setup (`test-0920f0be863672d2298e`).
- `given_external_output_through_operator_when_compiled_then_normal_typed_edges_are_used` exercises given external output through operator when compiled then normal typed edges are used under its recorded setup (`test-16199d206f3d5dfd3054`).

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

The claims on **External PCM input is saturated** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/extensions/audio_input/buffer.rs:1-367` (`DIRECT`)

For **External PCM input is saturated**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
