// Asset Application - AssetTransaction DTO

use crate::domain::entities::asset_transaction::AssetTransaction;
use async_graphql::SimpleObject;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct AssetTransactionDto {
    pub id: i32,
    pub type_: String,
    pub transaction_date: NaiveDate,
    pub quantity: String,
    pub unit_price: Option<String>,
    pub unit_cost_at_time: Option<String>,
    pub profit_loss: Option<String>,
    pub note: Option<String>,
    pub asset_id: i32,
}

impl From<AssetTransaction> for AssetTransactionDto {
    fn from(transaction: AssetTransaction) -> Self {
        Self {
            id: transaction.id().unwrap(),
            type_: transaction.type_().as_str().to_string(),
            transaction_date: transaction.transaction_date(),
            quantity: transaction.quantity().to_string(),
            unit_price: transaction.unit_price().map(|d| d.to_string()),
            unit_cost_at_time: transaction.unit_cost_at_time().map(|d| d.to_string()),
            profit_loss: transaction.profit_loss().map(|d| d.to_string()),
            note: transaction.note().map(String::from),
            asset_id: transaction.asset_id(),
        }
    }
}
