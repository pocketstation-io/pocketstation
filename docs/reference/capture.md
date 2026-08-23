# Capture API

<!-- claims: CLM-REF-003-CAP-001,CLM-REF-003-CAP-002,CLM-REF-003-CAP-003,CLM-REF-003-CAP-004,CLM-REF-003-CAP-005,CLM-REF-003-SOURCE-001 -->

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `INITIAL` | assoc_const | Provides the initial value for `PermissionEpoch`. | `src/capture/authorization.rs:270` |
| `INITIAL` | assoc_const | Provides the initial value for `SourceGeneration`. | `src/capture/events.rs:15` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | Classifies the observable application policy observation. | `src/capture/authorization.rs:231` |
| `pocketstation::capture::authorization::CaptureCapabilityState` | enum | Selects the capture capability state used by PocketStation. | `src/capture/authorization.rs:145` |
| `pocketstation::capture::authorization::CaptureError` | enum | Classifies failures reported as capture error. | `src/capture/authorization.rs:290` |
| `pocketstation::capture::authorization::CaptureOpenOutcome` | enum | Classifies the observable capture open outcome. | `src/capture/authorization.rs:281` |
| `pocketstation::capture::authorization::CaptureScope` | enum | Selects the capture scope used by PocketStation. | `src/capture/authorization.rs:248` |
| `pocketstation::capture::authorization::CaptureSessionGrant` | enum | Enumerates the supported capture session grant cases. | `src/capture/authorization.rs:240` |
| `pocketstation::capture::authorization::PermissionObservation` | enum | Classifies the observable permission observation. | `src/capture/authorization.rs:153` |
| `pocketstation::capture::authorization::SourceIdentityStrength` | enum | Enumerates the supported source identity strength cases. | `src/capture/authorization.rs:257` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass` | enum | Enumerates the supported capture runtime failure class cases. | `src/capture/events.rs:40` |
| `pocketstation::capture::events::SourceLifecycleEventKind` | enum | Selects the source lifecycle event kind used by PocketStation. | `src/capture/events.rs:25` |
| `pocketstation::capture::events::SourceRecoveryRequirement` | enum | Selects the source recovery requirement used by PocketStation. | `src/capture/events.rs:35` |
| `pocketstation::capture::events::SourceRuntimeEvent` | enum | Classifies the observable source runtime event. | `src/capture/events.rs:53` |
| `pocketstation::capture::events::SourceRuntimeEventDelivery` | enum | Enumerates the supported source runtime event delivery cases. | `src/capture/events.rs:96` |
| `pocketstation::capture::frame_stream::CapturedFrameDelivery` | enum | Enumerates the supported captured frame delivery cases. | `src/capture/frame_stream.rs:10` |
| `pocketstation::capture::identity::SourceKind` | enum | Selects the source kind used by PocketStation. | `src/capture/identity.rs:9` |
| `pocketstation::capture::identity::SourceState` | enum | Selects the source state used by PocketStation. | `src/capture/identity.rs:17` |
| `pocketstation::capture::query::SourceQuery` | enum | Enumerates the supported source query cases. | `src/capture/query.rs:13` |
| `pocketstation::capture::selection::CaptureMode` | enum | Selects the capture mode used by PocketStation. | `src/capture/selection.rs:16` |
| `pocketstation::capture::selection::InputDeviceSelector` | enum | Enumerates the supported input device selector cases. | `src/capture/selection.rs:9` |
| `pocketstation::capture::selection::ProcessTreeScope` | enum | Selects the process tree scope used by PocketStation. | `src/capture/selection.rs:83` |
| `pocketstation::capture::selection::SelectorPersistenceScope` | enum | Selects the selector persistence scope used by PocketStation. | `src/capture/selection.rs:73` |
| `ActiveCaptureBackend::observation_handle` | function | Returns a handle for reading observations from `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:107` |
| `ActiveCaptureBackend::observations` | function | Returns the observations exposed by `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:109` |
| `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| `ActiveCaptureBackend::stop_and_join` | function | Stops and join for `ActiveCaptureBackend`. | `src/capture/capture_owner.rs:111` |
| `CallbackCaptureBackend::prepare` | function | Prepares resources required by `CallbackCaptureBackend`. | `src/capture/capture_owner.rs:84` |
| `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |
| `before_open` | function | Records a source that was resolved but not opened because another required source failed first. | `src/capture/authorization.rs:60` |
| `current` | function | Returns the current value observed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:196` |
| `discover` | function | Discovers the resources visible to `LocalSourceProvider`. | `src/capture/query.rs:55` |
| `from_open_observations` | function | Records platform authorization observations without inferring them from a generic backend result. Callers must pass `NotObservable` when their platform has no authoritative query for the requested capture class. | `src/capture/authorization.rs:76` |
| `identity_strength` | function | Returns the identity strength associated with `CaptureSource`. | `src/capture/identity.rs:94` |
| `into_callback` | function | Converts `CapturedFrameSender` into callback. | `src/capture/frame_stream.rs:132` |
| `matches` | function | Returns whether an input satisfies `SourceQuery`. | `src/capture/query.rs:22` |
| `new` | function | Creates a new `CapturePermissionLifecycle`. | `src/capture/authorization.rs:189` |
| `new` | function | Creates a new `StableSourceId`. | `src/capture/identity.rs:33` |
| `next` | function | Advances the local evidence epoch after an observed authorization change or an explicit source reopen. | `src/capture/authorization.rs:274` |
| `next` | function | Returns the generation assigned after explicit rediscovery. | `src/capture/events.rs:18` |
| `observation_handle` | function | Returns a handle for reading observations from `SourceRuntimeEventSender`. | `src/capture/events.rs:270` |
| `observation_handle` | function | Returns a handle for reading observations from `CapturedFrameSender`. | `src/capture/frame_stream.rs:142` |
| `observations` | function | Returns the observations exposed by `SourceRuntimeEventObservationHandle`. | `src/capture/events.rs:205` |
| `observations` | function | Returns the observations exposed by `SourceRuntimeEventSender`. | `src/capture/events.rs:266` |
| `observations` | function | Returns the observations exposed by `CapturedFrameObservationHandle`. | `src/capture/frame_stream.rs:36` |
| `observations` | function | Returns the observations exposed by `CaptureObservationHandle`. | `src/capture/observations.rs:37` |
| `observe` | function | Returns the current observation exposed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:204` |
| `permission_epoch` | function | Returns the permission epoch associated with `CapturePermissionLifecycle`. | `src/capture/authorization.rs:200` |
| `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| `pocketstation::capture::query::discover_sources` | function | Discovers capture sources available from the local provider. | `src/capture/query.rs:85` |
| `pocketstation::capture::query::resolve_query` | function | Resolves query for `query`. | `src/capture/query.rs:40` |
| `process_tree_scope` | function | Reports the native process boundary represented by this discovery result without making the CLI reconstruct a private capture mode. | `src/capture/identity.rs:140` |
| `process_tree_scope` | function | Reports the process boundary requested from the native backend. | `src/capture/selection.rs:55` |
| `query::SourceProvider::discover` | function | Discovers the resources visible to `SourceProvider`. | `src/capture/query.rs:49` |
| `selector_persistence_scope` | function | Reports how long this discovered selector can be reused without rediscovery. The capture owner remains authoritative for opening it. | `src/capture/identity.rs:114` |
| `selector_persistence_scope` | function | Describes how long the selector may be reused without rediscovery. | `src/capture/selection.rs:36` |
| `source_id` | function | Derives the immutable captured-frame identity for this resolved source. | `src/capture/identity.rs:46` |
| `stats` | function | Returns the current statistics for `CapturedFrameSender`. | `src/capture/frame_stream.rs:138` |
| `try_send` | function | Publishes from a capture worker without blocking. When the bounded control channel is full, the newest event is dropped and counted. | `src/capture/events.rs:232` |
| `try_send` | function | Attempts to send a value through `CapturedFrameSender` without waiting for capacity. | `src/capture/frame_stream.rs:109` |
| `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| `identity` | module | Stable source identity and source discovery state. | `src/capture/identity.rs:1` |
| `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::authorization::PermissionEpoch` | struct | Represents permission epoch in the PocketStation API. | `src/capture/authorization.rs:267` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| `pocketstation::capture::events::CaptureRuntimeFailure` | struct | Reports a capture runtime failure. | `src/capture/events.rs:47` |
| `pocketstation::capture::events::SourceGeneration` | struct | Represents source generation in the PocketStation API. | `src/capture/events.rs:12` |
| `pocketstation::capture::events::SourceRuntimeEventObservationHandle` | struct | Owns bounded access to source runtime event observation. | `src/capture/events.rs:200` |
| `pocketstation::capture::events::SourceRuntimeEventObservations` | struct | Reports the source runtime event observations collected at an observation boundary. | `src/capture/events.rs:111` |
| `pocketstation::capture::events::SourceRuntimeEventSender` | struct | Represents source runtime event sender in the PocketStation API. | `src/capture/events.rs:224` |
| `pocketstation::capture::frame_stream::CapturedFrameObservationHandle` | struct | Owns bounded access to captured frame observation. | `src/capture/frame_stream.rs:31` |
| `pocketstation::capture::frame_stream::CapturedFrameSender` | struct | Single-producer endpoint passed into a platform capture callback. | `src/capture/frame_stream.rs:102` |
| `pocketstation::capture::frame_stream::CapturedFrameStreamStats` | struct | Reports the captured frame stream stats collected at an observation boundary. | `src/capture/frame_stream.rs:17` |
| `pocketstation::capture::identity::CaptureSource` | struct | Represents capture source in the PocketStation API. | `src/capture/identity.rs:82` |
| `pocketstation::capture::identity::StableSourceId` | struct | Uniquely identifies stable source. | `src/capture/identity.rs:26` |
| `pocketstation::capture::observations::CaptureObservationHandle` | struct | Owns bounded access to capture observation. | `src/capture/observations.rs:32` |
| `pocketstation::capture::observations::CaptureObservations` | struct | Reports the capture observations collected at an observation boundary. | `src/capture/observations.rs:8` |
| `pocketstation::capture::query::LocalSourceProvider` | struct | Represents local source provider in the PocketStation API. | `src/capture/query.rs:52` |
| `CaptureDelivery::frame_sender` | struct_field | Stores the frame sender associated with `CaptureDelivery`. | `src/capture/capture_owner.rs:74` |
| `CaptureDelivery::runtime_event_sender` | struct_field | Stores the runtime event sender associated with `CaptureDelivery`. | `src/capture/capture_owner.rs:75` |
| `CaptureObservations::callback_buffers_total` | struct_field | Counts the total number of callback buffers observed by `CaptureObservations`. | `src/capture/observations.rs:9` |
| `CaptureObservations::dispatch_queue_full_total` | struct_field | Counts the total number of dispatch queue full observed by `CaptureObservations`. | `src/capture/observations.rs:12` |
| `CaptureObservations::frames_enqueued_total` | struct_field | Counts the total number of frames enqueued observed by `CaptureObservations`. | `src/capture/observations.rs:10` |
| `CaptureObservations::invalid_buffer_total` | struct_field | Counts the total number of invalid buffer observed by `CaptureObservations`. | `src/capture/observations.rs:13` |
| `CaptureObservations::oversized_buffer_total` | struct_field | Counts the total number of oversized buffer observed by `CaptureObservations`. | `src/capture/observations.rs:14` |
| `CaptureObservations::pool_exhausted_total` | struct_field | Counts the total number of pool exhausted observed by `CaptureObservations`. | `src/capture/observations.rs:11` |
| `CaptureObservations::stream_errors_total` | struct_field | Counts the total number of stream errors observed by `CaptureObservations`. | `src/capture/observations.rs:15` |
| `CaptureObservations::timestamp_epoch_clamps_total` | struct_field | Counts the total number of timestamp epoch clamps observed by `CaptureObservations`. | `src/capture/observations.rs:16` |
| `CaptureOwnerObservations::backend` | struct_field | Stores the backend associated with `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:161` |
| `CaptureOwnerObservations::frame_stream` | struct_field | Stores the frame stream associated with `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:162` |
| `CaptureOwnerObservations::runtime_events` | struct_field | Stores the runtime events associated with `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:163` |
| `CapturedFrameStreamStats::delivered_frames` | struct_field | Stores the delivered frames associated with `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:18` |
| `CapturedFrameStreamStats::dropped_newest_frames` | struct_field | Stores the dropped newest frames associated with `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:19` |
| `CapturedFrameStreamStats::frames_discarded_before_start_total` | struct_field | Counts the total number of frames discarded before start observed by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:20` |
| `authorization::CaptureAuthorizationSnapshot::application_policy` | struct_field | Stores the application policy associated with `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:20` |
| `authorization::CaptureAuthorizationSnapshot::capability` | struct_field | Stores the capability associated with `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:18` |
| `authorization::CaptureAuthorizationSnapshot::capture_scope` | struct_field | Stores the capture scope associated with `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:22` |
| `authorization::CaptureAuthorizationSnapshot::identity_strength` | struct_field | Stores the identity strength associated with `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:23` |
| `authorization::CaptureAuthorizationSnapshot::observed_at_ns` | struct_field | Stores the observed at value for `CaptureAuthorizationSnapshot`, in nanoseconds. | `src/capture/authorization.rs:25` |
| `authorization::CaptureAuthorizationSnapshot::open_outcome` | struct_field | Stores the open outcome associated with `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:26` |
| `authorization::CaptureAuthorizationSnapshot::os_permission` | struct_field | Stores the os permission associated with `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:19` |
| `authorization::CaptureAuthorizationSnapshot::permission_epoch` | struct_field | Stores the permission epoch associated with `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:24` |
| `authorization::CaptureAuthorizationSnapshot::session_grant` | struct_field | Stores the session grant associated with `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:21` |
| `authorization::CaptureError::BackendSetupRequired::action` | struct_field | Stores the action associated with `BackendSetupRequired`. | `src/capture/authorization.rs:298` |
| `authorization::CaptureError::BackendSetupRequired::backend` | struct_field | Stores the backend associated with `BackendSetupRequired`. | `src/capture/authorization.rs:297` |
| `authorization::CaptureError::BackendStatus::operation` | struct_field | Stores the operation associated with `BackendStatus`. | `src/capture/authorization.rs:304` |
| `authorization::CaptureError::BackendStatus::status_code` | struct_field | Stores the status code associated with `BackendStatus`. | `src/capture/authorization.rs:305` |
| `authorization::CaptureError::CaptureWorkerPanicked::worker` | struct_field | Stores the worker associated with `CaptureWorkerPanicked`. | `src/capture/authorization.rs:316` |
| `authorization::CaptureError::PermissionDenied::operation` | struct_field | Stores the operation associated with `PermissionDenied`. | `src/capture/authorization.rs:301` |
| `authorization::CaptureError::SourceUnavailable::stable_key` | struct_field | Stores the stable key associated with `SourceUnavailable`. | `src/capture/authorization.rs:308` |
| `authorization::CapturePermissionTransition::current` | struct_field | Stores the current associated with `CapturePermissionTransition`. | `src/capture/authorization.rs:171` |
| `authorization::CapturePermissionTransition::kind` | struct_field | Stores the kind associated with `CapturePermissionTransition`. | `src/capture/authorization.rs:169` |
| `authorization::CapturePermissionTransition::permission_epoch` | struct_field | Stores the permission epoch associated with `CapturePermissionTransition`. | `src/capture/authorization.rs:172` |
| `authorization::CapturePermissionTransition::previous` | struct_field | Stores the previous associated with `CapturePermissionTransition`. | `src/capture/authorization.rs:170` |
| `authorization::CaptureScope::ExactApplication::stable_id` | struct_field | Identifies the stable associated with `ExactApplication`. | `src/capture/authorization.rs:249` |
| `authorization::CaptureScope::ExactInputDevice::stable_id` | struct_field | Identifies the stable associated with `ExactInputDevice`. | `src/capture/authorization.rs:250` |
| `authorization::CaptureScope::ExactOutputDevice::stable_id` | struct_field | Identifies the stable associated with `ExactOutputDevice`. | `src/capture/authorization.rs:251` |
| `events::CaptureRuntimeFailure::error_class` | struct_field | Stores the error class associated with `CaptureRuntimeFailure`. | `src/capture/events.rs:49` |
| `events::CaptureRuntimeFailure::operation` | struct_field | Stores the operation associated with `CaptureRuntimeFailure`. | `src/capture/events.rs:48` |
| `events::CaptureRuntimeFailureClass::BackendClass::class` | struct_field | Stores the class associated with `BackendClass`. | `src/capture/events.rs:43` |
| `events::CaptureRuntimeFailureClass::PlatformStatus::status_code` | struct_field | Stores the status code associated with `PlatformStatus`. | `src/capture/events.rs:42` |
| `events::SourceRuntimeEvent::BackendFailure::failure` | struct_field | Carries the failure reported by `BackendFailure`. | `src/capture/events.rs:63` |
| `events::SourceRuntimeEvent::BackendFailure::generation` | struct_field | Stores the generation associated with `BackendFailure`. | `src/capture/events.rs:62` |
| `events::SourceRuntimeEvent::BackendFailure::stable_id` | struct_field | Identifies the stable associated with `BackendFailure`. | `src/capture/events.rs:61` |
| `events::SourceRuntimeEvent::SourceUnavailable::failure` | struct_field | Carries the failure reported by `SourceUnavailable`. | `src/capture/events.rs:58` |
| `events::SourceRuntimeEvent::SourceUnavailable::generation` | struct_field | Stores the generation associated with `SourceUnavailable`. | `src/capture/events.rs:56` |
| `events::SourceRuntimeEvent::SourceUnavailable::recovery_requirement` | struct_field | Stores the recovery requirement associated with `SourceUnavailable`. | `src/capture/events.rs:57` |
| `events::SourceRuntimeEvent::SourceUnavailable::stable_id` | struct_field | Identifies the stable associated with `SourceUnavailable`. | `src/capture/events.rs:55` |
| `events::SourceRuntimeEventObservations::capacity_event_count` | struct_field | Sets the capacity event count available to `SourceRuntimeEventObservations`. | `src/capture/events.rs:112` |
| `events::SourceRuntimeEventObservations::depth_events` | struct_field | Reports the depth events observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:115` |
| `events::SourceRuntimeEventObservations::depth_owned_bytes` | struct_field | Stores the depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:116` |
| `events::SourceRuntimeEventObservations::events_dropped_oversized_total` | struct_field | Counts the total number of events dropped oversized observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:120` |
| `events::SourceRuntimeEventObservations::events_dropped_total` | struct_field | Counts the total number of events dropped observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:119` |
| `events::SourceRuntimeEventObservations::events_enqueued_total` | struct_field | Counts the total number of events enqueued observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:118` |
| `events::SourceRuntimeEventObservations::maximum_buffered_owned_bytes` | struct_field | Stores the maximum buffered owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:114` |
| `events::SourceRuntimeEventObservations::maximum_event_owned_bytes` | struct_field | Stores the maximum event owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:113` |
| `events::SourceRuntimeEventObservations::peak_depth_owned_bytes` | struct_field | Stores the peak depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:117` |
| `identity::CaptureSource::app_id` | struct_field | Identifies the app associated with `CaptureSource`. | `src/capture/identity.rs:86` |
| `identity::CaptureSource::channels` | struct_field | Stores the channels associated with `CaptureSource`. | `src/capture/identity.rs:90` |
| `identity::CaptureSource::device_uid` | struct_field | Stores the device uid associated with `CaptureSource`. | `src/capture/identity.rs:87` |
| `identity::CaptureSource::name` | struct_field | Stores the name associated with `CaptureSource`. | `src/capture/identity.rs:84` |
| `identity::CaptureSource::process_id` | struct_field | Identifies the process associated with `CaptureSource`. | `src/capture/identity.rs:85` |
| `identity::CaptureSource::sample_rate_hz` | struct_field | Stores the sample rate value for `CaptureSource`, in hertz. | `src/capture/identity.rs:89` |
| `identity::CaptureSource::stable_id` | struct_field | Identifies the stable associated with `CaptureSource`. | `src/capture/identity.rs:83` |
| `identity::CaptureSource::state` | struct_field | Stores the state associated with `CaptureSource`. | `src/capture/identity.rs:88` |
| `identity::StableSourceId::kind` | struct_field | Stores the kind associated with `StableSourceId`. | `src/capture/identity.rs:28` |
| `identity::StableSourceId::platform` | struct_field | Stores the platform associated with `StableSourceId`. | `src/capture/identity.rs:27` |
| `identity::StableSourceId::stable_key` | struct_field | Stores the stable key associated with `StableSourceId`. | `src/capture/identity.rs:29` |
| `selection::CaptureMode::ExactApplication::process_id` | struct_field | Identifies the process associated with `ExactApplication`. | `src/capture/selection.rs:22` |
| `selection::CaptureMode::ExactApplication::stable_id` | struct_field | Identifies the stable associated with `ExactApplication`. | `src/capture/selection.rs:23` |
| `selection::CaptureMode::ExactApplicationStable::stable_id` | struct_field | Identifies the stable associated with `ExactApplicationStable`. | `src/capture/selection.rs:26` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::capture::query::SourceProvider` | trait | Defines the implementation contract for source. | `src/capture/query.rs:48` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::Allowed` | variant | Selects allowed behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:232` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::Denied` | variant | Selects denied behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:233` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::NotApplicable` | variant | Selects not applicable behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:235` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::NotObservable` | variant | Selects not observable behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:234` |
| `pocketstation::capture::authorization::CaptureCapabilityState::Available` | variant | Indicates the available state for `CaptureCapabilityState`. | `src/capture/authorization.rs:146` |
| `pocketstation::capture::authorization::CaptureCapabilityState::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/capture/authorization.rs:147` |
| `pocketstation::capture::authorization::CaptureCapabilityState::Unsupported` | variant | Reports that the requested operation is unsupported. | `src/capture/authorization.rs:148` |
| `pocketstation::capture::authorization::CaptureError::BackendInit` | variant | Reports backend init. | `src/capture/authorization.rs:294` |
| `pocketstation::capture::authorization::CaptureError::BackendSetupRequired` | variant | Reports backend setup required. | `src/capture/authorization.rs:296` |
| `pocketstation::capture::authorization::CaptureError::BackendStatus` | variant | Reports backend status. | `src/capture/authorization.rs:303` |
| `pocketstation::capture::authorization::CaptureError::CaptureWorkerPanicked` | variant | Reports capture worker panicked. | `src/capture/authorization.rs:316` |
| `pocketstation::capture::authorization::CaptureError::InvalidRuntimeEventCapacity` | variant | Reports invalid runtime event capacity. | `src/capture/authorization.rs:314` |
| `pocketstation::capture::authorization::CaptureError::InvalidStreamCapacity` | variant | Reports invalid stream capacity. | `src/capture/authorization.rs:312` |
| `pocketstation::capture::authorization::CaptureError::ModeUnsupported` | variant | Reports mode unsupported. | `src/capture/authorization.rs:310` |
| `pocketstation::capture::authorization::CaptureError::NotSupported` | variant | Reports not supported. | `src/capture/authorization.rs:292` |
| `pocketstation::capture::authorization::CaptureError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:301` |
| `pocketstation::capture::authorization::CaptureError::SourceUnavailable` | variant | Reports source unavailable. | `src/capture/authorization.rs:308` |
| `pocketstation::capture::authorization::CaptureOpenOutcome::BackendFailed` | variant | Indicates the backend failed state for `CaptureOpenOutcome`. | `src/capture/authorization.rs:286` |
| `pocketstation::capture::authorization::CaptureOpenOutcome::NotAttempted` | variant | Indicates the not attempted state for `CaptureOpenOutcome`. | `src/capture/authorization.rs:282` |
| `pocketstation::capture::authorization::CaptureOpenOutcome::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:284` |
| `pocketstation::capture::authorization::CaptureOpenOutcome::SourceUnavailable` | variant | Indicates the source unavailable state for `CaptureOpenOutcome`. | `src/capture/authorization.rs:285` |
| `pocketstation::capture::authorization::CaptureOpenOutcome::Succeeded` | variant | Indicates the succeeded state for `CaptureOpenOutcome`. | `src/capture/authorization.rs:283` |
| `pocketstation::capture::authorization::CaptureScope::ExactApplication` | variant | Selects exact application behavior for `CaptureScope`. | `src/capture/authorization.rs:249` |
| `pocketstation::capture::authorization::CaptureScope::ExactInputDevice` | variant | Selects exact input device behavior for `CaptureScope`. | `src/capture/authorization.rs:250` |
| `pocketstation::capture::authorization::CaptureScope::ExactOutputDevice` | variant | Selects exact output device behavior for `CaptureScope`. | `src/capture/authorization.rs:251` |
| `pocketstation::capture::authorization::CaptureScope::SystemMix` | variant | Selects system mix behavior for `CaptureScope`. | `src/capture/authorization.rs:252` |
| `pocketstation::capture::authorization::CaptureSessionGrant::Denied` | variant | Represents the denied case of `CaptureSessionGrant`. | `src/capture/authorization.rs:242` |
| `pocketstation::capture::authorization::CaptureSessionGrant::GrantedByExplicitSelection` | variant | Represents the granted by explicit selection case of `CaptureSessionGrant`. | `src/capture/authorization.rs:241` |
| `pocketstation::capture::authorization::CaptureSessionGrant::NotEvaluated` | variant | Represents the not evaluated case of `CaptureSessionGrant`. | `src/capture/authorization.rs:243` |
| `pocketstation::capture::authorization::PermissionObservation::Allowed` | variant | Represents the allowed case of `PermissionObservation`. | `src/capture/authorization.rs:154` |
| `pocketstation::capture::authorization::PermissionObservation::Denied` | variant | Represents the denied case of `PermissionObservation`. | `src/capture/authorization.rs:155` |
| `pocketstation::capture::authorization::PermissionObservation::NotApplicable` | variant | Represents the not applicable case of `PermissionObservation`. | `src/capture/authorization.rs:160` |
| `pocketstation::capture::authorization::PermissionObservation::NotDetermined` | variant | Represents the not determined case of `PermissionObservation`. | `src/capture/authorization.rs:157` |
| `pocketstation::capture::authorization::PermissionObservation::NotObservable` | variant | Represents the not observable case of `PermissionObservation`. | `src/capture/authorization.rs:159` |
| `pocketstation::capture::authorization::PermissionObservation::Restricted` | variant | Represents the restricted case of `PermissionObservation`. | `src/capture/authorization.rs:156` |
| `pocketstation::capture::authorization::PermissionObservation::Revoked` | variant | Represents the revoked case of `PermissionObservation`. | `src/capture/authorization.rs:158` |
| `pocketstation::capture::authorization::SourceIdentityStrength::ApplicationIdAndProcessId` | variant | Represents the application id and process identifier case of `SourceIdentityStrength`. | `src/capture/authorization.rs:258` |
| `pocketstation::capture::authorization::SourceIdentityStrength::PlatformStableId` | variant | Represents the platform stable identifier case of `SourceIdentityStrength`. | `src/capture/authorization.rs:262` |
| `pocketstation::capture::authorization::SourceIdentityStrength::ProcessId` | variant | Represents the process identifier case of `SourceIdentityStrength`. | `src/capture/authorization.rs:260` |
| `pocketstation::capture::authorization::SourceIdentityStrength::StableApplicationId` | variant | Represents the stable application identifier case of `SourceIdentityStrength`. | `src/capture/authorization.rs:259` |
| `pocketstation::capture::authorization::SourceIdentityStrength::StableDeviceUid` | variant | Represents the stable device uid case of `SourceIdentityStrength`. | `src/capture/authorization.rs:261` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass::BackendClass` | variant | Reports backend class. | `src/capture/events.rs:43` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass::PlatformStatus` | variant | Reports platform status. | `src/capture/events.rs:42` |
| `pocketstation::capture::events::CaptureRuntimeFailureClass::SourceInstanceExited` | variant | Reports source instance exited. | `src/capture/events.rs:41` |
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionChanged` | variant | Indicates the permission changed state for `SourceLifecycleEventKind`. | `src/capture/events.rs:28` |
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionRevoked` | variant | Indicates the permission revoked state for `SourceLifecycleEventKind`. | `src/capture/events.rs:29` |
| `pocketstation::capture::events::SourceLifecycleEventKind::ReplacementObserved` | variant | Indicates the replacement observed state for `SourceLifecycleEventKind`. | `src/capture/events.rs:27` |
| `pocketstation::capture::events::SourceLifecycleEventKind::SourceReappeared` | variant | Indicates the source reappeared state for `SourceLifecycleEventKind`. | `src/capture/events.rs:30` |
| `pocketstation::capture::events::SourceLifecycleEventKind::SourceUnavailable` | variant | Indicates the source unavailable state for `SourceLifecycleEventKind`. | `src/capture/events.rs:26` |
| `pocketstation::capture::events::SourceRecoveryRequirement::ExplicitRediscoveryAndNewSession` | variant | Selects explicit rediscovery and new session behavior for `SourceRecoveryRequirement`. | `src/capture/events.rs:36` |
| `pocketstation::capture::events::SourceRuntimeEvent::BackendFailure` | variant | Indicates the backend failure state for `SourceRuntimeEvent`. | `src/capture/events.rs:60` |
| `pocketstation::capture::events::SourceRuntimeEvent::SourceUnavailable` | variant | Indicates the source unavailable state for `SourceRuntimeEvent`. | `src/capture/events.rs:54` |
| `pocketstation::capture::events::SourceRuntimeEventDelivery::DroppedFull` | variant | Indicates the dropped full state for `SourceRuntimeEventDelivery`. | `src/capture/events.rs:98` |
| `pocketstation::capture::events::SourceRuntimeEventDelivery::DroppedOversized` | variant | Indicates the dropped oversized state for `SourceRuntimeEventDelivery`. | `src/capture/events.rs:99` |
| `pocketstation::capture::events::SourceRuntimeEventDelivery::Enqueued` | variant | Indicates the enqueued state for `SourceRuntimeEventDelivery`. | `src/capture/events.rs:97` |
| `pocketstation::capture::events::SourceRuntimeEventDelivery::ReceiverClosed` | variant | Indicates the receiver closed state for `SourceRuntimeEventDelivery`. | `src/capture/events.rs:100` |
| `pocketstation::capture::frame_stream::CapturedFrameDelivery::Delivered` | variant | Indicates the delivered state for `CapturedFrameDelivery`. | `src/capture/frame_stream.rs:11` |
| `pocketstation::capture::frame_stream::CapturedFrameDelivery::DiscardedBeforeStart` | variant | Indicates the discarded before start state for `CapturedFrameDelivery`. | `src/capture/frame_stream.rs:13` |
| `pocketstation::capture::frame_stream::CapturedFrameDelivery::DroppedNewest` | variant | Indicates the dropped newest state for `CapturedFrameDelivery`. | `src/capture/frame_stream.rs:12` |
| `pocketstation::capture::identity::SourceKind::Application` | variant | Selects application behavior for `SourceKind`. | `src/capture/identity.rs:10` |
| `pocketstation::capture::identity::SourceKind::InputDevice` | variant | Selects input device behavior for `SourceKind`. | `src/capture/identity.rs:12` |
| `pocketstation::capture::identity::SourceKind::OutputDevice` | variant | Selects output device behavior for `SourceKind`. | `src/capture/identity.rs:11` |
| `pocketstation::capture::identity::SourceKind::SystemMix` | variant | Selects system mix behavior for `SourceKind`. | `src/capture/identity.rs:13` |
| `pocketstation::capture::identity::SourceState::Available` | variant | Indicates the available state for `SourceState`. | `src/capture/identity.rs:18` |
| `pocketstation::capture::identity::SourceState::PermissionBlocked` | variant | Indicates the permission blocked state for `SourceState`. | `src/capture/identity.rs:22` |
| `pocketstation::capture::identity::SourceState::Playing` | variant | Indicates the playing state for `SourceState`. | `src/capture/identity.rs:19` |
| `pocketstation::capture::identity::SourceState::Silent` | variant | Indicates the silent state for `SourceState`. | `src/capture/identity.rs:20` |
| `pocketstation::capture::identity::SourceState::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/capture/identity.rs:21` |
| `pocketstation::capture::query::SourceQuery::Any` | variant | Represents the any case of `SourceQuery`. | `src/capture/query.rs:14` |
| `pocketstation::capture::query::SourceQuery::App` | variant | Represents the app case of `SourceQuery`. | `src/capture/query.rs:15` |
| `pocketstation::capture::query::SourceQuery::ByKind` | variant | Represents the by kind case of `SourceQuery`. | `src/capture/query.rs:16` |
| `pocketstation::capture::query::SourceQuery::ByStableKey` | variant | Represents the by stable key case of `SourceQuery`. | `src/capture/query.rs:17` |
| `pocketstation::capture::query::SourceQuery::Playing` | variant | Represents the playing case of `SourceQuery`. | `src/capture/query.rs:18` |
| `pocketstation::capture::selection::CaptureMode::Application` | variant | Selects application behavior for `CaptureMode`. | `src/capture/selection.rs:19` |
| `pocketstation::capture::selection::CaptureMode::ExactApplication` | variant | Selects exact application behavior for `CaptureMode`. | `src/capture/selection.rs:21` |
| `pocketstation::capture::selection::CaptureMode::ExactApplicationStable` | variant | Selects exact application stable behavior for `CaptureMode`. | `src/capture/selection.rs:25` |
| `pocketstation::capture::selection::CaptureMode::InputDevice` | variant | Selects input device behavior for `CaptureMode`. | `src/capture/selection.rs:28` |
| `pocketstation::capture::selection::CaptureMode::Process` | variant | Selects process behavior for `CaptureMode`. | `src/capture/selection.rs:20` |
| `pocketstation::capture::selection::CaptureMode::SystemMix` | variant | Selects system mix behavior for `CaptureMode`. | `src/capture/selection.rs:18` |
| `pocketstation::capture::selection::InputDeviceSelector::Default` | variant | Selects default behavior for `InputDeviceSelector`. | `src/capture/selection.rs:11` |
| `pocketstation::capture::selection::InputDeviceSelector::StableId` | variant | Selects stable identifier behavior for `InputDeviceSelector`. | `src/capture/selection.rs:12` |
| `pocketstation::capture::selection::ProcessTreeScope::ApplicationIdentity` | variant | Selects application identity behavior for `ProcessTreeScope`. | `src/capture/selection.rs:86` |
| `pocketstation::capture::selection::ProcessTreeScope::NotApplicable` | variant | Selects not applicable behavior for `ProcessTreeScope`. | `src/capture/selection.rs:87` |
| `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessAndDescendants` | variant | Selects selected process and descendants behavior for `ProcessTreeScope`. | `src/capture/selection.rs:85` |
| `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessOnly` | variant | Selects selected process only behavior for `ProcessTreeScope`. | `src/capture/selection.rs:84` |
| `pocketstation::capture::selection::SelectorPersistenceScope::ApplicationIdentity` | variant | Selects application identity behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:75` |
| `pocketstation::capture::selection::SelectorPersistenceScope::DeviceIdentity` | variant | Selects device identity behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:76` |
| `pocketstation::capture::selection::SelectorPersistenceScope::PlatformIdentity` | variant | Selects platform identity behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:78` |
| `pocketstation::capture::selection::SelectorPersistenceScope::ProcessLifetime` | variant | Selects process lifetime behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:74` |
| `pocketstation::capture::selection::SelectorPersistenceScope::SessionDefaultDevice` | variant | Selects session default device behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:77` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/mod.rs:1-65` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
