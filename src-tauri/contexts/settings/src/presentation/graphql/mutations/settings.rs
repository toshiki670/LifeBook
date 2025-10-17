// Presentation Layer - Settings GraphQL Mutation

use crate::application::{
    dto::{AppearanceSettingsDto, DatabaseSettingsDto, GeneralSettingsDto},
    services::SettingsService,
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
            .map_err(|e| Error::new(e.to_string()))
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
            .map_err(|e| Error::new(e.to_string()))
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
            .map_err(|e| Error::new(e.to_string()))
    }

    /// すべての設定をリセット
    async fn reset_settings(&self, ctx: &Context<'_>) -> Result<bool> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .reset_all_settings()
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(true)
    }
}

