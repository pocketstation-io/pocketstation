# Recording API

<!-- claims: CLM-REF-008-SCOPE-001,CLM-REF-008-TEXT-001,CLM-REF-008-TEXT-002,CLM-REF-008-SOURCE-001 -->

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

The scope of **Recording API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Recording API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-954bd5d43215025cbd67 | `pocketstation::recording::endpoint::MULTISTEM_GROUP_CONFIGURATION_KEY` | constant | Defines multistem group configuration key as `"recording_group_id"` for the owning public contract. | `src/recording/endpoint.rs:24` |
| sym-2a6d00c225bdfa9776c5 | `pocketstation::recording::endpoint::MULTISTEM_NAME_CONFIGURATION_KEY` | constant | Defines multistem name configuration key as `"stem_name"` for the owning public contract. | `src/recording/endpoint.rs:25` |
| sym-145f00201ff09d31790f | `pocketstation::recording::config::PermissionDecision` | enum | Records whether recording permission was granted, denied, or not observable. | `src/recording/config.rs:43` |
| sym-b8eeef703f71e3e42e6a | `pocketstation::recording::config::PermissionScope` | enum | Selects the permission scope used by PocketStation. | `src/recording/config.rs:50` |
| sym-701a0fa1d075d83a1e69 | `pocketstation::recording::config::RecorderLineageField` | enum | Identifies the lineage field that differs while validating a recording stem. | `src/recording/config.rs:10` |
| sym-57e0714519b0b3c6db3a | `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| sym-03b3fb37a492e410c60b | `pocketstation::recording::writer::DiscontinuityKind` | enum | Selects the discontinuity kind used by PocketStation. | `src/recording/writer.rs:105` |
| sym-a42d8491068ec2279223 | `pocketstation::recording::writer::RecorderError` | enum | Classifies failures surfaced by recorder operations. | `src/recording/writer.rs:24` |
| sym-8d0381d85b60078d9ed3 | `pocketstation::recording::writer::RecordingState` | enum | Selects the recording state used by PocketStation. | `src/recording/writer.rs:86` |
| sym-30397adab4cc0ba2e559 | `as_str` | function | Returns the stable string representation of `StemLabel`. | `src/recording/config.rs:36` |
| sym-02f17b61160530b6c94b | `as_str` | function | Returns the stable string representation of `RecordingErrorCode`. | `src/recording/error_code.rs:32` |
| sym-9ab28f94b738bac6ae31 | `code` | function | Returns the stable error or status code represented by `RecorderError`. | `src/recording/error_code.rs:59` |
| sym-bf00675d3ce79f830ecb | `drop` | function | Releases resources owned by `MultistemRecording`. | `src/recording/writer.rs:361` |
| sym-dd38ad71820965668aba | `finish` | function | Finishes work owned by `MultistemRecording`. | `src/recording/writer.rs:283` |
| sym-f98868400346c425f7be | `group_id` | function | Returns the group identifier held by `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:70` |
| sym-f7519d2b4cb7be1e15b3 | `new` | function | Creates a new `StemLabel`. | `src/recording/config.rs:23` |
| sym-1aca5dad2dcbd16384aa | `new` | function | Creates a new `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:56` |
| sym-697f8ecef5dd23861a71 | `observations` | function | Returns the observations exposed by `MultistemRecording`. | `src/recording/writer.rs:255` |
| sym-38498759647f8a8f4471 | `output_root` | function | Returns the output root associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:65` |
| sym-31cc3e49969412418e61 | `pocketstation::recording::error_code::recording_outcome_error_code` | function | Returns the recording outcome error code held by `error_code`. | `src/recording/error_code.rs:82` |
| sym-ccf25713d1363e17c791 | `preparation_group` | function | Returns the preparation group associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:82` |
| sym-bb5d5fe39646ae5bb11c | `prepare` | function | Prepares resources required by `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:99` |
| sym-9225c439dda86e4640fc | `receipt` | function | Returns the receipt associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:74` |
| sym-7ff05de91f06e67e578e | `request_stop` | function | Requests a graceful stop from `MultistemRecording`. | `src/recording/writer.rs:277` |
| sym-4b718d65571b93843407 | `result` | function | Returns the result represented by `MultistemRecordingReceipt`. | `src/recording/endpoint.rs:33` |
| sym-de855f294ed777beba6f | `pocketstation::recording::config::RecorderStemConfig` | struct | Configures recorder stem behavior at its owning API boundary. | `src/recording/config.rs:55` |
| sym-24870ae1cdb40ba62366 | `pocketstation::recording::config::StemLabel` | struct | Stores the validated human-readable label used for one recording stem. | `src/recording/config.rs:20` |
| sym-38051cba9e220e6e06d4 | `pocketstation::recording::endpoint::MultistemRecordingReceipt` | struct | Retains the identity and observation access returned for multistem recording. | `src/recording/endpoint.rs:28` |
| sym-3542d4de8fc719bc2c06 | `pocketstation::recording::endpoint::SessionMultistemEndpointCoordinator` | struct | Canonical Session-owned multistem recorder declaration. | `src/recording/endpoint.rs:49` |
| sym-688465041cd82d29a469 | `pocketstation::recording::writer::DiscontinuityRecord` | struct | Records one immutable discontinuity observation. | `src/recording/writer.rs:93` |
| sym-93addc782c5e8e6b3271 | `pocketstation::recording::writer::MultistemRecording` | struct | Owns the per-stem recording workers and coordinates their terminal finalization outcome. | `src/recording/writer.rs:139` |
| sym-a780bdc9245464b4deb9 | `pocketstation::recording::writer::RecordingObservations` | struct | Reports the recording observations collected at an observation boundary. | `src/recording/writer.rs:131` |
| sym-d36af3820ac5cb9b5294 | `pocketstation::recording::writer::RecordingOutcome` | struct | Reports the structured recording outcome. | `src/recording/writer.rs:112` |
| sym-0b38412f23568b36ab45 | `pocketstation::recording::writer::RecordingStemOutcome` | struct | Reports the structured recording stem outcome. | `src/recording/writer.rs:121` |
| sym-9e941a881632af08bf23 | `DiscontinuityRecord::kind` | struct_field | Records the kind selected for `DiscontinuityRecord`. | `src/recording/writer.rs:96` |
| sym-03ec7a71680c75a71368 | `DiscontinuityRecord::label` | struct_field | Stores the human-readable label used to identify `DiscontinuityRecord`. | `src/recording/writer.rs:95` |
| sym-ac6bd0adc25798673aab | `DiscontinuityRecord::sequence_end` | struct_field | Records the last sequence number covered by `DiscontinuityRecord`. | `src/recording/writer.rs:100` |
| sym-2f28406e52717c3a205d | `DiscontinuityRecord::sequence_start` | struct_field | Records the first sequence number covered by `DiscontinuityRecord`. | `src/recording/writer.rs:99` |
| sym-adc81069ce45696c3093 | `DiscontinuityRecord::stem_id` | struct_field | Identifies the stem identifier recorded by `DiscontinuityRecord`. | `src/recording/writer.rs:94` |
| sym-e5fd2de238aea90ac3fd | `DiscontinuityRecord::timestamp_end_ns` | struct_field | Stores the timestamp end value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:98` |
| sym-eb813f0373241f2bfa70 | `DiscontinuityRecord::timestamp_start_ns` | struct_field | Stores the timestamp start value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:97` |
| sym-fbdd84800ad4ca934c0c | `RecorderError::FrameSpecMismatch::actual_channels` | struct_field | Contains the actual channels owned or reported by `FrameSpecMismatch`. | `src/recording/writer.rs:62` |
| sym-c16279fac7c130087a6e | `RecorderError::FrameSpecMismatch::actual_rate_hz` | struct_field | Stores the actual rate value for `FrameSpecMismatch`, in hertz. | `src/recording/writer.rs:61` |
| sym-24a41ce2d0d004086cda | `RecorderError::FrameSpecMismatch::expected_channels` | struct_field | Contains the expected channels owned or reported by `FrameSpecMismatch`. | `src/recording/writer.rs:64` |
| sym-fb560a6c63bda0736640 | `RecorderError::FrameSpecMismatch::expected_rate_hz` | struct_field | Stores the expected rate value for `FrameSpecMismatch`, in hertz. | `src/recording/writer.rs:63` |
| sym-d37cbb3ea82b89c7a330 | `RecorderError::FrameSpecMismatch::label` | struct_field | Stores the human-readable label used to identify `FrameSpecMismatch`. | `src/recording/writer.rs:60` |
| sym-d26b3ce2fc6f17d13c54 | `RecorderError::GapTooLarge::duration_ns` | struct_field | Stores the duration value for `GapTooLarge`, in nanoseconds. | `src/recording/writer.rs:71` |
| sym-905e952e31087239c1b8 | `RecorderError::GapTooLarge::label` | struct_field | Stores the human-readable label used to identify `GapTooLarge`. | `src/recording/writer.rs:71` |
| sym-242f9b584650610f7625 | `RecorderError::InvalidSampleSpec::channels` | struct_field | Contains the channels owned or reported by `InvalidSampleSpec`. | `src/recording/writer.rs:43` |
| sym-966feac0aca6a44f4a90 | `RecorderError::InvalidSampleSpec::label` | struct_field | Stores the human-readable label used to identify `InvalidSampleSpec`. | `src/recording/writer.rs:41` |
| sym-9c7f05d988547182c5ea | `RecorderError::InvalidSampleSpec::sample_rate_hz` | struct_field | Stores the sample rate value for `InvalidSampleSpec`, in hertz. | `src/recording/writer.rs:42` |
| sym-7aa58db63062f1afbac9 | `RecorderError::LineageMismatch::actual` | struct_field | Records the value observed by `LineageMismatch`. | `src/recording/writer.rs:55` |
| sym-ef2eb2cc10fd85fcb60b | `RecorderError::LineageMismatch::expected` | struct_field | Records the value expected by `LineageMismatch`. | `src/recording/writer.rs:56` |
| sym-b566e4bd36366e72f99d | `RecorderError::LineageMismatch::field` | struct_field | Stores the field as a `RecorderLineageField` value in `LineageMismatch`. | `src/recording/writer.rs:54` |
| sym-9a002a0640cd66f9ecf9 | `RecorderError::LineageMismatch::label` | struct_field | Stores the human-readable label used to identify `LineageMismatch`. | `src/recording/writer.rs:53` |
| sym-2138f7f644c4d524bec5 | `RecorderError::SessionMismatch::actual` | struct_field | Records the value observed by `SessionMismatch`. | `src/recording/writer.rs:34` |
| sym-1d3fb51a9707e3f036b0 | `RecorderError::SessionMismatch::expected` | struct_field | Records the value expected by `SessionMismatch`. | `src/recording/writer.rs:35` |
| sym-9dd80dcbf6d6283756aa | `RecorderError::SessionMismatch::label` | struct_field | Stores the human-readable label used to identify `SessionMismatch`. | `src/recording/writer.rs:33` |
| sym-318026910b3b8a9c1215 | `RecorderError::SourceMismatch::actual` | struct_field | Records the value observed by `SourceMismatch`. | `src/recording/writer.rs:48` |
| sym-dc91ced665ac8ef2e2a7 | `RecorderError::SourceMismatch::expected` | struct_field | Records the value expected by `SourceMismatch`. | `src/recording/writer.rs:49` |
| sym-9c7ebccea34099688080 | `RecorderError::SourceMismatch::label` | struct_field | Stores the human-readable label used to identify `SourceMismatch`. | `src/recording/writer.rs:47` |
| sym-22d19ddffe19d25a86ed | `RecorderStemConfig::channels` | struct_field | Contains the channels owned or reported by `RecorderStemConfig`. | `src/recording/config.rs:66` |
| sym-f38130c2fad8fd98a9ff | `RecorderStemConfig::clock_id` | struct_field | Identifies the clock identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:59` |
| sym-8760034a6327e4361a3e | `RecorderStemConfig::label` | struct_field | Stores the human-readable label used to identify `RecorderStemConfig`. | `src/recording/config.rs:64` |
| sym-b4b869b508c00922bfa1 | `RecorderStemConfig::permission` | struct_field | Stores the permission as a `PermissionDecision` value in `RecorderStemConfig`. | `src/recording/config.rs:63` |
| sym-a407e04d6cef589b4df2 | `RecorderStemConfig::permission_epoch` | struct_field | Identifies the permission-observation generation attached to `RecorderStemConfig`. | `src/recording/config.rs:61` |
| sym-9c5d49f737f979fed8c0 | `RecorderStemConfig::permission_scope` | struct_field | Stores the permission scope as a `PermissionScope` value in `RecorderStemConfig`. | `src/recording/config.rs:62` |
| sym-9e4c78bd164b555cfe3b | `RecorderStemConfig::sample_rate_hz` | struct_field | Stores the sample rate value for `RecorderStemConfig`, in hertz. | `src/recording/config.rs:65` |
| sym-dc1ff0dc0cf84d71bf8a | `RecorderStemConfig::session_id` | struct_field | Identifies the session identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:56` |
| sym-89360da39a6ddf56eb2c | `RecorderStemConfig::source_generation` | struct_field | References the source generation participating in `RecorderStemConfig`. | `src/recording/config.rs:60` |
| sym-37c313abbccb5717ac05 | `RecorderStemConfig::source_id` | struct_field | Identifies the source identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:57` |
| sym-f1110ae1a4b69575d2b1 | `RecorderStemConfig::stem_id` | struct_field | Identifies the stem identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:58` |
| sym-b5fecfbb0e3e03b133f4 | `RecorderStemConfig::timeline_mapping` | struct_field | Maps source timestamps into the Session timeline for `RecorderStemConfig`. | `src/recording/config.rs:67` |
| sym-94733d80b8d4c4ee69cb | `RecordingObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `RecordingObservations`. | `src/recording/writer.rs:135` |
| sym-13d19ab6eab7887174b5 | `RecordingObservations::failures_total` | struct_field | Counts the total number of failures observed by `RecordingObservations`. | `src/recording/writer.rs:136` |
| sym-4d09259616c8e924d325 | `RecordingObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `RecordingObservations`. | `src/recording/writer.rs:132` |
| sym-97cbb27eb143381a12cb | `RecordingObservations::frames_rejected_total` | struct_field | Counts the total number of frames rejected observed by `RecordingObservations`. | `src/recording/writer.rs:134` |
| sym-323c023545b92c242e9b | `RecordingObservations::frames_written_total` | struct_field | Counts the total number of frames written observed by `RecordingObservations`. | `src/recording/writer.rs:133` |
| sym-3e25b88341cc7c8a704c | `RecordingOutcome::completed_stems` | struct_field | Contains the completed stems owned or reported by `RecordingOutcome`. | `src/recording/writer.rs:115` |
| sym-42d3fade12a4b11815aa | `RecordingOutcome::failed_stems` | struct_field | Contains the failed stems owned or reported by `RecordingOutcome`. | `src/recording/writer.rs:116` |
| sym-e0e6d9a62d466f6760b1 | `RecordingOutcome::session_dir` | struct_field | Points to the directory containing the Session recording represented by `RecordingOutcome`. | `src/recording/writer.rs:113` |
| sym-506ed4409ed08fb26990 | `RecordingOutcome::state` | struct_field | Records the state selected for `RecordingOutcome`. | `src/recording/writer.rs:114` |
| sym-67c3734e45702552c46d | `RecordingOutcome::stems` | struct_field | Contains the stems owned or reported by `RecordingOutcome`. | `src/recording/writer.rs:117` |
| sym-e13b68e70fe9c0649e88 | `RecordingStemOutcome::edge_observations` | struct_field | References the edge observations participating in `RecordingStemOutcome`. | `src/recording/writer.rs:127` |
| sym-72a89b0a596b23cb5c48 | `RecordingStemOutcome::error` | struct_field | Stores the error component of `RecordingStemOutcome`. | `src/recording/writer.rs:126` |
| sym-8fa3dd94d66a46911497 | `RecordingStemOutcome::gap_ranges` | struct_field | Contains the gap ranges owned or reported by `RecordingStemOutcome`. | `src/recording/writer.rs:125` |
| sym-4f2de8c2b3babdf85240 | `RecordingStemOutcome::label` | struct_field | Stores the human-readable label used to identify `RecordingStemOutcome`. | `src/recording/writer.rs:122` |
| sym-ca4b86eadd092aeb2c5b | `RecordingStemOutcome::stale_frames` | struct_field | Contains the stale frames owned or reported by `RecordingStemOutcome`. | `src/recording/writer.rs:124` |
| sym-60f07bcfb21e28c95634 | `RecordingStemOutcome::written_frames` | struct_field | Contains the written frames owned or reported by `RecordingStemOutcome`. | `src/recording/writer.rs:123` |
| sym-4139c58288925a6fd466 | `pocketstation::recording::config::PermissionDecision::Allowed` | variant | Represents the allowed alternative defined by `PermissionDecision`. | `src/recording/config.rs:44` |
| sym-2c42f67f70b486c879b7 | `pocketstation::recording::config::PermissionDecision::Denied` | variant | Represents the denied alternative defined by `PermissionDecision`. | `src/recording/config.rs:45` |
| sym-72dfabd62faca652bb06 | `pocketstation::recording::config::PermissionScope::SessionCaptureGrant` | variant | Limits the permission decision to session capture grant. | `src/recording/config.rs:51` |
| sym-d96127c11864649ebe7d | `pocketstation::recording::config::RecorderLineageField::Clock` | variant | Represents the clock alternative defined by `RecorderLineageField`. | `src/recording/config.rs:14` |
| sym-0367231a7b610784733d | `pocketstation::recording::config::RecorderLineageField::PermissionEpoch` | variant | Represents the permission epoch alternative defined by `RecorderLineageField`. | `src/recording/config.rs:16` |
| sym-650f042a3c53ea89a277 | `pocketstation::recording::config::RecorderLineageField::Session` | variant | Represents the session alternative defined by `RecorderLineageField`. | `src/recording/config.rs:11` |
| sym-a54152169703a18001ca | `pocketstation::recording::config::RecorderLineageField::Source` | variant | Represents the source alternative defined by `RecorderLineageField`. | `src/recording/config.rs:12` |
| sym-ee3be57514e2866b3493 | `pocketstation::recording::config::RecorderLineageField::SourceGeneration` | variant | Represents the source generation alternative defined by `RecorderLineageField`. | `src/recording/config.rs:15` |
| sym-2e5a5cbf66a695eace88 | `pocketstation::recording::config::RecorderLineageField::Stem` | variant | Represents the stem alternative defined by `RecorderLineageField`. | `src/recording/config.rs:13` |
| sym-de698bba8189939e1086 | `pocketstation::recording::error_code::RecordingErrorCode::DuplicateStemLabel` | variant | Reports that stem label duplicates an existing declaration or record. | `src/recording/error_code.rs:12` |
| sym-12b138a7064e71a0cda7 | `pocketstation::recording::error_code::RecordingErrorCode::FrameSpecMismatch` | variant | Reports that frame spec does not match the expected contract. | `src/recording/error_code.rs:18` |
| sym-e528bba8aa257bdd951c | `pocketstation::recording::error_code::RecordingErrorCode::GapTooLarge` | variant | Reports that gap exceeds the supported size limit. | `src/recording/error_code.rs:21` |
| sym-2b31271acad6cbf07679 | `pocketstation::recording::error_code::RecordingErrorCode::Incomplete` | variant | Reports that the operation ended without producing a complete terminal result. | `src/recording/error_code.rs:28` |
| sym-995c89d1d07551e2fd7a | `pocketstation::recording::error_code::RecordingErrorCode::InvalidSampleSpec` | variant | Reports that the supplied sample spec is invalid. | `src/recording/error_code.rs:15` |
| sym-94d294658d46c4f1d601 | `pocketstation::recording::error_code::RecordingErrorCode::InvalidStemLabel` | variant | Reports that the supplied stem label is invalid. | `src/recording/error_code.rs:11` |
| sym-023581c92d22cc90e089 | `pocketstation::recording::error_code::RecordingErrorCode::IoFailed` | variant | Reports that I/O failed. | `src/recording/error_code.rs:24` |
| sym-232b41897f6144e2296b | `pocketstation::recording::error_code::RecordingErrorCode::JsonFailed` | variant | Reports that json failed. | `src/recording/error_code.rs:26` |
| sym-bb95f5419b2243cec930 | `pocketstation::recording::error_code::RecordingErrorCode::LineageMismatch` | variant | Reports that lineage does not match the expected contract. | `src/recording/error_code.rs:17` |
| sym-f33ab449eb0d58eeeb14 | `pocketstation::recording::error_code::RecordingErrorCode::NotFinalized` | variant | Reports that no t finalized is available. | `src/recording/error_code.rs:27` |
| sym-e5f184f5829c9a168425 | `pocketstation::recording::error_code::RecordingErrorCode::OutputExists` | variant | Reports that output already exists and would be overwritten. | `src/recording/error_code.rs:10` |
| sym-9886bba784496517f781 | `pocketstation::recording::error_code::RecordingErrorCode::PermissionDenied` | variant | Reports that the required permission was denied. | `src/recording/error_code.rs:14` |
| sym-5f0acc1d064226c4e815 | `pocketstation::recording::error_code::RecordingErrorCode::SessionMismatch` | variant | Reports that session does not match the expected contract. | `src/recording/error_code.rs:13` |
| sym-c1cd6de5268ac4b8d885 | `pocketstation::recording::error_code::RecordingErrorCode::SourceMismatch` | variant | Reports that source does not match the expected contract. | `src/recording/error_code.rs:16` |
| sym-c12cfbbb8a6917e88723 | `pocketstation::recording::error_code::RecordingErrorCode::TimestampOutOfRange` | variant | Reports that timestamp falls outside the supported range. | `src/recording/error_code.rs:20` |
| sym-6388befc975243af8db7 | `pocketstation::recording::error_code::RecordingErrorCode::TooManyGaps` | variant | Reports that the number of gaps exceeds the supported limit. | `src/recording/error_code.rs:22` |
| sym-dff5f8d8a97ab6d32afa | `pocketstation::recording::error_code::RecordingErrorCode::UnalignedSamples` | variant | Reports that samples does not align to complete frames or channels. | `src/recording/error_code.rs:19` |
| sym-ef257966e67e1472fdfb | `pocketstation::recording::error_code::RecordingErrorCode::WavFailed` | variant | Reports that wav failed. | `src/recording/error_code.rs:25` |
| sym-0030708c2ea3eccf3813 | `pocketstation::recording::error_code::RecordingErrorCode::WorkerPanicked` | variant | Reports that worker panicked while the operation was active. | `src/recording/error_code.rs:23` |
| sym-c3ba172d10fca913b9d5 | `pocketstation::recording::writer::DiscontinuityKind::OverlapRejected` | variant | Classifies the observed stream discontinuity as overlap rejected. | `src/recording/writer.rs:108` |
| sym-21f0b61d2c70ae437768 | `pocketstation::recording::writer::DiscontinuityKind::SequenceGap` | variant | Classifies the observed stream discontinuity as sequence gap. | `src/recording/writer.rs:107` |
| sym-7d360341f21b92581d5c | `pocketstation::recording::writer::DiscontinuityKind::TimestampGap` | variant | Classifies the observed stream discontinuity as timestamp gap. | `src/recording/writer.rs:106` |
| sym-09a86bc1b2a3681702bc | `pocketstation::recording::writer::RecorderError::DuplicateStemLabel` | variant | Reports that stem label duplicates an existing declaration or record. | `src/recording/writer.rs:30` |
| sym-8ac9048e6258db768d31 | `pocketstation::recording::writer::RecorderError::FrameSpecMismatch` | variant | Reports that frame spec does not match the expected contract. | `src/recording/writer.rs:59` |
| sym-3bf811546a88f51368d8 | `pocketstation::recording::writer::RecorderError::GapTooLarge` | variant | Reports that gap exceeds the supported size limit. | `src/recording/writer.rs:71` |
| sym-ff6f9b3ee4fee92478db | `pocketstation::recording::writer::RecorderError::InvalidSampleSpec` | variant | Reports that the supplied sample spec is invalid. | `src/recording/writer.rs:40` |
| sym-7ea829746a98e1385e78 | `pocketstation::recording::writer::RecorderError::InvalidStemLabel` | variant | Reports that the supplied stem label is invalid. | `src/recording/writer.rs:28` |
| sym-8764eef568a0a7905493 | `pocketstation::recording::writer::RecorderError::Io` | variant | Reports an operating-system or filesystem I/O failure. | `src/recording/writer.rs:77` |
| sym-874aaad200ad7e3b0a87 | `pocketstation::recording::writer::RecorderError::Json` | variant | Reports that JSON serialization or parsing failed. | `src/recording/writer.rs:81` |
| sym-d67027b1a399227ee00e | `pocketstation::recording::writer::RecorderError::LineageMismatch` | variant | Reports that lineage does not match the expected contract. | `src/recording/writer.rs:52` |
| sym-0fa716dacfbf164f7d8d | `pocketstation::recording::writer::RecorderError::OutputExists` | variant | Reports that output already exists and would be overwritten. | `src/recording/writer.rs:26` |
| sym-1fedc505f7b4b3a39caf | `pocketstation::recording::writer::RecorderError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/recording/writer.rs:38` |
| sym-59a79c0477a191febf84 | `pocketstation::recording::writer::RecorderError::SessionMismatch` | variant | Reports that session does not match the expected contract. | `src/recording/writer.rs:32` |
| sym-f0ab33824f7b789d3541 | `pocketstation::recording::writer::RecorderError::SourceMismatch` | variant | Reports that source does not match the expected contract. | `src/recording/writer.rs:46` |
| sym-0df6acf2d649c40b87cf | `pocketstation::recording::writer::RecorderError::TimestampOutOfRange` | variant | Reports that timestamp falls outside the supported range. | `src/recording/writer.rs:69` |
| sym-c1e195e527879d463869 | `pocketstation::recording::writer::RecorderError::TooManyGaps` | variant | Reports that the number of gaps exceeds the supported limit. | `src/recording/writer.rs:73` |
| sym-9b49168aab7a0155f7bf | `pocketstation::recording::writer::RecorderError::UnalignedSamples` | variant | Reports that samples does not align to complete frames or channels. | `src/recording/writer.rs:67` |
| sym-d45cc49d5cb4f2b6859b | `pocketstation::recording::writer::RecorderError::Wav` | variant | Classifies a failure at the wav stage or component of `RecorderError`. | `src/recording/writer.rs:79` |
| sym-21cbed79c30e8122d7b2 | `pocketstation::recording::writer::RecorderError::WorkerPanicked` | variant | Reports that worker panicked while the operation was active. | `src/recording/writer.rs:75` |
| sym-5b5bbe3d94b04ab61c8c | `pocketstation::recording::writer::RecordingState::Complete` | variant | Identifies the complete state or stage represented by `RecordingState`. | `src/recording/writer.rs:88` |
| sym-759e4b539d6888eb49cf | `pocketstation::recording::writer::RecordingState::Incomplete` | variant | Identifies the incomplete state or stage represented by `RecordingState`. | `src/recording/writer.rs:89` |
| sym-d8c7ab99450027b2acd0 | `pocketstation::recording::writer::RecordingState::Recording` | variant | Identifies the recording state or stage represented by `RecordingState`. | `src/recording/writer.rs:87` |

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

The claims on **Recording API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/recording/mod.rs:1-4` (`DECLARED`)

For **Recording API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
