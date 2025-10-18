// Settings Application Layer - Settings DTO

use crate::domain::entities::{AppearanceSettings, DatabaseSettings, GeneralSettings};
use async_graphql::SimpleObject;

/// 一般設定のDTO
#[derive(Debug, Clone, SimpleObject)]
pub struct GeneralSettingsDto {
    pub language: String,
}

impl From<GeneralSettings> for GeneralSettingsDto {
    fn from(settings: GeneralSettings) -> Self {
        Self {
            language: settings.language.to_string(),
        }
    }
}

/// 表示設定のDTO
#[derive(Debug, Clone, SimpleObject)]
pub struct AppearanceSettingsDto {
    pub theme: String,
}

impl From<AppearanceSettings> for AppearanceSettingsDto {
    fn from(settings: AppearanceSettings) -> Self {
        Self {
            theme: settings.theme.to_string(),
        }
    }
}

/// データベース設定のDTO
#[derive(Debug, Clone, SimpleObject)]
pub struct DatabaseSettingsDto {
    pub database_directory: String,
}

impl From<DatabaseSettings> for DatabaseSettingsDto {
    fn from(settings: DatabaseSettings) -> Self {
        Self {
            database_directory: settings.database_directory.to_string_lossy().to_string(),
        }
    }
}
