// Asset Domain - Manufacturer Repository Interface

use super::entity::Manufacturer;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use sea_orm::DatabaseTransaction;

/// Manufacturer リポジトリのインターフェース
#[async_trait]
pub trait ManufacturerRepository: Send + Sync {
    /// IDでメーカーを検索
    async fn find_by_id(&self, id: i32) -> Result<Option<Manufacturer>, DomainError>;

    /// すべてのメーカーを取得（デフォルトでアクティブのみ）
    async fn find_all(&self, include_deleted: bool) -> Result<Vec<Manufacturer>, DomainError>;

    /// メーカーを保存（新規作成または更新）
    async fn save(&self, manufacturer: Manufacturer) -> Result<Manufacturer, DomainError>;

    /// メーカーをソフトデリート
    async fn soft_delete(&self, id: i32) -> Result<(), DomainError>;

    /// メーカーがProductに紐づいているかチェック
    async fn has_products(&self, id: i32) -> Result<bool, DomainError>;

    /// トランザクション付き操作
    async fn find_by_id_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
    ) -> Result<Option<Manufacturer>, DomainError>;

    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        manufacturer: Manufacturer,
    ) -> Result<Manufacturer, DomainError>;
}
