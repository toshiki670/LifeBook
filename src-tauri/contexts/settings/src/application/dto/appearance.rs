// Settings Application Layer - Appearance Settings DTO

use crate::domain::entities::AppearanceSettings;
use async_graphql::SimpleObject;

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
