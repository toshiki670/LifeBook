// Product Entity - 商品

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "String(StringLen::N(200))")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub unit: String, // MeasurementUnit as string
    pub manufacturer_id: i32,
    pub deleted_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::manufacturer::Entity",
        from = "Column::ManufacturerId",
        to = "super::manufacturer::Column::Id"
    )]
    Manufacturer,
    #[sea_orm(has_many = "super::product_category::Entity")]
    ProductCategory,
    #[sea_orm(has_one = "super::asset::Entity")]
    Asset,
}

impl Related<super::manufacturer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Manufacturer.def()
    }
}

impl Related<super::product_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductCategory.def()
    }
}

impl Related<super::asset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Asset.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
