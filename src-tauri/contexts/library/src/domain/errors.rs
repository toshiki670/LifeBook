// Library Domain Layer - Domain Error Types

use thiserror::Error;

/// Library ドメイン層のエラー型
#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("I/O error: {0}")]
    IoError(String),
}
