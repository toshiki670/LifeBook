// Presentation Layer - Settings GraphQL Query

use crate::application::{
    dto::{AppearanceSettingsDto, DatabaseSettingsDto, GeneralSettingsDto},
    services::SettingsService,
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
            .map_err(|e| Error::new(e.to_string()))
    }

    /// 表示設定を取得
    async fn appearance_settings(&self, ctx: &Context<'_>) -> Result<AppearanceSettingsDto> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .get_appearance_settings()
            .await
            .map_err(|e| Error::new(e.to_string()))
    }

    /// データベース設定を取得
    async fn database_settings(&self, ctx: &Context<'_>) -> Result<DatabaseSettingsDto> {
        let settings_service = ctx
            .data::<Arc<SettingsService>>()
            .map_err(|_| Error::new("SettingsService not found"))?;

        settings_service
            .get_database_settings()
            .await
            .map_err(|e| Error::new(e.to_string()))
    }
}
