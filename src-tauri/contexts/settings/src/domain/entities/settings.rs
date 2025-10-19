// Settings Domain Layer - App Settings Entity

use super::{AppearanceSettings, DatabaseSettings, GeneralSettings};
use serde::{Deserialize, Serialize};

/// アプリケーション全体の設定
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub general: GeneralSettings,
    pub appearance: AppearanceSettings,
    pub database: DatabaseSettings,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Language, Theme};
    use std::path::PathBuf;

    #[test]
    fn test_app_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.general.language, Language::Japanese);
        assert_eq!(settings.appearance.theme, Theme::System);
        assert_eq!(settings.database.database_directory, PathBuf::new());
    }

    #[test]
    fn test_settings_serialization() {
        let settings = Settings::default();
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"language\":\"ja\""));
        assert!(json.contains("\"theme\":\"system\""));
        assert!(json.contains("\"database_directory\":\"\""));
    }

    #[test]
    fn test_settings_deserialization() {
        let json = r#"{
            "general": {"language": "en"},
            "appearance": {"theme": "dark"},
            "database": {"database_directory": "/custom/path"}
        }"#;
        let settings: Settings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.general.language, Language::English);
        assert_eq!(settings.appearance.theme, Theme::Dark);
        assert_eq!(
            settings.database.database_directory,
            PathBuf::from("/custom/path")
        );
    }
}
