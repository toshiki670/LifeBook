// Settings Domain Layer - Domain Error Types

use thiserror::Error;

/// Settings ドメイン層のエラー型
#[derive(Error, Debug, Clone)]
pub enum SettingsDomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Settings file corrupted: {0}")]
    SettingsFileCorrupted(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

