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
    #[strum(serialize = "ja")]
    Japanese,
    #[serde(rename = "en")]
    #[strum(serialize = "en")]
    English,
    // 将来的に追加可能
    // #[serde(rename = "zh")]
    // #[strum(serialize = "zh")]
    // Chinese,
    // #[serde(rename = "ko")]
    // #[strum(serialize = "ko")]
    // Korean,
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
        assert_eq!(Language::from_str("JA").unwrap(), Language::Japanese); // 大文字小文字を区別しない
        assert_eq!(Language::from_str("EN").unwrap(), Language::English);
    }

    #[test]
    fn test_language_from_str_invalid() {
        assert!(Language::from_str("fr").is_err());
        assert!(Language::from_str("").is_err());
        assert!(Language::from_str("japanese").is_err()); // 長い形式は受け付けない
        assert!(Language::from_str("english").is_err());
    }

    #[test]
    fn test_language_as_ref() {
        // AsRefStr は serialize 値を返す
        assert_eq!(Language::Japanese.as_ref(), "ja");
        assert_eq!(Language::English.as_ref(), "en");
    }

    #[test]
    fn test_language_default() {
        assert_eq!(Language::default(), Language::Japanese);
    }
}
