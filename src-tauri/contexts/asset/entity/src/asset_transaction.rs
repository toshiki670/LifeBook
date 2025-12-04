// AssetTransaction Entity - 資産トランザクション

use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "asset_transaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub type_: String, // TransactionType as string
    #[sea_orm(column_type = "Date")]
    pub transaction_date: Date,
    #[sea_orm(column_type = "Decimal(Some((15, 4)))")]
    pub quantity: Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub unit_price: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))", nullable)]
    pub unit_cost_at_time: Option<Decimal>,
    #[sea_orm(column_type = "Text", nullable)]
    pub note: Option<String>,
    pub asset_id: i32,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::asset::Entity",
        from = "Column::AssetId",
        to = "super::asset::Column::Id"
    )]
    Asset,
}

impl Related<super::asset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Asset.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
