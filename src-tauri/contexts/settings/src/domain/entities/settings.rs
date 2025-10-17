// Settings Domain Layer - Settings Entities

use crate::domain::value_objects::{Language, Theme};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// アプリケーション全体の設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub appearance: AppearanceSettings,
    pub database: DatabaseSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            appearance: AppearanceSettings::default(),
            database: DatabaseSettings::default(),
        }
    }
}

/// 一般設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub language: Language,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            language: Language::default(),
        }
    }
}

/// 表示設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: Theme,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

/// データベース設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    pub database_directory: PathBuf,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            // デフォルトは相対パス（実際の実装では app_local_data_dir を使う）
            database_directory: PathBuf::from("./data"),
        }
    }
}

// PathBufのシリアライズヘルパー
fn serialize_pathbuf<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&path.to_string_lossy())
}

// PathBufのデシリアライズヘルパー
fn deserialize_pathbuf<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(PathBuf::from(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.general.language, Language::Japanese);
        assert_eq!(settings.appearance.theme, Theme::System);
        assert_eq!(
            settings.database.database_directory,
            PathBuf::from("./data")
        );
    }

    #[test]
    fn test_settings_serialization() {
        let settings = AppSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"language\":\"ja\""));
        assert!(json.contains("\"theme\":\"system\""));
        assert!(json.contains("\"database_directory\":\"./data\""));
    }

    #[test]
    fn test_settings_deserialization() {
        let json = r#"{
            "general": {"language": "en"},
            "appearance": {"theme": "dark"},
            "database": {"database_directory": "/custom/path"}
        }"#;
        let settings: AppSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.general.language, Language::English);
        assert_eq!(settings.appearance.theme, Theme::Dark);
        assert_eq!(
            settings.database.database_directory,
            PathBuf::from("/custom/path")
        );
    }
}

