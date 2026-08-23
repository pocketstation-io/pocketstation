# C ABI reference

<!-- claims: CLM-REF-012-CAP-001,CLM-REF-012-CAP-002,CLM-REF-012-SOURCE-001 -->

## Scope

- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `pocketstation::abi::extension::PKS_EXTENSION_ABI_MAJOR` | constant | Defines the major version of extension ABI. | `src/abi/extension.rs:7` |
| `pocketstation::abi::extension::PKS_EXTENSION_ABI_MINOR` | constant | Defines the minor version of extension ABI. | `src/abi/extension.rs:8` |
| `pocketstation::abi::extension::PksExtensionKind` | enum | Selects the extension kind used by PocketStation. | `src/abi/extension.rs:32` |
| `pocketstation::abi::extension::PksExtensionPortDirection` | enum | Selects the extension port direction used by PocketStation. | `src/abi/extension.rs:40` |
| `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Enumerates the supported session status code cases. | `src/abi/session/abi.rs:79` |
| `new` | function | Creates a new `PksSessionStatus`. | `src/abi/session/abi.rs:69` |
| `ok` | function | Creates a successful status value for `PksSessionStatus`. | `src/abi/session/abi.rs:62` |
| `pocketstation::abi::executable_extension::PksExtensionCallbacks` | struct | Represents extension callbacks in the PocketStation API. | `src/abi/executable_extension.rs:91` |
| `pocketstation::abi::executable_extension::PksExtensionLibrary` | struct | Represents extension library in the PocketStation API. | `src/abi/executable_extension.rs:123` |
| `pocketstation::abi::executable_extension::PksExtensionPipelineDeclaration` | struct | Describes the extension pipeline declaration contract. | `src/abi/executable_extension.rs:168` |
| `pocketstation::abi::executable_extension::PksExtensionSignalBuffer` | struct | Represents extension signal buffer in the PocketStation API. | `src/abi/executable_extension.rs:153` |
| `pocketstation::abi::executable_extension::PksExtensionSignalView` | struct | Represents extension signal view in the PocketStation API. | `src/abi/executable_extension.rs:138` |
| `pocketstation::abi::extension::PksExtensionAbiVersion` | struct | Represents extension ABI version in the PocketStation API. | `src/abi/extension.rs:14` |
| `pocketstation::abi::extension::PksExtensionDescriptor` | struct | Describes the extension descriptor contract. | `src/abi/extension.rs:47` |
| `pocketstation::abi::extension::PksExtensionPort` | struct | Represents extension port in the PocketStation API. | `src/abi/extension.rs:60` |
| `pocketstation::abi::session::abi::PksSessionStatus` | struct | Reports the structured session status. | `src/abi/session/abi.rs:56` |
| `pocketstation::abi::session::abi::PksSessionUtf8` | struct | Represents session UTF-8 in the PocketStation API. | `src/abi/session/abi.rs:101` |
| `PksExtensionAbiVersion::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:16` |
| `PksExtensionAbiVersion::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:17` |
| `PksExtensionAbiVersion::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionAbiVersion` ABI structure. | `src/abi/extension.rs:15` |
| `PksExtensionCallbacks::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:93` |
| `PksExtensionCallbacks::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:94` |
| `PksExtensionCallbacks::create` | struct_field | Provides the create callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:99` |
| `PksExtensionCallbacks::destroy_instance` | struct_field | Provides the destroy instance callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:106` |
| `PksExtensionCallbacks::destroy_registration` | struct_field | Provides the destroy registration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:107` |
| `PksExtensionCallbacks::endpoint_consume` | struct_field | Provides the endpoint consume callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:103` |
| `PksExtensionCallbacks::finish` | struct_field | Provides the finish callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:105` |
| `PksExtensionCallbacks::max_payload_bytes` | struct_field | Limits payload storage for `PksExtensionCallbacks`, in bytes. | `src/abi/executable_extension.rs:96` |
| `PksExtensionCallbacks::operator_process` | struct_field | Provides the operator process callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:102` |
| `PksExtensionCallbacks::prepare` | struct_field | Provides the prepare callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:100` |
| `PksExtensionCallbacks::registration_context` | struct_field | Carries the opaque registration context used by `PksExtensionCallbacks` callbacks. | `src/abi/executable_extension.rs:95` |
| `PksExtensionCallbacks::request_stop` | struct_field | Provides the request stop callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:104` |
| `PksExtensionCallbacks::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:97` |
| `PksExtensionCallbacks::source_next` | struct_field | Provides the source next callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:101` |
| `PksExtensionCallbacks::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionCallbacks` ABI structure. | `src/abi/executable_extension.rs:92` |
| `PksExtensionCallbacks::validate_configuration` | struct_field | Provides the validate configuration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:98` |
| `PksExtensionDescriptor::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:49` |
| `PksExtensionDescriptor::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:50` |
| `PksExtensionDescriptor::extension_id` | struct_field | Identifies the extension associated with `PksExtensionDescriptor`. | `src/abi/extension.rs:55` |
| `PksExtensionDescriptor::generation` | struct_field | Stores the generation associated with `PksExtensionDescriptor`. | `src/abi/extension.rs:53` |
| `PksExtensionDescriptor::kind` | struct_field | Stores the kind associated with `PksExtensionDescriptor`. | `src/abi/extension.rs:51` |
| `PksExtensionDescriptor::port_count` | struct_field | Stores the number of port represented by `PksExtensionDescriptor`. | `src/abi/extension.rs:54` |
| `PksExtensionDescriptor::revision` | struct_field | Stores the revision associated with `PksExtensionDescriptor`. | `src/abi/extension.rs:52` |
| `PksExtensionDescriptor::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionDescriptor` ABI structure. | `src/abi/extension.rs:48` |
| `PksExtensionLibrary::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:125` |
| `PksExtensionLibrary::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:126` |
| `PksExtensionLibrary::acquire_registration` | struct_field | Provides the acquire registration callback used by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:130` |
| `PksExtensionLibrary::library_context` | struct_field | Carries the opaque library context used by `PksExtensionLibrary` callbacks. | `src/abi/executable_extension.rs:129` |
| `PksExtensionLibrary::registration_count` | struct_field | Stores the number of registration represented by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:127` |
| `PksExtensionLibrary::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionLibrary`. | `src/abi/executable_extension.rs:128` |
| `PksExtensionLibrary::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionLibrary` ABI structure. | `src/abi/executable_extension.rs:124` |
| `PksExtensionPipelineDeclaration::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:170` |
| `PksExtensionPipelineDeclaration::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:171` |
| `PksExtensionPipelineDeclaration::endpoint_id` | struct_field | Identifies the endpoint associated with `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:177` |
| `PksExtensionPipelineDeclaration::endpoint_input_port` | struct_field | Stores the endpoint input port associated with `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:178` |
| `PksExtensionPipelineDeclaration::operator_id` | struct_field | Identifies the operator associated with `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:174` |
| `PksExtensionPipelineDeclaration::operator_input_port` | struct_field | Stores the operator input port associated with `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:175` |
| `PksExtensionPipelineDeclaration::operator_output_port` | struct_field | Stores the operator output port associated with `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:176` |
| `PksExtensionPipelineDeclaration::source_id` | struct_field | Identifies the source associated with `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:172` |
| `PksExtensionPipelineDeclaration::source_output_port` | struct_field | Stores the source output port associated with `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:173` |
| `PksExtensionPipelineDeclaration::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPipelineDeclaration` ABI structure. | `src/abi/executable_extension.rs:169` |
| `PksExtensionPort::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:62` |
| `PksExtensionPort::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:63` |
| `PksExtensionPort::direction` | struct_field | Stores the direction associated with `PksExtensionPort`. | `src/abi/extension.rs:64` |
| `PksExtensionPort::name` | struct_field | Stores the name associated with `PksExtensionPort`. | `src/abi/extension.rs:66` |
| `PksExtensionPort::required` | struct_field | Indicates whether required applies to `PksExtensionPort`. | `src/abi/extension.rs:65` |
| `PksExtensionPort::schema` | struct_field | Stores the schema associated with `PksExtensionPort`. | `src/abi/extension.rs:69` |
| `PksExtensionPort::semantic_role` | struct_field | Stores the semantic role associated with `PksExtensionPort`. | `src/abi/extension.rs:68` |
| `PksExtensionPort::signal_id` | struct_field | Identifies the signal associated with `PksExtensionPort`. | `src/abi/extension.rs:67` |
| `PksExtensionPort::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPort` ABI structure. | `src/abi/extension.rs:61` |
| `PksExtensionSignalBuffer::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:155` |
| `PksExtensionSignalBuffer::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:156` |
| `PksExtensionSignalBuffer::capacity_bytes` | struct_field | Stores the capacity size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:158` |
| `PksExtensionSignalBuffer::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:157` |
| `PksExtensionSignalBuffer::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:163` |
| `PksExtensionSignalBuffer::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:160` |
| `PksExtensionSignalBuffer::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:159` |
| `PksExtensionSignalBuffer::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:161` |
| `PksExtensionSignalBuffer::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:162` |
| `PksExtensionSignalBuffer::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalBuffer` ABI structure. | `src/abi/executable_extension.rs:154` |
| `PksExtensionSignalView::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:140` |
| `PksExtensionSignalView::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:141` |
| `PksExtensionSignalView::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:142` |
| `PksExtensionSignalView::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:147` |
| `PksExtensionSignalView::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:144` |
| `PksExtensionSignalView::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalView`, in bytes. | `src/abi/executable_extension.rs:143` |
| `PksExtensionSignalView::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:145` |
| `PksExtensionSignalView::sequence_number` | struct_field | Stores the sequence number associated with `PksExtensionSignalView`. | `src/abi/executable_extension.rs:148` |
| `PksExtensionSignalView::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:146` |
| `PksExtensionSignalView::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalView` ABI structure. | `src/abi/executable_extension.rs:139` |
| `PksSessionStatus::code` | struct_field | Stores the code associated with `PksSessionStatus`. | `src/abi/session/abi.rs:57` |
| `PksSessionStatus::detail` | struct_field | Stores the detail associated with `PksSessionStatus`. | `src/abi/session/abi.rs:58` |
| `PksSessionUtf8::data` | struct_field | Carries the data owned or referenced by `PksSessionUtf8`. | `src/abi/session/abi.rs:102` |
| `PksSessionUtf8::len_bytes` | struct_field | Stores the len size for `PksSessionUtf8`, in bytes. | `src/abi/session/abi.rs:103` |
| `pocketstation::abi::executable_extension::PksExtensionAcquireRegistrationCallback` | type_alias | Defines the optional C callback used to acquire an extension registration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:110` |
| `pocketstation::abi::executable_extension::PksExtensionCreateCallback` | type_alias | Defines the optional C callback used to create an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:56` |
| `pocketstation::abi::executable_extension::PksExtensionDestroyCallback` | type_alias | Defines the optional C callback used to destroy extension-owned context; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:87` |
| `pocketstation::abi::executable_extension::PksExtensionEndpointConsumeCallback` | type_alias | Defines the optional C callback used to consume an endpoint input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:77` |
| `pocketstation::abi::executable_extension::PksExtensionFinishCallback` | type_alias | Defines the optional C callback used to finish extension work; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:85` |
| `pocketstation::abi::executable_extension::PksExtensionLibraryEntrypoint` | type_alias | Names the extension library entrypoint type used by the public API. | `src/abi/executable_extension.rs:133` |
| `pocketstation::abi::executable_extension::PksExtensionOperatorProcessCallback` | type_alias | Defines the optional C callback used to process an operator input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:70` |
| `pocketstation::abi::executable_extension::PksExtensionPrepareCallback` | type_alias | Defines the optional C callback used to prepare an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:48` |
| `pocketstation::abi::executable_extension::PksExtensionSourceNextCallback` | type_alias | Defines the optional C callback used to produce the next source signal; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:63` |
| `pocketstation::abi::executable_extension::PksExtensionStopCallback` | type_alias | Defines the optional C callback used to request an extension instance to stop; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:83` |
| `pocketstation::abi::executable_extension::PksExtensionValidateConfigurationCallback` | type_alias | Defines the optional C callback used to validate extension configuration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:50` |
| `pocketstation::abi::extension::PksExtensionKind::Endpoint` | variant | Selects endpoint behavior for `PksExtensionKind`. | `src/abi/extension.rs:35` |
| `pocketstation::abi::extension::PksExtensionKind::Operator` | variant | Selects operator behavior for `PksExtensionKind`. | `src/abi/extension.rs:34` |
| `pocketstation::abi::extension::PksExtensionKind::Source` | variant | Selects source behavior for `PksExtensionKind`. | `src/abi/extension.rs:33` |
| `pocketstation::abi::extension::PksExtensionPortDirection::Input` | variant | Selects input behavior for `PksExtensionPortDirection`. | `src/abi/extension.rs:41` |
| `pocketstation::abi::extension::PksExtensionPortDirection::Output` | variant | Selects output behavior for `PksExtensionPortDirection`. | `src/abi/extension.rs:42` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | Indicates the backend failure state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:93` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | Indicates that the operation was cancelled. | `src/abi/session/abi.rs:94` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | Indicates the foreign handle state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:90` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | Indicates the index out of range state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:95` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | Indicates the internal panic state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:87` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidArgument` | variant | Indicates the invalid argument state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:89` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidHandle` | variant | Indicates the invalid handle state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:84` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidLifecycleState` | variant | Indicates the invalid lifecycle state state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:91` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidStructSize` | variant | Indicates the invalid struct size state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:83` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::MisalignedPointer` | variant | Indicates the misaligned pointer state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:88` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::NoCapacity` | variant | Indicates the no capacity state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:86` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::NullArgument` | variant | Indicates the null argument state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:81` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::Ok` | variant | Indicates that the operation completed successfully. | `src/abi/session/abi.rs:80` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::StaleHandle` | variant | Indicates the stale handle state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:85` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMajor` | variant | Indicates the unsupported ABI major state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:82` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMinor` | variant | Indicates the unsupported ABI minor state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:96` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::WouldBlock` | variant | Indicates the would block state for `PksSessionStatusCode`. | `src/abi/session/abi.rs:92` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [C ABI ownership](/docs/concepts/c-abi-ownership.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `include/pocketstation.h:1-615` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
