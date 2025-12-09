// Asset Domain - Asset Repository Interface

use super::asset::Asset;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use sea_orm::DatabaseTransaction;

/// Asset リポジトリのインターフェース
#[async_trait]
pub trait AssetRepository: Send + Sync {
    /// IDで資産を検索
    async fn find_by_id(&self, id: i32) -> Result<Option<Asset>, DomainError>;

    /// すべての資産を取得
    async fn find_all(&self, owned_only: bool) -> Result<Vec<Asset>, DomainError>;

    /// Product IDで資産を検索
    async fn find_by_product_id(&self, product_id: i32) -> Result<Option<Asset>, DomainError>;

    /// 資産を保存（新規作成または更新）
    async fn save(&self, asset: Asset) -> Result<Asset, DomainError>;

    /// トランザクション付き操作
    async fn find_by_id_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
    ) -> Result<Option<Asset>, DomainError>;

    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        asset: Asset,
    ) -> Result<Asset, DomainError>;
}
