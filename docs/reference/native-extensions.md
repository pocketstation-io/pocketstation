# Native extension API

<!-- claims: CLM-REF-010-SCOPE-001,CLM-REF-010-TEXT-001,CLM-REF-010-TEXT-002,CLM-REF-010-SOURCE-001 -->

## Scope

- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.

The scope of **Native extension API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Native extension API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-3eb7316123cefcdc0aa2 | `pocketstation::abi::extension::PKS_EXTENSION_ABI_MAJOR` | constant | Defines the major version of extension ABI. | `src/abi/extension.rs:7` |
| sym-a60dfa04bb15c0df7de2 | `pocketstation::abi::extension::PKS_EXTENSION_ABI_MINOR` | constant | Defines the minor version of extension ABI. | `src/abi/extension.rs:8` |
| sym-61cc214c9f17f5efc20a | `pocketstation::native_extension::EXTENSION_LIBRARY_ENTRYPOINT_V1` | constant | Exact exported symbol required from a native Extension ABI v1 dynamic library. The suffix follows the ABI major; compatible minor revisions use the same entrypoint. | `src/native_extension/mod.rs:24` |
| sym-ce4c2a4d569001a1911b | `pocketstation::abi::extension::PksExtensionKind` | enum | Selects the extension kind used by PocketStation. | `src/abi/extension.rs:32` |
| sym-9296aa9eb2ebd632f22e | `pocketstation::abi::extension::PksExtensionPortDirection` | enum | Selects the extension port direction used by PocketStation. | `src/abi/extension.rs:40` |
| sym-81e037fbd73e62639416 | `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Provides stable C ABI status categories returned by Session operations. | `src/abi/session/abi.rs:79` |
| sym-3b00d82a04d35afbba20 | `pocketstation::native_extension::NativeExtensionKind` | enum | Selects the native extension kind used by PocketStation. | `src/native_extension/mod.rs:27` |
| sym-c144f166f28ea7fc2604 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | enum | Provides stable categories for native-extension load and validation failures. | `src/native_extension/mod.rs:78` |
| sym-504709804e2abc76d105 | `as_str` | function | Returns the stable string representation of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:97` |
| sym-337292e97a842fdf2668 | `canonical_path` | function | Returns the canonical path associated with `NativeExtensionLibrary`. | `src/native_extension/mod.rs:68` |
| sym-e74cb3e63e1024a31011 | `code` | function | Returns the stable error or status code represented by `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:131` |
| sym-0fb2f47c4afb90cac532 | `generation` | function | Returns the generation associated with `NativeExtensionRegistration`. | `src/native_extension/mod.rs:54` |
| sym-5910add440db99cd12a3 | `id` | function | Returns the id held by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:42` |
| sym-e73975dc1292035caa2d | `kind` | function | Returns the kind represented by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:46` |
| sym-7c29f2093b359646c32c | `message` | function | Returns the diagnostic message reported by `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:135` |
| sym-a4c74d7d271cbf17cdd3 | `new` | function | Creates a new `PksSessionStatus`. | `src/abi/session/abi.rs:69` |
| sym-1f0d61c4d674829cdd06 | `ok` | function | Creates a successful status value for `PksSessionStatus`. | `src/abi/session/abi.rs:62` |
| sym-33ac3ff52504ab1aa3eb | `path` | function | Returns the path associated with `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:139` |
| sym-14292393f6d9f0b57801 | `registrations` | function | Returns the registrations associated with `NativeExtensionLibrary`. | `src/native_extension/mod.rs:72` |
| sym-ca9371c7d79ce617e7c9 | `revision` | function | Returns the revision held by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:50` |
| sym-b9ca441db554330302d9 | `pocketstation::abi::executable_extension::PksExtensionCallbacks` | struct | Defines the optional function table through which a native extension prepares, runs, stops, and releases instances. | `src/abi/executable_extension.rs:91` |
| sym-ab00ae321687f73286ca | `pocketstation::abi::executable_extension::PksExtensionLibrary` | struct | Owns a loaded native-extension library and the registrations imported from its validated descriptor. | `src/abi/executable_extension.rs:123` |
| sym-f632b21433ce6ab18231 | `pocketstation::abi::executable_extension::PksExtensionPipelineDeclaration` | struct | Declares one extension pipeline instance and the native registrations it uses. | `src/abi/executable_extension.rs:168` |
| sym-a6d4a635612b1aaf4d15 | `pocketstation::abi::executable_extension::PksExtensionSignalBuffer` | struct | Provides bounded extension-owned storage for a signal returned through the native ABI. | `src/abi/executable_extension.rs:153` |
| sym-f18199c6541f4add2eeb | `pocketstation::abi::executable_extension::PksExtensionSignalView` | struct | Borrows one signal payload and metadata for delivery into a native-extension callback. | `src/abi/executable_extension.rs:138` |
| sym-7c08b84faec371e236f1 | `pocketstation::abi::extension::PksExtensionAbiVersion` | struct | Carries the major and minor native-extension ABI versions checked during loading. | `src/abi/extension.rs:14` |
| sym-271409255d3a0750ce7b | `pocketstation::abi::extension::PksExtensionDescriptor` | struct | Declares a native extension's ABI version, library callbacks, and registration entrypoint. | `src/abi/extension.rs:47` |
| sym-7b822cc6a15684bd7f84 | `pocketstation::abi::extension::PksExtensionPort` | struct | Describes one native-extension port across the C ABI, including direction and signal metadata. | `src/abi/extension.rs:60` |
| sym-0ed9edb0f5260fbc6220 | `pocketstation::abi::session::abi::PksSessionStatus` | struct | Reports the structured session status. | `src/abi/session/abi.rs:56` |
| sym-0d106bb641ad4be881c4 | `pocketstation::abi::session::abi::PksSessionUtf8` | struct | Borrows a UTF-8 byte range across the C Session ABI as a pointer and length. | `src/abi/session/abi.rs:101` |
| sym-53933707dc1f0cf8a775 | `pocketstation::native_extension::NativeExtensionLibrary` | struct | Immutable receipt for registrations imported into one Session. Executable code ownership remains internal to the registered factories and drivers. | `src/native_extension/mod.rs:62` |
| sym-cd19944854ed52da6f88 | `pocketstation::native_extension::NativeExtensionLibraryError` | struct | Reports a native extension library error. | `src/native_extension/mod.rs:124` |
| sym-f2148d5a24dce2026a1f | `pocketstation::native_extension::NativeExtensionRegistration` | struct | Identifies one node registration imported transactionally from a native extension. | `src/native_extension/mod.rs:34` |
| sym-9d17e8591a9ffed70991 | `PksExtensionAbiVersion::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:16` |
| sym-fee228f569854aa8601d | `PksExtensionAbiVersion::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:17` |
| sym-f8cef8ca2c3556aae4ab | `PksExtensionAbiVersion::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionAbiVersion` ABI structure. | `src/abi/extension.rs:15` |
| sym-83b2969d41412b4d7a37 | `PksExtensionCallbacks::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:93` |
| sym-b9ec1cd153ae49f53149 | `PksExtensionCallbacks::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:94` |
| sym-d1651e41136e427ac15c | `PksExtensionCallbacks::create` | struct_field | Provides the create callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:99` |
| sym-fa371318b9fa5538ab95 | `PksExtensionCallbacks::destroy_instance` | struct_field | Provides the destroy instance callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:106` |
| sym-fd8e8561d432e6ca0902 | `PksExtensionCallbacks::destroy_registration` | struct_field | Provides the destroy registration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:107` |
| sym-cdf572e57f6d9cf70938 | `PksExtensionCallbacks::endpoint_consume` | struct_field | Provides the endpoint consume callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:103` |
| sym-4eb162bc05a3e9ec5044 | `PksExtensionCallbacks::finish` | struct_field | Provides the finish callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:105` |
| sym-d20d58f4a00e154cceaf | `PksExtensionCallbacks::max_payload_bytes` | struct_field | Limits payload storage for `PksExtensionCallbacks`, in bytes. | `src/abi/executable_extension.rs:96` |
| sym-8423ba41181baeea0642 | `PksExtensionCallbacks::operator_process` | struct_field | Provides the operator process callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:102` |
| sym-3300284912e249800c1f | `PksExtensionCallbacks::prepare` | struct_field | Provides the prepare callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:100` |
| sym-523f4f07fe0b25036492 | `PksExtensionCallbacks::registration_context` | struct_field | Carries the opaque registration context used by `PksExtensionCallbacks` callbacks. | `src/abi/executable_extension.rs:95` |
| sym-34377707f885299004be | `PksExtensionCallbacks::request_stop` | struct_field | Provides the request stop callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:104` |
| sym-8ab345c39de303abf923 | `PksExtensionCallbacks::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:97` |
| sym-7e8d6042a0cf3e833b88 | `PksExtensionCallbacks::source_next` | struct_field | Provides the source next callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:101` |
| sym-b7abfaa36ba35b10fa98 | `PksExtensionCallbacks::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionCallbacks` ABI structure. | `src/abi/executable_extension.rs:92` |
| sym-b27888731b6a70f14b95 | `PksExtensionCallbacks::validate_configuration` | struct_field | Provides the validate configuration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:98` |
| sym-de7af8723bb618302962 | `PksExtensionDescriptor::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:49` |
| sym-fb4fceba7f10043d503e | `PksExtensionDescriptor::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:50` |
| sym-b09b163379b1e632ca91 | `PksExtensionDescriptor::extension_id` | struct_field | Identifies the extension identifier recorded by `PksExtensionDescriptor`. | `src/abi/extension.rs:55` |
| sym-cd76df9447a5702c0696 | `PksExtensionDescriptor::generation` | struct_field | Identifies the generation of the resource represented by `PksExtensionDescriptor`. | `src/abi/extension.rs:53` |
| sym-848be399c38fd5e3f56e | `PksExtensionDescriptor::kind` | struct_field | Records the kind selected for `PksExtensionDescriptor`. | `src/abi/extension.rs:51` |
| sym-6b86e7d040a2d2300864 | `PksExtensionDescriptor::port_count` | struct_field | Stores the number of port represented by `PksExtensionDescriptor`. | `src/abi/extension.rs:54` |
| sym-6c10774fd0745685cb27 | `PksExtensionDescriptor::revision` | struct_field | Stores the revision component of `PksExtensionDescriptor`. | `src/abi/extension.rs:52` |
| sym-cd0a07994945fa3a75b1 | `PksExtensionDescriptor::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionDescriptor` ABI structure. | `src/abi/extension.rs:48` |
| sym-bed4ba74d429e253186a | `PksExtensionLibrary::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:125` |
| sym-c684b876503061de1dd2 | `PksExtensionLibrary::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:126` |
| sym-f6923e733b4102570f6a | `PksExtensionLibrary::acquire_registration` | struct_field | Provides the acquire registration callback used by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:130` |
| sym-d2a462ea5971aeadf65c | `PksExtensionLibrary::library_context` | struct_field | Carries the opaque library context used by `PksExtensionLibrary` callbacks. | `src/abi/executable_extension.rs:129` |
| sym-5325c06e543fe3fb7e86 | `PksExtensionLibrary::registration_count` | struct_field | Stores the number of registration represented by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:127` |
| sym-1e04b4fa1e558d0d8ec0 | `PksExtensionLibrary::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionLibrary`. | `src/abi/executable_extension.rs:128` |
| sym-633e3ddad5eae6455415 | `PksExtensionLibrary::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionLibrary` ABI structure. | `src/abi/executable_extension.rs:124` |
| sym-492ec59461a98588d709 | `PksExtensionPipelineDeclaration::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:170` |
| sym-7b594b2883fcfe1fd9b2 | `PksExtensionPipelineDeclaration::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:171` |
| sym-290fa7d306be450d3937 | `PksExtensionPipelineDeclaration::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:177` |
| sym-68738b1952cdefe542fd | `PksExtensionPipelineDeclaration::endpoint_input_port` | struct_field | References the endpoint input port participating in `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:178` |
| sym-ecddf8d73c8e9d6284de | `PksExtensionPipelineDeclaration::operator_id` | struct_field | Identifies the operator identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:174` |
| sym-4c8bec614b543c654393 | `PksExtensionPipelineDeclaration::operator_input_port` | struct_field | References the operator input port participating in `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:175` |
| sym-46c0b0de16ff6914228d | `PksExtensionPipelineDeclaration::operator_output_port` | struct_field | References the operator output port participating in `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:176` |
| sym-dfc95dd8fbd85a2da7c2 | `PksExtensionPipelineDeclaration::source_id` | struct_field | Identifies the source identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:172` |
| sym-3def34d3b3e284749986 | `PksExtensionPipelineDeclaration::source_output_port` | struct_field | References the source output port participating in `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:173` |
| sym-abf6494dea7fa11f40a2 | `PksExtensionPipelineDeclaration::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPipelineDeclaration` ABI structure. | `src/abi/executable_extension.rs:169` |
| sym-92b6de43d2bd3d825994 | `PksExtensionPort::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:62` |
| sym-2f878cc54cb97dd7c8c0 | `PksExtensionPort::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:63` |
| sym-c3e7ffa3de4c0ec33dd5 | `PksExtensionPort::direction` | struct_field | Records the direction selected for `PksExtensionPort`. | `src/abi/extension.rs:64` |
| sym-b138dd2d12d72a0caf00 | `PksExtensionPort::name` | struct_field | Stores the human-readable name used to identify `PksExtensionPort`. | `src/abi/extension.rs:66` |
| sym-818c1e80bf72e7a8b6e6 | `PksExtensionPort::required` | struct_field | Indicates whether required applies to `PksExtensionPort`. | `src/abi/extension.rs:65` |
| sym-85feaf1eeb9223172262 | `PksExtensionPort::schema` | struct_field | Records the schema selected for `PksExtensionPort`. | `src/abi/extension.rs:69` |
| sym-d45470b7ba721a6b7ae6 | `PksExtensionPort::semantic_role` | struct_field | Names the semantic role assigned to the extension port in `PksExtensionPort`. | `src/abi/extension.rs:68` |
| sym-5bcff79544e6a681652d | `PksExtensionPort::signal_id` | struct_field | Identifies the signal identifier recorded by `PksExtensionPort`. | `src/abi/extension.rs:67` |
| sym-5ee9368f2d7b3929c0fd | `PksExtensionPort::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPort` ABI structure. | `src/abi/extension.rs:61` |
| sym-536f459749f3b16614fe | `PksExtensionSignalBuffer::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:155` |
| sym-d19ceb062a9973011b63 | `PksExtensionSignalBuffer::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:156` |
| sym-ffa5f045303ccd040870 | `PksExtensionSignalBuffer::capacity_bytes` | struct_field | Stores the capacity size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:158` |
| sym-ccaa1c0a9bd3280ec321 | `PksExtensionSignalBuffer::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:157` |
| sym-f6b8f42abb75af395c08 | `PksExtensionSignalBuffer::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:163` |
| sym-d06178d02b904847b699 | `PksExtensionSignalBuffer::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:160` |
| sym-bb1d0f45d5053827a1a4 | `PksExtensionSignalBuffer::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:159` |
| sym-689e33ea28a222ec6a20 | `PksExtensionSignalBuffer::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:161` |
| sym-7c88133aeb8bd6977839 | `PksExtensionSignalBuffer::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:162` |
| sym-7a56c132b5667065efc9 | `PksExtensionSignalBuffer::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalBuffer` ABI structure. | `src/abi/executable_extension.rs:154` |
| sym-61507865b14f20ecaad3 | `PksExtensionSignalView::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:140` |
| sym-270a32533e02e0419217 | `PksExtensionSignalView::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:141` |
| sym-e9ddc46f15f2bbd137ac | `PksExtensionSignalView::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:142` |
| sym-e46da0951e8cca035a04 | `PksExtensionSignalView::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:147` |
| sym-3213ff660528d242f617 | `PksExtensionSignalView::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:144` |
| sym-13edc82c1dcd24f80eb5 | `PksExtensionSignalView::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalView`, in bytes. | `src/abi/executable_extension.rs:143` |
| sym-d71866163388ce8559ba | `PksExtensionSignalView::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:145` |
| sym-5ff4b121979a55de8c16 | `PksExtensionSignalView::sequence_number` | struct_field | Orders `PksExtensionSignalView` within its protocol or stream sequence. | `src/abi/executable_extension.rs:148` |
| sym-4b76013d0b87cdfde384 | `PksExtensionSignalView::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:146` |
| sym-70ccfc11e50c3418fa7a | `PksExtensionSignalView::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalView` ABI structure. | `src/abi/executable_extension.rs:139` |
| sym-81dcdf507990a523fe0e | `PksSessionStatus::code` | struct_field | Stores the code component of `PksSessionStatus`. | `src/abi/session/abi.rs:57` |
| sym-3314c210b3949d1bcd1d | `PksSessionStatus::detail` | struct_field | Stores the detail component of `PksSessionStatus`. | `src/abi/session/abi.rs:58` |
| sym-4598b1658958f9bd6a71 | `PksSessionUtf8::data` | struct_field | Carries the data owned or referenced by `PksSessionUtf8`. | `src/abi/session/abi.rs:102` |
| sym-062d1ef0071e13f96677 | `PksSessionUtf8::len_bytes` | struct_field | Stores the len size for `PksSessionUtf8`, in bytes. | `src/abi/session/abi.rs:103` |
| sym-aeeaa5604243332b5394 | `pocketstation::abi::executable_extension::PksExtensionAcquireRegistrationCallback` | type_alias | Defines the optional C callback used to acquire an extension registration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:110` |
| sym-086627215efa123e182d | `pocketstation::abi::executable_extension::PksExtensionCreateCallback` | type_alias | Defines the optional C callback used to create an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:56` |
| sym-32efbef473a9a5697a1c | `pocketstation::abi::executable_extension::PksExtensionDestroyCallback` | type_alias | Defines the optional C callback used to destroy extension-owned context; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:87` |
| sym-03cc3c3d4a79421ab83c | `pocketstation::abi::executable_extension::PksExtensionEndpointConsumeCallback` | type_alias | Defines the optional C callback used to consume an endpoint input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:77` |
| sym-09f27cd611795c65bbc8 | `pocketstation::abi::executable_extension::PksExtensionFinishCallback` | type_alias | Defines the optional C callback used to finish extension work; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:85` |
| sym-497f48c6c93d03e14de7 | `pocketstation::abi::executable_extension::PksExtensionLibraryEntrypoint` | type_alias | Exposes `PksSessionStatus` as the public `PksExtensionLibraryEntrypoint` alias at this API boundary. | `src/abi/executable_extension.rs:133` |
| sym-e3262cbc0403e5714c35 | `pocketstation::abi::executable_extension::PksExtensionOperatorProcessCallback` | type_alias | Defines the optional C callback used to process an operator input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:70` |
| sym-98dd08100b2055a1b49b | `pocketstation::abi::executable_extension::PksExtensionPrepareCallback` | type_alias | Defines the optional C callback used to prepare an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:48` |
| sym-cf706ae4f39c2bc1b613 | `pocketstation::abi::executable_extension::PksExtensionSourceNextCallback` | type_alias | Defines the optional C callback used to produce the next source signal; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:63` |
| sym-8ed828f9fa01d64e22e6 | `pocketstation::abi::executable_extension::PksExtensionStopCallback` | type_alias | Defines the optional C callback used to request an extension instance to stop; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:83` |
| sym-802847170fd9ec2831b4 | `pocketstation::abi::executable_extension::PksExtensionValidateConfigurationCallback` | type_alias | Defines the optional C callback used to validate extension configuration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:50` |
| sym-56c936241baf53dc318b | `pocketstation::abi::extension::PksExtensionKind::Endpoint` | variant | Registers the native extension as a endpoint implementation. | `src/abi/extension.rs:35` |
| sym-5397638d011436a1d64f | `pocketstation::abi::extension::PksExtensionKind::Operator` | variant | Registers the native extension as a operator implementation. | `src/abi/extension.rs:34` |
| sym-6cdad11f0f57d9b7536c | `pocketstation::abi::extension::PksExtensionKind::Source` | variant | Registers the native extension as a source implementation. | `src/abi/extension.rs:33` |
| sym-de4a9df3781e78ea305d | `pocketstation::abi::extension::PksExtensionPortDirection::Input` | variant | Declares a native-extension port as input. | `src/abi/extension.rs:41` |
| sym-682b021885ebe1d80fb1 | `pocketstation::abi::extension::PksExtensionPortDirection::Output` | variant | Declares a native-extension port as output. | `src/abi/extension.rs:42` |
| sym-88a5c91a8b7995b883c6 | `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | Identifies the backend failure state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:93` |
| sym-4c8096adcbaca6dc4a3b | `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | Indicates that the operation was cancelled. | `src/abi/session/abi.rs:94` |
| sym-90f9d5d0c6c0d83ae11e | `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | Identifies the foreign handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:90` |
| sym-e0d4f694924adb5b5024 | `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | Identifies the index out of range state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:95` |
| sym-fec2d38860664c7ec550 | `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | Identifies the internal panic state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:87` |
| sym-07f3c3d53a9850d3339a | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidArgument` | variant | Identifies the invalid argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:89` |
| sym-40f070ac74e52d3c1b18 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidHandle` | variant | Identifies the invalid handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:84` |
| sym-d43fee26dadd663d4521 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidLifecycleState` | variant | Identifies the invalid lifecycle state state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:91` |
| sym-529b3161af5cd85f4d11 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidStructSize` | variant | Identifies the invalid struct size state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:83` |
| sym-3bd08434e914c776ed21 | `pocketstation::abi::session::abi::PksSessionStatusCode::MisalignedPointer` | variant | Identifies the misaligned pointer state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:88` |
| sym-c12ad57ffb8e0b63f712 | `pocketstation::abi::session::abi::PksSessionStatusCode::NoCapacity` | variant | Identifies the no capacity state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:86` |
| sym-bc160c9b6402292fae07 | `pocketstation::abi::session::abi::PksSessionStatusCode::NullArgument` | variant | Identifies the null argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:81` |
| sym-d36caf9deaa7e27f346e | `pocketstation::abi::session::abi::PksSessionStatusCode::Ok` | variant | Indicates that the operation completed successfully. | `src/abi/session/abi.rs:80` |
| sym-df81b8d4f63657ebd2c0 | `pocketstation::abi::session::abi::PksSessionStatusCode::StaleHandle` | variant | Identifies the stale handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:85` |
| sym-c02a61702684195fdbc7 | `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMajor` | variant | Identifies the unsupported ABI major state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:82` |
| sym-71be19497787c8ce4b62 | `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMinor` | variant | Identifies the unsupported ABI minor state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:96` |
| sym-3427ce389f485bbdf541 | `pocketstation::abi::session::abi::PksSessionStatusCode::WouldBlock` | variant | Identifies the would block state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:92` |
| sym-9ac15cf1a9fcb5fd0049 | `pocketstation::native_extension::NativeExtensionKind::Endpoint` | variant | Classifies the loaded native extension as endpoint. | `src/native_extension/mod.rs:30` |
| sym-cda90339325d8912240c | `pocketstation::native_extension::NativeExtensionKind::Operator` | variant | Classifies the loaded native extension as operator. | `src/native_extension/mod.rs:29` |
| sym-97a72cc297fac195637f | `pocketstation::native_extension::NativeExtensionKind::Source` | variant | Classifies the loaded native extension as source. | `src/native_extension/mod.rs:28` |
| sym-d04a12f5276e75e7fa39 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::DuplicateRegistration` | variant | Reports that registration duplicates an existing declaration or record. | `src/native_extension/mod.rs:92` |
| sym-0cb39a2ef4b2ec75b861 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointFailed` | variant | Reports that entrypoint failed. | `src/native_extension/mod.rs:85` |
| sym-f0fbbf4e893a1816faed | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointMissing` | variant | Classifies a failure at the entrypoint missing stage or component of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:83` |
| sym-00f358f7b87a5ae34033 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointPanicked` | variant | Reports that entrypoint panicked while the operation was active. | `src/native_extension/mod.rs:84` |
| sym-76d5dc8173f5517c54fd | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidLibraryDescriptor` | variant | Reports that the supplied library descriptor is invalid. | `src/native_extension/mod.rs:88` |
| sym-c6031a1c816a569c5631 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidRegistration` | variant | Reports that the supplied registration is invalid. | `src/native_extension/mod.rs:91` |
| sym-a2c4921680a7f521aae7 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::LibraryLoadFailed` | variant | Reports that library load failed. | `src/native_extension/mod.rs:82` |
| sym-6fff5ddf0723aca51c02 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathCanonicalizationFailed` | variant | Reports that path canonicalization failed. | `src/native_extension/mod.rs:80` |
| sym-d5e9531372dc38b02581 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotAbsolute` | variant | Classifies a failure at the path not absolute stage or component of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:79` |
| sym-167af7ba4cf1f211d713 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotFile` | variant | Classifies a failure at the path not file stage or component of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:81` |
| sym-14602df76b6f8f5edabf | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationAcquisitionFailed` | variant | Reports that registration acquisition failed. | `src/native_extension/mod.rs:90` |
| sym-9ac83079cf775bb2358b | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationAcquisitionPanicked` | variant | Reports that registration acquisition panicked while the operation was active. | `src/native_extension/mod.rs:89` |
| sym-1c1a4d173444716528f0 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationStateUnavailable` | variant | Reports that registration state is unavailable. | `src/native_extension/mod.rs:93` |
| sym-1c4a9e98244853258d35 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::UnsupportedAbiMajor` | variant | Reports that the requested ABI major is unsupported. | `src/native_extension/mod.rs:86` |
| sym-b3acb550909cebd73afb | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::UnsupportedAbiMinor` | variant | Reports that the requested ABI minor is unsupported. | `src/native_extension/mod.rs:87` |

## Interpretation

The **Native extension API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Glossary](/docs/glossary.md)
- [Native extension libraries](/docs/concepts/native-extensions.md)
- [PocketStation](/README.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)

## Evidence boundary

The claims on **Native extension API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/native_extension/mod.rs:24-24` (`DIRECT`)
- `src/native_extension/mod.rs:26-26` (`DIRECT`)
- `src/native_extension/mod.rs:26-26` (`DIRECT`)
- `src/native_extension/mod.rs:26-26` (`DIRECT`)
- `src/native_extension/mod.rs:27-31` (`DIRECT`)
- `src/native_extension/mod.rs:28-28` (`DIRECT`)
- `src/native_extension/mod.rs:29-29` (`DIRECT`)
- `src/native_extension/mod.rs:30-30` (`DIRECT`)
- `src/native_extension/mod.rs:33-33` (`DIRECT`)
- `src/native_extension/mod.rs:33-33` (`DIRECT`)
- `src/native_extension/mod.rs:33-33` (`DIRECT`)
- `src/native_extension/mod.rs:34-39` (`DIRECT`)
- `src/native_extension/mod.rs:35-35` (`DIRECT`)
- `src/native_extension/mod.rs:36-36` (`DIRECT`)
- `src/native_extension/mod.rs:37-37` (`DIRECT`)
- `src/native_extension/mod.rs:38-38` (`DIRECT`)
- `src/native_extension/mod.rs:42-44` (`DIRECT`)
- `src/native_extension/mod.rs:46-48` (`DIRECT`)
- `src/native_extension/mod.rs:50-52` (`DIRECT`)
- `src/native_extension/mod.rs:54-56` (`DIRECT`)
- `src/native_extension/mod.rs:61-61` (`DIRECT`)
- `src/native_extension/mod.rs:61-61` (`DIRECT`)
- `src/native_extension/mod.rs:61-61` (`DIRECT`)
- `src/native_extension/mod.rs:62-65` (`DIRECT`)

For **Native extension API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
