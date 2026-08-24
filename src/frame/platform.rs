//! Host platform identity; not part of audio-buffer ownership.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Identifies the operating-system platform attached to captured lineage."]
pub enum Platform {
    #[doc = "Represents the macos case of `Platform`."]
    Macos,
    #[doc = "Represents the windows case of `Platform`."]
    Windows,
    #[doc = "Represents the linux case of `Platform`."]
    Linux,
    #[doc = "Represents the ios case of `Platform`."]
    Ios,
    #[doc = "Represents the android case of `Platform`."]
    Android,
    #[doc = "Represents the web case of `Platform`."]
    Web,
    #[doc = "Represents the unknown case of `Platform`."]
    Unknown,
}
