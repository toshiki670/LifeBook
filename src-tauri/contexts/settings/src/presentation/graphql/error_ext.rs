// Presentation Layer - GraphQL Error Extensions

use crate::application::errors::ApplicationError;
use async_graphql::{Error, ErrorExtensions};

/// ApplicationErrorをGraphQLエラーに変換（エラーコード付き）
pub fn to_graphql_error(e: ApplicationError) -> Error {
    match e {
        ApplicationError::InvalidLanguage(msg) => Error::new(msg).extend_with(|_, ext| {
            ext.set("code", "INVALID_LANGUAGE");
        }),
        ApplicationError::InvalidTheme(msg) => Error::new(msg).extend_with(|_, ext| {
            ext.set("code", "INVALID_THEME");
        }),
        ApplicationError::InvalidDatabaseDirectory(msg) => Error::new(msg).extend_with(|_, ext| {
            ext.set("code", "INVALID_DATABASE_DIRECTORY");
        }),
        ApplicationError::Domain(e) => {
            Error::new(format!("Domain error: {}", e)).extend_with(|_, ext| {
                ext.set("code", "DOMAIN_ERROR");
            })
        }
    }
}
