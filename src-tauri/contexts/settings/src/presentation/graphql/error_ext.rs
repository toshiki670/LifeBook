// Presentation Layer - GraphQL Error Extensions

use crate::application::errors::SettingsError;
use async_graphql::{Error, ErrorExtensions};

/// SettingsErrorをGraphQLエラーに変換（エラーコード付き）
pub fn to_graphql_error(e: SettingsError) -> Error {
    match e {
        SettingsError::InvalidLanguage(msg) => Error::new(msg).extend_with(|_, ext| {
            ext.set("code", "INVALID_LANGUAGE");
        }),
        SettingsError::InvalidTheme(msg) => Error::new(msg).extend_with(|_, ext| {
            ext.set("code", "INVALID_THEME");
        }),
        SettingsError::InvalidDatabaseDirectory(msg) => {
            Error::new(msg).extend_with(|_, ext| {
                ext.set("code", "INVALID_DATABASE_DIRECTORY");
            })
        }
        SettingsError::ParseError(e) => {
            Error::new(format!("Parse error: {}", e)).extend_with(|_, ext| {
                ext.set("code", "PARSE_ERROR");
            })
        }
        SettingsError::Repository(e) => {
            Error::new(format!("Repository error: {}", e)).extend_with(|_, ext| {
                ext.set("code", "REPOSITORY_ERROR");
            })
        }
    }
}

