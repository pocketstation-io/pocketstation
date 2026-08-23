# Recording API

<!-- claims: CLM-REF-008-CAP-001,CLM-REF-008-SOURCE-001 -->

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

The scope of **Recording API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Recording API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-96f659bd569e1533d2d8 | `pocketstation::recording::endpoint::MULTISTEM_GROUP_CONFIGURATION_KEY` | constant | Defines the public multistem group configuration key value. | `src/recording/endpoint.rs:24` |
| sym-aa9aa2ac45f752b6f16a | `pocketstation::recording::endpoint::MULTISTEM_NAME_CONFIGURATION_KEY` | constant | Defines the public multistem name configuration key value. | `src/recording/endpoint.rs:25` |
| sym-257559049c741e0b7fdd | `pocketstation::recording::config::PermissionDecision` | enum | Enumerates the supported permission decision cases. | `src/recording/config.rs:43` |
| sym-c1c408fed30ef9989c9e | `pocketstation::recording::config::PermissionScope` | enum | Selects the permission scope used by PocketStation. | `src/recording/config.rs:50` |
| sym-d088b4347207780b0458 | `pocketstation::recording::config::RecorderLineageField` | enum | Enumerates the supported recorder lineage field cases. | `src/recording/config.rs:10` |
| sym-9703518ae35e1e6a80d6 | `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| sym-d6ebc5c666e65cd800a5 | `pocketstation::recording::writer::DiscontinuityKind` | enum | Selects the discontinuity kind used by PocketStation. | `src/recording/writer.rs:104` |
| sym-3bdbbc813bb88358314c | `pocketstation::recording::writer::RecorderError` | enum | Classifies failures reported as recorder error. | `src/recording/writer.rs:23` |
| sym-3e43b04c63f0fb274f6e | `pocketstation::recording::writer::RecordingState` | enum | Selects the recording state used by PocketStation. | `src/recording/writer.rs:85` |
| sym-e380c2ab6ae4344224a7 | `as_str` | function | Returns the stable string representation of `StemLabel`. | `src/recording/config.rs:36` |
| sym-77781a10d7c76e7960c7 | `as_str` | function | Returns the stable string representation of `RecordingErrorCode`. | `src/recording/error_code.rs:32` |
| sym-e11f8c63a3184f4b90fa | `code` | function | Returns the stable error or status code represented by `RecorderError`. | `src/recording/error_code.rs:59` |
| sym-7d4e6f0d540eb10ca70a | `drop` | function | Releases resources owned by `MultistemRecording`. | `src/recording/writer.rs:355` |
| sym-68e27ca34c656c2a0992 | `finish` | function | Finishes work owned by `MultistemRecording`. | `src/recording/writer.rs:278` |
| sym-11060a30b8e68b52a0a1 | `group_id` | function | Returns the group identifier held by `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:70` |
| sym-018a439dc3f2a9da4841 | `new` | function | Creates a new `StemLabel`. | `src/recording/config.rs:23` |
| sym-681108e3e5d1ba2c1c54 | `new` | function | Creates a new `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:56` |
| sym-583d951b814628e2833f | `observations` | function | Returns the observations exposed by `MultistemRecording`. | `src/recording/writer.rs:250` |
| sym-2d05b989e1f3feab47de | `output_root` | function | Returns the output root associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:65` |
| sym-2eb6b21ee19ecb2856b6 | `pocketstation::recording::error_code::recording_outcome_error_code` | function | Returns the recording outcome error code held by `error_code`. | `src/recording/error_code.rs:82` |
| sym-b4e2dc46188095e6f2d6 | `preparation_group` | function | Returns the preparation group associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:82` |
| sym-7348171d3ec34d64b215 | `prepare` | function | Prepares resources required by `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:99` |
| sym-2a1b0d259f87ca189edb | `receipt` | function | Returns the receipt associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:74` |
| sym-edb622dbf724fc82a214 | `request_stop` | function | Requests a graceful stop from `MultistemRecording`. | `src/recording/writer.rs:272` |
| sym-fe9f27f4c3d118b2be48 | `result` | function | Returns the result represented by `MultistemRecordingReceipt`. | `src/recording/endpoint.rs:33` |
| sym-310a3085646b09a76102 | `pocketstation::recording::config::RecorderStemConfig` | struct | Configures recorder stem behavior at its owning API boundary. | `src/recording/config.rs:55` |
| sym-7a5d9b850a0a307684ee | `pocketstation::recording::config::StemLabel` | struct | Stores the validated human-readable label used for one recording stem. | `src/recording/config.rs:20` |
| sym-7f9542d3376d16437ab7 | `pocketstation::recording::endpoint::MultistemRecordingReceipt` | struct | Retains the identity and observation access returned for multistem recording. | `src/recording/endpoint.rs:28` |
| sym-23af3968ee72443beb9e | `pocketstation::recording::endpoint::SessionMultistemEndpointCoordinator` | struct | Canonical Session-owned multistem recorder declaration. | `src/recording/endpoint.rs:49` |
| sym-8395072f714378c544cb | `pocketstation::recording::writer::DiscontinuityRecord` | struct | Records one immutable discontinuity observation. | `src/recording/writer.rs:92` |
| sym-bcc0c641f0c70c19d325 | `pocketstation::recording::writer::MultistemRecording` | struct | Owns the per-stem recording workers and coordinates their terminal finalization outcome. | `src/recording/writer.rs:138` |
| sym-712535870fc618757b72 | `pocketstation::recording::writer::RecordingObservations` | struct | Reports the recording observations collected at an observation boundary. | `src/recording/writer.rs:130` |
| sym-bf5ce1b3444891dee634 | `pocketstation::recording::writer::RecordingOutcome` | struct | Reports the structured recording outcome. | `src/recording/writer.rs:111` |
| sym-94317d1483a96e604808 | `pocketstation::recording::writer::RecordingStemOutcome` | struct | Reports the structured recording stem outcome. | `src/recording/writer.rs:120` |
| sym-86d1d823e3794685596a | `DiscontinuityRecord::kind` | struct_field | Stores the kind used by `DiscontinuityRecord`. | `src/recording/writer.rs:95` |
| sym-2441ca89ecc458e9d840 | `DiscontinuityRecord::label` | struct_field | Stores the label used by `DiscontinuityRecord`. | `src/recording/writer.rs:94` |
| sym-5910cad8a108299c9553 | `DiscontinuityRecord::sequence_end` | struct_field | Stores the sequence end used by `DiscontinuityRecord`. | `src/recording/writer.rs:99` |
| sym-83781326ab920ef86604 | `DiscontinuityRecord::sequence_start` | struct_field | Stores the sequence start used by `DiscontinuityRecord`. | `src/recording/writer.rs:98` |
| sym-e1d1fb413497c6b7bc3f | `DiscontinuityRecord::stem_id` | struct_field | Identifies the stem identifier recorded by `DiscontinuityRecord`. | `src/recording/writer.rs:93` |
| sym-b900e3a681279e2edfbf | `DiscontinuityRecord::timestamp_end_ns` | struct_field | Stores the timestamp end value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:97` |
| sym-ae88bc73ec6d0e0cd105 | `DiscontinuityRecord::timestamp_start_ns` | struct_field | Stores the timestamp start value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:96` |
| sym-71238b467d4d0940767d | `RecorderError::FrameSpecMismatch::actual_channels` | struct_field | Stores the actual channels used by `FrameSpecMismatch`. | `src/recording/writer.rs:61` |
| sym-8294d3e3948681723ffd | `RecorderError::FrameSpecMismatch::actual_rate_hz` | struct_field | Stores the actual rate value for `FrameSpecMismatch`, in hertz. | `src/recording/writer.rs:60` |
| sym-ae395f7b6665c7ad0257 | `RecorderError::FrameSpecMismatch::expected_channels` | struct_field | Stores the expected channels used by `FrameSpecMismatch`. | `src/recording/writer.rs:63` |
| sym-8d51670c5d0ce778b89b | `RecorderError::FrameSpecMismatch::expected_rate_hz` | struct_field | Stores the expected rate value for `FrameSpecMismatch`, in hertz. | `src/recording/writer.rs:62` |
| sym-ecf0a3a88b242ac63017 | `RecorderError::FrameSpecMismatch::label` | struct_field | Stores the label used by `FrameSpecMismatch`. | `src/recording/writer.rs:59` |
| sym-652d1d091b04748211e7 | `RecorderError::GapTooLarge::duration_ns` | struct_field | Stores the duration value for `GapTooLarge`, in nanoseconds. | `src/recording/writer.rs:70` |
| sym-8c3d161113387de15e72 | `RecorderError::GapTooLarge::label` | struct_field | Stores the label used by `GapTooLarge`. | `src/recording/writer.rs:70` |
| sym-b219d7b9e5d6723afd96 | `RecorderError::InvalidSampleSpec::channels` | struct_field | Stores the channels used by `InvalidSampleSpec`. | `src/recording/writer.rs:42` |
| sym-d9e82eb856f533090b38 | `RecorderError::InvalidSampleSpec::label` | struct_field | Stores the label used by `InvalidSampleSpec`. | `src/recording/writer.rs:40` |
| sym-6ea612a44f559762056e | `RecorderError::InvalidSampleSpec::sample_rate_hz` | struct_field | Stores the sample rate value for `InvalidSampleSpec`, in hertz. | `src/recording/writer.rs:41` |
| sym-50eef94c282d64b04916 | `RecorderError::LineageMismatch::actual` | struct_field | Records the value observed by `LineageMismatch`. | `src/recording/writer.rs:54` |
| sym-5bc3401f723c24ad75e4 | `RecorderError::LineageMismatch::expected` | struct_field | Records the value expected by `LineageMismatch`. | `src/recording/writer.rs:55` |
| sym-ebbac0ff38977c987aac | `RecorderError::LineageMismatch::field` | struct_field | Stores the field used by `LineageMismatch`. | `src/recording/writer.rs:53` |
| sym-4ef03f5878022534ba51 | `RecorderError::LineageMismatch::label` | struct_field | Stores the label used by `LineageMismatch`. | `src/recording/writer.rs:52` |
| sym-1864bc6e2d75f2f14118 | `RecorderError::SessionMismatch::actual` | struct_field | Records the value observed by `SessionMismatch`. | `src/recording/writer.rs:33` |
| sym-2bf6b3f917bc84462f34 | `RecorderError::SessionMismatch::expected` | struct_field | Records the value expected by `SessionMismatch`. | `src/recording/writer.rs:34` |
| sym-b189c2bd0a53defdea5e | `RecorderError::SessionMismatch::label` | struct_field | Stores the label used by `SessionMismatch`. | `src/recording/writer.rs:32` |
| sym-f3b4fc16ca189f99b9f0 | `RecorderError::SourceMismatch::actual` | struct_field | Records the value observed by `SourceMismatch`. | `src/recording/writer.rs:47` |
| sym-e85ede34f6f81087152c | `RecorderError::SourceMismatch::expected` | struct_field | Records the value expected by `SourceMismatch`. | `src/recording/writer.rs:48` |
| sym-49e2f3a0387d6e36a214 | `RecorderError::SourceMismatch::label` | struct_field | Stores the label used by `SourceMismatch`. | `src/recording/writer.rs:46` |
| sym-0986cab597e6de1b7695 | `RecorderStemConfig::channels` | struct_field | Stores the channels used by `RecorderStemConfig`. | `src/recording/config.rs:66` |
| sym-8d49b77e7fdf693eacbc | `RecorderStemConfig::clock_id` | struct_field | Identifies the clock identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:59` |
| sym-92e037e3886b75006889 | `RecorderStemConfig::label` | struct_field | Stores the label used by `RecorderStemConfig`. | `src/recording/config.rs:64` |
| sym-2eafceaeacba85623d18 | `RecorderStemConfig::permission` | struct_field | Stores the permission used by `RecorderStemConfig`. | `src/recording/config.rs:63` |
| sym-5386f08f66607517e9cd | `RecorderStemConfig::permission_epoch` | struct_field | Stores the permission epoch used by `RecorderStemConfig`. | `src/recording/config.rs:61` |
| sym-38ae5238c40c96a92225 | `RecorderStemConfig::permission_scope` | struct_field | Stores the permission scope used by `RecorderStemConfig`. | `src/recording/config.rs:62` |
| sym-78bdd138f370820cdff4 | `RecorderStemConfig::sample_rate_hz` | struct_field | Stores the sample rate value for `RecorderStemConfig`, in hertz. | `src/recording/config.rs:65` |
| sym-2561bafb64813167480d | `RecorderStemConfig::session_id` | struct_field | Identifies the session identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:56` |
| sym-7f976c36294e47df090e | `RecorderStemConfig::source_generation` | struct_field | Stores the source generation used by `RecorderStemConfig`. | `src/recording/config.rs:60` |
| sym-648645677ef7c62f35e2 | `RecorderStemConfig::source_id` | struct_field | Identifies the source identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:57` |
| sym-6fceafe803bbefd0086c | `RecorderStemConfig::stem_id` | struct_field | Identifies the stem identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:58` |
| sym-c1936056dc0a991fd665 | `RecorderStemConfig::timeline_mapping` | struct_field | Stores the timeline mapping used by `RecorderStemConfig`. | `src/recording/config.rs:67` |
| sym-907a92060e1b3d3f0117 | `RecordingObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `RecordingObservations`. | `src/recording/writer.rs:134` |
| sym-dff1691ebf43fa4208f5 | `RecordingObservations::failures_total` | struct_field | Counts the total number of failures observed by `RecordingObservations`. | `src/recording/writer.rs:135` |
| sym-9403802bd3b2aa8d8a29 | `RecordingObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `RecordingObservations`. | `src/recording/writer.rs:131` |
| sym-8277db04333ba81da2a5 | `RecordingObservations::frames_rejected_total` | struct_field | Counts the total number of frames rejected observed by `RecordingObservations`. | `src/recording/writer.rs:133` |
| sym-1cf065a263b2c545a9d8 | `RecordingObservations::frames_written_total` | struct_field | Counts the total number of frames written observed by `RecordingObservations`. | `src/recording/writer.rs:132` |
| sym-cce6423e7357e2b8f22c | `RecordingOutcome::completed_stems` | struct_field | Stores the completed stems used by `RecordingOutcome`. | `src/recording/writer.rs:114` |
| sym-9e70d3f3fdc9c1516e2a | `RecordingOutcome::failed_stems` | struct_field | Stores the failed stems used by `RecordingOutcome`. | `src/recording/writer.rs:115` |
| sym-6aa351dac5c1cb34b677 | `RecordingOutcome::session_dir` | struct_field | Stores the session dir used by `RecordingOutcome`. | `src/recording/writer.rs:112` |
| sym-044c131802ff86e1be01 | `RecordingOutcome::state` | struct_field | Stores the state used by `RecordingOutcome`. | `src/recording/writer.rs:113` |
| sym-5ad93b4b3edb53d5b6a5 | `RecordingOutcome::stems` | struct_field | Stores the stems used by `RecordingOutcome`. | `src/recording/writer.rs:116` |
| sym-5b6c7f6a0aa1e99c38d9 | `RecordingStemOutcome::edge_observations` | struct_field | Stores the edge observations used by `RecordingStemOutcome`. | `src/recording/writer.rs:126` |
| sym-2d5e4745b4e61b79fecb | `RecordingStemOutcome::error` | struct_field | Stores the error used by `RecordingStemOutcome`. | `src/recording/writer.rs:125` |
| sym-f25ff7308e04715af922 | `RecordingStemOutcome::gap_ranges` | struct_field | Stores the gap ranges used by `RecordingStemOutcome`. | `src/recording/writer.rs:124` |
| sym-a8bad434fe08af8bbc6b | `RecordingStemOutcome::label` | struct_field | Stores the label used by `RecordingStemOutcome`. | `src/recording/writer.rs:121` |
| sym-9c5a22fd95dc6959fd3d | `RecordingStemOutcome::stale_frames` | struct_field | Stores the stale frames used by `RecordingStemOutcome`. | `src/recording/writer.rs:123` |
| sym-c350fe48abaced46c0d5 | `RecordingStemOutcome::written_frames` | struct_field | Stores the written frames used by `RecordingStemOutcome`. | `src/recording/writer.rs:122` |
| sym-1051b0c2eb03babe6742 | `pocketstation::recording::config::PermissionDecision::Allowed` | variant | Represents the allowed alternative defined by `PermissionDecision`. | `src/recording/config.rs:44` |
| sym-9b2324ae246c45f16c4f | `pocketstation::recording::config::PermissionDecision::Denied` | variant | Represents the denied alternative defined by `PermissionDecision`. | `src/recording/config.rs:45` |
| sym-7f4bf2169422bbba6e7d | `pocketstation::recording::config::PermissionScope::SessionCaptureGrant` | variant | Selects session capture grant behavior for `PermissionScope`. | `src/recording/config.rs:51` |
| sym-bc72319fd0af645d0508 | `pocketstation::recording::config::RecorderLineageField::Clock` | variant | Represents the clock alternative defined by `RecorderLineageField`. | `src/recording/config.rs:14` |
| sym-edc80b30cffb9d7bdbda | `pocketstation::recording::config::RecorderLineageField::PermissionEpoch` | variant | Represents the permission epoch alternative defined by `RecorderLineageField`. | `src/recording/config.rs:16` |
| sym-6d41fc8c3dc0142f91b9 | `pocketstation::recording::config::RecorderLineageField::Session` | variant | Represents the session alternative defined by `RecorderLineageField`. | `src/recording/config.rs:11` |
| sym-63ce6c5b7236bef69978 | `pocketstation::recording::config::RecorderLineageField::Source` | variant | Represents the source alternative defined by `RecorderLineageField`. | `src/recording/config.rs:12` |
| sym-34776690b0720e1eb5c4 | `pocketstation::recording::config::RecorderLineageField::SourceGeneration` | variant | Represents the source generation alternative defined by `RecorderLineageField`. | `src/recording/config.rs:15` |
| sym-8ecaef8805638fac01db | `pocketstation::recording::config::RecorderLineageField::Stem` | variant | Represents the stem alternative defined by `RecorderLineageField`. | `src/recording/config.rs:13` |
| sym-7d85bd344060da5b9f89 | `pocketstation::recording::error_code::RecordingErrorCode::DuplicateStemLabel` | variant | Reported when the owning operation encounters duplicate stem label. | `src/recording/error_code.rs:12` |
| sym-b7f2ed2090a92554d1ed | `pocketstation::recording::error_code::RecordingErrorCode::FrameSpecMismatch` | variant | Reported when the owning operation encounters frame spec mismatch. | `src/recording/error_code.rs:18` |
| sym-4d21318f4fd9ea0a7fb4 | `pocketstation::recording::error_code::RecordingErrorCode::GapTooLarge` | variant | Reported when the owning operation encounters gap too large. | `src/recording/error_code.rs:21` |
| sym-6f6a262d68369fc05e8b | `pocketstation::recording::error_code::RecordingErrorCode::Incomplete` | variant | Reported when the owning operation encounters incomplete. | `src/recording/error_code.rs:28` |
| sym-bdd235056a4f9790f4f8 | `pocketstation::recording::error_code::RecordingErrorCode::InvalidSampleSpec` | variant | Reported when the owning operation encounters invalid sample spec. | `src/recording/error_code.rs:15` |
| sym-181929c742670b831954 | `pocketstation::recording::error_code::RecordingErrorCode::InvalidStemLabel` | variant | Reported when the owning operation encounters invalid stem label. | `src/recording/error_code.rs:11` |
| sym-18fd50278ed6840f60be | `pocketstation::recording::error_code::RecordingErrorCode::IoFailed` | variant | Reported when the owning operation encounters I/O failed. | `src/recording/error_code.rs:24` |
| sym-dcb3d2c64d65124fa855 | `pocketstation::recording::error_code::RecordingErrorCode::JsonFailed` | variant | Reported when the owning operation encounters json failed. | `src/recording/error_code.rs:26` |
| sym-0c3a5287294bdb68c6e8 | `pocketstation::recording::error_code::RecordingErrorCode::LineageMismatch` | variant | Reported when the owning operation encounters lineage mismatch. | `src/recording/error_code.rs:17` |
| sym-e870f098f3fc6f37a6df | `pocketstation::recording::error_code::RecordingErrorCode::NotFinalized` | variant | Reported when the owning operation encounters not finalized. | `src/recording/error_code.rs:27` |
| sym-a2a90cdd4a99b624fa65 | `pocketstation::recording::error_code::RecordingErrorCode::OutputExists` | variant | Reported when the owning operation encounters output exists. | `src/recording/error_code.rs:10` |
| sym-d077e4454fa188e87f61 | `pocketstation::recording::error_code::RecordingErrorCode::PermissionDenied` | variant | Reports that the required permission was denied. | `src/recording/error_code.rs:14` |
| sym-4628e03b1ab60563e31d | `pocketstation::recording::error_code::RecordingErrorCode::SessionMismatch` | variant | Reported when the owning operation encounters session mismatch. | `src/recording/error_code.rs:13` |
| sym-fa8ca7fd54c6dad87881 | `pocketstation::recording::error_code::RecordingErrorCode::SourceMismatch` | variant | Reported when the owning operation encounters source mismatch. | `src/recording/error_code.rs:16` |
| sym-80eb48ce6fb03179ad93 | `pocketstation::recording::error_code::RecordingErrorCode::TimestampOutOfRange` | variant | Reported when the owning operation encounters timestamp out of range. | `src/recording/error_code.rs:20` |
| sym-187654a56f50bdc64104 | `pocketstation::recording::error_code::RecordingErrorCode::TooManyGaps` | variant | Reported when the owning operation encounters too many gaps. | `src/recording/error_code.rs:22` |
| sym-dc3f942a67f028277891 | `pocketstation::recording::error_code::RecordingErrorCode::UnalignedSamples` | variant | Reported when the owning operation encounters unaligned samples. | `src/recording/error_code.rs:19` |
| sym-d30e3ded5ad282d52460 | `pocketstation::recording::error_code::RecordingErrorCode::WavFailed` | variant | Reported when the owning operation encounters wav failed. | `src/recording/error_code.rs:25` |
| sym-0197e76cc91a39c99b7f | `pocketstation::recording::error_code::RecordingErrorCode::WorkerPanicked` | variant | Reported when the owning operation encounters worker panicked. | `src/recording/error_code.rs:23` |
| sym-52a62e0c868aa1e2fc01 | `pocketstation::recording::writer::DiscontinuityKind::OverlapRejected` | variant | Selects overlap rejected behavior for `DiscontinuityKind`. | `src/recording/writer.rs:107` |
| sym-59887c5a079f1b20ea6f | `pocketstation::recording::writer::DiscontinuityKind::SequenceGap` | variant | Selects sequence gap behavior for `DiscontinuityKind`. | `src/recording/writer.rs:106` |
| sym-5df382854d46d7ec5af0 | `pocketstation::recording::writer::DiscontinuityKind::TimestampGap` | variant | Selects timestamp gap behavior for `DiscontinuityKind`. | `src/recording/writer.rs:105` |
| sym-69b7754427bd82572376 | `pocketstation::recording::writer::RecorderError::DuplicateStemLabel` | variant | Reported when the owning operation encounters duplicate stem label. | `src/recording/writer.rs:29` |
| sym-6cbe33f995c99621972e | `pocketstation::recording::writer::RecorderError::FrameSpecMismatch` | variant | Reported when the owning operation encounters frame spec mismatch. | `src/recording/writer.rs:58` |
| sym-35235e7c62cda3697266 | `pocketstation::recording::writer::RecorderError::GapTooLarge` | variant | Reported when the owning operation encounters gap too large. | `src/recording/writer.rs:70` |
| sym-0f6c4f657e98fdd19871 | `pocketstation::recording::writer::RecorderError::InvalidSampleSpec` | variant | Reported when the owning operation encounters invalid sample spec. | `src/recording/writer.rs:39` |
| sym-74c1146dfcbd54ad1e9b | `pocketstation::recording::writer::RecorderError::InvalidStemLabel` | variant | Reported when the owning operation encounters invalid stem label. | `src/recording/writer.rs:27` |
| sym-3b68e704962144ca19c6 | `pocketstation::recording::writer::RecorderError::Io` | variant | Reported when the owning operation encounters I/O. | `src/recording/writer.rs:76` |
| sym-c9c8b30e2e6f65347ff8 | `pocketstation::recording::writer::RecorderError::Json` | variant | Reported when the owning operation encounters json. | `src/recording/writer.rs:80` |
| sym-c4476c73d1b1124b4b8c | `pocketstation::recording::writer::RecorderError::LineageMismatch` | variant | Reported when the owning operation encounters lineage mismatch. | `src/recording/writer.rs:51` |
| sym-ec4b5863a964193e05e8 | `pocketstation::recording::writer::RecorderError::OutputExists` | variant | Reported when the owning operation encounters output exists. | `src/recording/writer.rs:25` |
| sym-4858643bf5eb0df27a40 | `pocketstation::recording::writer::RecorderError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/recording/writer.rs:37` |
| sym-12f9f5a214e19457f852 | `pocketstation::recording::writer::RecorderError::SessionMismatch` | variant | Reported when the owning operation encounters session mismatch. | `src/recording/writer.rs:31` |
| sym-09f3d07430eef7d27c59 | `pocketstation::recording::writer::RecorderError::SourceMismatch` | variant | Reported when the owning operation encounters source mismatch. | `src/recording/writer.rs:45` |
| sym-ca54c477fcd3b74cf4d7 | `pocketstation::recording::writer::RecorderError::TimestampOutOfRange` | variant | Reported when the owning operation encounters timestamp out of range. | `src/recording/writer.rs:68` |
| sym-c72eeddfea335de04be5 | `pocketstation::recording::writer::RecorderError::TooManyGaps` | variant | Reported when the owning operation encounters too many gaps. | `src/recording/writer.rs:72` |
| sym-b24861afdeb150023f3d | `pocketstation::recording::writer::RecorderError::UnalignedSamples` | variant | Reported when the owning operation encounters unaligned samples. | `src/recording/writer.rs:66` |
| sym-75fa0a90f8a898449d9d | `pocketstation::recording::writer::RecorderError::Wav` | variant | Reported when the owning operation encounters wav. | `src/recording/writer.rs:78` |
| sym-08fb6e771620eedfa601 | `pocketstation::recording::writer::RecorderError::WorkerPanicked` | variant | Reported when the owning operation encounters worker panicked. | `src/recording/writer.rs:74` |
| sym-14c652d6945ce064f300 | `pocketstation::recording::writer::RecordingState::Complete` | variant | Identifies the complete state or stage represented by `RecordingState`. | `src/recording/writer.rs:87` |
| sym-d8adec0971c94b5450bf | `pocketstation::recording::writer::RecordingState::Incomplete` | variant | Identifies the incomplete state or stage represented by `RecordingState`. | `src/recording/writer.rs:88` |
| sym-e1aa3176ee3237d9baab | `pocketstation::recording::writer::RecordingState::Recording` | variant | Identifies the recording state or stage represented by `RecordingState`. | `src/recording/writer.rs:86` |

## Interpretation

The **Recording API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Glossary](/docs/glossary.md)
- [Multistem recording](/docs/concepts/multistem-recording.md)
- [PocketStation](/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md)
- [Record independent stems](/docs/how-to/record-stems.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)

## Evidence boundary

The claims on **Recording API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/recording/mod.rs:1-25` (`DIRECT`)

For **Recording API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
