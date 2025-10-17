// Settings Domain Layer - Language Value Object

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// 言語のValue Object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "en")]
    English,
    // 将来的に追加可能
    // #[serde(rename = "zh")]
    // Chinese,
    // #[serde(rename = "ko")]
    // Korean,
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ja" | "japanese" => Ok(Self::Japanese),
            "en" | "english" => Ok(Self::English),
            _ => Err(format!("Invalid language: '{}'. Expected 'ja' or 'en'", s)),
        }
    }
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
