// Settings Domain Layer - Language Value Object

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

/// 言語のValue Object
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Language {
    #[strum(serialize = "ja", serialize = "japanese")]
    Japanese,
    #[strum(serialize = "en", serialize = "english")]
    English,
    // 将来的に追加可能
    // #[strum(serialize = "zh", serialize = "chinese")]
    // Chinese,
    // #[strum(serialize = "ko", serialize = "korean")]
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
        assert_eq!(Language::from_str("japanese").unwrap(), Language::Japanese);
        assert_eq!(Language::from_str("ENGLISH").unwrap(), Language::English);
    }

    #[test]
    fn test_language_from_str_invalid() {
        assert!(Language::from_str("fr").is_err());
        assert!(Language::from_str("").is_err());
    }

    #[test]
    fn test_language_as_ref() {
        // AsRefStr は variant名の小文字を返す
        assert_eq!(Language::Japanese.as_ref(), "japanese");
        assert_eq!(Language::English.as_ref(), "english");
    }

    #[test]
    fn test_language_default() {
        assert_eq!(Language::default(), Language::Japanese);
    }
}
