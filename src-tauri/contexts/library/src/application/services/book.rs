// Library Application Layer - Book アプリケーションサービス

use crate::application::dto::book::BookDto;
use crate::domain::{entities::book::Book, repositories::book::BookRepository};
use shared::application::errors::ApplicationError;
use std::sync::Arc;

/// Book管理のユースケースを実装するサービス
pub struct BookService {
    repository: Arc<dyn BookRepository>,
}

impl BookService {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }

    /// 新しい本を作成
    pub async fn create_book(
        &self,
        title: String,
        author: Option<String>,
        description: Option<String>,
        published_year: Option<i32>,
    ) -> Result<BookDto, ApplicationError> {
        // 1. ドメインエンティティ作成（バリデーション実行）
        let book = Book::new(title, author, description, published_year)?;

        // 2. リポジトリで永続化
        let saved_book = self.repository.save(book).await?;

        // 3. DTOに変換して返す
        Ok(BookDto::from(saved_book))
    }

    /// すべての本を取得
    pub async fn get_all_books(&self) -> Result<Vec<BookDto>, ApplicationError> {
        let books = self.repository.find_all().await?;
        Ok(books.into_iter().map(BookDto::from).collect())
    }

    /// IDで本を取得
    pub async fn get_book(&self, id: i32) -> Result<Option<BookDto>, ApplicationError> {
        let book = self.repository.find_by_id(id).await?;
        Ok(book.map(BookDto::from))
    }

    /// 本を更新
    pub async fn update_book(
        &self,
        id: i32,
        title: Option<String>,
        author: Option<String>,
        description: Option<String>,
        published_year: Option<i32>,
    ) -> Result<BookDto, ApplicationError> {
        // 1. 既存の本を取得
        let mut book =
            self.repository.find_by_id(id).await?.ok_or_else(|| {
                ApplicationError::NotFound(format!("Book with id {} not found", id))
            })?;

        // 2. ドメインロジックで更新（バリデーション実行）
        book.update_details(title, author, description, published_year)?;

        // 3. リポジトリで保存
        let updated_book = self.repository.save(book).await?;

        // 4. DTOに変換して返す
        Ok(BookDto::from(updated_book))
    }

    /// 本を削除
    pub async fn delete_book(&self, id: i32) -> Result<(), ApplicationError> {
        // 存在確認
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound(format!("Book with id {} not found", id)))?;

        // 削除実行
        self.repository.delete(id).await?;

        Ok(())
    }
}
