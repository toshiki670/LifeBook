// Library Domain Layer - Book リポジトリインターフェース

use crate::modules::library::domain::entities::book::Book;
use crate::modules::shared::domain::errors::DomainError;
use async_trait::async_trait;

/// Book リポジトリのインターフェース
/// インフラ層がこのtraitを実装する
#[async_trait]
pub trait BookRepository: Send + Sync {
    /// IDで本を検索
    async fn find_by_id(&self, id: i32) -> Result<Option<Book>, DomainError>;

    /// すべての本を取得
    async fn find_all(&self) -> Result<Vec<Book>, DomainError>;

    /// 本を保存（新規作成または更新）
    async fn save(&self, book: Book) -> Result<Book, DomainError>;

    /// 本を削除
    async fn delete(&self, id: i32) -> Result<(), DomainError>;
}

