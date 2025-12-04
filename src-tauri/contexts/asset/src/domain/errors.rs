// Asset Context - Domain Errors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Cannot set self as parent")]
    CannotSetSelfAsParent,

    #[error("Cannot create circular reference")]
    CircularReference,

    #[error("Insufficient quantity for operation")]
    InsufficientQuantity,

    #[error("Cannot delete: {0}")]
    CannotDelete(String),

    #[error("Arithmetic overflow or underflow")]
    ArithmeticError,
}
