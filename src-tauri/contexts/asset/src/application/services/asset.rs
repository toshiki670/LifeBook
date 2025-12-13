// Asset Application - Asset Service

use crate::application::dto::asset::AssetDto;
use crate::application::dto::asset_transaction::AssetTransactionDto;
use crate::application::errors::ApplicationError;
use crate::domain::asset::{
    Asset, AssetRepository, AssetTransaction, AssetTransactionRepository, TransactionType,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, TransactionTrait};
use std::sync::Arc;

pub struct AssetService {
    db: DatabaseConnection,
    asset_repository: Arc<dyn AssetRepository>,
    transaction_repository: Arc<dyn AssetTransactionRepository>,
}

impl AssetService {
    pub fn new(
        db: DatabaseConnection,
        asset_repository: Arc<dyn AssetRepository>,
        transaction_repository: Arc<dyn AssetTransactionRepository>,
    ) -> Self {
        Self {
            db,
            asset_repository,
            transaction_repository,
        }
    }

    pub async fn create_asset(
        &self,
        name: String,
        description: Option<String>,
        product_id: i32,
    ) -> Result<AssetDto, ApplicationError> {
        let asset = Asset::new(name, description, product_id)?;
        let saved = self.asset_repository.save(asset).await?;
        Ok(AssetDto::from(saved))
    }

    pub async fn get_all_assets(
        &self,
        owned_only: bool,
    ) -> Result<Vec<AssetDto>, ApplicationError> {
        let assets = self.asset_repository.find_all(owned_only).await?;
        Ok(assets.into_iter().map(AssetDto::from).collect())
    }

    pub async fn get_asset(&self, id: i32) -> Result<Option<AssetDto>, ApplicationError> {
        let asset = self.asset_repository.find_by_id(id).await?;
        Ok(asset.map(AssetDto::from))
    }

    pub async fn update_asset(
        &self,
        id: i32,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<AssetDto, ApplicationError> {
        let mut asset =
            self.asset_repository.find_by_id(id).await?.ok_or_else(|| {
                ApplicationError::NotFound(format!("Asset with id {} not found", id))
            })?;

        asset.update_details(name, description)?;
        let updated = self.asset_repository.save(asset).await?;
        Ok(AssetDto::from(updated))
    }

    /// 購入を追加（トランザクション管理）
    pub async fn add_purchase(
        &self,
        asset_id: i32,
        quantity: Decimal,
        unit_price: Decimal,
        transaction_date: NaiveDate,
        note: Option<String>,
    ) -> Result<AssetTransactionDto, ApplicationError> {
        // トランザクション開始
        let tx = self.db.begin().await.map_err(|e| {
            ApplicationError::DatabaseError(format!("Failed to begin transaction: {}", e))
        })?;

        // 1. Assetを取得
        let mut asset = self
            .asset_repository
            .find_by_id_with_tx(&tx, asset_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Asset with id {} not found", asset_id))
            })?;

        // 2. ドメインロジック実行（購入）
        asset.add_purchase(quantity, unit_price)?;
        let avg_cost = asset.average_unit_cost();

        // 3. Assetを保存
        self.asset_repository.save_with_tx(&tx, asset).await?;

        // 4. AssetTransactionを作成・保存
        let transaction = AssetTransaction::new(
            TransactionType::Purchase,
            transaction_date,
            quantity,
            Some(unit_price),
            Some(avg_cost),
            note,
            asset_id,
        )?;

        let saved = self
            .transaction_repository
            .save_with_tx(&tx, transaction)
            .await?;

        // コミット
        tx.commit().await.map_err(|e| {
            ApplicationError::DatabaseError(format!("Failed to commit transaction: {}", e))
        })?;

        Ok(AssetTransactionDto::from(saved))
    }

    /// 売却を追加（トランザクション管理）
    pub async fn add_sale(
        &self,
        asset_id: i32,
        quantity: Decimal,
        unit_price: Decimal,
        transaction_date: NaiveDate,
        note: Option<String>,
    ) -> Result<AssetTransactionDto, ApplicationError> {
        let tx = self.db.begin().await.map_err(|e| {
            ApplicationError::DatabaseError(format!("Failed to begin transaction: {}", e))
        })?;

        let mut asset = self
            .asset_repository
            .find_by_id_with_tx(&tx, asset_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Asset with id {} not found", asset_id))
            })?;

        // 売却前の平均取得単価を取得
        let avg_cost = asset.add_sale(quantity)?;

        self.asset_repository.save_with_tx(&tx, asset).await?;

        let transaction = AssetTransaction::new(
            TransactionType::Sale,
            transaction_date,
            quantity,
            Some(unit_price),
            Some(avg_cost),
            note,
            asset_id,
        )?;

        let saved = self
            .transaction_repository
            .save_with_tx(&tx, transaction)
            .await?;

        tx.commit().await.map_err(|e| {
            ApplicationError::DatabaseError(format!("Failed to commit transaction: {}", e))
        })?;

        Ok(AssetTransactionDto::from(saved))
    }

    /// 廃棄を追加（トランザクション管理）
    pub async fn add_disposal(
        &self,
        asset_id: i32,
        quantity: Decimal,
        transaction_date: NaiveDate,
        note: Option<String>,
    ) -> Result<AssetTransactionDto, ApplicationError> {
        let tx = self.db.begin().await.map_err(|e| {
            ApplicationError::DatabaseError(format!("Failed to begin transaction: {}", e))
        })?;

        let mut asset = self
            .asset_repository
            .find_by_id_with_tx(&tx, asset_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Asset with id {} not found", asset_id))
            })?;

        let avg_cost = asset.add_disposal(quantity)?;

        self.asset_repository.save_with_tx(&tx, asset).await?;

        let transaction = AssetTransaction::new(
            TransactionType::Disposal,
            transaction_date,
            quantity,
            None, // 廃棄なので売却価格なし
            Some(avg_cost),
            note,
            asset_id,
        )?;

        let saved = self
            .transaction_repository
            .save_with_tx(&tx, transaction)
            .await?;

        tx.commit().await.map_err(|e| {
            ApplicationError::DatabaseError(format!("Failed to commit transaction: {}", e))
        })?;

        Ok(AssetTransactionDto::from(saved))
    }

    pub async fn get_asset_transactions(
        &self,
        asset_id: i32,
    ) -> Result<Vec<AssetTransactionDto>, ApplicationError> {
        let transactions = self
            .transaction_repository
            .find_by_asset_id(asset_id)
            .await?;
        Ok(transactions
            .into_iter()
            .map(AssetTransactionDto::from)
            .collect())
    }
}
