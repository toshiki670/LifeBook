// Settings Application Layer - Error Types

use shared::domain::errors::DomainError;
use thiserror::Error;

/// Settings コンテキスト専用のエラー型
#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Invalid language code: {0}")]
    InvalidLanguage(String),

    #[error("Invalid theme: {0}")]
    InvalidTheme(String),

    #[error("Invalid database directory: {0}")]
    InvalidDatabaseDirectory(String),

    #[error("Failed to parse settings file: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Repository error: {0}")]
    Repository(#[from] DomainError),
}

// strumのParseErrorからの変換
impl From<strum::ParseError> for SettingsError {
    fn from(e: strum::ParseError) -> Self {
        SettingsError::InvalidLanguage(e.to_string())
    }
}
