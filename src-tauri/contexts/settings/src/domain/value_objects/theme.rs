// Settings Domain Layer - Theme Value Object

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

/// テーマのValue Object
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, AsRefStr, Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    /// Themeを文字列に変換
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::System
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_theme_from_str() {
        assert_eq!(Theme::from_str("light").unwrap(), Theme::Light);
        assert_eq!(Theme::from_str("dark").unwrap(), Theme::Dark);
        assert_eq!(Theme::from_str("system").unwrap(), Theme::System);
        assert_eq!(Theme::from_str("LIGHT").unwrap(), Theme::Light); // 大文字小文字を区別しない
    }

    #[test]
    fn test_theme_from_str_invalid() {
        assert!(Theme::from_str("invalid").is_err());
        assert!(Theme::from_str("").is_err());
    }

    #[test]
    fn test_theme_as_str() {
        assert_eq!(Theme::Light.as_str(), "light");
        assert_eq!(Theme::Dark.as_str(), "dark");
        assert_eq!(Theme::System.as_str(), "system");
    }

    #[test]
    fn test_theme_default() {
        assert_eq!(Theme::default(), Theme::System);
    }
}
