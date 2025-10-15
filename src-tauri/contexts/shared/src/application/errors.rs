// Shared Application Layer - エラー型

use crate::domain::errors::DomainError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),

    #[error("Not found: {0}")]
    NotFound(String),
}
