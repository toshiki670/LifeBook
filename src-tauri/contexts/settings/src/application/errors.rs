// Settings Application Layer - Error Types

use crate::domain::errors::SettingsDomainError;
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

    #[error("Domain error: {0}")]
    Domain(#[from] SettingsDomainError),
}
