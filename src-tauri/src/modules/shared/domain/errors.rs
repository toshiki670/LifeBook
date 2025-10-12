// Shared Domain Layer - エラー型

use std::fmt;

#[derive(Debug, Clone)]
pub enum DomainError {
    ValidationError(String),
    NotFound(String),
    InvalidState(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DomainError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DomainError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DomainError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}
