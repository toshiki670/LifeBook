// Presentation Layer - GraphQL Error Extensions

use crate::application::errors::ApplicationError;
use crate::domain::errors::DomainError;
use async_graphql::{Error, ErrorExtensions};

/// ApplicationErrorをGraphQLエラーに変換（エラーコード付き）
pub fn to_graphql_error(e: ApplicationError) -> Error {
    match e {
        ApplicationError::NotFound(msg) => Error::new(msg).extend_with(|_, ext| {
            ext.set("code", "NOT_FOUND");
        }),
        ApplicationError::Domain(domain_err) => match domain_err {
            DomainError::ValidationError(msg) => Error::new(msg).extend_with(|_, ext| {
                ext.set("code", "VALIDATION_ERROR");
            }),
            DomainError::NotFound(msg) => Error::new(msg).extend_with(|_, ext| {
                ext.set("code", "NOT_FOUND");
            }),
            DomainError::InvalidState(msg) => Error::new(msg).extend_with(|_, ext| {
                ext.set("code", "INVALID_STATE");
            }),
            DomainError::IoError(msg) => Error::new(msg).extend_with(|_, ext| {
                ext.set("code", "IO_ERROR");
            }),
        },
    }
}
