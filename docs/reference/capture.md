# Capture API

<!-- claims: CLM-REF-003-CAP-001,CLM-REF-003-CAP-002,CLM-REF-003-CAP-003,CLM-REF-003-CAP-004,CLM-REF-003-CAP-005,CLM-REF-003-SOURCE-001 -->

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Capture API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Capture API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-d45896d5cc6abbfcd3e2 | `INITIAL` | assoc_const | Provides the initial value for `PermissionEpoch`. | `src/capture/authorization.rs:270` |
| sym-c22033b2bb1da21bc723 | `INITIAL` | assoc_const | Provides the initial value for `SourceGeneration`. | `src/capture/events.rs:15` |
| sym-27e7db0a9ab3edb1a125 | `pocketstation::capture::capture_owner::CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Monotonic timestamp domain used by native capture backends. | `src/capture/capture_owner.rs:20` |
| sym-3ddc5124a70a7fd4c6e0 | `pocketstation::capture::events::MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES` | constant | Maximum heap storage retained by one queued capture-runtime event. | `src/capture/events.rs:72` |
| sym-45544e6ea59c7420a9f9 | `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| sym-8e6567a1c2e9d6a4db50 | `pocketstation::capture::authorization::CaptureCapabilityState` | enum | Selects the capture capability state used by PocketStation. | `src/capture/authorization.rs:145` |
| sym-d136a6815bee037339a8 | `pocketstation::capture::authorization::CaptureError` | enum | Classifies failures reported as capture error. | `src/capture/authorization.rs:290` |
| sym-5f31b7e8f71b798c2bc3 | `pocketstation::capture::authorization::CaptureOpenOutcome` | enum | Classifies the observable capture open outcome. | `src/capture/authorization.rs:281` |
| sym-00e7060ce8fe6b8d80c9 | `pocketstation::capture::authorization::CaptureScope` | enum | Selects the capture scope used by PocketStation. | `src/capture/authorization.rs:248` |
| sym-62a2caf00b43e1bc9477 | `pocketstation::capture::authorization::CaptureSessionGrant` | enum | Enumerates the supported capture session grant cases. | `src/capture/authorization.rs:240` |
| sym-19385c71e1663f64be9e | `pocketstation::capture::authorization::PermissionObservation` | enum | Classifies the observable permission observation. | `src/capture/authorization.rs:153` |
| sym-cb9329e443cc1161cd37 | `pocketstation::capture::authorization::SourceIdentityStrength` | enum | Enumerates the supported source identity strength cases. | `src/capture/authorization.rs:257` |
| sym-a4c1d5e417a864f7851c | `pocketstation::capture::events::CaptureRuntimeFailureClass` | enum | Enumerates the supported capture runtime failure class cases. | `src/capture/events.rs:40` |
| sym-2de679293542511064e1 | `pocketstation::capture::events::SourceLifecycleEventKind` | enum | Selects the source lifecycle event kind used by PocketStation. | `src/capture/events.rs:25` |
| sym-73291cc2f1ac425b2f2f | `pocketstation::capture::events::SourceRecoveryRequirement` | enum | Selects the source recovery requirement used by PocketStation. | `src/capture/events.rs:35` |
| sym-64eaba6d85604fbcf37f | `pocketstation::capture::events::SourceRuntimeEvent` | enum | Classifies the observable source runtime event. | `src/capture/events.rs:53` |
| sym-ec6a41a2470440748019 | `pocketstation::capture::events::SourceRuntimeEventDelivery` | enum | Enumerates the supported source runtime event delivery cases. | `src/capture/events.rs:96` |
| sym-4fd8cc22a3328607bc3f | `pocketstation::capture::events::SourceRuntimeEventReceive` | enum | Enumerates the supported source runtime event receive cases. | `src/capture/events.rs:104` |
| sym-a9f392ffee51c33f8cd9 | `pocketstation::capture::frame_stream::CapturedFrameDelivery` | enum | Enumerates the supported captured frame delivery cases. | `src/capture/frame_stream.rs:10` |
| sym-6feea6318d057dc0a2fa | `pocketstation::capture::identity::SourceKind` | enum | Selects the source kind used by PocketStation. | `src/capture/identity.rs:9` |
| sym-a477f29ca688939ef8c1 | `pocketstation::capture::identity::SourceState` | enum | Selects the source state used by PocketStation. | `src/capture/identity.rs:17` |
| sym-8e57c3486871455ef5f0 | `pocketstation::capture::lifecycle_registry::SourceGenerationTransition` | enum | Enumerates the supported source generation transition cases. | `src/capture/lifecycle_registry.rs:8` |
| sym-42dfecbc7ac0471e6287 | `pocketstation::capture::query::SourceQuery` | enum | Enumerates the supported source query cases. | `src/capture/query.rs:13` |
| sym-270a444524627875d7e2 | `pocketstation::capture::selection::CaptureMode` | enum | Selects the capture mode used by PocketStation. | `src/capture/selection.rs:16` |
| sym-a982dfd305482431b4c3 | `pocketstation::capture::selection::InputDeviceSelector` | enum | Enumerates the supported input device selector cases. | `src/capture/selection.rs:9` |
| sym-21e17566ba71f7fbee27 | `pocketstation::capture::selection::ProcessTreeScope` | enum | Selects the process tree scope used by PocketStation. | `src/capture/selection.rs:83` |
| sym-fa016117ddeaa461fb2a | `pocketstation::capture::selection::SelectorPersistenceScope` | enum | Selects the selector persistence scope used by PocketStation. | `src/capture/selection.rs:73` |
| sym-9709f9e8853c47ef7563 | `pocketstation::capture::timeline::CaptureSampleTimelineError` | enum | Classifies failures reported as capture sample timeline error. | `src/capture/timeline.rs:41` |
| sym-bceadb7ddb30e7466491 | `ActiveCaptureBackend::observation_handle` | function | Returns a handle for reading observations from `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:107` |
| sym-5d9a59fe0fc2f3b7037e | `ActiveCaptureBackend::observations` | function | Returns the observations exposed by `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:109` |
| sym-a0a449182596416f9615 | `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| sym-74a75f4e4c0263afe3b0 | `ActiveCaptureBackend::stop_and_join` | function | Stops `ActiveCaptureBackend`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:111` |
| sym-1b834e57e91f557c870d | `CallbackCaptureBackend::prepare` | function | Prepares resources required by `CallbackCaptureBackend`. | `src/capture/capture_owner.rs:84` |
| sym-f70e9390f0489e77ec23 | `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| sym-0760bc020ba894b3f5bf | `advance_from_source_position` | function | Returns a buffer's source-time start from its native sample-frame position. Forward gaps are preserved in the returned timestamp without separately advancing this clock from an aggregate drop counter. | `src/capture/timeline.rs:80` |
| sym-fa0ffc427c129433bceb | `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| sym-d0f5605e55cfb06011a6 | `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |
| sym-2ecc6be8dc695c834430 | `before_open` | function | Records a source that was resolved but not opened because another required source failed first. | `src/capture/authorization.rs:60` |
| sym-099e7f35723bb117ce7d | `capacity_frames` | function | Returns the capacity frames held by `CapturedFrameStream`. | `src/capture/frame_stream.rs:165` |
| sym-b75c58a2c4f1f420edb8 | `capture_mode` | function | Returns the capture mode held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:57` |
| sym-1e412832c9049ffd81cf | `capture_mode` | function | Returns the capture mode held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:44` |
| sym-b9dec8c2506c097ed3e7 | `current` | function | Returns the current value observed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:196` |
| sym-f7b04b27aaa8ae652791 | `discover` | function | Discovers the resources visible to `LocalSourceProvider`. | `src/capture/query.rs:55` |
| sym-7a9a3832c93c1ee7b3f2 | `drop` | function | Releases resources owned by `MacosInputSource`. | `src/capture/platform/macos/input.rs:251` |
| sym-c4021746d79020bddaed | `drop` | function | Releases resources owned by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:299` |
| sym-5eba9d84e7e7e2217921 | `frame_stream_closed` | function | Returns the frame stream closed associated with `CaptureOwner`. | `src/capture/capture_owner.rs:251` |
| sym-e258bcaf92e5c1d9247d | `from_open_observations` | function | Records platform authorization observations without inferring them from a generic backend result. Callers must pass `NotObservable` when their platform has no authoritative query for the requested capture class. | `src/capture/authorization.rs:76` |
| sym-66709b3ff0592c65224c | `generation` | function | Returns the generation associated with `SourceLifecycleRegistry`. | `src/capture/lifecycle_registry.rs:84` |
| sym-26e629a3439fbbf76585 | `identity_strength` | function | Returns the identity strength held by `CaptureSource`. | `src/capture/identity.rs:94` |
| sym-4e8e191c150a473e6652 | `into_callback` | function | Converts `CapturedFrameSender` into callback. | `src/capture/frame_stream.rs:132` |
| sym-8cedd9b3063f0be9d99e | `is_closed` | function | Returns whether closed applies to `CapturedFrameStream`. | `src/capture/frame_stream.rs:170` |
| sym-5e0b2d5ceaf474c219a7 | `matches` | function | Returns whether an input satisfies `SourceQuery`. | `src/capture/query.rs:22` |
| sym-c5d5c62df51e1409b550 | `new` | function | Creates a new `CapturePermissionLifecycle`. | `src/capture/authorization.rs:189` |
| sym-d4ae2b6e62b4da1a9cbc | `new` | function | Creates a new `CaptureLineageSeed`. | `src/capture/capture_owner.rs:31` |
| sym-607b63179a58247f45c4 | `new` | function | Creates a new `StableSourceId`. | `src/capture/identity.rs:33` |
| sym-63ee209fc6b8809d2745 | `new` | function | Creates a new `CaptureSampleTimeline`. | `src/capture/timeline.rs:52` |
| sym-68f41330fa1008ddb0d5 | `next` | function | Advances the local evidence epoch after an observed authorization change or an explicit source reopen. | `src/capture/authorization.rs:274` |
| sym-16beb1bd75823a8e4a9a | `next` | function | Returns the generation assigned after explicit rediscovery. | `src/capture/events.rs:18` |
| sym-72186c7686c046cfb200 | `observation_handle` | function | Returns a handle for reading observations from `SourceRuntimeEventSender`. | `src/capture/events.rs:270` |
| sym-5b48ccb23dd8ea3456d6 | `observation_handle` | function | Returns a handle for reading observations from `SourceRuntimeEventReceiver`. | `src/capture/events.rs:321` |
| sym-84a783bb57fc8776c9fe | `observation_handle` | function | Returns a handle for reading observations from `CapturedFrameSender`. | `src/capture/frame_stream.rs:142` |
| sym-a64edc3f92284994a561 | `observation_handle` | function | Returns a handle for reading observations from `CapturedFrameStream`. | `src/capture/frame_stream.rs:183` |
| sym-f2db5c62a69fa044699c | `observation_handle` | function | Returns a handle for reading observations from `CaptureObservationCounters`. | `src/capture/observations.rs:107` |
| sym-a5cb1adefd7a956798a4 | `observation_handle` | function | Returns a handle for reading observations from `MacosInputSource`. | `src/capture/platform/macos/input.rs:231` |
| sym-75fe343f4d5a6f0542a0 | `observation_handle` | function | Returns a handle for reading observations from `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:267` |
| sym-91c2d86b10a9a63997b5 | `observation_handle` | function | Returns a handle for reading observations from `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:96` |
| sym-c9f6042eaf57175c5119 | `observation_receipt` | function | Returns the observation receipt associated with `CaptureOwner`. | `src/capture/capture_owner.rs:260` |
| sym-e02982d65399a150f192 | `observations` | function | Returns the observations exposed by `CaptureObservationReceipt`. | `src/capture/capture_owner.rs:174` |
| sym-78249af275b9cc106fe6 | `observations` | function | Returns the observations exposed by `CaptureOwner`. | `src/capture/capture_owner.rs:256` |
| sym-835351f51a8f903c484a | `observations` | function | Returns the observations exposed by `SourceRuntimeEventObservationHandle`. | `src/capture/events.rs:205` |
| sym-a5581c8cbad9390c13a5 | `observations` | function | Returns the observations exposed by `SourceRuntimeEventSender`. | `src/capture/events.rs:266` |
| sym-d54552db99b1952baa2e | `observations` | function | Returns the observations exposed by `SourceRuntimeEventReceiver`. | `src/capture/events.rs:317` |
| sym-5e1cf4492d30804e230b | `observations` | function | Returns the observations exposed by `CapturedFrameObservationHandle`. | `src/capture/frame_stream.rs:36` |
| sym-5b477e17989793fa8dc0 | `observations` | function | Returns the observations exposed by `CaptureObservationHandle`. | `src/capture/observations.rs:37` |
| sym-ba5eb2c43c17b7c5d1e9 | `observations` | function | Returns the observations exposed by `MacosInputSource`. | `src/capture/platform/macos/input.rs:227` |
| sym-c1049f62bd4f9fd89692 | `observations` | function | Returns the observations exposed by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:248` |
| sym-353c484ee921c68ef410 | `observations` | function | Returns the observations exposed by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:82` |
| sym-cdccd0ea8e9123cb3513 | `observe` | function | Returns the current observation exposed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:204` |
| sym-a3e36ca12a52a223a0c4 | `observe_callback_buffer` | function | Records an observation for callback buffer for `CaptureObservationCounters`. | `src/capture/observations.rs:52` |
| sym-f7d3912ca925866be07c | `observe_complete_snapshot` | function | Records an observation for complete snapshot for `SourceLifecycleRegistry`. | `src/capture/lifecycle_registry.rs:36` |
| sym-8f8fbc2026eedef1dda1 | `observe_dispatch_queue_full` | function | Records an observation for dispatch queue full for `CaptureObservationCounters`. | `src/capture/observations.rs:70` |
| sym-022fd20e0b5e41fce56a | `observe_dispatch_queue_full_frames` | function | Records a known number of frames lost at a bounded native or Rust delivery edge. | `src/capture/observations.rs:76` |
| sym-9cbb51e6fbef2902b38d | `observe_enqueued_frame` | function | Records an observation for enqueued frame for `CaptureObservationCounters`. | `src/capture/observations.rs:58` |
| sym-b7beacb693d60aee3290 | `observe_oversized_buffer` | function | Records an observation for oversized buffer for `CaptureObservationCounters`. | `src/capture/observations.rs:89` |
| sym-450456c740c1b596179f | `observe_pool_exhaustion` | function | Records an observation for pool exhaustion for `CaptureObservationCounters`. | `src/capture/observations.rs:64` |
| sym-45976a1abbb2588ac007 | `observe_stream_error` | function | Records an observation for stream error for `CaptureObservationCounters`. | `src/capture/observations.rs:95` |
| sym-73a6a2afe3673c227258 | `observe_timestamp_epoch_clamp` | function | Records an observation for timestamp epoch clamp for `CaptureObservationCounters`. | `src/capture/observations.rs:101` |
| sym-97dd85058ee8a8017a24 | `open` | function | Opens the resource represented by `PreparedCapture`. | `src/capture/capture_owner.rs:128` |
| sym-9fd0341145c82b3ee451 | `open` | function | Opens the resource represented by `CaptureDeliveryStartGateController`. | `src/capture/frame_stream.rs:77` |
| sym-2f821d7fbe0146feae40 | `open_metadata` | function | Returns the open metadata associated with `CaptureOwner`. | `src/capture/capture_owner.rs:242` |
| sym-253ac881e05a7dfc0b4a | `permission_epoch` | function | Returns the permission epoch held by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:200` |
| sym-9511cd04174451157d94 | `pocketstation::capture::capture_owner::join_capture_worker` | function | Joins one owned capture worker and preserves panic as a typed failure. | `src/capture/capture_owner.rs:334` |
| sym-0eb973c9ae1a8b49fb1e | `pocketstation::capture::capture_owner::prepare_capture` | function | Prepares a bounded capture owner without starting native delivery. | `src/capture/capture_owner.rs:298` |
| sym-e0ce04ad290fdad7a8b9 | `pocketstation::capture::capture_owner::prepare_capture_with_start_gate` | function | Prepares a bounded capture owner behind a caller-owned one-way start gate. | `src/capture/capture_owner.rs:306` |
| sym-098dd94d3d6620409e45 | `pocketstation::capture::events::publish_backend_failure` | function | Publishes one exact post-open backend failure without introducing another event queue or worker. | `src/capture/events.rs:280` |
| sym-c69563551656dae0f59c | `pocketstation::capture::events::source_runtime_event_channel` | function | Creates the bounded sender and receiver used for source runtime events. | `src/capture/events.rs:328` |
| sym-6f274ded6b73f261f576 | `pocketstation::capture::frame_stream::capture_delivery_start_gate` | function | Creates a closed Session-owned controller and callback-visible start gate. | `src/capture/frame_stream.rs:83` |
| sym-a3ad138af4976282293f | `pocketstation::capture::frame_stream::captured_frame_stream` | function | Wraps the supplied capture receiver as a stream of captured frames. | `src/capture/frame_stream.rs:191` |
| sym-babc87b1b6b9aeb16c0a | `pocketstation::capture::platform::macos::input::discover_input_sources_native` | function | Discovers microphone input sources through the native macOS backend. | `src/capture/platform/macos/input.rs:256` |
| sym-5bd4e29b7f4eba05db84 | `pocketstation::capture::platform::macos::macos_tap::discover_sources_native` | function | Enumerate all running processes that have audio output. Returns an empty `Vec` on macOS < 14.4 (public support floor) or on non-macOS platforms. | `src/capture/platform/macos/macos_tap.rs:87` |
| sym-a9490be832ccb13675e0 | `pocketstation::capture::platform::macos::macos_tap::tap_available` | function | Returns `true` when the CoreAudio process tap API is available. | `src/capture/platform/macos/macos_tap.rs:76` |
| sym-43501bc28a7080cc39cf | `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| sym-751f1bdabf888c4613df | `pocketstation::capture::query::discover_sources` | function | Discovers capture sources available from the local provider. | `src/capture/query.rs:85` |
| sym-fff0de4ef5a205420372 | `pocketstation::capture::query::resolve_query` | function | Resolves query for `query`. | `src/capture/query.rs:40` |
| sym-2400599b268a81f31496 | `pocketstation::capture::timeline::initialize_monotonic_timestamp_domain` | function | Initializes the process-wide capture timestamp domain from a setup thread. | `src/capture/timeline.rs:11` |
| sym-608f588747dc366db0ae | `pocketstation::capture::timeline::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain used by every capture adapter. The value is non-zero and comparable across PocketStation crates in the same process; it is never derived from a wall clock and cannot jump. | `src/capture/timeline.rs:18` |
| sym-9e73113df0713054f152 | `prepare` | function | Prepares resources required by `DesktopCaptureBackend`. | `src/capture/platform/macos/session_backend.rs:22` |
| sym-7d8fd92059fe75b9ce25 | `process_tree_scope` | function | Reports the native process boundary represented by this discovery result without making the CLI reconstruct a private capture mode. | `src/capture/identity.rs:140` |
| sym-f53955b6600b5a85ec04 | `process_tree_scope` | function | Reports the process boundary requested from the native backend. | `src/capture/selection.rs:55` |
| sym-542f581ec3f3b1083410 | `query::SourceProvider::discover` | function | Discovers the resources visible to `SourceProvider`. | `src/capture/query.rs:49` |
| sym-83ca05faf24683720bbd | `selector_persistence_scope` | function | Reports how long this discovered selector can be reused without rediscovery. The capture owner remains authoritative for opening it. | `src/capture/identity.rs:114` |
| sym-f495eaea1846fa5eb990 | `selector_persistence_scope` | function | Describes how long the selector may be reused without rediscovery. | `src/capture/selection.rs:36` |
| sym-480dc2e46b656bb6ea71 | `session_id` | function | Returns the session identifier held by `CaptureLineageSeed`. | `src/capture/capture_owner.rs:38` |
| sym-2063e5c7bdc649c7cbd5 | `snapshot` | function | Returns a point-in-time snapshot of `CaptureObservationCounters`. | `src/capture/observations.rs:113` |
| sym-b1421c39551045540b2c | `source_id` | function | Derives the immutable captured-frame identity for this resolved source. | `src/capture/identity.rs:46` |
| sym-32d2a7ed59af69cfb712 | `source_id` | function | Returns the source identifier held by `MacosInputSource`. | `src/capture/platform/macos/input.rs:223` |
| sym-4ec6139dfc44bad9c5ba | `source_id` | function | Returns the source identifier held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:255` |
| sym-67d23693651f132a604e | `source_id` | function | Returns the source identifier held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:89` |
| sym-faaa010a8fce3934ca85 | `stats` | function | Returns the current statistics for `CapturedFrameSender`. | `src/capture/frame_stream.rs:138` |
| sym-d03ce71523419d47f749 | `stats` | function | Returns the current statistics for `CapturedFrameStream`. | `src/capture/frame_stream.rs:179` |
| sym-f9518dff6232229a6843 | `stem_id` | function | Returns the stem identifier held by `CaptureLineageSeed`. | `src/capture/capture_owner.rs:42` |
| sym-3f7f81492dc47ddf5f22 | `stop_and_join` | function | Stops `CaptureOwner`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:264` |
| sym-bc33a88ba937e5600efe | `stop_and_join` | function | Stops `MacosInputSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/input.rs:235` |
| sym-7c11be2e65aaa2fdfdfa | `stop_and_join` | function | Stops `SystemLoopbackSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/loopback.rs:274` |
| sym-0e5c4a2c142daf106c45 | `stop_and_join` | function | Stops `DesktopCaptureSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/mod.rs:103` |
| sym-7096dce6e4b8356e1b35 | `try_next` | function | Attempts to next through `CapturedFrameStream`. | `src/capture/frame_stream.rs:160` |
| sym-760838edced2ebbd0123 | `try_next_lineaged_frame` | function | Attempts to next lineaged frame through `CaptureOwner`. | `src/capture/capture_owner.rs:213` |
| sym-1eb7daad98e7e6a2fcdb | `try_recv` | function | Attempts to receive the next value from `SourceRuntimeEventReceiver` without waiting. | `src/capture/events.rs:304` |
| sym-43780d528cfa4194d637 | `try_recv_runtime_event` | function | Attempts to recv runtime event through `CaptureOwner`. | `src/capture/capture_owner.rs:205` |
| sym-07196501368e626e171a | `try_send` | function | Publishes from a capture worker without blocking. When the bounded control channel is full, the newest event is dropped and counted. | `src/capture/events.rs:232` |
| sym-9fb62ce79920ca104da3 | `try_send` | function | Attempts to send a value through `CapturedFrameSender` without waiting for capacity. | `src/capture/frame_stream.rs:109` |
| sym-4da030ade0b9f5f20efb | `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| sym-2e9979a7622aa06e45c4 | `identity` | module | Stable source identity and source discovery state. | `src/capture/identity.rs:1` |
| sym-aa210816831d394482ce | `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| sym-8e4bf95b7cdd0b95ce18 | `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| sym-8fcc600b77a4781feb1f | `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| sym-bda1f6e65224cdffaaba | `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| sym-dff2ac7d1dec9ec6fe1a | `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| sym-714e41ee82c6f1511e60 | `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |
| sym-fccbb77f706d93ad2d08 | `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| sym-78605017adba4fbddc4a | `pocketstation::capture::capture_owner::CaptureLineageSeed` | struct | Stable session and stem identity assigned before an exact source is opened. | `src/capture/capture_owner.rs:25` |
| sym-3c0c69c620f6dd6deda0 | `pocketstation::capture::capture_owner::CaptureObservationReceipt` | struct | Retains the identity and observation access returned for capture observation. | `src/capture/capture_owner.rs:167` |
| sym-6ae72c6f6afc98127bca | `pocketstation::capture::capture_owner::CaptureOpenMetadata` | struct | Authoritative lineage state established only after native capture opens. | `src/capture/capture_owner.rs:49` |
| sym-a35ed17eb84ee0dd6c5b | `pocketstation::capture::capture_owner::CaptureOwner` | struct | RAII owner for native capture, its bounded frame stream, and runtime events. | `src/capture/capture_owner.rs:194` |
| sym-4a8719ab63c1ef34fe48 | `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| sym-4c867c58f7305d568f6f | `pocketstation::capture::capture_owner::CapturePrepareRequest` | struct | Setup-time request for one bounded callback-oriented capture owner. | `src/capture/capture_owner.rs:61` |
| sym-9c2561aa4757d483954b | `pocketstation::capture::capture_owner::CaptureStopOutcome` | struct | Final observations returned only after backend stop and join complete. | `src/capture/capture_owner.rs:185` |
| sym-ffb3c52b846161e5c59a | `pocketstation::capture::capture_owner::PreparedCapture` | struct | Prepared capture plus its preallocated delivery endpoints. | `src/capture/capture_owner.rs:119` |
| sym-a79b62eb7cf5031444c1 | `pocketstation::capture::events::CaptureRuntimeFailure` | struct | Reports a capture runtime failure. | `src/capture/events.rs:47` |
| sym-fdac50d90e108fc47f99 | `pocketstation::capture::events::SourceGeneration` | struct | Identifies one appearance generation of a capture source across loss and reappearance. | `src/capture/events.rs:12` |
| sym-b1b9f5177636a52d174c | `pocketstation::capture::events::SourceRuntimeEventObservationHandle` | struct | Owns bounded access to source runtime event observation. | `src/capture/events.rs:200` |
| sym-c79856166f9cb5662eac | `pocketstation::capture::events::SourceRuntimeEventObservations` | struct | Reports the source runtime event observations collected at an observation boundary. | `src/capture/events.rs:111` |
| sym-820c883f3a87ad3206b6 | `pocketstation::capture::events::SourceRuntimeEventReceiver` | struct | Receives source runtime event values across its declared ownership boundary. | `src/capture/events.rs:298` |
| sym-18ae885cdb68fbf9848d | `pocketstation::capture::events::SourceRuntimeEventSender` | struct | Sends source runtime event values across its declared ownership boundary. | `src/capture/events.rs:224` |
| sym-0607c39706b2cfbfb184 | `pocketstation::capture::frame_stream::CaptureDeliveryStartGate` | struct | Read-only one-way start barrier checked by capture delivery callbacks. | `src/capture/frame_stream.rs:54` |
| sym-ba87f1a978171c732fee | `pocketstation::capture::frame_stream::CaptureDeliveryStartGateController` | struct | Session-owned authority that opens one capture delivery start gate. | `src/capture/frame_stream.rs:72` |
| sym-4e0d846572401687d759 | `pocketstation::capture::frame_stream::CapturedFrameObservationHandle` | struct | Owns bounded access to captured frame observation. | `src/capture/frame_stream.rs:31` |
| sym-a36525bd590a744112f0 | `pocketstation::capture::frame_stream::CapturedFrameSender` | struct | Single-producer endpoint passed into a platform capture callback. | `src/capture/frame_stream.rs:102` |
| sym-db323b040cf0438fd279 | `pocketstation::capture::frame_stream::CapturedFrameStream` | struct | Non-blocking consumer for captured `AudioFrame`s. | `src/capture/frame_stream.rs:154` |
| sym-68f1c481c04b6f6aea69 | `pocketstation::capture::frame_stream::CapturedFrameStreamStats` | struct | Reports the captured frame stream stats collected at an observation boundary. | `src/capture/frame_stream.rs:17` |
| sym-6c2c56e62ec561ad6b99 | `pocketstation::capture::identity::CaptureSource` | struct | Owns production of capture values and its lifecycle state. | `src/capture/identity.rs:82` |
| sym-a29db69c4e99d0c44e31 | `pocketstation::capture::identity::StableSourceId` | struct | Uniquely identifies stable source within its PocketStation ownership scope. | `src/capture/identity.rs:26` |
| sym-de769116ed5e8392b258 | `pocketstation::capture::lifecycle_registry::SourceLifecycleRegistry` | struct | Assigns source generations across complete discovery snapshots. | `src/capture/lifecycle_registry.rs:31` |
| sym-66b06d778e4b81bce7de | `pocketstation::capture::observations::CaptureObservationCounters` | struct | Setup-time cloneable handle; every observation is one relaxed atomic operation and remains allocation-free, lock-free, and log-free. | `src/capture/observations.rs:46` |
| sym-715462cb7417f2b40112 | `pocketstation::capture::observations::CaptureObservationHandle` | struct | Owns bounded access to capture observation. | `src/capture/observations.rs:32` |
| sym-1da6281c321b80d52835 | `pocketstation::capture::observations::CaptureObservations` | struct | Reports the capture observations collected at an observation boundary. | `src/capture/observations.rs:8` |
| sym-1e7720cfde78f1f8b2fe | `pocketstation::capture::platform::macos::DesktopCaptureSource` | struct | Owns production of desktop capture values and its lifecycle state. | `src/capture/platform/macos/mod.rs:33` |
| sym-ff885ee5cdaf51d69b81 | `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:68` |
| sym-9a418761411cb1aa102e | `pocketstation::capture::platform::macos::loopback::SystemLoopbackSource` | struct | Manages a macOS loopback capture session. | `src/capture/platform/macos/loopback.rs:53` |
| sym-941215720358956ba0e2 | `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |
| sym-d56cebd8ad916b797d1f | `pocketstation::capture::query::LocalSourceProvider` | struct | Discovers and resolves capture sources through the target platform backend. | `src/capture/query.rs:52` |
| sym-0ed3e83b4ab45dd014cb | `pocketstation::capture::timeline::CaptureSampleTimeline` | struct | Source-time clock for capture streams whose media cadence is defined by the number of sample frames produced by the device. | `src/capture/timeline.rs:31` |
| sym-38c79b01d994239517d7 | `CaptureDelivery::frame_sender` | struct_field | Stores the frame sender used by `CaptureDelivery`. | `src/capture/capture_owner.rs:74` |
| sym-184f1403da6183166817 | `CaptureDelivery::runtime_event_sender` | struct_field | Stores the runtime event sender used by `CaptureDelivery`. | `src/capture/capture_owner.rs:75` |
| sym-cfcf6d455b4ac6269f0a | `CaptureObservations::callback_buffers_total` | struct_field | Counts the total number of callback buffers observed by `CaptureObservations`. | `src/capture/observations.rs:9` |
| sym-558b4639e8c15e8c4933 | `CaptureObservations::dispatch_queue_full_total` | struct_field | Counts the total number of dispatch queue full observed by `CaptureObservations`. | `src/capture/observations.rs:12` |
| sym-f1354342b21d545e43d0 | `CaptureObservations::frames_enqueued_total` | struct_field | Counts the total number of frames enqueued observed by `CaptureObservations`. | `src/capture/observations.rs:10` |
| sym-f2cd3e04115cceb22798 | `CaptureObservations::invalid_buffer_total` | struct_field | Counts the total number of invalid buffer observed by `CaptureObservations`. | `src/capture/observations.rs:13` |
| sym-b558b3b06363e5fb40e4 | `CaptureObservations::oversized_buffer_total` | struct_field | Counts the total number of oversized buffer observed by `CaptureObservations`. | `src/capture/observations.rs:14` |
| sym-4ca717256cb392393317 | `CaptureObservations::pool_exhausted_total` | struct_field | Counts the total number of pool exhausted observed by `CaptureObservations`. | `src/capture/observations.rs:11` |
| sym-41f6c642b6797e94e80e | `CaptureObservations::stream_errors_total` | struct_field | Counts the total number of stream errors observed by `CaptureObservations`. | `src/capture/observations.rs:15` |
| sym-b6eaec6997de39decfff | `CaptureObservations::timestamp_epoch_clamps_total` | struct_field | Counts the total number of timestamp epoch clamps observed by `CaptureObservations`. | `src/capture/observations.rs:16` |
| sym-e84bcedb6149e2c78a6f | `CaptureOpenMetadata::clock_id` | struct_field | Identifies the clock identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:53` |
| sym-fc7fa7010a37b17da2c5 | `CaptureOpenMetadata::discontinuity_epoch` | struct_field | Stores the discontinuity epoch used by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:55` |
| sym-52b2fc6425ad3e13ebb2 | `CaptureOpenMetadata::permission_epoch` | struct_field | Stores the permission epoch used by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:56` |
| sym-098564a8d55bed3f48d3 | `CaptureOpenMetadata::session_id` | struct_field | Identifies the session identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:50` |
| sym-ac5bd4262b98592d78f5 | `CaptureOpenMetadata::source_generation` | struct_field | Stores the source generation used by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:54` |
| sym-e7013d71bbec92f27427 | `CaptureOpenMetadata::source_id` | struct_field | Identifies the source identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:51` |
| sym-6ac703dc19190afa5202 | `CaptureOpenMetadata::stem_id` | struct_field | Identifies the stem identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:52` |
| sym-375e0cbb3cc9eacb7aa1 | `CaptureOwnerObservations::backend` | struct_field | Stores the backend used by `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:161` |
| sym-c507584e4bbc449174f2 | `CaptureOwnerObservations::frame_stream` | struct_field | Stores the frame stream used by `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:162` |
| sym-87dcecd21143dc6e9434 | `CaptureOwnerObservations::runtime_events` | struct_field | Stores the runtime events used by `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:163` |
| sym-778180323cfdf5e12eb8 | `CapturePrepareRequest::frame_capacity_frames` | struct_field | Sets the frame capacity frames available to `CapturePrepareRequest`. | `src/capture/capture_owner.rs:64` |
| sym-7207623794ba4a77f51d | `CapturePrepareRequest::lineage_seed` | struct_field | Stores the lineage seed used by `CapturePrepareRequest`. | `src/capture/capture_owner.rs:63` |
| sym-ed8aef993492af565c3e | `CapturePrepareRequest::mode` | struct_field | Stores the mode used by `CapturePrepareRequest`. | `src/capture/capture_owner.rs:62` |
| sym-3fe909146d9788d27014 | `CapturePrepareRequest::runtime_event_capacity_events` | struct_field | Sets the runtime event capacity events available to `CapturePrepareRequest`. | `src/capture/capture_owner.rs:65` |
| sym-c147acda6c2632026425 | `CaptureSampleTimelineError::SourcePositionMovedBackward::expected_at_least` | struct_field | Stores the expected at least used by `SourcePositionMovedBackward`. | `src/capture/timeline.rs:45` |
| sym-75b22b8047340c14d50f | `CaptureSampleTimelineError::SourcePositionMovedBackward::observed` | struct_field | Stores the observed used by `SourcePositionMovedBackward`. | `src/capture/timeline.rs:46` |
| sym-2ebcb5bf416a4de25c48 | `CaptureStopOutcome::observations` | struct_field | Carries the observations collected for `CaptureStopOutcome`. | `src/capture/capture_owner.rs:186` |
| sym-f7ddbb59cf345aefc3d4 | `CapturedFrameStreamStats::delivered_frames` | struct_field | Stores the delivered frames used by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:18` |
| sym-599900d91119a9fa314a | `CapturedFrameStreamStats::dropped_newest_frames` | struct_field | Stores the dropped newest frames used by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:19` |
| sym-bba867916e478ded5b1a | `CapturedFrameStreamStats::frames_discarded_before_start_total` | struct_field | Counts the total number of frames discarded before start observed by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:20` |
| sym-7664c4456b7b644bd5cb | `authorization::CaptureAuthorizationSnapshot::application_policy` | struct_field | Stores the application policy used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:20` |
| sym-637ec0825fe64893013b | `authorization::CaptureAuthorizationSnapshot::capability` | struct_field | Stores the capability used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:18` |
| sym-43c8c0739b846d1f9934 | `authorization::CaptureAuthorizationSnapshot::capture_scope` | struct_field | Stores the capture scope used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:22` |
| sym-bfb980861013d712937b | `authorization::CaptureAuthorizationSnapshot::identity_strength` | struct_field | Stores the identity strength used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:23` |
| sym-073af6c2662e8cb30e5d | `authorization::CaptureAuthorizationSnapshot::observed_at_ns` | struct_field | Stores the observed at value for `CaptureAuthorizationSnapshot`, in nanoseconds. | `src/capture/authorization.rs:25` |
| sym-74695c5f870e4a669f17 | `authorization::CaptureAuthorizationSnapshot::open_outcome` | struct_field | Stores the open outcome used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:26` |
| sym-78476799989c93fd00f8 | `authorization::CaptureAuthorizationSnapshot::os_permission` | struct_field | Stores the os permission used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:19` |
| sym-da440180eacfdaa511cb | `authorization::CaptureAuthorizationSnapshot::permission_epoch` | struct_field | Stores the permission epoch used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:24` |
| sym-3188d8fad2d775859a43 | `authorization::CaptureAuthorizationSnapshot::session_grant` | struct_field | Stores the session grant used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:21` |
| sym-37fdbe707be788a82fb5 | `authorization::CaptureError::BackendSetupRequired::action` | struct_field | Stores the action used by `BackendSetupRequired`. | `src/capture/authorization.rs:298` |
| sym-6900d16093aa9e0be4b3 | `authorization::CaptureError::BackendSetupRequired::backend` | struct_field | Stores the backend used by `BackendSetupRequired`. | `src/capture/authorization.rs:297` |
| sym-c2ed0449b0dea0b67ecb | `authorization::CaptureError::BackendStatus::operation` | struct_field | Stores the operation used by `BackendStatus`. | `src/capture/authorization.rs:304` |
| sym-bf639884f816cbf95df4 | `authorization::CaptureError::BackendStatus::status_code` | struct_field | Stores the status code used by `BackendStatus`. | `src/capture/authorization.rs:305` |
| sym-b40f7f41ec9bc37c240e | `authorization::CaptureError::CaptureWorkerPanicked::worker` | struct_field | Stores the worker used by `CaptureWorkerPanicked`. | `src/capture/authorization.rs:316` |
| sym-0b436b7ab46d1c6d4e5f | `authorization::CaptureError::PermissionDenied::operation` | struct_field | Stores the operation used by `PermissionDenied`. | `src/capture/authorization.rs:301` |
| sym-35d71fd94c50c32dfab7 | `authorization::CaptureError::SourceUnavailable::stable_key` | struct_field | Stores the stable key used by `SourceUnavailable`. | `src/capture/authorization.rs:308` |
| sym-3dccdf6602fe6deb6c67 | `authorization::CapturePermissionTransition::current` | struct_field | Stores the current used by `CapturePermissionTransition`. | `src/capture/authorization.rs:171` |
| sym-260e3b0756d352d4cdde | `authorization::CapturePermissionTransition::kind` | struct_field | Stores the kind used by `CapturePermissionTransition`. | `src/capture/authorization.rs:169` |
| sym-a53df0e8bcb46547e36a | `authorization::CapturePermissionTransition::permission_epoch` | struct_field | Stores the permission epoch used by `CapturePermissionTransition`. | `src/capture/authorization.rs:172` |
| sym-747552518d78c8ec1099 | `authorization::CapturePermissionTransition::previous` | struct_field | Stores the previous used by `CapturePermissionTransition`. | `src/capture/authorization.rs:170` |
| sym-6e5e1ff83c5743c0996e | `authorization::CaptureScope::ExactApplication::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplication`. | `src/capture/authorization.rs:249` |
| sym-5ed06979ef16447a6b13 | `authorization::CaptureScope::ExactInputDevice::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactInputDevice`. | `src/capture/authorization.rs:250` |
| sym-6d84e3217f4d63c8514b | `authorization::CaptureScope::ExactOutputDevice::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactOutputDevice`. | `src/capture/authorization.rs:251` |
| sym-7b35e6776a9ad0f6857e | `events::CaptureRuntimeFailure::error_class` | struct_field | Stores the error class used by `CaptureRuntimeFailure`. | `src/capture/events.rs:49` |
| sym-3807587d486e070e9441 | `events::CaptureRuntimeFailure::operation` | struct_field | Stores the operation used by `CaptureRuntimeFailure`. | `src/capture/events.rs:48` |
| sym-bfe85cfae39c03c55b2e | `events::CaptureRuntimeFailureClass::BackendClass::class` | struct_field | Stores the class used by `BackendClass`. | `src/capture/events.rs:43` |
| sym-dfb974cf26237ca42c98 | `events::CaptureRuntimeFailureClass::PlatformStatus::status_code` | struct_field | Stores the status code used by `PlatformStatus`. | `src/capture/events.rs:42` |
| sym-0f09d21fcfb9896181e4 | `events::SourceRuntimeEvent::BackendFailure::failure` | struct_field | Carries the failure reported by `BackendFailure`. | `src/capture/events.rs:63` |
| sym-8cc396e9c576990b1626 | `events::SourceRuntimeEvent::BackendFailure::generation` | struct_field | Stores the generation used by `BackendFailure`. | `src/capture/events.rs:62` |
| sym-5e338da4ad5b5a28605e | `events::SourceRuntimeEvent::BackendFailure::stable_id` | struct_field | Identifies the stable identifier recorded by `BackendFailure`. | `src/capture/events.rs:61` |
| sym-29a0a855d7399dcfa721 | `events::SourceRuntimeEvent::SourceUnavailable::failure` | struct_field | Carries the failure reported by `SourceUnavailable`. | `src/capture/events.rs:58` |
| sym-930c8762a56effaecc0d | `events::SourceRuntimeEvent::SourceUnavailable::generation` | struct_field | Stores the generation used by `SourceUnavailable`. | `src/capture/events.rs:56` |
| sym-b6254ed8c91ae7794dca | `events::SourceRuntimeEvent::SourceUnavailable::recovery_requirement` | struct_field | Stores the recovery requirement used by `SourceUnavailable`. | `src/capture/events.rs:57` |
| sym-700e9374d1d49f5cd2a5 | `events::SourceRuntimeEvent::SourceUnavailable::stable_id` | struct_field | Identifies the stable identifier recorded by `SourceUnavailable`. | `src/capture/events.rs:55` |
| sym-e9e12b180de73f91fada | `events::SourceRuntimeEventObservations::capacity_event_count` | struct_field | Sets the capacity event count available to `SourceRuntimeEventObservations`. | `src/capture/events.rs:112` |
| sym-576f1fa5808bb74f0e50 | `events::SourceRuntimeEventObservations::depth_events` | struct_field | Reports the depth events observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:115` |
| sym-97221a2cf4ff68787c57 | `events::SourceRuntimeEventObservations::depth_owned_bytes` | struct_field | Stores the depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:116` |
| sym-e911cc094999d6f2cce8 | `events::SourceRuntimeEventObservations::events_dropped_oversized_total` | struct_field | Counts the total number of events dropped oversized observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:120` |
| sym-83e9ab76972024638989 | `events::SourceRuntimeEventObservations::events_dropped_total` | struct_field | Counts the total number of events dropped observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:119` |
| sym-a69a3772f376c36ea735 | `events::SourceRuntimeEventObservations::events_enqueued_total` | struct_field | Counts the total number of events enqueued observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:118` |
| sym-5e2b5f1b156347230739 | `events::SourceRuntimeEventObservations::maximum_buffered_owned_bytes` | struct_field | Stores the maximum buffered owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:114` |
| sym-e9a15487b85ce9efc151 | `events::SourceRuntimeEventObservations::maximum_event_owned_bytes` | struct_field | Stores the maximum event owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:113` |
| sym-4466dd4912bee96392a9 | `events::SourceRuntimeEventObservations::peak_depth_owned_bytes` | struct_field | Stores the peak depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:117` |
| sym-e4030a6410f700cc5dcc | `identity::CaptureSource::app_id` | struct_field | Identifies the app identifier recorded by `CaptureSource`. | `src/capture/identity.rs:86` |
| sym-b739ff2f382d7dcbf9c8 | `identity::CaptureSource::channels` | struct_field | Stores the channels used by `CaptureSource`. | `src/capture/identity.rs:90` |
| sym-d36900aa9174e272ab96 | `identity::CaptureSource::device_uid` | struct_field | Stores the device uid used by `CaptureSource`. | `src/capture/identity.rs:87` |
| sym-7b5c76d328c34c5a816d | `identity::CaptureSource::name` | struct_field | Stores the name used by `CaptureSource`. | `src/capture/identity.rs:84` |
| sym-d40befabb7e0de351b5a | `identity::CaptureSource::process_id` | struct_field | Identifies the process identifier recorded by `CaptureSource`. | `src/capture/identity.rs:85` |
| sym-af5d68e009d0a63951c9 | `identity::CaptureSource::sample_rate_hz` | struct_field | Stores the sample rate value for `CaptureSource`, in hertz. | `src/capture/identity.rs:89` |
| sym-a738e42405aa73b5ce30 | `identity::CaptureSource::stable_id` | struct_field | Identifies the stable identifier recorded by `CaptureSource`. | `src/capture/identity.rs:83` |
| sym-4831e34c203fe51d8896 | `identity::CaptureSource::state` | struct_field | Stores the state used by `CaptureSource`. | `src/capture/identity.rs:88` |
| sym-3e2e7e8983dd10ab814d | `identity::StableSourceId::kind` | struct_field | Stores the kind used by `StableSourceId`. | `src/capture/identity.rs:28` |
| sym-4abf90caaf0d81f1b139 | `identity::StableSourceId::platform` | struct_field | Stores the platform used by `StableSourceId`. | `src/capture/identity.rs:27` |
| sym-ba9aa6872ba7a3de4ecf | `identity::StableSourceId::stable_key` | struct_field | Stores the stable key used by `StableSourceId`. | `src/capture/identity.rs:29` |
| sym-456d36fc5ce16183d24f | `lifecycle_registry::SourceGenerationTransition::Disappeared::generation` | struct_field | Stores the generation used by `Disappeared`. | `src/capture/lifecycle_registry.rs:11` |
| sym-b726b1261e3ddf381f57 | `lifecycle_registry::SourceGenerationTransition::Disappeared::stable_id` | struct_field | Identifies the stable identifier recorded by `Disappeared`. | `src/capture/lifecycle_registry.rs:10` |
| sym-9b6d08a34d1db71df987 | `lifecycle_registry::SourceGenerationTransition::Reappeared::generation` | struct_field | Stores the generation used by `Reappeared`. | `src/capture/lifecycle_registry.rs:16` |
| sym-4024e6be8248dd03cd51 | `lifecycle_registry::SourceGenerationTransition::Reappeared::previous_generation` | struct_field | Stores the previous generation used by `Reappeared`. | `src/capture/lifecycle_registry.rs:15` |
| sym-db0383bcd447d6a31389 | `lifecycle_registry::SourceGenerationTransition::Reappeared::stable_id` | struct_field | Identifies the stable identifier recorded by `Reappeared`. | `src/capture/lifecycle_registry.rs:14` |
| sym-84282b1095338b4ef024 | `selection::CaptureMode::ExactApplication::process_id` | struct_field | Identifies the process identifier recorded by `ExactApplication`. | `src/capture/selection.rs:22` |
| sym-f8d6a4d0f6384da1cf41 | `selection::CaptureMode::ExactApplication::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplication`. | `src/capture/selection.rs:23` |
| sym-33273e59149260413aee | `selection::CaptureMode::ExactApplicationStable::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplicationStable`. | `src/capture/selection.rs:26` |
| sym-bc4d349ab8b98a15a530 | `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| sym-8f6c56609c608d400c5f | `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| sym-daefb89a465f85894b9d | `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| sym-0b8e49c6c2e292c1b1a7 | `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| sym-9d85a11352d79f01f1d3 | `pocketstation::capture::authorization::ApplicationPolicyObservation::Allowed` | variant | Selects allowed behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:232` |
| sym-fe9d9eac044ef1f58ceb | `pocketstation::capture::authorization::ApplicationPolicyObservation::Denied` | variant | Selects denied behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:233` |
| sym-ac58d63ca2febd6a15ca | `pocketstation::capture::authorization::ApplicationPolicyObservation::NotApplicable` | variant | Selects not applicable behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:235` |
| sym-44e567dfd2c2f2b9b542 | `pocketstation::capture::authorization::ApplicationPolicyObservation::NotObservable` | variant | Selects not observable behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:234` |
| sym-8c1f79805150c9ccc20f | `pocketstation::capture::authorization::CaptureCapabilityState::Available` | variant | Identifies the available state or stage represented by `CaptureCapabilityState`. | `src/capture/authorization.rs:146` |
| sym-bca501cab1774fe943c1 | `pocketstation::capture::authorization::CaptureCapabilityState::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/capture/authorization.rs:147` |
| sym-66ee5f7f8569898ee0e5 | `pocketstation::capture::authorization::CaptureCapabilityState::Unsupported` | variant | Reports that the requested operation is unsupported. | `src/capture/authorization.rs:148` |
| sym-7419845e9f6b90c0b1da | `pocketstation::capture::authorization::CaptureError::BackendInit` | variant | Reported when the owning operation encounters backend init. | `src/capture/authorization.rs:294` |
| sym-7e170ddda34f65f4e3a5 | `pocketstation::capture::authorization::CaptureError::BackendSetupRequired` | variant | Reported when the owning operation encounters backend setup required. | `src/capture/authorization.rs:296` |
| sym-730ae118b5ac64faa504 | `pocketstation::capture::authorization::CaptureError::BackendStatus` | variant | Reported when the owning operation encounters backend status. | `src/capture/authorization.rs:303` |
| sym-e52fe39c34aba3580d17 | `pocketstation::capture::authorization::CaptureError::CaptureWorkerPanicked` | variant | Reported when the owning operation encounters capture worker panicked. | `src/capture/authorization.rs:316` |
| sym-0426859199cf142736d8 | `pocketstation::capture::authorization::CaptureError::InvalidRuntimeEventCapacity` | variant | Reported when the owning operation encounters invalid runtime event capacity. | `src/capture/authorization.rs:314` |
| sym-4228008a504ef2c88e48 | `pocketstation::capture::authorization::CaptureError::InvalidStreamCapacity` | variant | Reported when the owning operation encounters invalid stream capacity. | `src/capture/authorization.rs:312` |
| sym-cc6306620d673442212b | `pocketstation::capture::authorization::CaptureError::ModeUnsupported` | variant | Reported when the owning operation encounters mode unsupported. | `src/capture/authorization.rs:310` |
| sym-be0fe58f7d919ff27687 | `pocketstation::capture::authorization::CaptureError::NotSupported` | variant | Reported when the owning operation encounters not supported. | `src/capture/authorization.rs:292` |
| sym-56efe5b4102fb9c13d9c | `pocketstation::capture::authorization::CaptureError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:301` |
| sym-c4ccd46a71b412131e37 | `pocketstation::capture::authorization::CaptureError::SourceUnavailable` | variant | Reported when the owning operation encounters source unavailable. | `src/capture/authorization.rs:308` |
| sym-32a65ffe51c76a00fb56 | `pocketstation::capture::authorization::CaptureOpenOutcome::BackendFailed` | variant | Identifies the backend failed state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:286` |
| sym-579f00235bce31081f79 | `pocketstation::capture::authorization::CaptureOpenOutcome::NotAttempted` | variant | Identifies the not attempted state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:282` |
| sym-db271f950d22d660ccff | `pocketstation::capture::authorization::CaptureOpenOutcome::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:284` |
| sym-f02538797584a16ec25a | `pocketstation::capture::authorization::CaptureOpenOutcome::SourceUnavailable` | variant | Identifies the source unavailable state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:285` |
| sym-eb44f98338206b8a62cd | `pocketstation::capture::authorization::CaptureOpenOutcome::Succeeded` | variant | Identifies the succeeded state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:283` |
| sym-9dcb10a46b543c5d91e5 | `pocketstation::capture::authorization::CaptureScope::ExactApplication` | variant | Selects exact application behavior for `CaptureScope`. | `src/capture/authorization.rs:249` |
| sym-f1578f1bc1c096ad11cb | `pocketstation::capture::authorization::CaptureScope::ExactInputDevice` | variant | Selects exact input device behavior for `CaptureScope`. | `src/capture/authorization.rs:250` |
| sym-2ba5ef3cc738a6d31f5b | `pocketstation::capture::authorization::CaptureScope::ExactOutputDevice` | variant | Selects exact output device behavior for `CaptureScope`. | `src/capture/authorization.rs:251` |
| sym-167981219e08deb8c616 | `pocketstation::capture::authorization::CaptureScope::SystemMix` | variant | Selects system mix behavior for `CaptureScope`. | `src/capture/authorization.rs:252` |
| sym-4c0055e7ec9408115cae | `pocketstation::capture::authorization::CaptureSessionGrant::Denied` | variant | Represents the denied alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:242` |
| sym-0cf6353da61564c0ca58 | `pocketstation::capture::authorization::CaptureSessionGrant::GrantedByExplicitSelection` | variant | Represents the granted by explicit selection alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:241` |
| sym-f4078600b4352349cee1 | `pocketstation::capture::authorization::CaptureSessionGrant::NotEvaluated` | variant | Represents the not evaluated alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:243` |
| sym-1fc87c8305313531efe6 | `pocketstation::capture::authorization::PermissionObservation::Allowed` | variant | Represents the allowed alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:154` |
| sym-facf3ff04a88c04996da | `pocketstation::capture::authorization::PermissionObservation::Denied` | variant | Represents the denied alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:155` |
| sym-4c790e42737adb1f1c14 | `pocketstation::capture::authorization::PermissionObservation::NotApplicable` | variant | Represents the not applicable alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:160` |

## Interpretation

The **Capture API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Capture failures](/docs/errors/capture.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)

## Evidence boundary

The claims on **Capture API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/mod.rs:1-65` (`DIRECT`)

For **Capture API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
