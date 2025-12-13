// Asset Presentation - Manufacturer Mutations
// プレースホルダー実装

use crate::application::dto::manufacturer::ManufacturerDto;
use crate::application::errors::ApplicationError;
use crate::application::services::manufacturer::ManufacturerService;
use async_graphql::{Context, ErrorExtensions, ID, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct ManufacturerMutation;

#[Object]
impl ManufacturerMutation {
    async fn create_manufacturer(
        &self,
        ctx: &Context<'_>,
        name: String,
        website: Option<String>,
        contact_email: Option<String>,
    ) -> Result<ManufacturerDto> {
        let service = ctx.data::<Arc<ManufacturerService>>()?;
        service
            .create_manufacturer(name, website, contact_email)
            .await
            .map_err(|e: ApplicationError| e.extend())
    }

    async fn update_manufacturer(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: Option<String>,
        website: Option<String>,
        contact_email: Option<String>,
    ) -> Result<ManufacturerDto> {
        let service = ctx.data::<Arc<ManufacturerService>>()?;
        let id: i32 = id.parse()?;
        service
            .update_manufacturer(id, name, website, contact_email)
            .await
            .map_err(|e: ApplicationError| e.extend())
    }

    async fn delete_manufacturer(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let service = ctx.data::<Arc<ManufacturerService>>()?;
        let id: i32 = id.parse()?;
        service
            .delete_manufacturer(id)
            .await
            .map_err(|e: ApplicationError| e.extend())?;
        Ok(true)
    }
}
