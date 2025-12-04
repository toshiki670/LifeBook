// Asset Domain - Category Repository Interface

use crate::domain::entities::category::Category;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use sea_orm::DatabaseTransaction;

/// Category リポジトリのインターフェース
#[async_trait]
pub trait CategoryRepository: Send + Sync {
    /// IDでカテゴリを検索
    async fn find_by_id(&self, id: i32) -> Result<Option<Category>, DomainError>;

    /// すべてのカテゴリを取得
    async fn find_all(&self) -> Result<Vec<Category>, DomainError>;

    /// ルートカテゴリを取得
    async fn find_roots(&self) -> Result<Vec<Category>, DomainError>;

    /// 直接の子カテゴリを取得
    async fn find_children(&self, parent_id: i32) -> Result<Vec<Category>, DomainError>;

    /// すべての子孫カテゴリを取得（WITH RECURSIVE）
    async fn find_descendants(&self, id: i32) -> Result<Vec<Category>, DomainError>;

    /// カテゴリツリーを取得（ルートから全体）
    async fn find_tree(&self) -> Result<Vec<Category>, DomainError>;

    /// カテゴリを保存（新規作成または更新）
    async fn save(&self, category: Category) -> Result<Category, DomainError>;

    /// カテゴリをハードデリート
    async fn delete(&self, id: i32) -> Result<(), DomainError>;

    /// カテゴリがProductに紐づいているかチェック
    async fn has_products(&self, id: i32) -> Result<bool, DomainError>;

    /// 循環参照チェック（指定されたcategory_idが自分の子孫か）
    async fn is_descendant_of(
        &self,
        category_id: i32,
        ancestor_id: i32,
    ) -> Result<bool, DomainError>;

    /// サブツリーのdepthを一括更新（WITH RECURSIVE）
    async fn update_subtree_depth(&self, id: i32, depth_diff: i32) -> Result<(), DomainError>;

    /// 親の深さを取得
    async fn get_parent_depth(&self, parent_id: Option<i32>) -> Result<i32, DomainError>;

    /// トランザクション付き操作
    async fn find_by_id_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
    ) -> Result<Option<Category>, DomainError>;

    async fn find_children_with_tx(
        &self,
        tx: &DatabaseTransaction,
        parent_id: i32,
    ) -> Result<Vec<Category>, DomainError>;

    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        category: Category,
    ) -> Result<Category, DomainError>;

    async fn delete_with_tx(&self, tx: &DatabaseTransaction, id: i32) -> Result<(), DomainError>;

    async fn update_subtree_depth_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
        depth_diff: i32,
    ) -> Result<(), DomainError>;
}
