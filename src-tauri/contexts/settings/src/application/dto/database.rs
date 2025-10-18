// Settings Application Layer - Database Settings DTO

use crate::domain::entities::DatabaseSettings;
use async_graphql::SimpleObject;

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
