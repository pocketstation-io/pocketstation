# Session failures

<!-- claims: CLM-ERR-001-CAP-001,CLM-ERR-001-CAP-002,CLM-ERR-001-CAP-003,CLM-ERR-001-CAP-004,CLM-ERR-001-SOURCE-001,CLM-ERR-001-ERROR-0001,CLM-ERR-001-ERROR-0002,CLM-ERR-001-ERROR-0003,CLM-ERR-001-ERROR-0004,CLM-ERR-001-ERROR-0005,CLM-ERR-001-ERROR-0006,CLM-ERR-001-ERROR-0007,CLM-ERR-001-ERROR-0008,CLM-ERR-001-ERROR-0009,CLM-ERR-001-ERROR-0010,CLM-ERR-001-ERROR-0011,CLM-ERR-001-ERROR-0012,CLM-ERR-001-ERROR-0013,CLM-ERR-001-ERROR-0014,CLM-ERR-001-ERROR-0015,CLM-ERR-001-ERROR-0016,CLM-ERR-001-ERROR-0017,CLM-ERR-001-ERROR-0018,CLM-ERR-001-ERROR-0019,CLM-ERR-001-ERROR-0020,CLM-ERR-001-ERROR-0021,CLM-ERR-001-ERROR-0022,CLM-ERR-001-ERROR-0023,CLM-ERR-001-ERROR-0024,CLM-ERR-001-ERROR-0025,CLM-ERR-001-ERROR-0026,CLM-ERR-001-ERROR-0027,CLM-ERR-001-ERROR-0028,CLM-ERR-001-ERROR-0029,CLM-ERR-001-ERROR-0030,CLM-ERR-001-ERROR-0031,CLM-ERR-001-ERROR-0032,CLM-ERR-001-ERROR-0033,CLM-ERR-001-ERROR-0034,CLM-ERR-001-ERROR-0035,CLM-ERR-001-ERROR-0036,CLM-ERR-001-ERROR-0037,CLM-ERR-001-ERROR-0038,CLM-ERR-001-ERROR-0039,CLM-ERR-001-ERROR-0040,CLM-ERR-001-ERROR-0041,CLM-ERR-001-ERROR-0042,CLM-ERR-001-ERROR-0043,CLM-ERR-001-ERROR-0044,CLM-ERR-001-ERROR-0045,CLM-ERR-001-ERROR-0046,CLM-ERR-001-ERROR-0047,CLM-ERR-001-ERROR-0048,CLM-ERR-001-ERROR-0049,CLM-ERR-001-ERROR-0050,CLM-ERR-001-ERROR-0051,CLM-ERR-001-ERROR-0052,CLM-ERR-001-ERROR-0053,CLM-ERR-001-ERROR-0054,CLM-ERR-001-ERROR-0055,CLM-ERR-001-ERROR-0056,CLM-ERR-001-ERROR-0057,CLM-ERR-001-ERROR-0058,CLM-ERR-001-ERROR-0059,CLM-ERR-001-ERROR-0060,CLM-ERR-001-ERROR-0061,CLM-ERR-001-ERROR-0062,CLM-ERR-001-ERROR-0063,CLM-ERR-001-ERROR-0064,CLM-ERR-001-ERROR-0065,CLM-ERR-001-ERROR-0066,CLM-ERR-001-ERROR-0067,CLM-ERR-001-ERROR-0068,CLM-ERR-001-ERROR-0069,CLM-ERR-001-ERROR-0070,CLM-ERR-001-ERROR-0071,CLM-ERR-001-ERROR-0072,CLM-ERR-001-ERROR-0073,CLM-ERR-001-ERROR-0074,CLM-ERR-001-ERROR-0075,CLM-ERR-001-ERROR-0076,CLM-ERR-001-ERROR-0077,CLM-ERR-001-ERROR-0078,CLM-ERR-001-ERROR-0079,CLM-ERR-001-ERROR-0080,CLM-ERR-001-ERROR-0081,CLM-ERR-001-ERROR-0082,CLM-ERR-001-ERROR-0083,CLM-ERR-001-ERROR-0084,CLM-ERR-001-ERROR-0085,CLM-ERR-001-ERROR-0086,CLM-ERR-001-ERROR-0087,CLM-ERR-001-ERROR-0088,CLM-ERR-001-ERROR-0089,CLM-ERR-001-ERROR-0090,CLM-ERR-001-ERROR-0091,CLM-ERR-001-ERROR-0092,CLM-ERR-001-ERROR-0093,CLM-ERR-001-ERROR-0094,CLM-ERR-001-ERROR-0095,CLM-ERR-001-ERROR-0096,CLM-ERR-001-ERROR-0097,CLM-ERR-001-ERROR-0098,CLM-ERR-001-ERROR-0099,CLM-ERR-001-ERROR-0100,CLM-ERR-001-ERROR-0101,CLM-ERR-001-ERROR-0102,CLM-ERR-001-ERROR-0103,CLM-ERR-001-ERROR-0104,CLM-ERR-001-ERROR-0105,CLM-ERR-001-ERROR-0106,CLM-ERR-001-ERROR-0107,CLM-ERR-001-ERROR-0108,CLM-ERR-001-ERROR-0109,CLM-ERR-001-ERROR-0110,CLM-ERR-001-ERROR-0111,CLM-ERR-001-ERROR-0112,CLM-ERR-001-ERROR-0113,CLM-ERR-001-ERROR-0114,CLM-ERR-001-ERROR-0115,CLM-ERR-001-ERROR-0116,CLM-ERR-001-ERROR-0117,CLM-ERR-001-ERROR-0118,CLM-ERR-001-ERROR-0119,CLM-ERR-001-ERROR-0120,CLM-ERR-001-ERROR-0121,CLM-ERR-001-ERROR-0122,CLM-ERR-001-ERROR-0123,CLM-ERR-001-ERROR-0124,CLM-ERR-001-ERROR-0125,CLM-ERR-001-ERROR-0126,CLM-ERR-001-ERROR-0127,CLM-ERR-001-ERROR-0128,CLM-ERR-001-ERROR-0129,CLM-ERR-001-ERROR-0130,CLM-ERR-001-ERROR-0131,CLM-ERR-001-ERROR-0132,CLM-ERR-001-ERROR-0133,CLM-ERR-001-ERROR-0134,CLM-ERR-001-ERROR-0135,CLM-ERR-001-ERROR-0136,CLM-ERR-001-ERROR-0137,CLM-ERR-001-ERROR-0138,CLM-ERR-001-ERROR-0139,CLM-ERR-001-ERROR-0140,CLM-ERR-001-ERROR-0141,CLM-ERR-001-ERROR-0142,CLM-ERR-001-ERROR-0143,CLM-ERR-001-ERROR-0144,CLM-ERR-001-ERROR-0145,CLM-ERR-001-ERROR-0146,CLM-ERR-001-ERROR-0147,CLM-ERR-001-ERROR-0148,CLM-ERR-001-ERROR-0149,CLM-ERR-001-ERROR-0150,CLM-ERR-001-ERROR-0151,CLM-ERR-001-ERROR-0152,CLM-ERR-001-ERROR-0153,CLM-ERR-001-ERROR-0154,CLM-ERR-001-ERROR-0155,CLM-ERR-001-ERROR-0156,CLM-ERR-001-ERROR-0157,CLM-ERR-001-ERROR-0158,CLM-ERR-001-ERROR-0159,CLM-ERR-001-ERROR-0160,CLM-ERR-001-ERROR-0161,CLM-ERR-001-ERROR-0162,CLM-ERR-001-ERROR-0163,CLM-ERR-001-ERROR-0164,CLM-ERR-001-ERROR-0165,CLM-ERR-001-ERROR-0166,CLM-ERR-001-ERROR-0167,CLM-ERR-001-ERROR-0168,CLM-ERR-001-ERROR-0169,CLM-ERR-001-ERROR-0170,CLM-ERR-001-ERROR-0171,CLM-ERR-001-ERROR-0172,CLM-ERR-001-ERROR-0173,CLM-ERR-001-ERROR-0174,CLM-ERR-001-ERROR-0175,CLM-ERR-001-ERROR-0176,CLM-ERR-001-ERROR-0177,CLM-ERR-001-ERROR-0178,CLM-ERR-001-ERROR-0179,CLM-ERR-001-ERROR-0180,CLM-ERR-001-ERROR-0181,CLM-ERR-001-ERROR-0182,CLM-ERR-001-ERROR-0183,CLM-ERR-001-ERROR-0184,CLM-ERR-001-ERROR-0185,CLM-ERR-001-ERROR-0186,CLM-ERR-001-ERROR-0187,CLM-ERR-001-ERROR-0188,CLM-ERR-001-ERROR-0189,CLM-ERR-001-ERROR-0190,CLM-ERR-001-ERROR-0191,CLM-ERR-001-ERROR-0192,CLM-ERR-001-ERROR-0193,CLM-ERR-001-ERROR-0194,CLM-ERR-001-ERROR-0195,CLM-ERR-001-ERROR-0196,CLM-ERR-001-ERROR-0197,CLM-ERR-001-ERROR-0198,CLM-ERR-001-ERROR-0199,CLM-ERR-001-ERROR-0200,CLM-ERR-001-ERROR-0201,CLM-ERR-001-ERROR-0202,CLM-ERR-001-ERROR-0203,CLM-ERR-001-ERROR-0204,CLM-ERR-001-ERROR-0205,CLM-ERR-001-ERROR-0206,CLM-ERR-001-ERROR-0207,CLM-ERR-001-ERROR-0208,CLM-ERR-001-ERROR-0209,CLM-ERR-001-ERROR-0210,CLM-ERR-001-ERROR-0211,CLM-ERR-001-ERROR-0212,CLM-ERR-001-ERROR-0213,CLM-ERR-001-ERROR-0214,CLM-ERR-001-ERROR-0215,CLM-ERR-001-ERROR-0216,CLM-ERR-001-ERROR-0217,CLM-ERR-001-ERROR-0218,CLM-ERR-001-ERROR-0219,CLM-ERR-001-ERROR-0220,CLM-ERR-001-ERROR-0221,CLM-ERR-001-ERROR-0222,CLM-ERR-001-ERROR-0223,CLM-ERR-001-ERROR-0224,CLM-ERR-001-ERROR-0225,CLM-ERR-001-ERROR-0226,CLM-ERR-001-ERROR-0227,CLM-ERR-001-ERROR-0228,CLM-ERR-001-ERROR-0229,CLM-ERR-001-ERROR-0230,CLM-ERR-001-ERROR-0231,CLM-ERR-001-ERROR-0232,CLM-ERR-001-ERROR-0233,CLM-ERR-001-ERROR-0234,CLM-ERR-001-ERROR-0235,CLM-ERR-001-ERROR-0236,CLM-ERR-001-ERROR-0237,CLM-ERR-001-ERROR-0238,CLM-ERR-001-ERROR-0239,CLM-ERR-001-ERROR-0240,CLM-ERR-001-ERROR-0241,CLM-ERR-001-ERROR-0242,CLM-ERR-001-ERROR-0243,CLM-ERR-001-ERROR-0244,CLM-ERR-001-ERROR-0245,CLM-ERR-001-ERROR-0246,CLM-ERR-001-ERROR-0247,CLM-ERR-001-ERROR-0248,CLM-ERR-001-ERROR-0249,CLM-ERR-001-ERROR-0250,CLM-ERR-001-ERROR-0251,CLM-ERR-001-ERROR-0252,CLM-ERR-001-ERROR-0253,CLM-ERR-001-ERROR-0254,CLM-ERR-001-ERROR-0255,CLM-ERR-001-ERROR-0256,CLM-ERR-001-ERROR-0257,CLM-ERR-001-ERROR-0258,CLM-ERR-001-ERROR-0259,CLM-ERR-001-ERROR-0260,CLM-ERR-001-ERROR-0261,CLM-ERR-001-ERROR-0262,CLM-ERR-001-ERROR-0263,CLM-ERR-001-ERROR-0264,CLM-ERR-001-ERROR-0265 -->

## Scope

- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Error inventory

| Evidence record | Type | Variant | Retryable | Recoverable | Defined |
|---|---|---|---|---|---|
| error-00e5716261eba0f8cf3d | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `OutputSignalMismatch` | unknown | unknown | `src/session/declaration/typed_stream.rs:207` |
| error-00f6e798d158df66c847 | `pocketstation::session::error::SessionError` | `UnknownStem` | unknown | unknown | `src/session/error.rs:46` |
| error-01d3fc855e2a00319076 | `pocketstation::session::error_code::SessionStartErrorCode` | `StartCancelled` | unknown | unknown | `src/session/error_code.rs:64` |
| error-023d6ab0b23a50a614ff | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `OperatorPrepare` | unknown | unknown | `src/session/lifecycle/start_contract.rs:144` |
| error-0279b2b6b0cb3b5801bc | `pocketstation::session::error_code::SessionStartErrorCode` | `TraceRecorderSetupFailed` | unknown | unknown | `src/session/error_code.rs:82` |
| error-037ddc3e193da74177f8 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingOperatorSignalInput` | unknown | unknown | `src/session/prepare/error.rs:78` |
| error-05c60389efcb84311921 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `InvalidLayout` | unknown | unknown | `src/session/lifecycle/trace.rs:364` |
| error-085082b521c14e5ecd1e | `pocketstation::session::prepare::error::SessionPrepareError` | type | unknown | unknown | `src/session/prepare/error.rs:9` |
| error-08a7536094bfb2242b17 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | `Closed` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:300` |
| error-09837185c7fca0f70618 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | `EndpointExtensionRegistration` | unknown | unknown | `src/session/lifecycle/host.rs:368` |
| error-0bc2f7c0b9f9dbf8ddd7 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `MissingEndpointDeclaration` | unknown | unknown | `src/session/lifecycle/start_contract.rs:150` |
| error-0bd6f58be40ade9a01fe | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` | `ZeroCapacity` | unknown | unknown | `src/session/lifecycle/trace.rs:90` |
| error-0c04a3eedb823da29323 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `SequenceGap` | unknown | unknown | `src/session/lifecycle/trace.rs:372` |
| error-0cc0ae8a8cc4f1e05996 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingExternalAudioIngress` | unknown | unknown | `src/session/prepare/error.rs:19` |
| error-0ce1015c73b65576cbeb | `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | `DuplicateSidecarId` | unknown | unknown | `src/session/lifecycle/engine.rs:301` |
| error-0d567cf627daa0adfee1 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `TimestampRegression` | unknown | unknown | `src/session/lifecycle/trace.rs:376` |
| error-0e46a3d13215bfc3898f | `pocketstation::session::error_code::SessionStartErrorCode` | `MissingEndpointDeclaration` | unknown | unknown | `src/session/error_code.rs:71` |
| error-108ece57ea443c789d81 | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | type | unknown | unknown | `src/session/extensions/audio_input/mod.rs:77` |
| error-11863b3a293345b0bb2d | `pocketstation::session::extensions::audio_input::source::AudioInputError` | `Manifest` | unknown | unknown | `src/session/extensions/audio_input/source.rs:91` |
| error-1281b697f9f4d62194b1 | `pocketstation::session::compile::error::SessionCompileError` | `UnknownEndpointInputPort` | unknown | unknown | `src/session/compile/error.rs:83` |
| error-12fef698a1fbec823e7e | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingTypedEdgePlan` | unknown | unknown | `src/session/prepare/error.rs:66` |
| error-1310461ef521d30d4686 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingAsyncOperatorFactory` | unknown | unknown | `src/session/prepare/error.rs:68` |
| error-13dd584b4e2e8eaa490c | `pocketstation::session::error_code::SessionStartErrorCode` | `MissingEventReceiver` | unknown | unknown | `src/session/error_code.rs:81` |
| error-16e269f1786471c2db63 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `RecordAfterTerminal` | unknown | unknown | `src/session/lifecycle/trace.rs:384` |
| error-16edb8f15b75c471db64 | `pocketstation::session::error::SessionError` | `UnknownSourceOutput` | unknown | unknown | `src/session/error.rs:54` |
| error-17674f66426c713d90a2 | `pocketstation::session::compile::error::SessionCompileError` | `AmbiguousEndpointInput` | unknown | unknown | `src/session/compile/error.rs:27` |
| error-1955a522796dc25c325d | `pocketstation::session::lifecycle::events::SessionRollbackFailure` | type | unknown | unknown | `src/session/lifecycle/events.rs:165` |
| error-1bd7ae7942029f778071 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `MissingPort` | unknown | unknown | `src/session/declaration/typed_stream.rs:201` |
| error-1c7816652dd061fb1141 | `pocketstation::session::error_code::SessionStartErrorCode` | type | unknown | unknown | `src/session/error_code.rs:61` |
| error-1de3680efda0db59054d | `pocketstation::session::extensions::audio_input::source::AudioInputError` | type | unknown | unknown | `src/session/extensions/audio_input/source.rs:85` |
| error-1e032bb24ceeb3261f64 | `pocketstation::session::extensions::audio_input::source::AudioInputError` | `InstanceIdentityExhausted` | unknown | unknown | `src/session/extensions/audio_input/source.rs:101` |
| error-1e6e9f452ca83bcd4874 | `pocketstation::session::lifecycle::engine::SessionEngineStartError` | `Prepare` | unknown | unknown | `src/session/lifecycle/engine.rs:321` |
| error-209d179a581da98afe01 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `NoRoutes` | unknown | unknown | `src/session/error_code.rs:12` |
| error-211a6d9f0455db430c11 | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | `Driver` | unknown | unknown | `src/session/lifecycle/engine.rs:309` |
| error-216e792e17260369097f | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `OperatorHasNoDestination` | unknown | unknown | `src/session/error_code.rs:27` |
| error-21833c6ece960cb29b1a | `pocketstation::session::prepare::error::SessionPrepareError` | `DuplicateSourceNode` | unknown | unknown | `src/session/prepare/error.rs:17` |
| error-2298e500c2912a26af60 | `pocketstation::session::extensions::source::SourceManifestError` | `UnsupportedExecutionPartition` | unknown | unknown | `src/session/extensions/source.rs:697` |
| error-23114bd8e3f808749e63 | `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | `InvalidConfiguration` | unknown | unknown | `src/session/lifecycle/engine.rs:299` |
| error-231922d7357983437139 | `pocketstation::session::error::SessionError` | type | unknown | unknown | `src/session/error.rs:6` |
| error-23ee10c2333375238483 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | `Cancelled` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:277` |
| error-249698d47095b3e4105e | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingSourceNode` | unknown | unknown | `src/session/prepare/error.rs:15` |
| error-24e3a41f1052bc9e8067 | `pocketstation::session::error_code::SessionStartErrorCode` | `HostSetupFailed` | unknown | unknown | `src/session/error_code.rs:62` |
| error-255f37a95c44fee26de7 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingGeneratedAudioIngress` | unknown | unknown | `src/session/prepare/error.rs:31` |
| error-25c97310a1bd860164c2 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `InvalidChecksum` | unknown | unknown | `src/session/lifecycle/trace.rs:368` |
| error-25fddf9b7d8fbe793b33 | `pocketstation::session::extensions::source::SourceRegistrationError` | `InvalidManifest` | unknown | unknown | `src/session/extensions/source.rs:703` |
| error-26842744fd1145353bbf | `pocketstation::session::extensions::source::SourceManifestError` | `ZeroVersion` | unknown | unknown | `src/session/extensions/source.rs:681` |
| error-26be83c931a21d700eb4 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `UnknownPort` | unknown | unknown | `src/session/declaration/typed_stream.rs:196` |
| error-276d20029a8dc90436ae | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | `Full` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:273` |
| error-289376b9ac4aa3a1d17e | `pocketstation::session::error_code::PolledAudioPollErrorCode` | `LeaseCapacityExhausted` | unknown | unknown | `src/session/error_code.rs:133` |
| error-290a009997dd350cf09b | `pocketstation::session::extensions::audio_input::source::AudioInputError` | `SourceTypeId` | unknown | unknown | `src/session/extensions/audio_input/source.rs:89` |
| error-2945028cfdff13616843 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `UnsupportedSourceTopology` | unknown | unknown | `src/session/lifecycle/start_contract.rs:117` |
| error-29839254992425eaa579 | `pocketstation::session::extensions::source::SourceDriverError` | type | unknown | unknown | `src/session/extensions/source.rs:748` |
| error-299164d5726d38d867d0 | `pocketstation::session::error_code::SessionStopFailureCode` | `SourceSendRejected` | unknown | unknown | `src/session/error_code.rs:178` |
| error-29cdb6e4ee558791e0bd | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `SessionMismatch` | unknown | unknown | `src/session/lifecycle/trace.rs:374` |
| error-29e8a5682912ea4971e4 | `pocketstation::session::extensions::source::SourceManifestError` | `DuplicateOutputName` | unknown | unknown | `src/session/extensions/source.rs:689` |
| error-2b2745ea7a7d2bbb54dd | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `UnknownStem` | unknown | unknown | `src/session/error_code.rs:24` |
| error-2eddf18e1bd6fc4c9f22 | `pocketstation::session::compile::error::SessionCompileError` | `InvalidSpec` | unknown | unknown | `src/session/compile/error.rs:9` |
| error-2f946b289820404e569f | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `InputSignalMismatch` | unknown | unknown | `src/session/declaration/typed_stream.rs:205` |
| error-2fa73573a64a036aa810 | `pocketstation::session::prepare::error::SessionPrepareError` | `InvalidOperatorInputPort` | unknown | unknown | `src/session/prepare/error.rs:49` |
| error-31c21328acfbcc53b842 | `pocketstation::session::error::SessionError` | `UnknownOperatorInstance` | unknown | unknown | `src/session/error.rs:59` |
| error-325a6ea90a98f7e7d0c2 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `InvalidMagic` | unknown | unknown | `src/session/lifecycle/trace.rs:360` |
| error-32f264c57f6993869b57 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `DraftFrozen` | unknown | unknown | `src/session/error_code.rs:19` |
| error-33e12ce8b939aad4cd4e | `pocketstation::session::error_code::SessionStopFailureCode` | `RuntimeWorkerPanicked` | unknown | unknown | `src/session/error_code.rs:172` |
| error-33f6d0dbd8c458cff568 | `pocketstation::session::extensions::source::SourceManifestError` | `EmptySourceTypeId` | unknown | unknown | `src/session/extensions/source.rs:679` |
| error-343fe878313f07b392b0 | `pocketstation::session::error_code::SessionStartErrorCode` | `RuntimePrepareFailed` | unknown | unknown | `src/session/error_code.rs:68` |
| error-371aeb908799f91d1c55 | `pocketstation::session::error_code::SessionStartErrorCode` | `InvalidStartOptions` | unknown | unknown | `src/session/error_code.rs:69` |
| error-375e85b534dc099bec22 | `pocketstation::session::error_code::SessionStartErrorCode` | `DeclarationInvalid` | unknown | unknown | `src/session/error_code.rs:66` |
| error-396be01d86a31314ead0 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `Cancelled` | unknown | unknown | `src/session/lifecycle/start_contract.rs:194` |
| error-39763febbf62db230561 | `pocketstation::session::extensions::source::SourceTypeIdError` | `Empty` | unknown | unknown | `src/session/extensions/source.rs:70` |
| error-39a01345f007dfc03b7b | `pocketstation::session::error_code::SessionStartErrorCode` | `MissingAudioReceipt` | unknown | unknown | `src/session/error_code.rs:79` |
| error-3aeefaa523816923ed8e | `pocketstation::session::error_code::SessionStartErrorCode` | `InvalidSelector` | unknown | unknown | `src/session/error_code.rs:65` |
| error-3b2bd543dd62c7d7bf22 | `pocketstation::session::compile::error::SessionCompileError` | `UnknownExternalSourceOutput` | unknown | unknown | `src/session/compile/error.rs:73` |
| error-3b793d23067a001d2c2d | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingWorkerTarget` | unknown | unknown | `src/session/prepare/error.rs:39` |
| error-3bb59cc626f1567ad2c3 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | `OperatorRegistration` | unknown | unknown | `src/session/lifecycle/host.rs:370` |
| error-3cd9e5a0dc374170dc7a | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `ExternalAudioBridge` | unknown | unknown | `src/session/lifecycle/start_contract.rs:124` |
| error-3ceb72242dbd422cae60 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | type | unknown | unknown | `src/session/lifecycle/trace.rs:356` |
| error-3cff2ccd0e457a9fbd0e | `pocketstation::session::compile::error::SessionCompileError` | `DuplicateOperatorInputConnection` | unknown | unknown | `src/session/compile/error.rs:62` |
| error-3d34cc2931d2f468b483 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `UnsupportedVersion` | unknown | unknown | `src/session/lifecycle/trace.rs:362` |
| error-3d614715948b82f1e9b4 | `pocketstation::session::compile::error::SessionCompileError` | `InvalidExternalSourceConfiguration` | unknown | unknown | `src/session/compile/error.rs:78` |
| error-3d94b7c95345e673d89d | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `NoSources` | unknown | unknown | `src/session/error_code.rs:11` |
| error-3deaf32f6de511f0f379 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `InternalStateUnavailable` | unknown | unknown | `src/session/error_code.rs:20` |
| error-4091cfce39877a118646 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` | `OutputExists` | unknown | unknown | `src/session/lifecycle/trace.rs:92` |
| error-4188b0e503f060db1c19 | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | `ZeroFrameSamples` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:87` |
| error-41cb10fd8d51a765e08a | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | type | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:271` |
| error-41f2089211d24584b24f | `pocketstation::session::lifecycle::events::SessionFinalizationFailure` | type | unknown | unknown | `src/session/lifecycle/events.rs:186` |
| error-425c4c27bc7253e0a25a | `pocketstation::session::lifecycle::engine::SessionEngineStartError` | type | unknown | unknown | `src/session/lifecycle/engine.rs:315` |
| error-428f59c894663eeacfa3 | `pocketstation::session::extensions::audio_input::source::AudioInputError` | `RegistrationStateUnavailable` | unknown | unknown | `src/session/extensions/audio_input/source.rs:95` |
| error-43cb1681c9f9f2858e6c | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | `WrongFrameLength` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:289` |
| error-45e86f8306f19f6519ac | `pocketstation::session::prepare::error::SessionPrepareError` | `DuplicateWorkerRoute` | unknown | unknown | `src/session/prepare/error.rs:53` |
| error-463e60123d12dc890ccb | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `InvalidEndpoint` | unknown | unknown | `src/session/error_code.rs:15` |
| error-4686258fd27d4a5a4f9f | `pocketstation::session::compile::error::SessionCompileError` | `AmbiguousOperatorPort` | unknown | unknown | `src/session/compile/error.rs:32` |
| error-4a9972fe9560338c20d0 | `pocketstation::session::compile::error::SessionCompileError` | `GraphCompile` | unknown | unknown | `src/session/compile/error.rs:88` |
| error-4be106ea8f79891a38b7 | `pocketstation::session::error_code::SessionStartErrorCode` | `RuntimeStartFailed` | unknown | unknown | `src/session/error_code.rs:78` |
| error-5164b8ec69b0c3230ba9 | `pocketstation::session::error::SessionError` | `UnknownEndpoint` | unknown | unknown | `src/session/error.rs:44` |
| error-54cfc365460333fd61a4 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `InvalidRoute` | unknown | unknown | `src/session/error_code.rs:17` |
| error-5604cdc007b2e62e318b | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `ForeignEndpoint` | unknown | unknown | `src/session/error_code.rs:18` |
| error-5619d76cf5c38cef98e0 | `pocketstation::session::compile::error::SessionCompileError` | type | unknown | unknown | `src/session/compile/error.rs:7` |
| error-566c23ce11a1e1d5dfd4 | `pocketstation::session::error_code::PolledAudioPollErrorCode` | `InternalStateUnavailable` | unknown | unknown | `src/session/error_code.rs:134` |
| error-567afcee36d517bc2b12 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `InvalidOperator` | unknown | unknown | `src/session/error_code.rs:16` |
| error-571a8dd79ebc75a2a9d4 | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | `UnsupportedSampleFormat` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:83` |
| error-57cf7b46240bcd35d91c | `pocketstation::session::error::SessionError` | `DraftFrozen` | unknown | unknown | `src/session/error.rs:36` |
| error-5aa27838dcaa21b67918 | `pocketstation::session::extensions::source::SourceManifestError` | `SignalMediaMismatch` | unknown | unknown | `src/session/extensions/source.rs:693` |
| error-5c22e5fd098b376f042d | `pocketstation::session::error_code::PolledAudioPollErrorCode` | `Empty` | unknown | unknown | `src/session/error_code.rs:132` |
| error-5c531f7163a5a5123988 | `pocketstation::session::prepare::error::SessionPrepareError` | `OperatorDeclarationMismatch` | unknown | unknown | `src/session/prepare/error.rs:70` |
| error-5f08d312a132aa730ae4 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | type | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:298` |
| error-603415dd16d43373c5dd | `pocketstation::session::extensions::builtins::SessionGraphRegistrationError` | type | unknown | unknown | `src/session/extensions/builtins.rs:30` |
| error-60a199328b42c5541bb9 | `pocketstation::session::prepare::error::SessionPrepareError` | `SignalRouteMismatch` | unknown | unknown | `src/session/prepare/error.rs:82` |
| error-6113e3ee5c4ecfc18606 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | `Empty` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:285` |
| error-617d858378abe4343a28 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingExternalSourceDefinition` | unknown | unknown | `src/session/prepare/error.rs:24` |
| error-623a923e6f0c3b4455c3 | `pocketstation::session::lifecycle::events::SessionControlFailure` | type | unknown | unknown | `src/session/lifecycle/events.rs:70` |
| error-62eff8258aff985c50e3 | `pocketstation::session::compile::error::SessionCompileError` | `UnknownExternalSource` | unknown | unknown | `src/session/compile/error.rs:69` |
| error-64c7c068036dca6eaf92 | `pocketstation::session::error_code::SessionStopFailureCode` | `EndpointFinalizationFailed` | unknown | unknown | `src/session/error_code.rs:175` |
| error-650341b3b851310dbed8 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `InvalidOptions` | unknown | unknown | `src/session/lifecycle/start_contract.rs:115` |
| error-65b3e68f94711ce2eb1e | `pocketstation::session::prepare::error::SessionPrepareError` | `SourceChannel` | unknown | unknown | `src/session/prepare/error.rs:13` |
| error-670b037138954f7e2d21 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `RuntimeWorkerSpawn` | unknown | unknown | `src/session/lifecycle/start_contract.rs:184` |
| error-676e83ed356f2ac65364 | `pocketstation::session::error_code::SessionStartErrorCode` | `CaptureUnsupported` | unknown | unknown | `src/session/error_code.rs:75` |
| error-67f963992c0413215b19 | `pocketstation::session::error_code::SessionStopFailureCode` | `CaptureFinalizationFailed` | unknown | unknown | `src/session/error_code.rs:173` |
| error-688b27dc617fd0a70b8b | `pocketstation::session::compile::error::SessionCompileError` | `MissingRequiredOperatorInput` | unknown | unknown | `src/session/compile/error.rs:43` |
| error-693c21b41c40916ba663 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | type | unknown | unknown | `src/session/error_code.rs:10` |
| error-69a7f58bbf73880124f6 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `Io` | unknown | unknown | `src/session/lifecycle/trace.rs:358` |
| error-6b428ad170835e580c28 | `pocketstation::session::compile::error::SessionCompileError` | `UnknownOperatorPort` | unknown | unknown | `src/session/compile/error.rs:37` |
| error-6b449aab81da0a9d6727 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingWorkerCapacity` | unknown | unknown | `src/session/prepare/error.rs:45` |
| error-6c35b8127b70b76e56f8 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `InvalidManifest` | unknown | unknown | `src/session/declaration/typed_stream.rs:189` |
| error-6d122772366c417a64b8 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `OperatorRuntimeHost` | unknown | unknown | `src/session/lifecycle/start_contract.rs:139` |
| error-6d779f2544b76608d177 | `pocketstation::session::error_code::SessionStartErrorCode` | `UnsupportedPlatform` | unknown | unknown | `src/session/error_code.rs:63` |
| error-6f885734262eeb24eb20 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `NoSourceOutputs` | unknown | unknown | `src/session/error_code.rs:13` |
| error-6ffaf0879fb95bb130e1 | `pocketstation::session::extensions::source::SourceTypeIdError` | `InvalidContractSyntax` | unknown | unknown | `src/session/extensions/source.rs:81` |
| error-7435321e51fbb7c1318c | `pocketstation::session::error_code::SessionStartErrorCode` | `CompileFailed` | unknown | unknown | `src/session/error_code.rs:67` |
| error-75dab4cfd96eab9020e3 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `UnknownSource` | unknown | unknown | `src/session/error_code.rs:25` |
| error-799cb648c1c3f46484cb | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | `Io` | unknown | unknown | `src/session/lifecycle/trace.rs:104` |
| error-79d9eaf54262a90f48af | `pocketstation::session::compile::error::SessionCompileError` | `OperatorNodeTypeMismatch` | unknown | unknown | `src/session/compile/error.rs:15` |
| error-79eda3aa72ffd06cf2f5 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingWorkerSampleSpec` | unknown | unknown | `src/session/prepare/error.rs:47` |
| error-7a72149a5eb8947126fe | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `UnsupportedVersion` | unknown | unknown | `src/session/error_code.rs:22` |
| error-7bacfca08d73c577129c | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `UnknownRecordType` | unknown | unknown | `src/session/lifecycle/trace.rs:386` |
| error-7cff5baa2ae2862e0d91 | `pocketstation::session::extensions::source::SourceRegistrationError` | `NodeTypeConflict` | unknown | unknown | `src/session/extensions/source.rs:707` |
| error-81d80780ea7432d78eba | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `UnknownOperatorInstance` | unknown | unknown | `src/session/error_code.rs:26` |
| error-8444f3af4e1cba13c367 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `InvalidSelector` | unknown | unknown | `src/session/error_code.rs:14` |
| error-84e739b6016f4d5925c5 | `pocketstation::session::compile::error::SessionCompileError` | `RuntimePlan` | unknown | unknown | `src/session/compile/error.rs:90` |
| error-86c8a4bb1ea788f52b7d | `pocketstation::session::error::SessionError` | `InvalidSelector` | unknown | unknown | `src/session/error.rs:23` |
| error-881b798294dbf7bf777b | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | `ConflictingDefinition` | unknown | unknown | `src/session/lifecycle/engine.rs:311` |
| error-886fb5d45587ec66596a | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `RuntimeWorkerReady` | unknown | unknown | `src/session/lifecycle/start_contract.rs:189` |
| error-8a0115aac81fce8c8085 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | `Closed` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:275` |
| error-8ae88b06e37ec337ade1 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | `WrongSource` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:283` |
| error-8b33d995a520d51b577e | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `CaptureOpen` | unknown | unknown | `src/session/lifecycle/start_contract.rs:165` |
| error-8d128d781276e9bbfef8 | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | `InvalidCapacity` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:85` |
| error-8d87bbf8f6e564b8d5f8 | `pocketstation::session::prepare::error::SessionPrepareError` | `WorkerRouteMismatch` | unknown | unknown | `src/session/prepare/error.rs:57` |
| error-8f612196c6aa2fba6a5c | `pocketstation::session::extensions::source::SourceManifestError` | `InvalidSignal` | unknown | unknown | `src/session/extensions/source.rs:691` |
| error-8fa639ce75cf01cef8e9 | `pocketstation::session::extensions::source::SourceDriverError` | `Failed` | unknown | unknown | `src/session/extensions/source.rs:750` |
| error-920dd279ce4e6e998ab6 | `pocketstation::session::error::SessionError` | `NoSourceOutputRoutes` | unknown | unknown | `src/session/error.rs:18` |
| error-925fc1e530fa891aa2d8 | `pocketstation::session::error_code::SessionStartErrorCode` | `CaptureBackendFailed` | unknown | unknown | `src/session/error_code.rs:76` |
| error-934d30870063fae9f899 | `pocketstation::session::prepare::error::SessionPrepareError` | `IncompatibleNodeBinding` | unknown | unknown | `src/session/prepare/error.rs:74` |
| error-93a39899711693436df2 | `pocketstation::session::error::SessionError` | `InvalidRoute` | unknown | unknown | `src/session/error.rs:29` |
| error-93e3edb9aec3dbedb913 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `InvalidSignal` | unknown | unknown | `src/session/declaration/typed_stream.rs:187` |
| error-9466981c2d3dc2a34e25 | `pocketstation::session::extensions::source::SourceManifestError` | `InvalidSafetyContract` | unknown | unknown | `src/session/extensions/source.rs:695` |
| error-94a86cf600de12e0905c | `pocketstation::session::error_code::SessionStopFailureCode` | `OperatorFinalizationFailed` | unknown | unknown | `src/session/error_code.rs:174` |
| error-95dd9252b0f0e2463be3 | `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | `StructuralNodeRegistration` | unknown | unknown | `src/session/lifecycle/engine.rs:297` |
| error-96a01df45f95f265c187 | `pocketstation::session::extensions::source::SourceManifestError` | `NoOutputs` | unknown | unknown | `src/session/extensions/source.rs:683` |
| error-97b94a81429125a9361e | `pocketstation::session::compile::error::SessionCompileError` | `UnknownSourceNodeType` | unknown | unknown | `src/session/compile/error.rs:67` |
| error-989a0f48d6c83ba52fc8 | `pocketstation::session::error::SessionError` | `OperatorHasNoDestination` | unknown | unknown | `src/session/error.rs:63` |
| error-9c6c61ab147ddeda45c1 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `CapturePrepare` | unknown | unknown | `src/session/lifecycle/start_contract.rs:158` |
| error-9cf8d614b0b883527bd9 | `pocketstation::session::error_code::SessionStopFailureCode` | `LineageFailed` | unknown | unknown | `src/session/error_code.rs:177` |
| error-9d62df633349f572f4a6 | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | type | unknown | unknown | `src/session/lifecycle/engine.rs:305` |
| error-9dedf8edf94ac1e55756 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | `Cancelled` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:301` |
| error-a11c604c8f34ec8d57f0 | `pocketstation::session::extensions::source::SourceManifestError` | `EmptyOutputName` | unknown | unknown | `src/session/extensions/source.rs:687` |
| error-a17217b9e6e97a216c72 | `pocketstation::session::extensions::source::SourceManifestError` | `NonOutputPort` | unknown | unknown | `src/session/extensions/source.rs:685` |
| error-a17a92a5ef775cbc4ce3 | `pocketstation::session::extensions::source::SourceTypeIdError` | type | unknown | unknown | `src/session/extensions/source.rs:68` |
| error-a1e701b141d4a31991e9 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `IdExhausted` | unknown | unknown | `src/session/error_code.rs:21` |
| error-a2c11e3b2fac86648d9b | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `ExternalSourcePrepare` | unknown | unknown | `src/session/lifecycle/start_contract.rs:119` |
| error-a2c3855ea65b50396cd2 | `pocketstation::session::error::SessionError` | `IdExhausted` | unknown | unknown | `src/session/error.rs:40` |
| error-a2c77a1a304559a12bdb | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `IncompleteTrace` | unknown | unknown | `src/session/lifecycle/trace.rs:370` |
| error-a5c6467b0389345a58a7 | `pocketstation::session::extensions::source::SourceRegistrationError` | `DuplicateSourceType` | unknown | unknown | `src/session/extensions/source.rs:705` |
| error-a5c6adb6a036dc5a996e | `pocketstation::session::lifecycle::engine::SessionEngineStartError` | `Freeze` | unknown | unknown | `src/session/lifecycle/engine.rs:317` |
| error-a6251ce3b327dcbc42c1 | `pocketstation::session::error_code::SessionStartErrorCode` | `CapturePermissionDenied` | unknown | unknown | `src/session/error_code.rs:73` |
| error-a95f9013c7f1dd284405 | `pocketstation::session::extensions::source::SourceTypeIdError` | `NonAscii` | unknown | unknown | `src/session/extensions/source.rs:79` |
| error-aa18bd1884c488f82a5d | `pocketstation::session::error_code::SessionStopFailureCode` | `RuntimeFailed` | unknown | unknown | `src/session/error_code.rs:176` |
| error-abee40c0f1b0aa530e5d | `pocketstation::session::error::SessionError` | `ForeignEndpoint` | unknown | unknown | `src/session/error.rs:31` |
| error-acb7df140b3a226751e0 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | `InvalidBuffer` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:302` |
| error-ad5f00b3df72f9cd2d2f | `pocketstation::session::prepare::error::SessionPrepareError` | `DuplicateOperatorInput` | unknown | unknown | `src/session/prepare/error.rs:76` |
| error-af71a7c14f7f3ae4e7d5 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `StreamInputMismatch` | unknown | unknown | `src/session/declaration/typed_stream.rs:211` |
| error-b0d733fc4a54e651f1b2 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | type | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:281` |
| error-b1ec4869b25659799bc3 | `pocketstation::session::error_code::SessionDeclarationErrorCode` | `UnknownEndpoint` | unknown | unknown | `src/session/error_code.rs:23` |
| error-b2cc2701192f8e0c66be | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | `PolledAudioEndpoint` | unknown | unknown | `src/session/lifecycle/host.rs:372` |
| error-b4e98230212106ff3ebf | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingNodeBinding` | unknown | unknown | `src/session/prepare/error.rs:72` |
| error-b5db1182dcb3054c3827 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingWorkerEdgeContract` | unknown | unknown | `src/session/prepare/error.rs:43` |
| error-b5faf333de07fa505592 | `pocketstation::session::error::SessionError` | `InvalidOperator` | unknown | unknown | `src/session/error.rs:27` |
| error-b68303516b4ff4d2e009 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | `UnsupportedPlatform` | unknown | unknown | `src/session/lifecycle/host.rs:378` |
| error-b7401f9fed346707f5a9 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `StemRequiresPcmAudio` | unknown | unknown | `src/session/declaration/typed_stream.rs:209` |
| error-b7d4071b3d47f050b640 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | `MissingApplicationBackend` | unknown | unknown | `src/session/lifecycle/host.rs:374` |
| error-b9879da6ee0f7fae83e9 | `pocketstation::session::error_code::PolledAudioPollErrorCode` | type | unknown | unknown | `src/session/error_code.rs:131` |
| error-ba47b6fce71590dcf460 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `InvalidLifecycleTransition` | unknown | unknown | `src/session/lifecycle/trace.rs:378` |
| error-bc7ff212fc2c81f83ebe | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingWorkerEdge` | unknown | unknown | `src/session/prepare/error.rs:41` |
| error-bc92e00331c1093a3a5f | `pocketstation::session::error_code::SessionRuntimeErrorCode` | `MissingMetricsSnapshot` | unknown | unknown | `src/session/error_code.rs:117` |
| error-bce4b849bfc960c45d79 | `pocketstation::session::error::SessionError` | `UnknownSourceInstance` | unknown | unknown | `src/session/error.rs:48` |
| error-bdb3331ea4ef7fe66d05 | `pocketstation::session::error_code::SessionRuntimeErrorCode` | type | unknown | unknown | `src/session/error_code.rs:116` |
| error-be29c32418daaa924ef9 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `OperatorIdentityMismatch` | unknown | unknown | `src/session/declaration/typed_stream.rs:191` |
| error-beaa9f589133fc98f1e7 | `pocketstation::session::error_code::SessionStartErrorCode` | `UnsupportedSourceTopology` | unknown | unknown | `src/session/error_code.rs:70` |
| error-beb4130127a375713c32 | `pocketstation::session::extensions::audio_input::source::AudioInputError` | `IncompatibleContract` | unknown | unknown | `src/session/extensions/audio_input/source.rs:99` |
| error-bf9caf926a103f498b58 | `pocketstation::session::prepare::error::SessionPrepareError` | `DuplicateSignalRoute` | unknown | unknown | `src/session/prepare/error.rs:80` |
| error-c07d72f5e30502c5e35e | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `EndpointStart` | unknown | unknown | `src/session/lifecycle/start_contract.rs:172` |
| error-c28e3e6c28087dff573a | `pocketstation::session::extensions::source::SourceTypeIdError` | `SurroundingWhitespace` | unknown | unknown | `src/session/extensions/source.rs:72` |
| error-c2d649937ae3a9ef7ab8 | `pocketstation::session::extensions::audio_input::source::AudioInputError` | `Configuration` | unknown | unknown | `src/session/extensions/audio_input/source.rs:87` |
| error-c3776894719f6597effe | `pocketstation::session::error_code::SessionStartErrorCode` | `EndpointStartFailed` | unknown | unknown | `src/session/error_code.rs:77` |
| error-c4078667a542ee4c22a9 | `pocketstation::session::lifecycle::events::SessionSourceFailure` | type | unknown | unknown | `src/session/lifecycle/events.rs:104` |
| error-c4b05b1ff0ce3eb51753 | `pocketstation::session::error::SessionError` | `NoSourceOutputs` | unknown | unknown | `src/session/error.rs:12` |
| error-c4db270fbfc99d3f8faa | `pocketstation::session::lifecycle::engine::SessionEngineStartError` | `Compile` | unknown | unknown | `src/session/lifecycle/engine.rs:319` |
| error-c5f1eb69ebb74cf15e64 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `ExternalSourceStart` | unknown | unknown | `src/session/lifecycle/start_contract.rs:134` |
| error-c63eb18f275acc6648a3 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingExternalSourceRouteEdge` | unknown | unknown | `src/session/prepare/error.rs:37` |
| error-c66f753f9e7e5e5256e0 | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | `Definition` | unknown | unknown | `src/session/lifecycle/engine.rs:307` |
| error-c6ba1b909fa3355e0f6e | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | `EndpointRegistration` | unknown | unknown | `src/session/lifecycle/host.rs:366` |
| error-c85d605c21711b7e10e2 | `pocketstation::session::extensions::builtins::SessionGraphRegistrationError` | `DuplicateNodeType` | unknown | unknown | `src/session/extensions/builtins.rs:32` |
| error-c903c954a8388e80c31b | `pocketstation::session::extensions::source::SourceTypeIdError` | `MissingSourceCategory` | unknown | unknown | `src/session/extensions/source.rs:83` |
| error-c93ad2041e473832a5af | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `Session` | unknown | unknown | `src/session/declaration/typed_stream.rs:213` |
| error-ca48b459481dadde3611 | `pocketstation::session::lifecycle::engine::SessionEngineStartError` | `Sidecar` | unknown | unknown | `src/session/lifecycle/engine.rs:325` |
| error-cc91b74bceb19cc51035 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | `Engine` | unknown | unknown | `src/session/lifecycle/host.rs:364` |
| error-ccb8feb1e059a3072bf5 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | `Capacity` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:294` |
| error-cde143927c1a2ed682e3 | `pocketstation::session::error_code::SessionStartErrorCode` | `MissingRecordingConfiguration` | unknown | unknown | `src/session/error_code.rs:80` |
| error-cf3f9d19e681cc44062f | `pocketstation::session::prepare::error::SessionPrepareError` | `UnknownWorkerRoute` | unknown | unknown | `src/session/prepare/error.rs:51` |
| error-cfbf77d2f976c5aed1ae | `pocketstation::session::lifecycle::engine::SessionEngineStartError` | `Start` | unknown | unknown | `src/session/lifecycle/engine.rs:323` |
| error-d00ac12fba0e6df50785 | `pocketstation::session::error::SessionError` | `NoSources` | unknown | unknown | `src/session/error.rs:8` |
| error-d03ca54170b81b828319 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `TerminalMismatch` | unknown | unknown | `src/session/lifecycle/trace.rs:382` |
| error-d52bb713e495501acaf1 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | `MissingMicrophoneBackend` | unknown | unknown | `src/session/lifecycle/host.rs:376` |
| error-d70859898391d7fb751e | `pocketstation::session::compile::error::SessionCompileError` | `AudioBridgeOutputNotExclusive` | unknown | unknown | `src/session/compile/error.rs:57` |
| error-d71d594435c92a39c42b | `pocketstation::session::compile::error::SessionCompileError` | `UnknownOperator` | unknown | unknown | `src/session/compile/error.rs:11` |
| error-d8a2c8fd93fb61da997b | `pocketstation::session::prepare::error::SessionPrepareError` | `WorkerTopologyMismatch` | unknown | unknown | `src/session/prepare/error.rs:86` |
| error-d8ba815f1c349040eb3c | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | `FrameSampleCountOverflow` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:89` |
| error-d9bda64e62b93396e451 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `RuntimeRunner` | unknown | unknown | `src/session/lifecycle/start_contract.rs:178` |
| error-daafed99d7930d99780f | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | `Full` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:299` |
| error-dbaec972d3e5e995315c | `pocketstation::session::prepare::error::SessionPrepareError` | `InvalidExternalAudioMedia` | unknown | unknown | `src/session/prepare/error.rs:26` |
| error-dc1470006f3a1879155a | `pocketstation::session::extensions::source::SourceRegistrationError` | type | unknown | unknown | `src/session/extensions/source.rs:701` |
| error-dd9b0944fab115622a9e | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | `ZeroSampleRate` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:79` |
| error-de346fd67cd741e2f720 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` | type | unknown | unknown | `src/session/lifecycle/trace.rs:88` |
| error-de62ad12abec4fe95634 | `pocketstation::session::compile::error::SessionCompileError` | `InvalidAudioBridgeOutput` | unknown | unknown | `src/session/compile/error.rs:50` |
| error-e0156673b6b92108993e | `pocketstation::session::error::SessionError` | `DraftPoisoned` | unknown | unknown | `src/session/error.rs:38` |
| error-e05d55d4a0c89abe05a2 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | type | unknown | unknown | `src/session/declaration/typed_stream.rs:185` |
| error-e0f047cc8c5e72fc743f | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | `ChannelClosed` | unknown | unknown | `src/session/lifecycle/trace.rs:100` |
| error-e38c6dc8a32ea675b428 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `Truncated` | unknown | unknown | `src/session/lifecycle/trace.rs:366` |
| error-e40d6018b2ff7744a576 | `pocketstation::session::prepare::error::SessionPrepareError` | `InvalidGeneratedAudioMedia` | unknown | unknown | `src/session/prepare/error.rs:33` |
| error-e40f381d12347f1ba25a | `pocketstation::session::extensions::source::SourceTypeIdError` | `TooLong` | unknown | unknown | `src/session/extensions/source.rs:74` |
| error-e4a18f2ff12c27b8a7d2 | `pocketstation::session::extensions::audio_input::source::AudioInputError` | `Session` | unknown | unknown | `src/session/extensions/audio_input/source.rs:93` |
| error-e5a0f4157139c02839e8 | `pocketstation::session::error::SessionError` | `UnsupportedVersion` | unknown | unknown | `src/session/error.rs:42` |
| error-e5e51729a7c2159c2fb5 | `pocketstation::session::prepare::error::SessionPrepareError` | `Runtime` | unknown | unknown | `src/session/prepare/error.rs:11` |
| error-eaf412c3ad6a57f484c2 | `pocketstation::session::prepare::error::SessionPrepareError` | `MissingGeneratedAudioBridge` | unknown | unknown | `src/session/prepare/error.rs:35` |
| error-eba421ec6fc6ea5b18b0 | `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | type | unknown | unknown | `src/session/lifecycle/engine.rs:295` |
| error-ee644567c179d92d391a | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteError` | type | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:305` |
| error-f1a93dad055a771b512b | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` | `Io` | unknown | unknown | `src/session/lifecycle/trace.rs:94` |
| error-f1f40c6209ca5c3905f9 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | type | unknown | unknown | `src/session/lifecycle/host.rs:362` |
| error-f25fb567a3c5ba52acf4 | `pocketstation::session::error_code::SessionStopFailureCode` | type | unknown | unknown | `src/session/error_code.rs:171` |
| error-f2f0cc43e10eef1e7547 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | `WorkerPanicked` | unknown | unknown | `src/session/lifecycle/trace.rs:102` |
| error-f31a1a7d9408b8dcf0e0 | `pocketstation::session::error::SessionError` | `InvalidEndpoint` | unknown | unknown | `src/session/error.rs:25` |
| error-f367afa5ea9efa8111db | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | `MissingTerminal` | unknown | unknown | `src/session/lifecycle/trace.rs:380` |
| error-f398bb77a81f0fce4dfa | `pocketstation::session::extensions::source::SourceManifestError` | type | unknown | unknown | `src/session/extensions/source.rs:677` |
| error-f7621d750f169d77f63d | `pocketstation::session::compile::error::SessionCompileError` | `UnknownAsyncOperator` | unknown | unknown | `src/session/compile/error.rs:21` |
| error-f7c057df554028e50776 | `pocketstation::session::lifecycle::events::SessionEndpointFailure` | type | unknown | unknown | `src/session/lifecycle/events.rs:125` |
| error-f7c2baaf0cc118d52fd0 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `GeneratedAudioBridge` | unknown | unknown | `src/session/lifecycle/start_contract.rs:129` |
| error-f9b141a8953e575484c3 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | type | unknown | unknown | `src/session/lifecycle/trace.rs:98` |
| error-fa13821e1d0e3f98cb79 | `pocketstation::session::lifecycle::start_contract::SessionStartError` | `EndpointPrepare` | unknown | unknown | `src/session/lifecycle/start_contract.rs:152` |
| error-fa4762eddc65a3e95cbb | `pocketstation::session::compile::error::SessionCompileError` | `UnknownEndpointNodeType` | unknown | unknown | `src/session/compile/error.rs:23` |
| error-fb043864344ef5585b53 | `pocketstation::session::error::SessionError` | `NoRoutes` | unknown | unknown | `src/session/error.rs:10` |
| error-fc4d25c0d599a2d1ebf2 | `pocketstation::session::lifecycle::start_contract::SessionStartFailure` | type | unknown | unknown | `src/session/lifecycle/start_contract.rs:263` |
| error-fc7681d3248ad549d0dd | `pocketstation::session::error_code::SessionStartErrorCode` | `EndpointPrepareFailed` | unknown | unknown | `src/session/error_code.rs:72` |
| error-fce0d35b53752cafb25d | `pocketstation::session::lifecycle::start_contract::SessionStartError` | type | unknown | unknown | `src/session/lifecycle/start_contract.rs:113` |
| error-fd4e96d8451eb5c557a7 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | `MisalignedChannels` | unknown | unknown | `src/session/extensions/audio_input/buffer.rs:287` |
| error-fd96648c87fb1288af53 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | `AmbiguousPort` | unknown | unknown | `src/session/declaration/typed_stream.rs:203` |
| error-ff123cc8964e77d29464 | `pocketstation::session::error_code::SessionStartErrorCode` | `CaptureSourceUnavailable` | unknown | unknown | `src/session/error_code.rs:74` |
| error-ffbf1295f7ee96519d1c | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | `UnsupportedChannelCount` | unknown | unknown | `src/session/extensions/audio_input/mod.rs:81` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)
- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Session API](/docs/reference/session.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/error.rs:1-67` (`DIRECT`)
- `src/session/lifecycle/events.rs:1-736` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
