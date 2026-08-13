//! Host platform identity; not part of audio-buffer ownership.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Macos,
    Windows,
    Linux,
    Ios,
    Android,
    Web,
    Unknown,
}
