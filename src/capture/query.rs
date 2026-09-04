//! Control-plane source discovery queries used by the first-party CLI.

#[cfg(feature = "native-capture-runtime")]
use std::collections::HashSet;

#[cfg(feature = "native-capture-runtime")]
use super::StableSourceId;
use super::{CaptureSource, SourceKind, SourceState};
#[cfg(feature = "native-capture-runtime")]
use crate::frame::Platform;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceQuery {
    Any,
    App(String),
    ByKind(SourceKind),
    ByStableKey(String),
    Playing,
}

impl SourceQuery {
    pub fn matches(&self, source: &CaptureSource) -> bool {
        match self {
            Self::Any => true,
            Self::App(name) => {
                let needle = name.to_lowercase();
                source
                    .app_id
                    .as_deref()
                    .is_some_and(|application| application.to_lowercase().contains(&needle))
                    || source.name.to_lowercase().contains(&needle)
            }
            Self::ByKind(kind) => source.stable_id.kind == *kind,
            Self::ByStableKey(key) => &source.stable_id.stable_key == key,
            Self::Playing => source.state == SourceState::Playing,
        }
    }
}

pub fn resolve_query(query: &SourceQuery, sources: &[CaptureSource]) -> Vec<CaptureSource> {
    sources
        .iter()
        .filter(|source| query.matches(source))
        .cloned()
        .collect()
}

pub trait SourceProvider {
    fn discover(&self, query: &SourceQuery) -> Vec<CaptureSource>;
}

pub struct LocalSourceProvider;

impl SourceProvider for LocalSourceProvider {
    fn discover(&self, query: &SourceQuery) -> Vec<CaptureSource> {
        resolve_query(query, &discover_sources())
    }
}

/// Reports whether this host exposes the native application-capture facility.
///
/// This is a control-plane capability query. It does not open a device, prompt
/// for permission, or create a capture/runtime owner.
pub fn application_capture_available() -> bool {
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    {
        super::platform::macos::tap_available()
    }
    #[cfg(any(
        all(target_os = "windows", feature = "wasapi-capture"),
        all(target_os = "linux", feature = "pipewire-capture")
    ))]
    {
        true
    }
    #[cfg(not(any(
        all(target_os = "macos", feature = "coreaudio-capture"),
        all(target_os = "windows", feature = "wasapi-capture"),
        all(target_os = "linux", feature = "pipewire-capture")
    )))]
    {
        false
    }
}

pub fn discover_sources() -> Vec<CaptureSource> {
    #[cfg(not(any(
        all(target_os = "macos", feature = "coreaudio-capture"),
        all(target_os = "windows", feature = "wasapi-capture"),
        all(
            target_os = "linux",
            any(feature = "pipewire-capture", feature = "alsa-fallback")
        )
    )))]
    return Vec::new();

    #[cfg(any(
        all(target_os = "macos", feature = "coreaudio-capture"),
        all(target_os = "windows", feature = "wasapi-capture"),
        all(
            target_os = "linux",
            any(feature = "pipewire-capture", feature = "alsa-fallback")
        )
    ))]
    {
        #[cfg(target_os = "macos")]
        let platform = Platform::Macos;
        #[cfg(target_os = "windows")]
        let platform = Platform::Windows;
        #[cfg(target_os = "linux")]
        let platform = Platform::Linux;
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        let platform = Platform::Unknown;

        let mut sources = vec![CaptureSource {
            stable_id: StableSourceId::new(platform, SourceKind::SystemMix, "system:mix"),
            name: "System Mix".to_owned(),
            process_id: None,
            app_id: None,
            device_uid: None,
            state: SourceState::Available,
            sample_rate_hz: 48_000,
            channels: 2,
        }];

        #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
        {
            sources.extend(super::platform::macos::discover_input_sources_native());
            sources.extend(super::platform::macos::discover_sources_native());
        }

        #[cfg(all(target_os = "windows", feature = "wasapi-capture"))]
        sources.extend(super::platform::windows::discover_sources_windows());

        #[cfg(all(
            target_os = "linux",
            any(feature = "pipewire-capture", feature = "alsa-fallback")
        ))]
        sources.extend(super::platform::linux::discover_sources_linux());

        let mut seen = HashSet::new();
        sources
            .into_iter()
            .filter(|source| {
                seen.insert((
                    source.stable_id.platform,
                    source.stable_id.kind,
                    source.stable_id.stable_key.clone(),
                    source.process_id,
                ))
            })
            .collect()
    }
}
