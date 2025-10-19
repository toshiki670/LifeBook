// Presentation Layer - Settings GraphQL Mutation

use crate::{
    application::{
        dto::{AppearanceSettingsDto, DatabaseSettingsDto, GeneralSettingsDto},
        services::SettingsService,
    },
    presentation::graphql::to_graphql_error,
};
use async_graphql::*;
use std::sync::Arc;

#[derive(Default)]
pub struct SettingsMutation;

#[Object]
impl SettingsMutation {
    /// 一般設定を更新
    async fn update_general_settings(
        &self,
        ctx: &Context<'_>,
        language: Option<String>,
    ) -> Result<GeneralSettingsDto> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .update_general_settings(language)
            .await
            .map_err(to_graphql_error)
    }

    /// 表示設定を更新
    async fn update_appearance_settings(
        &self,
        ctx: &Context<'_>,
        theme: Option<String>,
    ) -> Result<AppearanceSettingsDto> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .update_appearance_settings(theme)
            .await
            .map_err(to_graphql_error)
    }

    /// データベース設定を更新
    async fn update_database_settings(
        &self,
        ctx: &Context<'_>,
        database_directory: Option<String>,
    ) -> Result<DatabaseSettingsDto> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .update_database_settings(database_directory)
            .await
            .map_err(to_graphql_error)
    }

    /// すべての設定をリセット
    async fn reset_settings(&self, ctx: &Context<'_>) -> Result<bool> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .reset_all_settings()
            .await
            .map_err(to_graphql_error)?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::graphql::queries::SettingsQuery;
    use crate::presentation::integration::build_settings_service;
    use async_graphql::{EmptySubscription, Schema};
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_graphql_mutation_update_general_settings() {
        let temp_dir = TempDir::new().unwrap();
        let service = build_settings_service(
            temp_dir.path().to_path_buf(),
            PathBuf::from("/default/db"),
        );

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let mutation = r#"
            mutation {
                updateGeneralSettings(language: "en") {
                    language
                }
            }
        "#;

        let response = schema.execute(mutation).await;
        assert!(response.errors.is_empty(), "GraphQL errors: {:?}", response.errors);
        
        let data = response.data.into_json().unwrap();
        assert_eq!(data["updateGeneralSettings"]["language"], "en");
    }

    #[tokio::test]
    async fn test_graphql_mutation_update_appearance_settings() {
        let temp_dir = TempDir::new().unwrap();
        let service = build_settings_service(
            temp_dir.path().to_path_buf(),
            PathBuf::from("/default/db"),
        );

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let mutation = r#"
            mutation {
                updateAppearanceSettings(theme: "dark") {
                    theme
                }
            }
        "#;

        let response = schema.execute(mutation).await;
        assert!(response.errors.is_empty());
        
        let data = response.data.into_json().unwrap();
        assert_eq!(data["updateAppearanceSettings"]["theme"], "dark");
    }

    #[tokio::test]
    async fn test_graphql_mutation_update_database_settings() {
        let temp_dir = TempDir::new().unwrap();
        let valid_path = temp_dir.path().join("my_databases");
        let service = build_settings_service(
            temp_dir.path().to_path_buf(),
            PathBuf::from("/default/db"),
        );

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let mutation = format!(
            r#"
            mutation {{
                updateDatabaseSettings(databaseDirectory: "{}") {{
                    databaseDirectory
                }}
            }}
            "#,
            valid_path.to_str().unwrap()
        );

        let response = schema.execute(&mutation).await;
        assert!(response.errors.is_empty());
        
        let data = response.data.into_json().unwrap();
        assert_eq!(
            data["updateDatabaseSettings"]["databaseDirectory"],
            valid_path.to_str().unwrap()
        );
    }

    #[tokio::test]
    async fn test_graphql_mutation_with_invalid_language() {
        let temp_dir = TempDir::new().unwrap();
        let service = build_settings_service(
            temp_dir.path().to_path_buf(),
            PathBuf::from("/default/db"),
        );

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let mutation = r#"
            mutation {
                updateGeneralSettings(language: "invalid") {
                    language
                }
            }
        "#;

        let response = schema.execute(mutation).await;
        assert!(!response.errors.is_empty());
        
        // エラーコードの確認
        let error = &response.errors[0];
        if let Some(ext) = &error.extensions {
            if let Some(code_value) = ext.get("code") {
                assert_eq!(code_value.to_string(), r#""INVALID_LANGUAGE""#);
            } else {
                panic!("Error code not found in extensions");
            }
        } else {
            panic!("Extensions not found in error");
        }
    }

    #[tokio::test]
    async fn test_graphql_mutation_with_invalid_theme() {
        let temp_dir = TempDir::new().unwrap();
        let service = build_settings_service(
            temp_dir.path().to_path_buf(),
            PathBuf::from("/default/db"),
        );

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let mutation = r#"
            mutation {
                updateAppearanceSettings(theme: "invalid") {
                    theme
                }
            }
        "#;

        let response = schema.execute(mutation).await;
        assert!(!response.errors.is_empty());
        
        // エラーコードの確認
        let error = &response.errors[0];
        if let Some(ext) = &error.extensions {
            if let Some(code_value) = ext.get("code") {
                assert_eq!(code_value.to_string(), r#""INVALID_THEME""#);
            } else {
                panic!("Error code not found in extensions");
            }
        } else {
            panic!("Extensions not found in error");
        }
    }

    #[tokio::test]
    async fn test_graphql_mutation_with_invalid_path() {
        let temp_dir = TempDir::new().unwrap();
        let service = build_settings_service(
            temp_dir.path().to_path_buf(),
            PathBuf::from("/default/db"),
        );

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let mutation = r#"
            mutation {
                updateDatabaseSettings(databaseDirectory: "./relative/path") {
                    databaseDirectory
                }
            }
        "#;

        let response = schema.execute(mutation).await;
        assert!(!response.errors.is_empty());
        
        // エラーコードの確認
        let error = &response.errors[0];
        if let Some(ext) = &error.extensions {
            if let Some(code_value) = ext.get("code") {
                assert_eq!(code_value.to_string(), r#""INVALID_DATABASE_DIRECTORY""#);
            } else {
                panic!("Error code not found in extensions");
            }
        } else {
            panic!("Extensions not found in error");
        }
    }

    #[tokio::test]
    async fn test_graphql_mutation_reset_settings() {
        let temp_dir = TempDir::new().unwrap();
        let service = build_settings_service(
            temp_dir.path().to_path_buf(),
            PathBuf::from("/default/db"),
        );

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        // 設定を変更
        let mutation1 = r#"
            mutation {
                updateGeneralSettings(language: "en") {
                    language
                }
            }
        "#;
        schema.execute(mutation1).await;

        // リセット
        let mutation2 = r#"
            mutation {
                resetSettings
            }
        "#;
        let response = schema.execute(mutation2).await;
        assert!(response.errors.is_empty());
        
        let data = response.data.into_json().unwrap();
        assert_eq!(data["resetSettings"], true);
    }
}
