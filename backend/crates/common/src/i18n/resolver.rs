//! I18n message resolver using Fluent.

use std::collections::HashMap;
use std::sync::Arc;

use fluent_bundle::bundle::FluentBundle as FluentBundleGeneric;
use fluent_bundle::{FluentResource, FluentValue};
use intl_memoizer::concurrent::IntlLangMemoizer;
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

use super::{fluent_resources, MessageKey, SUPPORTED_LOCALES};

/// Concurrent FluentBundle type alias.
type FluentBundle = FluentBundleGeneric<FluentResource, IntlLangMemoizer>;

/// Supported locales enum for type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Locale {
    #[default]
    En,
    Sk,
    Cs,
    De,
}

impl Locale {
    /// Parse locale from string, with fallback to default.
    pub fn from_str_or_default(s: &str) -> Self {
        Self::from_str_opt(s).unwrap_or_default()
    }

    /// Try to parse locale from string.
    pub fn from_str_opt(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        // Handle full locale codes like "sk-SK", "cs-CZ", etc.
        let lang = s.split('-').next().unwrap_or(&s);
        let lang = lang.split('_').next().unwrap_or(lang);

        match lang {
            "en" => Some(Self::En),
            "sk" => Some(Self::Sk),
            "cs" => Some(Self::Cs),
            "de" => Some(Self::De),
            _ => None,
        }
    }

    /// Get the language identifier for this locale.
    pub fn language_id(&self) -> LanguageIdentifier {
        match self {
            Self::En => "en".parse().unwrap(),
            Self::Sk => "sk".parse().unwrap(),
            Self::Cs => "cs".parse().unwrap(),
            Self::De => "de".parse().unwrap(),
        }
    }

    /// Get the locale code as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Sk => "sk",
            Self::Cs => "cs",
            Self::De => "de",
        }
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Locale {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_opt(s).ok_or(())
    }
}

/// Thread-safe i18n message resolver.
///
/// Holds precompiled Fluent bundles for all supported locales.
#[derive(Clone)]
pub struct I18nResolver {
    bundles: Arc<HashMap<Locale, FluentBundle>>,
}

impl Default for I18nResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl I18nResolver {
    /// Create a new resolver with all locale bundles loaded.
    pub fn new() -> Self {
        let mut bundles = HashMap::new();

        // Load each locale bundle
        for locale_str in SUPPORTED_LOCALES {
            let locale = Locale::from_str_opt(locale_str).unwrap_or_default();
            let resource_str = match locale {
                Locale::En => fluent_resources::EN,
                Locale::Sk => fluent_resources::SK,
                Locale::Cs => fluent_resources::CS,
                Locale::De => fluent_resources::DE,
            };

            if let Some(bundle) = Self::create_bundle(locale, resource_str) {
                bundles.insert(locale, bundle);
            }
        }

        Self {
            bundles: Arc::new(bundles),
        }
    }

    /// Create a Fluent bundle for a locale.
    fn create_bundle(locale: Locale, resource_str: &str) -> Option<FluentBundle> {
        let resource = FluentResource::try_new(resource_str.to_string()).ok()?;
        let lang_id = locale.language_id();

        let mut bundle = FluentBundleGeneric::new_concurrent(vec![lang_id]);
        bundle.add_resource(resource).ok()?;

        Some(bundle)
    }

    /// Get a localized message for the given key and locale.
    ///
    /// Falls back to English if the message is not found in the requested locale.
    pub fn get(&self, locale: Locale, key: MessageKey) -> String {
        self.get_with_args(locale, key, &[])
    }

    /// Get a localized message with arguments.
    ///
    /// Arguments are passed as `(name, value)` pairs for Fluent placeholders.
    pub fn get_with_args(&self, locale: Locale, key: MessageKey, args: &[(&str, &str)]) -> String {
        let fluent_id = key.as_fluent_id();

        // Try requested locale first
        if let Some(msg) = self.format_message(locale, fluent_id, args) {
            return msg;
        }

        // Fall back to English
        if locale != Locale::En {
            if let Some(msg) = self.format_message(Locale::En, fluent_id, args) {
                return msg;
            }
        }

        // Last resort: return the message key
        fluent_id.to_string()
    }

    /// Format a message from a bundle.
    fn format_message(&self, locale: Locale, id: &str, args: &[(&str, &str)]) -> Option<String> {
        let bundle = self.bundles.get(&locale)?;
        let message = bundle.get_message(id)?;
        let pattern = message.value()?;

        let mut fluent_args = fluent_bundle::FluentArgs::new();
        for (name, value) in args {
            fluent_args.set(*name, FluentValue::from(*value));
        }

        let mut errors = vec![];
        let result = bundle.format_pattern(pattern, Some(&fluent_args), &mut errors);

        Some(result.to_string())
    }

    /// Get a message for the default locale.
    pub fn get_default(&self, key: MessageKey) -> String {
        self.get(Locale::default(), key)
    }

    /// Parse Accept-Language header and get the best matching locale.
    pub fn parse_accept_language(&self, header: &str) -> Locale {
        // Simple parsing: take the first supported locale from the header
        for part in header.split(',') {
            let lang = part.split(';').next().unwrap_or(part).trim();
            if let Some(locale) = Locale::from_str_opt(lang) {
                return locale;
            }
        }
        Locale::default()
    }
}

impl std::fmt::Debug for I18nResolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("I18nResolver")
            .field("locales", &SUPPORTED_LOCALES)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_parsing() {
        assert_eq!(Locale::from_str_opt("en"), Some(Locale::En));
        assert_eq!(Locale::from_str_opt("sk-SK"), Some(Locale::Sk));
        assert_eq!(Locale::from_str_opt("cs_CZ"), Some(Locale::Cs));
        assert_eq!(Locale::from_str_opt("DE"), Some(Locale::De));
        assert_eq!(Locale::from_str_opt("fr"), None);
    }

    #[test]
    fn test_locale_default() {
        assert_eq!(Locale::default(), Locale::En);
    }

    #[test]
    fn test_resolver_get_message() {
        let resolver = I18nResolver::new();

        // English should always work
        let msg = resolver.get(Locale::En, MessageKey::ErrorGeneric);
        assert!(!msg.is_empty());
        assert_ne!(msg, "error-generic"); // Should be resolved, not the key

        // Slovak should work
        let msg_sk = resolver.get(Locale::Sk, MessageKey::ErrorGeneric);
        assert!(!msg_sk.is_empty());
    }

    #[test]
    fn test_accept_language_parsing() {
        let resolver = I18nResolver::new();

        assert_eq!(
            resolver.parse_accept_language("sk-SK,sk;q=0.9,en;q=0.8"),
            Locale::Sk
        );
        assert_eq!(resolver.parse_accept_language("en-US,en;q=0.9"), Locale::En);
        assert_eq!(resolver.parse_accept_language("fr-FR,de;q=0.9"), Locale::De);
        assert_eq!(resolver.parse_accept_language("fr-FR"), Locale::En); // Fallback
    }
}
