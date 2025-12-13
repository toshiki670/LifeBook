// Asset Domain - AssetTransaction Repository Interface

use super::transaction::AssetTransaction;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use sea_orm::DatabaseTransaction;

/// AssetTransaction リポジトリのインターフェース
#[async_trait]
pub trait AssetTransactionRepository: Send + Sync {
    /// IDでトランザクションを検索
    async fn find_by_id(&self, id: i32) -> Result<Option<AssetTransaction>, DomainError>;

    /// Asset IDでトランザクションを検索
    async fn find_by_asset_id(&self, asset_id: i32) -> Result<Vec<AssetTransaction>, DomainError>;

    /// トランザクションを保存（新規作成のみ、更新不可）
    async fn save(&self, transaction: AssetTransaction) -> Result<AssetTransaction, DomainError>;

    /// トランザクション付き操作
    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        transaction: AssetTransaction,
    ) -> Result<AssetTransaction, DomainError>;
}
