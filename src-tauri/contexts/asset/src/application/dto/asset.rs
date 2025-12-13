// Asset Application - Asset DTO

use crate::domain::asset::Asset;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct AssetDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub product_id: i32,
    pub current_quantity: String,
    pub total_acquisition_cost: String,
    pub average_unit_cost: String,
    pub is_owned: bool,
}

impl From<Asset> for AssetDto {
    fn from(asset: Asset) -> Self {
        Self {
            id: asset.id().unwrap(),
            name: asset.name().to_string(),
            description: asset.description().map(String::from),
            product_id: asset.product_id(),
            current_quantity: asset.current_quantity().to_string(),
            total_acquisition_cost: asset.total_acquisition_cost().to_string(),
            average_unit_cost: asset.average_unit_cost().to_string(),
            is_owned: asset.is_owned(),
        }
    }
}
