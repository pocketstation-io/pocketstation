# Capture API

<!-- claims: CLM-REF-003-SCOPE-001,CLM-REF-003-TEXT-001,CLM-REF-003-TEXT-002,CLM-REF-003-SOURCE-001 -->

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
| sym-f490a9028837996edb07 | `INITIAL` | assoc_const | Provides the initial value for `PermissionEpoch`. | `src/capture/authorization.rs:270` |
| sym-b8894596b234e7b11358 | `INITIAL` | assoc_const | Provides the initial value for `SourceGeneration`. | `src/capture/events.rs:15` |
| sym-e70b2bf8528a2a7977f6 | `pocketstation::capture::capture_owner::CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Monotonic timestamp domain used by native capture backends. | `src/capture/capture_owner.rs:20` |
| sym-dcc09b0402dcc4a2b2e8 | `pocketstation::capture::events::MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES` | constant | Maximum heap storage retained by one queued capture-runtime event. | `src/capture/events.rs:72` |
| sym-fcd611770bad7c70afe3 | `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| sym-e47b2044d4d2d7687620 | `pocketstation::capture::authorization::CaptureCapabilityState` | enum | Selects the capture capability state used by PocketStation. | `src/capture/authorization.rs:145` |
| sym-fc12bf3aac349548522a | `pocketstation::capture::authorization::CaptureError` | enum | Classifies failures surfaced by capture operations. | `src/capture/authorization.rs:290` |
| sym-1890ca72d0990a96bb9b | `pocketstation::capture::authorization::CaptureOpenOutcome` | enum | Classifies the observable capture open outcome. | `src/capture/authorization.rs:281` |
| sym-97ee84b50bcbf0f9a179 | `pocketstation::capture::authorization::CaptureScope` | enum | Selects the capture scope used by PocketStation. | `src/capture/authorization.rs:248` |
| sym-5437ce70388c13d381cb | `pocketstation::capture::authorization::CaptureSessionGrant` | enum | Reports the Session-specific authorization available for capture. | `src/capture/authorization.rs:240` |
| sym-aa65d77d9c53f0f321ac | `pocketstation::capture::authorization::PermissionObservation` | enum | Classifies the observable permission observation. | `src/capture/authorization.rs:153` |
| sym-664a9f3276c944fb9071 | `pocketstation::capture::authorization::SourceIdentityStrength` | enum | Classifies how reliably a capture source identity binds to the same resource. | `src/capture/authorization.rs:257` |
| sym-a11764513121614cd19c | `pocketstation::capture::events::CaptureRuntimeFailureClass` | enum | Classifies the platform, permission, source, or worker cause of a capture failure. | `src/capture/events.rs:40` |
| sym-37ae7422bb95d06bd649 | `pocketstation::capture::events::SourceLifecycleEventKind` | enum | Selects the source lifecycle event kind used by PocketStation. | `src/capture/events.rs:25` |
| sym-12f3cc54979ed8d2e6b7 | `pocketstation::capture::events::SourceRecoveryRequirement` | enum | Selects the source recovery requirement used by PocketStation. | `src/capture/events.rs:35` |
| sym-1c3a6ae15cada35f369d | `pocketstation::capture::events::SourceRuntimeEvent` | enum | Classifies the observable source runtime event. | `src/capture/events.rs:53` |
| sym-d5525d995bef9e7d0117 | `pocketstation::capture::events::SourceRuntimeEventDelivery` | enum | Reports whether a source-runtime event was delivered, dropped, or rejected. | `src/capture/events.rs:96` |
| sym-58b4705ef8fa89213d0b | `pocketstation::capture::events::SourceRuntimeEventReceive` | enum | Reports the outcome of receiving a source-runtime event. | `src/capture/events.rs:104` |
| sym-4e7b74129f3699c44fdf | `pocketstation::capture::frame_stream::CapturedFrameDelivery` | enum | Reports whether a captured frame was accepted, dropped, or rejected by delivery. | `src/capture/frame_stream.rs:10` |
| sym-0ac7ee0914d008935e77 | `pocketstation::capture::identity::SourceKind` | enum | Selects the source kind used by PocketStation. | `src/capture/identity.rs:9` |
| sym-113f3cbbd2a52a3a398a | `pocketstation::capture::identity::SourceState` | enum | Selects the source state used by PocketStation. | `src/capture/identity.rs:17` |
| sym-e92f5dbaf28b24d71288 | `pocketstation::capture::lifecycle_registry::SourceGenerationTransition` | enum | Records whether a capture source disappeared, reappeared, or changed generation. | `src/capture/lifecycle_registry.rs:8` |
| sym-e96fae7f5820c2e17de1 | `pocketstation::capture::query::SourceQuery` | enum | Describes the source kind and optional application or device selector used for discovery. | `src/capture/query.rs:13` |
| sym-548c61f75ace7e30e746 | `pocketstation::capture::selection::CaptureMode` | enum | Selects the capture mode used by PocketStation. | `src/capture/selection.rs:16` |
| sym-c18a0760716b2deb267f | `pocketstation::capture::selection::InputDeviceSelector` | enum | Selects either the default input device or one exact device identity. | `src/capture/selection.rs:9` |
| sym-72d1ae94cff22bf30b0f | `pocketstation::capture::selection::ProcessTreeScope` | enum | Selects the process tree scope used by PocketStation. | `src/capture/selection.rs:83` |
| sym-c690fdfd5adff7b4edd2 | `pocketstation::capture::selection::SelectorPersistenceScope` | enum | Selects the selector persistence scope used by PocketStation. | `src/capture/selection.rs:73` |
| sym-f92ee6c928a919df45a7 | `pocketstation::capture::timeline::CaptureSampleTimelineError` | enum | Classifies failures surfaced by capture sample timeline operations. | `src/capture/timeline.rs:41` |
| sym-8c1e01f6032ee9f58e34 | `ActiveCaptureBackend::observation_handle` | function | Returns a handle for reading observations from `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:107` |
| sym-181be60c05b799d9aa88 | `ActiveCaptureBackend::observations` | function | Returns the observations exposed by `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:109` |
| sym-bb192a71775dad273804 | `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| sym-d5183a64d1a21d804af5 | `ActiveCaptureBackend::stop_and_join` | function | Stops `ActiveCaptureBackend`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:111` |
| sym-ce7aba33b4143704632a | `CallbackCaptureBackend::prepare` | function | Prepares resources required by `CallbackCaptureBackend`. | `src/capture/capture_owner.rs:84` |
| sym-1076d488d679ca8ce004 | `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| sym-3e242114552b00229671 | `advance` | function | Returns this buffer's source-time start and advances the next start. | `src/capture/timeline.rs:74` |
| sym-27a1b14acbb0dfc6e31c | `advance_from_source_position` | function | Returns a buffer's source-time start from its native sample-frame position. Forward gaps are preserved in the returned timestamp without separately advancing this clock from an aggregate drop counter. | `src/capture/timeline.rs:90` |
| sym-71d554d30a07cbb7cc77 | `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| sym-9db2a2c278a3325269ca | `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |
| sym-0d9c647ddb36695c0df0 | `anchored` | function | Creates a sample timeline whose first buffer starts at the supplied nonzero monotonic timestamp. | `src/capture/timeline.rs:62` |
| sym-4d439fa19abd2ab66f61 | `before_open` | function | Records a source that was resolved but not opened because another required source failed first. | `src/capture/authorization.rs:60` |
| sym-2ae2a0a1e4991fd03efd | `capacity_frames` | function | Returns the capacity frames held by `CapturedFrameStream`. | `src/capture/frame_stream.rs:165` |
| sym-9828266fb021eb2aa01c | `capture_mode` | function | Returns the capture mode held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:57` |
| sym-d6d0625df36b60fc05ad | `capture_mode` | function | Returns the capture mode held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:44` |
| sym-84dfe9847cb3775c63d5 | `current` | function | Returns the current value observed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:196` |
| sym-6dbe1ce0ed2ff967ed67 | `discover` | function | Discovers the resources visible to `LocalSourceProvider`. | `src/capture/query.rs:55` |
| sym-b839c4c5e4b32bc03a85 | `drop` | function | Releases resources owned by `MacosInputSource`. | `src/capture/platform/macos/input.rs:258` |
| sym-3f4c295f1659985a9a22 | `drop` | function | Releases resources owned by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:299` |
| sym-36cfcc7b327a2189a1ec | `frame_stream_closed` | function | Returns the frame stream closed associated with `CaptureOwner`. | `src/capture/capture_owner.rs:251` |
| sym-94ab6945ccd49e5981c0 | `from_open_observations` | function | Records platform authorization observations without inferring them from a generic backend result. Callers must pass `NotObservable` when their platform has no authoritative query for the requested capture class. | `src/capture/authorization.rs:76` |
| sym-902f4b348d0a0b41ec9a | `generation` | function | Returns the generation associated with `SourceLifecycleRegistry`. | `src/capture/lifecycle_registry.rs:84` |
| sym-ac5996bdaf9b5597f56e | `identity_strength` | function | Returns the identity strength held by `CaptureSource`. | `src/capture/identity.rs:94` |
| sym-45a4263550e920c7598d | `into_callback` | function | Converts `CapturedFrameSender` into callback. | `src/capture/frame_stream.rs:132` |
| sym-9bf1b58161c9c643628e | `is_closed` | function | Reports whether closed is true for `CapturedFrameStream`. | `src/capture/frame_stream.rs:170` |
| sym-2e8e580510f3a911800b | `matches` | function | Returns whether an input satisfies `SourceQuery`. | `src/capture/query.rs:22` |
| sym-4f9f71d68deb2e375cec | `new` | function | Creates a new `CapturePermissionLifecycle`. | `src/capture/authorization.rs:189` |
| sym-2bce5173c11b0fedfccf | `new` | function | Creates a new `CaptureLineageSeed`. | `src/capture/capture_owner.rs:31` |
| sym-ac4709197071b041b435 | `new` | function | Creates a new `StableSourceId`. | `src/capture/identity.rs:33` |
| sym-c7a07ecd7218e8ef4290 | `new` | function | Creates a new `CaptureSampleTimeline`. | `src/capture/timeline.rs:52` |
| sym-6bb46773032fd2641eff | `next` | function | Advances the local evidence epoch after an observed authorization change or an explicit source reopen. | `src/capture/authorization.rs:274` |
| sym-6a28ad2e5841da992baa | `next` | function | Returns the generation assigned after explicit rediscovery. | `src/capture/events.rs:18` |
| sym-ba67c9ad9110afe18990 | `observation_handle` | function | Returns a handle for reading observations from `SourceRuntimeEventSender`. | `src/capture/events.rs:270` |
| sym-9a6f131ff1bed5bba171 | `observation_handle` | function | Returns a handle for reading observations from `SourceRuntimeEventReceiver`. | `src/capture/events.rs:321` |
| sym-6cafcb762fcee8844383 | `observation_handle` | function | Returns a handle for reading observations from `CapturedFrameSender`. | `src/capture/frame_stream.rs:142` |
| sym-f16b50c61176e14cce08 | `observation_handle` | function | Returns a handle for reading observations from `CapturedFrameStream`. | `src/capture/frame_stream.rs:183` |
| sym-cfee5a73a4a1a0d26432 | `observation_handle` | function | Returns a handle for reading observations from `CaptureObservationCounters`. | `src/capture/observations.rs:107` |
| sym-9e3205058b70597fd267 | `observation_handle` | function | Returns a handle for reading observations from `MacosInputSource`. | `src/capture/platform/macos/input.rs:238` |
| sym-dbd38fdac902eda65ee0 | `observation_handle` | function | Returns a handle for reading observations from `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:267` |
| sym-4afc2da96bf440324791 | `observation_handle` | function | Returns a handle for reading observations from `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:96` |
| sym-6cd08650f2afcc43b517 | `observation_receipt` | function | Returns the observation receipt associated with `CaptureOwner`. | `src/capture/capture_owner.rs:260` |
| sym-9047a37d17f8c9a3a47e | `observations` | function | Returns the observations exposed by `CaptureObservationReceipt`. | `src/capture/capture_owner.rs:174` |
| sym-d4d05dfdf43eba23f452 | `observations` | function | Returns the observations exposed by `CaptureOwner`. | `src/capture/capture_owner.rs:256` |
| sym-4b3a171c8c6ecf236523 | `observations` | function | Returns the observations exposed by `SourceRuntimeEventObservationHandle`. | `src/capture/events.rs:205` |
| sym-5aed9a1ce34773b88ead | `observations` | function | Returns the observations exposed by `SourceRuntimeEventSender`. | `src/capture/events.rs:266` |
| sym-984a539d92e5ea942c2d | `observations` | function | Returns the observations exposed by `SourceRuntimeEventReceiver`. | `src/capture/events.rs:317` |
| sym-b74fbfcc413c6394e9a2 | `observations` | function | Returns the observations exposed by `CapturedFrameObservationHandle`. | `src/capture/frame_stream.rs:36` |
| sym-dae44d2785f01b630ae4 | `observations` | function | Returns the observations exposed by `CaptureObservationHandle`. | `src/capture/observations.rs:37` |
| sym-f90860471b2bdeebfdf2 | `observations` | function | Returns the observations exposed by `MacosInputSource`. | `src/capture/platform/macos/input.rs:234` |
| sym-838a00051f8d286c3fe1 | `observations` | function | Returns the observations exposed by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:248` |
| sym-73e229943d6a6eee5b3d | `observations` | function | Returns the observations exposed by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:82` |
| sym-5f3a9e2a447883ad6321 | `observe` | function | Returns the current observation exposed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:204` |
| sym-89e25f10fc9dcc43465d | `observe_callback_buffer` | function | Records an observation for callback buffer for `CaptureObservationCounters`. | `src/capture/observations.rs:52` |
| sym-ca5cadedf3e7bf840bbb | `observe_complete_snapshot` | function | Records an observation for complete snapshot for `SourceLifecycleRegistry`. | `src/capture/lifecycle_registry.rs:36` |
| sym-1db6dab530aa274c6481 | `observe_dispatch_queue_full` | function | Records an observation for dispatch queue full for `CaptureObservationCounters`. | `src/capture/observations.rs:70` |
| sym-898836c5e54b77ef4603 | `observe_dispatch_queue_full_frames` | function | Records a known number of frames lost at a bounded native or Rust delivery edge. | `src/capture/observations.rs:76` |
| sym-40816db4cd81050d5521 | `observe_enqueued_frame` | function | Records an observation for enqueued frame for `CaptureObservationCounters`. | `src/capture/observations.rs:58` |
| sym-a17aa53905e405582ed0 | `observe_oversized_buffer` | function | Records an observation for oversized buffer for `CaptureObservationCounters`. | `src/capture/observations.rs:89` |
| sym-f4d752494107139e9afa | `observe_pool_exhaustion` | function | Records an observation for pool exhaustion for `CaptureObservationCounters`. | `src/capture/observations.rs:64` |
| sym-1d38577ec6acc760dbf9 | `observe_stream_error` | function | Records an observation for stream error for `CaptureObservationCounters`. | `src/capture/observations.rs:95` |
| sym-91a3331001dec71cad22 | `observe_timestamp_epoch_clamp` | function | Records an observation for timestamp epoch clamp for `CaptureObservationCounters`. | `src/capture/observations.rs:101` |
| sym-527e677b3cfffbeaf56a | `open` | function | Opens the resource represented by `PreparedCapture`. | `src/capture/capture_owner.rs:128` |
| sym-3887d294de2eb7775f03 | `open` | function | Opens the resource represented by `CaptureDeliveryStartGateController`. | `src/capture/frame_stream.rs:77` |
| sym-5c090a7a6ea0f2676aff | `open_metadata` | function | Returns the open metadata associated with `CaptureOwner`. | `src/capture/capture_owner.rs:242` |
| sym-1d87a0ad18fa6d4f87c4 | `permission_epoch` | function | Returns the permission epoch held by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:200` |
| sym-221e5a8050de5301cd79 | `pocketstation::capture::capture_owner::join_capture_worker` | function | Joins one owned capture worker and preserves panic as a typed failure. | `src/capture/capture_owner.rs:334` |
| sym-1b3e57d59e646040c389 | `pocketstation::capture::capture_owner::prepare_capture` | function | Prepares a bounded capture owner without starting native delivery. | `src/capture/capture_owner.rs:298` |
| sym-94f34460ef980ae89e34 | `pocketstation::capture::capture_owner::prepare_capture_with_start_gate` | function | Prepares a bounded capture owner behind a caller-owned one-way start gate. | `src/capture/capture_owner.rs:306` |
| sym-179581e8a2aa8587517b | `pocketstation::capture::events::publish_backend_failure` | function | Publishes one exact post-open backend failure without introducing another event queue or worker. | `src/capture/events.rs:280` |
| sym-56233572de3bb3ef20fe | `pocketstation::capture::events::source_runtime_event_channel` | function | Creates the bounded sender and receiver used for source runtime events. | `src/capture/events.rs:328` |
| sym-ecab0c07b346f1031392 | `pocketstation::capture::frame_stream::capture_delivery_start_gate` | function | Creates a closed Session-owned controller and callback-visible start gate. | `src/capture/frame_stream.rs:83` |
| sym-bbc1cfea18e83b9a7588 | `pocketstation::capture::frame_stream::captured_frame_stream` | function | Wraps the supplied capture receiver as a stream of captured frames. | `src/capture/frame_stream.rs:191` |
| sym-d53722dc6aaa613337d8 | `pocketstation::capture::platform::macos::input::discover_input_sources_native` | function | Discovers microphone input sources through the native macOS backend. | `src/capture/platform/macos/input.rs:263` |
| sym-140a3f747734f8e81fc1 | `pocketstation::capture::platform::macos::macos_tap::discover_sources_native` | function | Enumerate all running processes that have audio output. Returns an empty `Vec` on macOS < 14.4 (public support floor) or on non-macOS platforms. | `src/capture/platform/macos/macos_tap.rs:87` |
| sym-ade1e9ee6092e84a7633 | `pocketstation::capture::platform::macos::macos_tap::tap_available` | function | Returns `true` when the CoreAudio process tap API is available. | `src/capture/platform/macos/macos_tap.rs:76` |
| sym-99ff6b9e702e2b18cb4f | `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| sym-6c41c9513e4c1cd3f646 | `pocketstation::capture::query::discover_sources` | function | Discovers capture sources available from the local provider. | `src/capture/query.rs:85` |
| sym-f5b166160c26c3ff3463 | `pocketstation::capture::query::resolve_query` | function | Filters discovered capture sources using the supplied source query. | `src/capture/query.rs:40` |
| sym-f02c42cdfe82b6305eaf | `pocketstation::capture::timeline::initialize_monotonic_timestamp_domain` | function | Initializes the process-wide capture timestamp domain from a setup thread. | `src/capture/timeline.rs:11` |
| sym-23bb2f2bb9eacdded34f | `pocketstation::capture::timeline::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain used by every capture adapter. The value is non-zero and comparable across PocketStation crates in the same process; it is never derived from a wall clock and cannot jump. | `src/capture/timeline.rs:18` |
| sym-036fa6b33dbbdd9ec6b7 | `prepare` | function | Prepares resources required by `DesktopCaptureBackend`. | `src/capture/platform/macos/session_backend.rs:22` |
| sym-f3e3f6bd39d9945c426b | `process_tree_scope` | function | Reports the native process boundary represented by this discovery result without making the CLI reconstruct a private capture mode. | `src/capture/identity.rs:140` |
| sym-1175eaebd4ae1636db04 | `process_tree_scope` | function | Reports the process boundary requested from the native backend. | `src/capture/selection.rs:55` |
| sym-a6c64d37c7096431b619 | `query::SourceProvider::discover` | function | Discovers the resources visible to `SourceProvider`. | `src/capture/query.rs:49` |
| sym-a17526c1972e9e7804b4 | `selector_persistence_scope` | function | Reports how long this discovered selector can be reused without rediscovery. The capture owner remains authoritative for opening it. | `src/capture/identity.rs:114` |
| sym-9ee94498311f09f4fec1 | `selector_persistence_scope` | function | Describes how long the selector may be reused without rediscovery. | `src/capture/selection.rs:36` |
| sym-f3081dd71c51b0b3c86d | `session_id` | function | Returns the session identifier held by `CaptureLineageSeed`. | `src/capture/capture_owner.rs:38` |
| sym-cc9f9c2b8135d828a747 | `snapshot` | function | Returns a point-in-time snapshot of `CaptureObservationCounters`. | `src/capture/observations.rs:113` |
| sym-e242c2581097f70c164a | `source_id` | function | Derives the immutable captured-frame identity for this resolved source. | `src/capture/identity.rs:46` |
| sym-fba5cce843823f0dee0a | `source_id` | function | Returns the source identifier held by `MacosInputSource`. | `src/capture/platform/macos/input.rs:230` |
| sym-023c662556cbd7906a4d | `source_id` | function | Returns the source identifier held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:255` |
| sym-d51c8a4ef1101beb5b61 | `source_id` | function | Returns the source identifier held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:89` |
| sym-0d95ad2d241532bab63f | `stats` | function | Returns the current statistics for `CapturedFrameSender`. | `src/capture/frame_stream.rs:138` |
| sym-9c6f7313a6ef848fa3f7 | `stats` | function | Returns the current statistics for `CapturedFrameStream`. | `src/capture/frame_stream.rs:179` |
| sym-04f3132fd50e72551536 | `stem_id` | function | Returns the stem identifier held by `CaptureLineageSeed`. | `src/capture/capture_owner.rs:42` |
| sym-25b059d674b5b37c1998 | `stop_and_join` | function | Stops `CaptureOwner`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:264` |
| sym-b9d59b97c1ce355391a0 | `stop_and_join` | function | Stops `MacosInputSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/input.rs:242` |
| sym-c9e0f85bce4565b39679 | `stop_and_join` | function | Stops `SystemLoopbackSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/loopback.rs:274` |
| sym-76f2b8525f01a2fa145a | `stop_and_join` | function | Stops `DesktopCaptureSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/mod.rs:103` |
| sym-2d38c15d8edcbd16f43b | `try_next` | function | Attempts to next through `CapturedFrameStream`. | `src/capture/frame_stream.rs:160` |
| sym-9ff7b9339b077183855b | `try_next_lineaged_frame` | function | Attempts to next lineaged frame through `CaptureOwner`. | `src/capture/capture_owner.rs:213` |
| sym-3ec7dd8424f8012b44e4 | `try_recv` | function | Attempts to receive the next value from `SourceRuntimeEventReceiver` without waiting. | `src/capture/events.rs:304` |
| sym-5858fde37bc6fdfd2345 | `try_recv_runtime_event` | function | Attempts to recv runtime event through `CaptureOwner`. | `src/capture/capture_owner.rs:205` |
| sym-258e94793e00760fa96c | `try_send` | function | Publishes from a capture worker without blocking. When the bounded control channel is full, the newest event is dropped and counted. | `src/capture/events.rs:232` |
| sym-56d3152240fe4fc51432 | `try_send` | function | Attempts to send a value through `CapturedFrameSender` without waiting for capacity. | `src/capture/frame_stream.rs:109` |
| sym-cba59f674b927f6a3e94 | `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| sym-b4ba56c43ee4555e78dd | `identity` | module | Stable source identity and source discovery state. | `src/capture/identity.rs:1` |
| sym-549a2b77c08e128f796b | `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| sym-419801ec17e4626ad6d5 | `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| sym-895ca8f26ee62f0755a3 | `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| sym-be7aba4ae15d283efb2a | `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| sym-6aa8b39acb3133ed2c09 | `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| sym-74330e7cd8c298c34020 | `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |
| sym-801e41161f622e4c8375 | `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| sym-c61304d3358e379705a0 | `pocketstation::capture::capture_owner::CaptureLineageSeed` | struct | Stable session and stem identity assigned before an exact source is opened. | `src/capture/capture_owner.rs:25` |
| sym-a009e79af7cb329a10a4 | `pocketstation::capture::capture_owner::CaptureObservationReceipt` | struct | Retains the identity and observation access returned for capture observation. | `src/capture/capture_owner.rs:167` |
| sym-8b016df4d1351142eb09 | `pocketstation::capture::capture_owner::CaptureOpenMetadata` | struct | Authoritative lineage state established only after native capture opens. | `src/capture/capture_owner.rs:49` |
| sym-0062f094cf04ce808da1 | `pocketstation::capture::capture_owner::CaptureOwner` | struct | RAII owner for native capture, its bounded frame stream, and runtime events. | `src/capture/capture_owner.rs:194` |
| sym-754798c8762195990799 | `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| sym-0ff19b79d15eb60b7944 | `pocketstation::capture::capture_owner::CapturePrepareRequest` | struct | Setup-time request for one bounded callback-oriented capture owner. | `src/capture/capture_owner.rs:61` |
| sym-ae71a73019da4eedbfdd | `pocketstation::capture::capture_owner::CaptureStopOutcome` | struct | Final observations returned only after backend stop and join complete. | `src/capture/capture_owner.rs:185` |
| sym-f68a0563e3fbb06758fd | `pocketstation::capture::capture_owner::PreparedCapture` | struct | Prepared capture plus its preallocated delivery endpoints. | `src/capture/capture_owner.rs:119` |
| sym-b6f34d0d3b18e6cab4b9 | `pocketstation::capture::events::CaptureRuntimeFailure` | struct | Reports a capture runtime failure. | `src/capture/events.rs:47` |
| sym-583e7e4b72bbe6e2124f | `pocketstation::capture::events::SourceGeneration` | struct | Identifies one appearance generation of a capture source across loss and reappearance. | `src/capture/events.rs:12` |
| sym-926445cfdcc4a65cb18b | `pocketstation::capture::events::SourceRuntimeEventObservationHandle` | struct | Holds the ownership or bounded access represented by source runtime event observation handle. | `src/capture/events.rs:200` |
| sym-a3891e64195ad9eefd10 | `pocketstation::capture::events::SourceRuntimeEventObservations` | struct | Reports the source runtime event observations collected at an observation boundary. | `src/capture/events.rs:111` |
| sym-0a97a8c5cc04ee68954b | `pocketstation::capture::events::SourceRuntimeEventReceiver` | struct | Receives source runtime event values across its declared ownership boundary. | `src/capture/events.rs:298` |
| sym-4100bfd509bc312470bf | `pocketstation::capture::events::SourceRuntimeEventSender` | struct | Sends source runtime event values across its declared ownership boundary. | `src/capture/events.rs:224` |
| sym-df376380871adcda955a | `pocketstation::capture::frame_stream::CaptureDeliveryStartGate` | struct | Read-only one-way start barrier checked by capture delivery callbacks. | `src/capture/frame_stream.rs:54` |
| sym-509e830b388e193452d0 | `pocketstation::capture::frame_stream::CaptureDeliveryStartGateController` | struct | Session-owned authority that opens one capture delivery start gate. | `src/capture/frame_stream.rs:72` |
| sym-4e918c3304fa3607c83d | `pocketstation::capture::frame_stream::CapturedFrameObservationHandle` | struct | Holds the ownership or bounded access represented by captured frame observation handle. | `src/capture/frame_stream.rs:31` |
| sym-011bce3cc267f0f461f1 | `pocketstation::capture::frame_stream::CapturedFrameSender` | struct | Single-producer endpoint passed into a platform capture callback. | `src/capture/frame_stream.rs:102` |
| sym-e50aeb0b46ace0193171 | `pocketstation::capture::frame_stream::CapturedFrameStream` | struct | Non-blocking consumer for captured `AudioFrame`s. | `src/capture/frame_stream.rs:154` |
| sym-8a20db7016746746f7a2 | `pocketstation::capture::frame_stream::CapturedFrameStreamStats` | struct | Reports the captured frame stream stats collected at an observation boundary. | `src/capture/frame_stream.rs:17` |
| sym-2900ed324c0cddac02d7 | `pocketstation::capture::identity::CaptureSource` | struct | Owns production of capture values and its lifecycle state. | `src/capture/identity.rs:82` |
| sym-2f4135245cffde0ce33f | `pocketstation::capture::identity::StableSourceId` | struct | Uniquely identifies stable source within its PocketStation ownership scope. | `src/capture/identity.rs:26` |
| sym-3a5b02712461d3956ee1 | `pocketstation::capture::lifecycle_registry::SourceLifecycleRegistry` | struct | Assigns source generations across complete discovery snapshots. | `src/capture/lifecycle_registry.rs:31` |
| sym-f002a9e3f754a8ed3282 | `pocketstation::capture::observations::CaptureObservationCounters` | struct | Setup-time cloneable handle; every observation is one relaxed atomic operation and remains allocation-free, lock-free, and log-free. | `src/capture/observations.rs:46` |
| sym-603db656d3f59b7c5d95 | `pocketstation::capture::observations::CaptureObservationHandle` | struct | Holds the ownership or bounded access represented by capture observation handle. | `src/capture/observations.rs:32` |
| sym-ad8a7d734a7b06b83340 | `pocketstation::capture::observations::CaptureObservations` | struct | Reports the capture observations collected at an observation boundary. | `src/capture/observations.rs:8` |
| sym-fdde89910590812c39a3 | `pocketstation::capture::platform::macos::DesktopCaptureSource` | struct | Owns production of desktop capture values and its lifecycle state. | `src/capture/platform/macos/mod.rs:33` |
| sym-f278f1a4ffd61d8c0158 | `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:65` |
| sym-f32c9d0ff1652848c4b6 | `pocketstation::capture::platform::macos::loopback::SystemLoopbackSource` | struct | Manages a macOS loopback capture session. | `src/capture/platform/macos/loopback.rs:53` |
| sym-ef99ea56dfe5149e45a8 | `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |
| sym-1e2e36ccd063adac6c20 | `pocketstation::capture::query::LocalSourceProvider` | struct | Discovers and resolves capture sources through the target platform backend. | `src/capture/query.rs:52` |
| sym-d588746d1ca3571cff04 | `pocketstation::capture::timeline::CaptureSampleTimeline` | struct | Source-time clock for capture streams whose media cadence is defined by the number of sample frames produced by the device. | `src/capture/timeline.rs:31` |
| sym-f8d352a600d3ff315814 | `CaptureDelivery::frame_sender` | struct_field | Sends captured frames from `CaptureDelivery` into the Session runtime. | `src/capture/capture_owner.rs:74` |
| sym-bdc39fcfa064a60d2306 | `CaptureDelivery::runtime_event_sender` | struct_field | Sends capture lifecycle and failure events from `CaptureDelivery` to the Session runtime. | `src/capture/capture_owner.rs:75` |
| sym-946cee9b73047bf5f3ad | `CaptureObservations::callback_buffers_total` | struct_field | Counts the total number of callback buffers observed by `CaptureObservations`. | `src/capture/observations.rs:9` |
| sym-f7dfab792d4767d20aaa | `CaptureObservations::dispatch_queue_full_total` | struct_field | Counts the total number of dispatch queue full observed by `CaptureObservations`. | `src/capture/observations.rs:12` |
| sym-d6b7afdc8574439c7e36 | `CaptureObservations::frames_enqueued_total` | struct_field | Counts the total number of frames enqueued observed by `CaptureObservations`. | `src/capture/observations.rs:10` |
| sym-40f1b50af6cd8dd44901 | `CaptureObservations::invalid_buffer_total` | struct_field | Counts the total number of invalid buffer observed by `CaptureObservations`. | `src/capture/observations.rs:13` |
| sym-27f712c546a6dd895221 | `CaptureObservations::oversized_buffer_total` | struct_field | Counts the total number of oversized buffer observed by `CaptureObservations`. | `src/capture/observations.rs:14` |
| sym-5ac3ff4ff8ee3a71c952 | `CaptureObservations::pool_exhausted_total` | struct_field | Counts the total number of pool exhausted observed by `CaptureObservations`. | `src/capture/observations.rs:11` |
| sym-1b04c98ba87668704d93 | `CaptureObservations::stream_errors_total` | struct_field | Counts the total number of stream errors observed by `CaptureObservations`. | `src/capture/observations.rs:15` |
| sym-39bbe39f601d1d91db4a | `CaptureObservations::timestamp_epoch_clamps_total` | struct_field | Counts the total number of timestamp epoch clamps observed by `CaptureObservations`. | `src/capture/observations.rs:16` |
| sym-9581a6832ae177d4d20d | `CaptureOpenMetadata::clock_id` | struct_field | Identifies the clock identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:53` |
| sym-a5fdf8f0d7203d8d8290 | `CaptureOpenMetadata::discontinuity_epoch` | struct_field | Identifies the discontinuity generation attached to `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:55` |
| sym-5839ebf0b360299b61b5 | `CaptureOpenMetadata::permission_epoch` | struct_field | Identifies the permission-observation generation attached to `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:56` |
| sym-1d56f52c749bd33c7318 | `CaptureOpenMetadata::session_id` | struct_field | Identifies the session identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:50` |
| sym-742271d74f0cbcccb9f3 | `CaptureOpenMetadata::source_generation` | struct_field | References the source generation participating in `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:54` |
| sym-6bd3fc8d78157a0489ca | `CaptureOpenMetadata::source_id` | struct_field | Identifies the source identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:51` |
| sym-b48a734f0148bcf322a9 | `CaptureOpenMetadata::stem_id` | struct_field | Identifies the stem identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:52` |
| sym-3570f8e51df4d215ffbf | `CaptureOwnerObservations::backend` | struct_field | Stores the backend as a `CaptureObservations` value in `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:161` |
| sym-c3bfaab3824d224f64d5 | `CaptureOwnerObservations::frame_stream` | struct_field | Stores the frame stream as a `CapturedFrameStreamStats` value in `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:162` |
| sym-965e7fb36a9e169d75e2 | `CaptureOwnerObservations::runtime_events` | struct_field | Contains the runtime events owned or reported by `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:163` |
| sym-f539df5c6e22c14da5bf | `CapturePrepareRequest::frame_capacity_frames` | struct_field | Sets the frame capacity frames available to `CapturePrepareRequest`. | `src/capture/capture_owner.rs:64` |
| sym-372f3e2fc658f3fcbf9a | `CapturePrepareRequest::lineage_seed` | struct_field | Supplies the initial lineage identity used when `CapturePrepareRequest` opens capture. | `src/capture/capture_owner.rs:63` |
| sym-33b9b2dc6e5767b5841c | `CapturePrepareRequest::mode` | struct_field | Stores the mode as a `CaptureMode` value in `CapturePrepareRequest`. | `src/capture/capture_owner.rs:62` |
| sym-6ed46377d3675e08fd87 | `CapturePrepareRequest::runtime_event_capacity_events` | struct_field | Sets the runtime event capacity events available to `CapturePrepareRequest`. | `src/capture/capture_owner.rs:65` |
| sym-f997ca49b1ce8cef7258 | `CaptureSampleTimelineError::SourcePositionMovedBackward::expected_at_least` | struct_field | Stores the expected at least component of `SourcePositionMovedBackward`. | `src/capture/timeline.rs:45` |
| sym-1b6f1aaa7ad89f48c375 | `CaptureSampleTimelineError::SourcePositionMovedBackward::observed` | struct_field | Stores the observed component of `SourcePositionMovedBackward`. | `src/capture/timeline.rs:46` |
| sym-7b5cf48f4ad1b2f180f2 | `CaptureStopOutcome::observations` | struct_field | Carries the observations collected for `CaptureStopOutcome`. | `src/capture/capture_owner.rs:186` |
| sym-f0b279d0e24131872d5d | `CapturedFrameStreamStats::delivered_frames` | struct_field | Contains the delivered frames owned or reported by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:18` |
| sym-9c25a7556595b2dd079b | `CapturedFrameStreamStats::dropped_newest_frames` | struct_field | Contains the dropped newest frames owned or reported by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:19` |
| sym-e64bfcfb14cca95eee75 | `CapturedFrameStreamStats::frames_discarded_before_start_total` | struct_field | Counts the total number of frames discarded before start observed by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:20` |
| sym-d3ef57efc035e370fdaa | `authorization::CaptureAuthorizationSnapshot::application_policy` | struct_field | Reports the application-level capture policy observed by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:20` |
| sym-2fd0c2d149e69098b259 | `authorization::CaptureAuthorizationSnapshot::capability` | struct_field | Stores the capability as a `CaptureCapabilityState` value in `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:18` |
| sym-0110608b4d90ebfbdcfa | `authorization::CaptureAuthorizationSnapshot::capture_scope` | struct_field | Declares the exact resource scope authorized by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:22` |
| sym-bad5218134c7bab1a456 | `authorization::CaptureAuthorizationSnapshot::identity_strength` | struct_field | Reports how strongly the selected source identity is bound in `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:23` |
| sym-6786cd68c1708d931a04 | `authorization::CaptureAuthorizationSnapshot::observed_at_ns` | struct_field | Stores the observed at value for `CaptureAuthorizationSnapshot`, in nanoseconds. | `src/capture/authorization.rs:25` |
| sym-8881e3efc02b5a8e9f82 | `authorization::CaptureAuthorizationSnapshot::open_outcome` | struct_field | Reports whether opening capture is allowed, denied, or requires setup in `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:26` |
| sym-6546ea14b8dd5c616bc6 | `authorization::CaptureAuthorizationSnapshot::os_permission` | struct_field | Reports the operating-system permission state observed by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:19` |
| sym-324bf896e839322c55a1 | `authorization::CaptureAuthorizationSnapshot::permission_epoch` | struct_field | Identifies the permission-observation generation attached to `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:24` |
| sym-3af6c6b819f8a1ecb411 | `authorization::CaptureAuthorizationSnapshot::session_grant` | struct_field | Reports whether the Session-specific capture grant is present for `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:21` |
| sym-f14eb2b2d9294d4668da | `authorization::CaptureError::BackendSetupRequired::action` | struct_field | Describes the corrective action reported with `BackendSetupRequired`. | `src/capture/authorization.rs:298` |
| sym-3bac54bd2f14fd0f2865 | `authorization::CaptureError::BackendSetupRequired::backend` | struct_field | Stores the backend component of `BackendSetupRequired`. | `src/capture/authorization.rs:297` |
| sym-b0dc71598094f0792079 | `authorization::CaptureError::BackendStatus::operation` | struct_field | Names the operation that produced `BackendStatus`. | `src/capture/authorization.rs:304` |
| sym-5a64301c9c536c12387d | `authorization::CaptureError::BackendStatus::status_code` | struct_field | Preserves the platform or protocol status code reported by `BackendStatus`. | `src/capture/authorization.rs:305` |
| sym-1428c658d60dfc6b7829 | `authorization::CaptureError::CaptureWorkerPanicked::worker` | struct_field | Stores the worker component of `CaptureWorkerPanicked`. | `src/capture/authorization.rs:316` |
| sym-1c348e186990e94407cf | `authorization::CaptureError::PermissionDenied::operation` | struct_field | Names the operation that produced `PermissionDenied`. | `src/capture/authorization.rs:301` |
| sym-239abfa4cef9a73b47de | `authorization::CaptureError::SourceUnavailable::stable_key` | struct_field | Stores the stable source key associated with `SourceUnavailable`. | `src/capture/authorization.rs:308` |
| sym-beea7f3e3748e7ba0ab6 | `authorization::CapturePermissionTransition::current` | struct_field | Stores the current as a `PermissionObservation` value in `CapturePermissionTransition`. | `src/capture/authorization.rs:171` |
| sym-e0c5160d0b04c368e706 | `authorization::CapturePermissionTransition::kind` | struct_field | Records the kind selected for `CapturePermissionTransition`. | `src/capture/authorization.rs:169` |
| sym-05ca6b08d4055835136e | `authorization::CapturePermissionTransition::permission_epoch` | struct_field | Identifies the permission-observation generation attached to `CapturePermissionTransition`. | `src/capture/authorization.rs:172` |
| sym-99ccabb5727887f5bad9 | `authorization::CapturePermissionTransition::previous` | struct_field | Contains the previous owned or reported by `CapturePermissionTransition`. | `src/capture/authorization.rs:170` |
| sym-2efcfc236510e43ce157 | `authorization::CaptureScope::ExactApplication::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplication`. | `src/capture/authorization.rs:249` |
| sym-d67f47bb4cc6ab2218fe | `authorization::CaptureScope::ExactInputDevice::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactInputDevice`. | `src/capture/authorization.rs:250` |
| sym-e5dd0f97470b834ad39f | `authorization::CaptureScope::ExactOutputDevice::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactOutputDevice`. | `src/capture/authorization.rs:251` |
| sym-bc2f6d9785044dca4c28 | `events::CaptureRuntimeFailure::error_class` | struct_field | Contains the error class owned or reported by `CaptureRuntimeFailure`. | `src/capture/events.rs:49` |
| sym-d8b03278d57ae8b670e5 | `events::CaptureRuntimeFailure::operation` | struct_field | Names the operation that produced `CaptureRuntimeFailure`. | `src/capture/events.rs:48` |
| sym-51ed6637889912ebd5f7 | `events::CaptureRuntimeFailureClass::BackendClass::class` | struct_field | Contains the class owned or reported by `BackendClass`. | `src/capture/events.rs:43` |
| sym-32d556ad550751c4f609 | `events::CaptureRuntimeFailureClass::PlatformStatus::status_code` | struct_field | Preserves the platform or protocol status code reported by `PlatformStatus`. | `src/capture/events.rs:42` |
| sym-0f9091de7a694dcf4118 | `events::SourceRuntimeEvent::BackendFailure::failure` | struct_field | Carries the failure reported by `BackendFailure`. | `src/capture/events.rs:63` |
| sym-718021fa09a4e5f78448 | `events::SourceRuntimeEvent::BackendFailure::generation` | struct_field | Identifies the generation of the resource represented by `BackendFailure`. | `src/capture/events.rs:62` |
| sym-b6cac24daa0bbd0f164b | `events::SourceRuntimeEvent::BackendFailure::stable_id` | struct_field | Identifies the stable identifier recorded by `BackendFailure`. | `src/capture/events.rs:61` |
| sym-958fdd7dfee0e0ba6bc2 | `events::SourceRuntimeEvent::SourceUnavailable::failure` | struct_field | Carries the failure reported by `SourceUnavailable`. | `src/capture/events.rs:58` |
| sym-9093ae9b0e9990551709 | `events::SourceRuntimeEvent::SourceUnavailable::generation` | struct_field | Identifies the generation of the resource represented by `SourceUnavailable`. | `src/capture/events.rs:56` |
| sym-eb5d2e8d0f6f67c1b6ae | `events::SourceRuntimeEvent::SourceUnavailable::recovery_requirement` | struct_field | Declares the recovery action required after the source event in `SourceUnavailable`. | `src/capture/events.rs:57` |
| sym-914c7166fd31047a7512 | `events::SourceRuntimeEvent::SourceUnavailable::stable_id` | struct_field | Identifies the stable identifier recorded by `SourceUnavailable`. | `src/capture/events.rs:55` |
| sym-31eb70081b1d8cbfeddc | `events::SourceRuntimeEventObservations::capacity_event_count` | struct_field | Sets the capacity event count available to `SourceRuntimeEventObservations`. | `src/capture/events.rs:112` |
| sym-213aab1d92988b9d1e7a | `events::SourceRuntimeEventObservations::depth_events` | struct_field | Reports the depth events observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:115` |
| sym-9f13913903153e3cc97b | `events::SourceRuntimeEventObservations::depth_owned_bytes` | struct_field | Stores the depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:116` |
| sym-e7a002fd63b713511326 | `events::SourceRuntimeEventObservations::events_dropped_oversized_total` | struct_field | Counts the total number of events dropped oversized observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:120` |
| sym-2c930f0bab38446e6cf4 | `events::SourceRuntimeEventObservations::events_dropped_total` | struct_field | Counts the total number of events dropped observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:119` |
| sym-26bd5b31a552d7ad0324 | `events::SourceRuntimeEventObservations::events_enqueued_total` | struct_field | Counts the total number of events enqueued observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:118` |
| sym-1cf87d35b7171c68647c | `events::SourceRuntimeEventObservations::maximum_buffered_owned_bytes` | struct_field | Stores the maximum buffered owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:114` |
| sym-20ce8fdb10817b100b20 | `events::SourceRuntimeEventObservations::maximum_event_owned_bytes` | struct_field | Stores the maximum event owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:113` |
| sym-8784bcf4ca188444a359 | `events::SourceRuntimeEventObservations::peak_depth_owned_bytes` | struct_field | Stores the peak depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:117` |
| sym-16305c01737e7713d5f9 | `identity::CaptureSource::app_id` | struct_field | Identifies the app identifier recorded by `CaptureSource`. | `src/capture/identity.rs:86` |
| sym-d225360a6ae1b977abb3 | `identity::CaptureSource::channels` | struct_field | Contains the channels owned or reported by `CaptureSource`. | `src/capture/identity.rs:90` |
| sym-3df6f05fe270375921df | `identity::CaptureSource::device_uid` | struct_field | Stores the device uid component of `CaptureSource`. | `src/capture/identity.rs:87` |
| sym-e4408de392f9a76613fe | `identity::CaptureSource::name` | struct_field | Stores the human-readable name used to identify `CaptureSource`. | `src/capture/identity.rs:84` |
| sym-82f007255ac4c63383e6 | `identity::CaptureSource::process_id` | struct_field | Identifies the process identifier recorded by `CaptureSource`. | `src/capture/identity.rs:85` |
| sym-4ea89de58375f6de493e | `identity::CaptureSource::sample_rate_hz` | struct_field | Stores the sample rate value for `CaptureSource`, in hertz. | `src/capture/identity.rs:89` |
| sym-6a93020e6ac8e5b5a388 | `identity::CaptureSource::stable_id` | struct_field | Identifies the stable identifier recorded by `CaptureSource`. | `src/capture/identity.rs:83` |
| sym-8fd041f3afc8e178a998 | `identity::CaptureSource::state` | struct_field | Records the state selected for `CaptureSource`. | `src/capture/identity.rs:88` |
| sym-fbb9a4a1e0b9a37de640 | `identity::StableSourceId::kind` | struct_field | Records the kind selected for `StableSourceId`. | `src/capture/identity.rs:28` |
| sym-2f193a52c138d27634e8 | `identity::StableSourceId::platform` | struct_field | Stores the platform as a `Platform` value in `StableSourceId`. | `src/capture/identity.rs:27` |
| sym-56a5f906e53f087b807f | `identity::StableSourceId::stable_key` | struct_field | Stores the stable source key associated with `StableSourceId`. | `src/capture/identity.rs:29` |
| sym-77c4144009ab0b7314ea | `lifecycle_registry::SourceGenerationTransition::Disappeared::generation` | struct_field | Identifies the generation of the resource represented by `Disappeared`. | `src/capture/lifecycle_registry.rs:11` |
| sym-09a3cdd7140fe84d7fb0 | `lifecycle_registry::SourceGenerationTransition::Disappeared::stable_id` | struct_field | Identifies the stable identifier recorded by `Disappeared`. | `src/capture/lifecycle_registry.rs:10` |
| sym-828ed5429fb6a2e1f8ed | `lifecycle_registry::SourceGenerationTransition::Reappeared::generation` | struct_field | Identifies the generation of the resource represented by `Reappeared`. | `src/capture/lifecycle_registry.rs:16` |
| sym-089a0fcbac44a118ef59 | `lifecycle_registry::SourceGenerationTransition::Reappeared::previous_generation` | struct_field | Identifies the generation that preceded the transition recorded by `Reappeared`. | `src/capture/lifecycle_registry.rs:15` |
| sym-5177764d9a2229260068 | `lifecycle_registry::SourceGenerationTransition::Reappeared::stable_id` | struct_field | Identifies the stable identifier recorded by `Reappeared`. | `src/capture/lifecycle_registry.rs:14` |
| sym-c71fbab34189dda90085 | `selection::CaptureMode::ExactApplication::process_id` | struct_field | Identifies the process identifier recorded by `ExactApplication`. | `src/capture/selection.rs:22` |
| sym-408321762eba07162e31 | `selection::CaptureMode::ExactApplication::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplication`. | `src/capture/selection.rs:23` |
| sym-6447901cd24495ba2dfb | `selection::CaptureMode::ExactApplicationStable::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplicationStable`. | `src/capture/selection.rs:26` |
| sym-fb9e4310f58f26299c89 | `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| sym-4da42fff88ec45e5e56b | `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| sym-a354a1be3ec1974fc5fe | `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| sym-b0869d923b3fd79406d8 | `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| sym-33c0221c0be7ca184faf | `pocketstation::capture::authorization::ApplicationPolicyObservation::Allowed` | variant | Reports the observed application policy as allowed. | `src/capture/authorization.rs:232` |
| sym-0fc0e1b2e536e8e3f13b | `pocketstation::capture::authorization::ApplicationPolicyObservation::Denied` | variant | Reports the observed application policy as denied. | `src/capture/authorization.rs:233` |
| sym-566c52c2a9ff86bd463b | `pocketstation::capture::authorization::ApplicationPolicyObservation::NotApplicable` | variant | Reports the observed application policy as not applicable. | `src/capture/authorization.rs:235` |
| sym-319b6fd00c14b98da3e9 | `pocketstation::capture::authorization::ApplicationPolicyObservation::NotObservable` | variant | Reports the observed application policy as not observable. | `src/capture/authorization.rs:234` |
| sym-506cb6d27f10d2778cc6 | `pocketstation::capture::authorization::CaptureCapabilityState::Available` | variant | Identifies the available state or stage represented by `CaptureCapabilityState`. | `src/capture/authorization.rs:146` |
| sym-fbb29cc04db014ac7fed | `pocketstation::capture::authorization::CaptureCapabilityState::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/capture/authorization.rs:147` |
| sym-8e1d9479ec55dc17f67f | `pocketstation::capture::authorization::CaptureCapabilityState::Unsupported` | variant | Reports that the requested operation is unsupported. | `src/capture/authorization.rs:148` |
| sym-fb15b1417c558dc3e721 | `pocketstation::capture::authorization::CaptureError::BackendInit` | variant | Classifies a failure at the backend init stage or component of `CaptureError`. | `src/capture/authorization.rs:294` |
| sym-14963fd27a47e148745e | `pocketstation::capture::authorization::CaptureError::BackendSetupRequired` | variant | Classifies a failure at the backend setup required stage or component of `CaptureError`. | `src/capture/authorization.rs:296` |
| sym-0c96a931316066180385 | `pocketstation::capture::authorization::CaptureError::BackendStatus` | variant | Classifies a failure at the backend status stage or component of `CaptureError`. | `src/capture/authorization.rs:303` |
| sym-1f3f4efaa42cdef56974 | `pocketstation::capture::authorization::CaptureError::CaptureWorkerPanicked` | variant | Reports that capture worker panicked while the operation was active. | `src/capture/authorization.rs:316` |
| sym-abc62254bcb0bacffd05 | `pocketstation::capture::authorization::CaptureError::InvalidRuntimeEventCapacity` | variant | Reports that the supplied runtime event capacity is invalid. | `src/capture/authorization.rs:314` |
| sym-7a6318761cc619d71e07 | `pocketstation::capture::authorization::CaptureError::InvalidStreamCapacity` | variant | Reports that the supplied stream capacity is invalid. | `src/capture/authorization.rs:312` |
| sym-a86d1e11b3ece7cbb4a8 | `pocketstation::capture::authorization::CaptureError::ModeUnsupported` | variant | Reports that mode is unsupported by the active backend or contract. | `src/capture/authorization.rs:310` |
| sym-95c001a9a5be7dc48b6a | `pocketstation::capture::authorization::CaptureError::NotSupported` | variant | Reports that no t supported is available. | `src/capture/authorization.rs:292` |
| sym-cc5abdd62a8be53377b9 | `pocketstation::capture::authorization::CaptureError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:301` |
| sym-0d50195408fa113a1d05 | `pocketstation::capture::authorization::CaptureError::SourceUnavailable` | variant | Reports that source is unavailable. | `src/capture/authorization.rs:308` |
| sym-e95e3b2bfbf3b0e7d031 | `pocketstation::capture::authorization::CaptureOpenOutcome::BackendFailed` | variant | Identifies the backend failed state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:286` |
| sym-4a4bb006ffadef4e4fc8 | `pocketstation::capture::authorization::CaptureOpenOutcome::NotAttempted` | variant | Identifies the not attempted state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:282` |
| sym-a13b9b4b05d56a0dbd22 | `pocketstation::capture::authorization::CaptureOpenOutcome::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:284` |
| sym-62a359b0809770b77466 | `pocketstation::capture::authorization::CaptureOpenOutcome::SourceUnavailable` | variant | Identifies the source unavailable state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:285` |
| sym-00ec5f8234fc9fcedb02 | `pocketstation::capture::authorization::CaptureOpenOutcome::Succeeded` | variant | Identifies the succeeded state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:283` |
| sym-3704169384e672bb3dad | `pocketstation::capture::authorization::CaptureScope::ExactApplication` | variant | Limits capture authorization to exact application. | `src/capture/authorization.rs:249` |
| sym-767238270726977cc51f | `pocketstation::capture::authorization::CaptureScope::ExactInputDevice` | variant | Limits capture authorization to exact input device. | `src/capture/authorization.rs:250` |
| sym-b56d7a97a07a40f0654d | `pocketstation::capture::authorization::CaptureScope::ExactOutputDevice` | variant | Limits capture authorization to exact output device. | `src/capture/authorization.rs:251` |
| sym-ee28c6fa465ee4b189ae | `pocketstation::capture::authorization::CaptureScope::SystemMix` | variant | Limits capture authorization to system mix. | `src/capture/authorization.rs:252` |
| sym-d57ec3d01d9ca01792c0 | `pocketstation::capture::authorization::CaptureSessionGrant::Denied` | variant | Represents the denied alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:242` |
| sym-4c4624f453b75741a427 | `pocketstation::capture::authorization::CaptureSessionGrant::GrantedByExplicitSelection` | variant | Represents the granted by explicit selection alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:241` |
| sym-3fe3f128e5f7492a3135 | `pocketstation::capture::authorization::CaptureSessionGrant::NotEvaluated` | variant | Represents the not evaluated alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:243` |
| sym-a59b1847f02845a0bba0 | `pocketstation::capture::authorization::PermissionObservation::Allowed` | variant | Represents the allowed alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:154` |

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

The claims on **Capture API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/mod.rs:1-10` (`DECLARED`)

For **Capture API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
