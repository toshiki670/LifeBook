// Asset Infrastructure - AssetTransaction Repository Implementation

use crate::domain::asset::AssetTransactionRepository;
use crate::domain::asset::{AssetTransaction, TransactionType};
use crate::domain::errors::DomainError;

use asset_entity::asset_transaction;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, NotSet, QueryOrder, Set,
    entity::prelude::*,
};

pub struct AssetTransactionRepositoryImpl {
    db: DatabaseConnection,
}

impl AssetTransactionRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn domain_to_active_model(transaction: &AssetTransaction) -> asset_transaction::ActiveModel {
        asset_transaction::ActiveModel {
            id: if transaction.id().is_some() {
                Set(transaction.id().unwrap())
            } else {
                NotSet
            },
            type_: Set(transaction.type_().as_str().to_string()),
            transaction_date: Set(transaction.transaction_date()),
            quantity: Set(transaction.quantity()),
            unit_price: Set(transaction.unit_price()),
            unit_cost_at_time: Set(transaction.unit_cost_at_time()),
            note: Set(transaction.note().map(String::from)),
            asset_id: Set(transaction.asset_id()),
            created_at: Set(chrono::Utc::now()),
        }
    }

    fn db_to_domain(model: asset_transaction::Model) -> AssetTransaction {
        let type_ = model.type_.parse().unwrap_or(TransactionType::Adjustment);
        AssetTransaction::reconstruct(crate::domain::asset::AssetTransactionParams {
            id: model.id,
            type_,
            transaction_date: model.transaction_date,
            quantity: model.quantity,
            unit_price: model.unit_price,
            unit_cost_at_time: model.unit_cost_at_time,
            note: model.note,
            asset_id: model.asset_id,
        })
    }
}

#[async_trait]
impl AssetTransactionRepository for AssetTransactionRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<AssetTransaction>, DomainError> {
        let transaction = asset_transaction::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(transaction.map(Self::db_to_domain))
    }

    async fn find_by_asset_id(&self, asset_id: i32) -> Result<Vec<AssetTransaction>, DomainError> {
        let transactions = asset_transaction::Entity::find()
            .filter(asset_transaction::Column::AssetId.eq(asset_id))
            .order_by_desc(asset_transaction::Column::TransactionDate)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(transactions.into_iter().map(Self::db_to_domain).collect())
    }

    async fn save(
        &self,
        mut transaction: AssetTransaction,
    ) -> Result<AssetTransaction, DomainError> {
        let active_model = Self::domain_to_active_model(&transaction);

        let result = active_model
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        transaction.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }

    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        mut transaction: AssetTransaction,
    ) -> Result<AssetTransaction, DomainError> {
        let active_model = Self::domain_to_active_model(&transaction);

        let result = active_model
            .insert(tx)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        transaction.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }
}
