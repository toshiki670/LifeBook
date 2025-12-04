// Asset Application - Category DTO

use crate::domain::entities::category::Category;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CategoryDto {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub depth: i32,
}

impl From<Category> for CategoryDto {
    fn from(category: Category) -> Self {
        Self {
            id: category.id().unwrap(),
            name: category.name().to_string(),
            parent_id: category.parent_id(),
            depth: category.depth(),
        }
    }
}
