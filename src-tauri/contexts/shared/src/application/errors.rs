// Shared Application Layer - エラー型

use crate::domain::errors::DomainError;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    DomainError(DomainError),
    NotFound(String),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::DomainError(e) => write!(f, "Domain error: {}", e),
            ApplicationError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for ApplicationError {}

impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        ApplicationError::DomainError(error)
    }
}
