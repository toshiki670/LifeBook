// Shared Domain Layer - エラー型

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}
