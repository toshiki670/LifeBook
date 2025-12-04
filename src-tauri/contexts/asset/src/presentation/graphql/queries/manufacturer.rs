// Asset Presentation - Manufacturer Queries
// プレースホルダー実装

use crate::application::dto::manufacturer::ManufacturerDto;
use crate::application::errors::ApplicationError;
use crate::application::services::manufacturer::ManufacturerService;
use async_graphql::{Context, ErrorExtensions, ID, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct ManufacturerQuery;

#[Object]
impl ManufacturerQuery {
    async fn manufacturer(&self, ctx: &Context<'_>, id: ID) -> Result<Option<ManufacturerDto>> {
        let service = ctx.data::<Arc<ManufacturerService>>()?;
        let id: i32 = id.parse()?;
        service
            .get_manufacturer(id)
            .await
            .map_err(|e: ApplicationError| e.extend())
    }

    async fn manufacturers(
        &self,
        ctx: &Context<'_>,
        include_deleted: Option<bool>,
    ) -> Result<Vec<ManufacturerDto>> {
        let service = ctx.data::<Arc<ManufacturerService>>()?;
        service
            .get_all_manufacturers(include_deleted.unwrap_or(false))
            .await
            .map_err(|e: ApplicationError| e.extend())
    }
}
