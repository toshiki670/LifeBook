// Presentation Layer - Settings GraphQL Query

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
pub struct SettingsQuery;

#[Object]
impl SettingsQuery {
    /// 一般設定を取得
    async fn general_settings(&self, ctx: &Context<'_>) -> Result<GeneralSettingsDto> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .get_general_settings()
            .await
            .map_err(to_graphql_error)
    }

    /// 表示設定を取得
    async fn appearance_settings(&self, ctx: &Context<'_>) -> Result<AppearanceSettingsDto> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .get_appearance_settings()
            .await
            .map_err(to_graphql_error)
    }

    /// データベース設定を取得
    async fn database_settings(&self, ctx: &Context<'_>) -> Result<DatabaseSettingsDto> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .get_database_settings()
            .await
            .map_err(to_graphql_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::graphql::mutations::SettingsMutation;
    use crate::presentation::integration::build_settings_service;
    use async_graphql::{EmptySubscription, Schema};
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_graphql_query_general_settings() {
        let temp_dir = TempDir::new().unwrap();
        let service =
            build_settings_service(temp_dir.path().to_path_buf(), PathBuf::from("/default/db"));

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let query = r#"
            query {
                generalSettings {
                    language
                }
            }
        "#;

        let response = schema.execute(query).await;
        assert!(
            response.errors.is_empty(),
            "GraphQL errors: {:?}",
            response.errors
        );

        let data = response.data.into_json().unwrap();
        assert!(data["generalSettings"]["language"].is_string());
        assert_eq!(data["generalSettings"]["language"], "ja");
    }

    #[tokio::test]
    async fn test_graphql_query_appearance_settings() {
        let temp_dir = TempDir::new().unwrap();
        let service =
            build_settings_service(temp_dir.path().to_path_buf(), PathBuf::from("/default/db"));

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let query = r#"
            query {
                appearanceSettings {
                    theme
                }
            }
        "#;

        let response = schema.execute(query).await;
        assert!(response.errors.is_empty());

        let data = response.data.into_json().unwrap();
        assert_eq!(data["appearanceSettings"]["theme"], "system");
    }

    #[tokio::test]
    async fn test_graphql_query_database_settings() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let service = build_settings_service(temp_dir.path().to_path_buf(), default_db_dir.clone());

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let query = r#"
            query {
                databaseSettings {
                    databaseDirectory
                }
            }
        "#;

        let response = schema.execute(query).await;
        assert!(response.errors.is_empty());

        let data = response.data.into_json().unwrap();
        assert_eq!(
            data["databaseSettings"]["databaseDirectory"],
            default_db_dir.to_str().unwrap()
        );
    }

    #[tokio::test]
    async fn test_graphql_query_all_sections() {
        let temp_dir = TempDir::new().unwrap();
        let service =
            build_settings_service(temp_dir.path().to_path_buf(), PathBuf::from("/default/db"));

        let schema = Schema::build(SettingsQuery, SettingsMutation, EmptySubscription)
            .data(service)
            .finish();

        let query = r#"
            query {
                generalSettings {
                    language
                }
                appearanceSettings {
                    theme
                }
                databaseSettings {
                    databaseDirectory
                }
            }
        "#;

        let response = schema.execute(query).await;
        assert!(response.errors.is_empty());

        let data = response.data.into_json().unwrap();
        assert_eq!(data["generalSettings"]["language"], "ja");
        assert_eq!(data["appearanceSettings"]["theme"], "system");
        assert!(data["databaseSettings"]["databaseDirectory"].is_string());
    }
}
