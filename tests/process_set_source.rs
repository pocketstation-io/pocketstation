use pocketstation::{
    ApplicationProcessSet, ApplicationSelector, CaptureMode, ObservedProcessInstance, Platform,
    ProcessId, ProcessSetError, Session, Source, SourceKind, SourceOutputHandle, StableSourceId,
};
fn member(pid: u32, platform: Platform) -> ObservedProcessInstance {
    ObservedProcessInstance::new(
        ProcessId::new(pid),
        StableSourceId::new(platform, SourceKind::Application, "identity"),
        100,
    )
    .unwrap()
}
#[test]
fn given_invalid_members_when_declared_then_public_constructor_rejects_them() {
    for pid in [0, i32::MAX as u32 + 1] {
        assert!(ObservedProcessInstance::new(
            ProcessId::new(pid),
            StableSourceId::new(Platform::Macos, SourceKind::Application, "identity"),
            100
        )
        .is_err());
    }
    assert!(ObservedProcessInstance::new(
        ProcessId::new(1),
        StableSourceId::new(Platform::Macos, SourceKind::Application, "identity"),
        0
    )
    .is_err());
    assert!(ObservedProcessInstance::new(
        ProcessId::new(1),
        StableSourceId::new(Platform::Macos, SourceKind::InputDevice, "identity"),
        100
    )
    .is_err());
    assert!(ObservedProcessInstance::new(
        ProcessId::new(1),
        StableSourceId::new(Platform::Macos, SourceKind::Application, " "),
        100
    )
    .is_err());
}
#[test]
fn given_oversized_or_cross_platform_set_when_declared_then_scope_is_rejected() {
    let oversized = (1..=33).map(|pid| member(pid, Platform::Macos)).collect();
    assert!(matches!(
        ApplicationProcessSet::new(oversized),
        Err(ProcessSetError::TooManyMembers { .. })
    ));
    assert!(matches!(
        ApplicationProcessSet::new(vec![
            member(1, Platform::Macos),
            member(2, Platform::Windows)
        ]),
        Err(ProcessSetError::MixedPlatforms)
    ));
    assert_eq!(
        ApplicationProcessSet::new((1..=32).map(|pid| member(pid, Platform::Macos)).collect())
            .unwrap()
            .members()
            .len(),
        32
    );
}
#[test]
fn given_existing_exhaustive_public_selectors_when_consumer_compiles_then_compatibility_is_preserved(
) {
    fn selector(s: ApplicationSelector) -> u8 {
        match s {
            ApplicationSelector::BundleId(_) => 0,
            ApplicationSelector::ProcessId(_) => 1,
            ApplicationSelector::ProcessInstance { .. } => 2,
            ApplicationSelector::StableId(_) => 3,
            ApplicationSelector::Name(_) => 4,
        }
    }
    fn source(s: Source) -> u8 {
        match s {
            Source::Application(_) => 0,
            Source::SystemAudio => 1,
            Source::Microphone(_) => 2,
        }
    }
    fn mode(m: CaptureMode) -> u8 {
        match m {
            CaptureMode::SystemMix => 0,
            CaptureMode::Application(_) => 1,
            CaptureMode::Process(_) => 2,
            CaptureMode::ExactApplication { .. } => 3,
            CaptureMode::ExactApplicationStable { .. } => 4,
            CaptureMode::InputDevice(_) => 5,
        }
    }
    assert_eq!(selector(ApplicationSelector::name("app")), 4);
    assert_eq!(source(Source::system_audio()), 1);
    assert_eq!(mode(CaptureMode::SystemMix), 0);
    let method: fn(&Session, ApplicationProcessSet) -> Result<SourceOutputHandle, ProcessSetError> =
        Session::capture_process_set;
    let _ = method;
}
