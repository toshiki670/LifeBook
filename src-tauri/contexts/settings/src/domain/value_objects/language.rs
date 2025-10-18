// Settings Domain Layer - Language Value Object

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

/// 言語のValue Object
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum Language {
    #[serde(rename = "ja")]
    #[strum(serialize = "ja", serialize = "japanese")]
    Japanese,
    #[serde(rename = "en")]
    #[strum(serialize = "en", serialize = "english")]
    English,
    // 将来的に追加可能
    // #[serde(rename = "zh")]
    // #[strum(serialize = "zh", serialize = "chinese")]
    // Chinese,
    // #[serde(rename = "ko")]
    // #[strum(serialize = "ko", serialize = "korean")]
    // Korean,
}

impl Language {
    /// Languageを文字列コードに変換
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Japanese => "ja",
            Self::English => "en",
        }
    }

    /// Languageを表示名に変換
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Japanese => "日本語",
            Self::English => "English",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::Japanese
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_language_from_str() {
        assert_eq!(Language::from_str("ja").unwrap(), Language::Japanese);
        assert_eq!(Language::from_str("en").unwrap(), Language::English);
        assert_eq!(Language::from_str("japanese").unwrap(), Language::Japanese);
        assert_eq!(Language::from_str("ENGLISH").unwrap(), Language::English);
    }

    #[test]
    fn test_language_from_str_invalid() {
        assert!(Language::from_str("fr").is_err());
        assert!(Language::from_str("").is_err());
    }

    #[test]
    fn test_language_as_str() {
        assert_eq!(Language::Japanese.as_str(), "ja");
        assert_eq!(Language::English.as_str(), "en");
    }

    #[test]
    fn test_language_display_name() {
        assert_eq!(Language::Japanese.display_name(), "日本語");
        assert_eq!(Language::English.display_name(), "English");
    }

    #[test]
    fn test_language_default() {
        assert_eq!(Language::default(), Language::Japanese);
    }
}
