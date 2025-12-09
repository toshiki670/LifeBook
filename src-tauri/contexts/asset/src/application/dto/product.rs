// Asset Application - Product DTO

use crate::domain::product::Product;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub unit: String,
    pub manufacturer_id: i32,
}

impl From<Product> for ProductDto {
    fn from(product: Product) -> Self {
        Self {
            id: product.id().unwrap(),
            name: product.name().to_string(),
            description: product.description().map(String::from),
            unit: product.unit().as_str().to_string(),
            manufacturer_id: product.manufacturer_id(),
        }
    }
}
