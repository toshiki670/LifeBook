// Asset Domain - Product Repository Interface

use crate::domain::entities::product::Product;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use sea_orm::DatabaseTransaction;

/// Product リポジトリのインターフェース
#[async_trait]
pub trait ProductRepository: Send + Sync {
    /// IDで商品を検索
    async fn find_by_id(&self, id: i32) -> Result<Option<Product>, DomainError>;

    /// すべての商品を取得（デフォルトでアクティブのみ）
    async fn find_all(&self, include_deleted: bool) -> Result<Vec<Product>, DomainError>;

    /// カテゴリIDで商品を検索
    async fn find_by_category(
        &self,
        category_id: i32,
        include_deleted: bool,
    ) -> Result<Vec<Product>, DomainError>;

    /// 商品を保存（新規作成または更新）
    async fn save(&self, product: Product) -> Result<Product, DomainError>;

    /// 商品をソフトデリート
    async fn soft_delete(&self, id: i32) -> Result<(), DomainError>;

    /// 商品がAssetに紐づいているかチェック
    async fn has_assets(&self, id: i32) -> Result<bool, DomainError>;

    /// 商品にカテゴリを割り当て
    async fn assign_categories(
        &self,
        product_id: i32,
        category_ids: Vec<i32>,
    ) -> Result<(), DomainError>;

    /// 商品のカテゴリを取得
    async fn get_categories(&self, product_id: i32) -> Result<Vec<i32>, DomainError>;

    /// トランザクション付き操作
    async fn find_by_id_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
    ) -> Result<Option<Product>, DomainError>;

    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        product: Product,
    ) -> Result<Product, DomainError>;
}
