// Asset Application - AssetTransaction Service
// プレースホルダー実装

use crate::application::dto::asset_transaction::AssetTransactionDto;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::asset_transaction::AssetTransactionRepository;
use std::sync::Arc;

pub struct AssetTransactionService {
    repository: Arc<dyn AssetTransactionRepository>,
}

impl AssetTransactionService {
    pub fn new(repository: Arc<dyn AssetTransactionRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_transaction(
        &self,
        id: i32,
    ) -> Result<Option<AssetTransactionDto>, ApplicationError> {
        let transaction = self.repository.find_by_id(id).await?;
        Ok(transaction.map(AssetTransactionDto::from))
    }

    pub async fn get_asset_transactions(
        &self,
        asset_id: i32,
    ) -> Result<Vec<AssetTransactionDto>, ApplicationError> {
        let transactions = self.repository.find_by_asset_id(asset_id).await?;
        Ok(transactions
            .into_iter()
            .map(AssetTransactionDto::from)
            .collect())
    }
}
