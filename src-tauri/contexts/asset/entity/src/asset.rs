// Asset Entity - 資産

use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "asset")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "String(StringLen::N(200))")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub product_id: i32,
    #[sea_orm(column_type = "Decimal(Some((15, 4)))")]
    pub current_quantity: Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    pub total_acquisition_cost: Decimal,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::ProductId",
        to = "super::product::Column::Id"
    )]
    Product,
    #[sea_orm(has_many = "super::asset_transaction::Entity")]
    AssetTransaction,
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::asset_transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssetTransaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
