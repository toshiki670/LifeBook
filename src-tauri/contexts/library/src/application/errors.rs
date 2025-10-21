// Library Application Layer - Error Types

use crate::domain::errors::DomainError;
use thiserror::Error;

/// Library コンテキスト専用のエラー型
#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
}
