// Settings Application Layer - General Settings DTO

use crate::domain::entities::GeneralSettings;
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

