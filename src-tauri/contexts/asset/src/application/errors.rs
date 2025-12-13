// Asset Context - Application Errors

use crate::domain::errors::DomainError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Cannot delete: {0}")]
    CannotDelete(String),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Database error: {0}")]
    DatabaseError(String),
}
