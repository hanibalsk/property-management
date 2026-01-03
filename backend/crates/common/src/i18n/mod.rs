//! Internationalization (i18n) support for the backend.
//!
//! This module provides localization capabilities using Fluent for message formatting.
//! Supports English (en), Slovak (sk), Czech (cs), German (de), Polish (pl), and Hungarian (hu).

mod messages;
mod resolver;

pub use messages::MessageKey;
pub use resolver::{I18nResolver, Locale};

/// Supported locales for the backend.
pub const SUPPORTED_LOCALES: &[&str] = &["en", "sk", "cs", "de", "pl", "hu"];

/// Default locale when no preference is specified.
pub const DEFAULT_LOCALE: &str = "en";

/// Fluent message files embedded at compile time.
pub mod fluent_resources {
    pub const EN: &str = include_str!("locales/en.ftl");
    pub const SK: &str = include_str!("locales/sk.ftl");
    pub const CS: &str = include_str!("locales/cs.ftl");
    pub const DE: &str = include_str!("locales/de.ftl");
    pub const PL: &str = include_str!("locales/pl.ftl");
    pub const HU: &str = include_str!("locales/hu.ftl");
}
