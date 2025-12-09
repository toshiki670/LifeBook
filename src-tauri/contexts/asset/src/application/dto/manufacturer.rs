// Asset Application - Manufacturer DTO

use crate::domain::manufacturer::Manufacturer;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ManufacturerDto {
    pub id: i32,
    pub name: String,
    pub website: Option<String>,
    pub contact_email: Option<String>,
}

impl From<Manufacturer> for ManufacturerDto {
    fn from(manufacturer: Manufacturer) -> Self {
        Self {
            id: manufacturer.id().unwrap(),
            name: manufacturer.name().to_string(),
            website: manufacturer.website().map(String::from),
            contact_email: manufacturer.contact_email().map(String::from),
        }
    }
}
