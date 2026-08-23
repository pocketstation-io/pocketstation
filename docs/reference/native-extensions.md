# Native extension API

<!-- claims: CLM-REF-010-CAP-001,CLM-REF-010-SOURCE-001 -->

## Scope

- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.

The scope of **Native extension API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Native extension API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-f711aa823577922f7731 | `pocketstation::abi::extension::PKS_EXTENSION_ABI_MAJOR` | constant | Defines the major version of extension ABI. | `src/abi/extension.rs:7` |
| sym-062a6480e9f37de71402 | `pocketstation::abi::extension::PKS_EXTENSION_ABI_MINOR` | constant | Defines the minor version of extension ABI. | `src/abi/extension.rs:8` |
| sym-374159181c3213969fb7 | `pocketstation::native_extension::EXTENSION_LIBRARY_ENTRYPOINT_V1` | constant | Exact exported symbol required from a native Extension ABI v1 dynamic library. The suffix follows the ABI major; compatible minor revisions use the same entrypoint. | `src/native_extension/mod.rs:24` |
| sym-36cd76830d91616ed45a | `pocketstation::abi::extension::PksExtensionKind` | enum | Selects the extension kind used by PocketStation. | `src/abi/extension.rs:32` |
| sym-664ec5a3767a0555bb69 | `pocketstation::abi::extension::PksExtensionPortDirection` | enum | Selects the extension port direction used by PocketStation. | `src/abi/extension.rs:40` |
| sym-082a210661ef8384587f | `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Enumerates the supported session status code cases. | `src/abi/session/abi.rs:79` |
| sym-350b38ad4fa5a2014d84 | `pocketstation::native_extension::NativeExtensionKind` | enum | Selects the native extension kind used by PocketStation. | `src/native_extension/mod.rs:27` |
| sym-3c3f70e43d0556be3c9e | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | enum | Enumerates the supported native extension library error code cases. | `src/native_extension/mod.rs:78` |
| sym-25e46e42e4bde888d842 | `as_str` | function | Returns the stable string representation of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:97` |
| sym-a23752ab1db9abce0522 | `canonical_path` | function | Returns the canonical path associated with `NativeExtensionLibrary`. | `src/native_extension/mod.rs:68` |
| sym-cfb1c8bf367e13c8d444 | `code` | function | Returns the stable error or status code represented by `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:131` |
| sym-ff7327324294fdd0e490 | `generation` | function | Returns the generation associated with `NativeExtensionRegistration`. | `src/native_extension/mod.rs:54` |
| sym-fabf53aef85f6d57579d | `id` | function | Returns the id held by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:42` |
| sym-f62a0b3d0c053e53cfe7 | `kind` | function | Returns the kind represented by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:46` |
| sym-dcfcca963568ac75044f | `message` | function | Returns the diagnostic message reported by `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:135` |
| sym-3db5f4df5763985a9d69 | `new` | function | Creates a new `PksSessionStatus`. | `src/abi/session/abi.rs:69` |
| sym-4ffe4e9de671e20f756d | `ok` | function | Creates a successful status value for `PksSessionStatus`. | `src/abi/session/abi.rs:62` |
| sym-55797113be143f702cbb | `path` | function | Returns the path associated with `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:139` |
| sym-a6a91667156fb0e2227e | `registrations` | function | Returns the registrations associated with `NativeExtensionLibrary`. | `src/native_extension/mod.rs:72` |
| sym-76dfec156970b96b4baa | `revision` | function | Returns the revision held by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:50` |
| sym-9bd58b6b5a2c501b0ef4 | `pocketstation::abi::executable_extension::PksExtensionCallbacks` | struct | Defines the optional function table through which a native extension prepares, runs, stops, and releases instances. | `src/abi/executable_extension.rs:91` |
| sym-1f6f74d8e3a6c6a532c1 | `pocketstation::abi::executable_extension::PksExtensionLibrary` | struct | Owns a loaded native-extension library and the registrations imported from its validated descriptor. | `src/abi/executable_extension.rs:123` |
| sym-618a77e04b1403f70a09 | `pocketstation::abi::executable_extension::PksExtensionPipelineDeclaration` | struct | Describes the extension pipeline declaration contract. | `src/abi/executable_extension.rs:168` |
| sym-2ceb679c2593bc6ff675 | `pocketstation::abi::executable_extension::PksExtensionSignalBuffer` | struct | Provides bounded extension-owned storage for a signal returned through the native ABI. | `src/abi/executable_extension.rs:153` |
| sym-860049bbb07b04ff2964 | `pocketstation::abi::executable_extension::PksExtensionSignalView` | struct | Borrows one signal payload and metadata for delivery into a native-extension callback. | `src/abi/executable_extension.rs:138` |
| sym-105d0da91c28e68138d0 | `pocketstation::abi::extension::PksExtensionAbiVersion` | struct | Carries the major and minor native-extension ABI versions checked during loading. | `src/abi/extension.rs:14` |
| sym-92f78c6ba703b0ab2f66 | `pocketstation::abi::extension::PksExtensionDescriptor` | struct | Describes the extension descriptor contract. | `src/abi/extension.rs:47` |
| sym-f579e6b2c4f5264875fe | `pocketstation::abi::extension::PksExtensionPort` | struct | Describes one native-extension port across the C ABI, including direction and signal metadata. | `src/abi/extension.rs:60` |
| sym-fc0b6bbde02125f27278 | `pocketstation::abi::session::abi::PksSessionStatus` | struct | Reports the structured session status. | `src/abi/session/abi.rs:56` |
| sym-8e9872a8d6c4dec6ba0f | `pocketstation::abi::session::abi::PksSessionUtf8` | struct | Borrows a UTF-8 byte range across the C Session ABI as a pointer and length. | `src/abi/session/abi.rs:101` |
| sym-daadf87bca13446b6fe2 | `pocketstation::native_extension::NativeExtensionLibrary` | struct | Immutable receipt for registrations imported into one Session. Executable code ownership remains internal to the registered factories and drivers. | `src/native_extension/mod.rs:62` |
| sym-f5c3060c94d92cd71443 | `pocketstation::native_extension::NativeExtensionLibraryError` | struct | Reports a native extension library error. | `src/native_extension/mod.rs:124` |
| sym-5dd141e95bf3c2701bdf | `pocketstation::native_extension::NativeExtensionRegistration` | struct | Identifies one node registration imported transactionally from a native extension. | `src/native_extension/mod.rs:34` |
| sym-0db6c4c8e3c8eef8794b | `PksExtensionAbiVersion::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:16` |
| sym-61bf3eecc5a6498ce0a5 | `PksExtensionAbiVersion::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:17` |
| sym-359a061e0f765240b0e7 | `PksExtensionAbiVersion::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionAbiVersion` ABI structure. | `src/abi/extension.rs:15` |
| sym-8bc7e5531d5f4ebeabda | `PksExtensionCallbacks::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:93` |
| sym-dab1881821172b856539 | `PksExtensionCallbacks::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:94` |
| sym-599b2a991f18d418e6df | `PksExtensionCallbacks::create` | struct_field | Provides the create callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:99` |
| sym-98126a76f7c0767dbb2f | `PksExtensionCallbacks::destroy_instance` | struct_field | Provides the destroy instance callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:106` |
| sym-3266527728cbcf1d1263 | `PksExtensionCallbacks::destroy_registration` | struct_field | Provides the destroy registration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:107` |
| sym-7df0c15a0078af5652ee | `PksExtensionCallbacks::endpoint_consume` | struct_field | Provides the endpoint consume callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:103` |
| sym-a1e986f527710e8625ba | `PksExtensionCallbacks::finish` | struct_field | Provides the finish callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:105` |
| sym-f4c8933bb9ce216ef72f | `PksExtensionCallbacks::max_payload_bytes` | struct_field | Limits payload storage for `PksExtensionCallbacks`, in bytes. | `src/abi/executable_extension.rs:96` |
| sym-522407c189871214eb21 | `PksExtensionCallbacks::operator_process` | struct_field | Provides the operator process callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:102` |
| sym-5d77bd53a7d9d8b2d701 | `PksExtensionCallbacks::prepare` | struct_field | Provides the prepare callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:100` |
| sym-4d09a07c66dbbf9811f4 | `PksExtensionCallbacks::registration_context` | struct_field | Carries the opaque registration context used by `PksExtensionCallbacks` callbacks. | `src/abi/executable_extension.rs:95` |
| sym-47be468b1863e10030a3 | `PksExtensionCallbacks::request_stop` | struct_field | Provides the request stop callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:104` |
| sym-53f881a9867fa7f1f1aa | `PksExtensionCallbacks::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:97` |
| sym-30f44a2d9d92969320be | `PksExtensionCallbacks::source_next` | struct_field | Provides the source next callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:101` |
| sym-3ae4ce3f92f139cdd659 | `PksExtensionCallbacks::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionCallbacks` ABI structure. | `src/abi/executable_extension.rs:92` |
| sym-eb0dca1e517ca0c1c0e2 | `PksExtensionCallbacks::validate_configuration` | struct_field | Provides the validate configuration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:98` |
| sym-abcd0237ad46888633d5 | `PksExtensionDescriptor::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:49` |
| sym-5ca838c1da4cb525fa85 | `PksExtensionDescriptor::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:50` |
| sym-0f4d4d22a0e6fd2eb32e | `PksExtensionDescriptor::extension_id` | struct_field | Identifies the extension identifier recorded by `PksExtensionDescriptor`. | `src/abi/extension.rs:55` |
| sym-1e35a852cf8422cfda75 | `PksExtensionDescriptor::generation` | struct_field | Stores the generation used by `PksExtensionDescriptor`. | `src/abi/extension.rs:53` |
| sym-89d0ad00d57c7921b628 | `PksExtensionDescriptor::kind` | struct_field | Stores the kind used by `PksExtensionDescriptor`. | `src/abi/extension.rs:51` |
| sym-f46920a3fa7f30ff7673 | `PksExtensionDescriptor::port_count` | struct_field | Stores the number of port represented by `PksExtensionDescriptor`. | `src/abi/extension.rs:54` |
| sym-a14b3b8e2f94ee7318d3 | `PksExtensionDescriptor::revision` | struct_field | Stores the revision used by `PksExtensionDescriptor`. | `src/abi/extension.rs:52` |
| sym-99130f9c9a586b1285aa | `PksExtensionDescriptor::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionDescriptor` ABI structure. | `src/abi/extension.rs:48` |
| sym-cb5ea79192e835be63b4 | `PksExtensionLibrary::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:125` |
| sym-0ca96d62385da73c2546 | `PksExtensionLibrary::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:126` |
| sym-0260646c2de1572ba96f | `PksExtensionLibrary::acquire_registration` | struct_field | Provides the acquire registration callback used by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:130` |
| sym-0d7fd33f9f0952bf4ee9 | `PksExtensionLibrary::library_context` | struct_field | Carries the opaque library context used by `PksExtensionLibrary` callbacks. | `src/abi/executable_extension.rs:129` |
| sym-302714882ef5b2a2fa83 | `PksExtensionLibrary::registration_count` | struct_field | Stores the number of registration represented by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:127` |
| sym-09da8e8904c29d2d2f56 | `PksExtensionLibrary::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionLibrary`. | `src/abi/executable_extension.rs:128` |
| sym-e50c6c86107f9d04ccfe | `PksExtensionLibrary::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionLibrary` ABI structure. | `src/abi/executable_extension.rs:124` |
| sym-e197462f13f33ef433f9 | `PksExtensionPipelineDeclaration::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:170` |
| sym-d744ad468d2792262a95 | `PksExtensionPipelineDeclaration::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:171` |
| sym-e72c8374c009be1e17cc | `PksExtensionPipelineDeclaration::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:177` |
| sym-c8d2d4bb6fadfa41d72e | `PksExtensionPipelineDeclaration::endpoint_input_port` | struct_field | Stores the endpoint input port used by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:178` |
| sym-7239e71c128d9ccb3a98 | `PksExtensionPipelineDeclaration::operator_id` | struct_field | Identifies the operator identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:174` |
| sym-08ab9b48097e42ccb58f | `PksExtensionPipelineDeclaration::operator_input_port` | struct_field | Stores the operator input port used by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:175` |
| sym-d47b28e1665ddbfc4c37 | `PksExtensionPipelineDeclaration::operator_output_port` | struct_field | Stores the operator output port used by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:176` |
| sym-77247e2a7fc20076a61f | `PksExtensionPipelineDeclaration::source_id` | struct_field | Identifies the source identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:172` |
| sym-3bf095f4da2d36534f8d | `PksExtensionPipelineDeclaration::source_output_port` | struct_field | Stores the source output port used by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:173` |
| sym-95b0b3761e220a8fd8bb | `PksExtensionPipelineDeclaration::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPipelineDeclaration` ABI structure. | `src/abi/executable_extension.rs:169` |
| sym-1ede5196a6baf13504c8 | `PksExtensionPort::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:62` |
| sym-92de720178a3677aaa75 | `PksExtensionPort::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:63` |
| sym-3430f45987e8b747886a | `PksExtensionPort::direction` | struct_field | Stores the direction used by `PksExtensionPort`. | `src/abi/extension.rs:64` |
| sym-24332ac9dcfae7bc9f70 | `PksExtensionPort::name` | struct_field | Stores the name used by `PksExtensionPort`. | `src/abi/extension.rs:66` |
| sym-29f827a3d1ea5505235d | `PksExtensionPort::required` | struct_field | Indicates whether required applies to `PksExtensionPort`. | `src/abi/extension.rs:65` |
| sym-f497b7f5351c84b5f18c | `PksExtensionPort::schema` | struct_field | Stores the schema used by `PksExtensionPort`. | `src/abi/extension.rs:69` |
| sym-b9da34912ba076cf18d0 | `PksExtensionPort::semantic_role` | struct_field | Stores the semantic role used by `PksExtensionPort`. | `src/abi/extension.rs:68` |
| sym-a3ebc882dc49ed1311dd | `PksExtensionPort::signal_id` | struct_field | Identifies the signal identifier recorded by `PksExtensionPort`. | `src/abi/extension.rs:67` |
| sym-9c4cd90eef0d3e802c7f | `PksExtensionPort::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPort` ABI structure. | `src/abi/extension.rs:61` |
| sym-a95b6ef65a7d569748f5 | `PksExtensionSignalBuffer::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:155` |
| sym-8fa2405a53f09a512414 | `PksExtensionSignalBuffer::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:156` |
| sym-2a42363a1803788c54f0 | `PksExtensionSignalBuffer::capacity_bytes` | struct_field | Stores the capacity size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:158` |
| sym-f47b06e230874e8f392e | `PksExtensionSignalBuffer::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:157` |
| sym-eda5ddd827a09334004e | `PksExtensionSignalBuffer::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:163` |
| sym-e23e3cff54d1066910c6 | `PksExtensionSignalBuffer::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:160` |
| sym-3f7d813fdaa6e1bbce85 | `PksExtensionSignalBuffer::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:159` |
| sym-c97d55f25ab252aee445 | `PksExtensionSignalBuffer::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:161` |
| sym-d5374acccd0afe623ad9 | `PksExtensionSignalBuffer::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:162` |
| sym-cd59cbb262c2aa7c160f | `PksExtensionSignalBuffer::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalBuffer` ABI structure. | `src/abi/executable_extension.rs:154` |
| sym-7c18c4e3d29a42664727 | `PksExtensionSignalView::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:140` |
| sym-a22b5837d46373c3682a | `PksExtensionSignalView::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:141` |
| sym-0a5eaaf109f4a21fc1b3 | `PksExtensionSignalView::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:142` |
| sym-fcb856959047ed19b3be | `PksExtensionSignalView::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:147` |
| sym-d292a82a33e80eaaa9f8 | `PksExtensionSignalView::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:144` |
| sym-c1c8d06838fb0e9fce04 | `PksExtensionSignalView::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalView`, in bytes. | `src/abi/executable_extension.rs:143` |
| sym-ac6eeb8886eb4844b4f0 | `PksExtensionSignalView::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:145` |
| sym-ae98ce87b33de0a77c79 | `PksExtensionSignalView::sequence_number` | struct_field | Stores the sequence number used by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:148` |
| sym-e24bb7bc7b74e43341b6 | `PksExtensionSignalView::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:146` |
| sym-8013fce198748680db57 | `PksExtensionSignalView::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalView` ABI structure. | `src/abi/executable_extension.rs:139` |
| sym-3aeea7e8779fe554c1dd | `PksSessionStatus::code` | struct_field | Stores the code used by `PksSessionStatus`. | `src/abi/session/abi.rs:57` |
| sym-a964cbfac0b18ce4d0a8 | `PksSessionStatus::detail` | struct_field | Stores the detail used by `PksSessionStatus`. | `src/abi/session/abi.rs:58` |
| sym-4850a01a3818712b3bc8 | `PksSessionUtf8::data` | struct_field | Carries the data owned or referenced by `PksSessionUtf8`. | `src/abi/session/abi.rs:102` |
| sym-f1d5e74b271b98a97960 | `PksSessionUtf8::len_bytes` | struct_field | Stores the len size for `PksSessionUtf8`, in bytes. | `src/abi/session/abi.rs:103` |
| sym-929589cb52631dd270e1 | `pocketstation::abi::executable_extension::PksExtensionAcquireRegistrationCallback` | type_alias | Defines the optional C callback used to acquire an extension registration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:110` |
| sym-8eadd98bd426760ed3bc | `pocketstation::abi::executable_extension::PksExtensionCreateCallback` | type_alias | Defines the optional C callback used to create an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:56` |
| sym-72a0d1a013e62c74c736 | `pocketstation::abi::executable_extension::PksExtensionDestroyCallback` | type_alias | Defines the optional C callback used to destroy extension-owned context; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:87` |
| sym-058bc6398ba9559930f5 | `pocketstation::abi::executable_extension::PksExtensionEndpointConsumeCallback` | type_alias | Defines the optional C callback used to consume an endpoint input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:77` |
| sym-40dddd7eddd2e900d81d | `pocketstation::abi::executable_extension::PksExtensionFinishCallback` | type_alias | Defines the optional C callback used to finish extension work; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:85` |
| sym-040e44a7efec0d272a56 | `pocketstation::abi::executable_extension::PksExtensionLibraryEntrypoint` | type_alias | Names the extension library entrypoint type used by the public API. | `src/abi/executable_extension.rs:133` |
| sym-cab0e0649d56d2d4021e | `pocketstation::abi::executable_extension::PksExtensionOperatorProcessCallback` | type_alias | Defines the optional C callback used to process an operator input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:70` |
| sym-51603d93967850b8bcbe | `pocketstation::abi::executable_extension::PksExtensionPrepareCallback` | type_alias | Defines the optional C callback used to prepare an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:48` |
| sym-d346b403b0052ab732f7 | `pocketstation::abi::executable_extension::PksExtensionSourceNextCallback` | type_alias | Defines the optional C callback used to produce the next source signal; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:63` |
| sym-5d1b8c85f5778fb1f9f5 | `pocketstation::abi::executable_extension::PksExtensionStopCallback` | type_alias | Defines the optional C callback used to request an extension instance to stop; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:83` |
| sym-28fc61ff23de4de04295 | `pocketstation::abi::executable_extension::PksExtensionValidateConfigurationCallback` | type_alias | Defines the optional C callback used to validate extension configuration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:50` |
| sym-172b87972a83540872e9 | `pocketstation::abi::extension::PksExtensionKind::Endpoint` | variant | Selects endpoint behavior for `PksExtensionKind`. | `src/abi/extension.rs:35` |
| sym-fae17803b87577e05dd6 | `pocketstation::abi::extension::PksExtensionKind::Operator` | variant | Selects operator behavior for `PksExtensionKind`. | `src/abi/extension.rs:34` |
| sym-4ab3d8a5880738f1694f | `pocketstation::abi::extension::PksExtensionKind::Source` | variant | Selects source behavior for `PksExtensionKind`. | `src/abi/extension.rs:33` |
| sym-57195f1473fa69a408c2 | `pocketstation::abi::extension::PksExtensionPortDirection::Input` | variant | Selects input behavior for `PksExtensionPortDirection`. | `src/abi/extension.rs:41` |
| sym-491654412e542b513980 | `pocketstation::abi::extension::PksExtensionPortDirection::Output` | variant | Selects output behavior for `PksExtensionPortDirection`. | `src/abi/extension.rs:42` |
| sym-c1fcb2ac1efedaf31c16 | `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | Identifies the backend failure state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:93` |
| sym-5a63c1ed4326cf37196e | `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | Indicates that the operation was cancelled. | `src/abi/session/abi.rs:94` |
| sym-936ba5fd69c6e7b77c23 | `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | Identifies the foreign handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:90` |
| sym-cab1eff076e630226b06 | `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | Identifies the index out of range state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:95` |
| sym-ccc76ce999251f56b0d4 | `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | Identifies the internal panic state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:87` |
| sym-d590715654d95777c466 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidArgument` | variant | Identifies the invalid argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:89` |
| sym-939c5af9e88df8f8760b | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidHandle` | variant | Identifies the invalid handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:84` |
| sym-d2d226c409a9bb33fa18 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidLifecycleState` | variant | Identifies the invalid lifecycle state state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:91` |
| sym-0453fb20ed68426edc03 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidStructSize` | variant | Identifies the invalid struct size state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:83` |
| sym-a60c1c41efe27d542e16 | `pocketstation::abi::session::abi::PksSessionStatusCode::MisalignedPointer` | variant | Identifies the misaligned pointer state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:88` |
| sym-213592b134be6e286ea7 | `pocketstation::abi::session::abi::PksSessionStatusCode::NoCapacity` | variant | Identifies the no capacity state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:86` |
| sym-24c3f427ad95bdf367ed | `pocketstation::abi::session::abi::PksSessionStatusCode::NullArgument` | variant | Identifies the null argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:81` |
| sym-e36d61b3a2c4f50cb3da | `pocketstation::abi::session::abi::PksSessionStatusCode::Ok` | variant | Indicates that the operation completed successfully. | `src/abi/session/abi.rs:80` |
| sym-dc58bedf22cc975fcc48 | `pocketstation::abi::session::abi::PksSessionStatusCode::StaleHandle` | variant | Identifies the stale handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:85` |
| sym-aa0c62cf4e33186d0878 | `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMajor` | variant | Identifies the unsupported ABI major state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:82` |
| sym-8443fe45e008bbfe7ac9 | `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMinor` | variant | Identifies the unsupported ABI minor state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:96` |
| sym-cf6189ca826520e36070 | `pocketstation::abi::session::abi::PksSessionStatusCode::WouldBlock` | variant | Identifies the would block state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:92` |
| sym-d30fd6e12689f5243904 | `pocketstation::native_extension::NativeExtensionKind::Endpoint` | variant | Selects endpoint behavior for `NativeExtensionKind`. | `src/native_extension/mod.rs:30` |
| sym-f408870952814e3618ef | `pocketstation::native_extension::NativeExtensionKind::Operator` | variant | Selects operator behavior for `NativeExtensionKind`. | `src/native_extension/mod.rs:29` |
| sym-6a44610e90e03cf7c6f2 | `pocketstation::native_extension::NativeExtensionKind::Source` | variant | Selects source behavior for `NativeExtensionKind`. | `src/native_extension/mod.rs:28` |
| sym-48ee726de0c65fe82dfe | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::DuplicateRegistration` | variant | Reported when the owning operation encounters duplicate registration. | `src/native_extension/mod.rs:92` |
| sym-779580127202705588b2 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointFailed` | variant | Reported when the owning operation encounters entrypoint failed. | `src/native_extension/mod.rs:85` |
| sym-a7bc23ed348a9a40e54b | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointMissing` | variant | Reported when the owning operation encounters entrypoint missing. | `src/native_extension/mod.rs:83` |
| sym-4181805148fc0b07f174 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointPanicked` | variant | Reported when the owning operation encounters entrypoint panicked. | `src/native_extension/mod.rs:84` |
| sym-af30c25caef06e1480ff | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidLibraryDescriptor` | variant | Reported when the owning operation encounters invalid library descriptor. | `src/native_extension/mod.rs:88` |
| sym-a2c38d2ed33b40f02cbb | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidRegistration` | variant | Reported when the owning operation encounters invalid registration. | `src/native_extension/mod.rs:91` |
| sym-d58bcae00a1d215a2450 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::LibraryLoadFailed` | variant | Reported when the owning operation encounters library load failed. | `src/native_extension/mod.rs:82` |
| sym-f03edb8da6e112e5d64b | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathCanonicalizationFailed` | variant | Reported when the owning operation encounters path canonicalization failed. | `src/native_extension/mod.rs:80` |
| sym-346b3b8e2172ad39d824 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotAbsolute` | variant | Reported when the owning operation encounters path not absolute. | `src/native_extension/mod.rs:79` |
| sym-e75efacbb2da9099c475 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotFile` | variant | Reported when the owning operation encounters path not file. | `src/native_extension/mod.rs:81` |
| sym-99e7e4f05b2be8a0e4a8 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationAcquisitionFailed` | variant | Reported when the owning operation encounters registration acquisition failed. | `src/native_extension/mod.rs:90` |
| sym-9210bc775734ea4a4f13 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationAcquisitionPanicked` | variant | Reported when the owning operation encounters registration acquisition panicked. | `src/native_extension/mod.rs:89` |
| sym-bc00a333660a6f92a8f3 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationStateUnavailable` | variant | Reported when the owning operation encounters registration state unavailable. | `src/native_extension/mod.rs:93` |
| sym-7eaa4451dcde1550c1b8 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::UnsupportedAbiMajor` | variant | Reported when the owning operation encounters unsupported ABI major. | `src/native_extension/mod.rs:86` |
| sym-7df0771866f13d3e1584 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::UnsupportedAbiMinor` | variant | Reported when the owning operation encounters unsupported ABI minor. | `src/native_extension/mod.rs:87` |

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

The claims on **Native extension API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/native_extension/mod.rs:1-171` (`DIRECT`)

For **Native extension API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
