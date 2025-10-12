// Library Infrastructure Layer - Book リポジトリ実装

use crate::infrastructure::models::book;
use crate::modules::library::domain::{
    entities::book::Book, repositories::book::BookRepository,
};
use crate::modules::shared::domain::errors::DomainError;
use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ActiveModelTrait, DatabaseConnection, NotSet, Set};

/// BookRepository の SeaORM実装
pub struct BookRepositoryImpl {
    db: DatabaseConnection,
}

impl BookRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// ドメインモデルをDBモデルに変換
    fn domain_to_active_model(book: &Book) -> book::ActiveModel {
        if let Some(id) = book.id() {
            // 既存の本（更新）
            book::ActiveModel {
                id: Set(id),
                title: Set(book.title().to_string()),
                author: Set(book.author().map(String::from)),
                description: Set(book.description().map(String::from)),
                published_year: Set(book.published_year()),
            }
        } else {
            // 新しい本（作成）
            book::ActiveModel {
                id: NotSet,
                title: Set(book.title().to_string()),
                author: Set(book.author().map(String::from)),
                description: Set(book.description().map(String::from)),
                published_year: Set(book.published_year()),
            }
        }
    }

    /// DBモデルをドメインモデルに変換
    fn db_to_domain(model: book::Model) -> Book {
        Book::reconstruct(
            model.id,
            model.title,
            model.author,
            model.description,
            model.published_year,
        )
    }
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Book>, DomainError> {
        let book = book::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(book.map(Self::db_to_domain))
    }

    async fn find_all(&self) -> Result<Vec<Book>, DomainError> {
        let books = book::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(books.into_iter().map(Self::db_to_domain).collect())
    }

    async fn save(&self, mut book: Book) -> Result<Book, DomainError> {
        let active_model = Self::domain_to_active_model(&book);

        let result = if book.id().is_some() {
            // 更新
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
        } else {
            // 新規作成
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
        };

        // 保存後のIDをドメインモデルにセット
        book.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }

    async fn delete(&self, id: i32) -> Result<(), DomainError> {
        let book = book::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
            .ok_or_else(|| DomainError::NotFound(format!("Book with id {} not found", id)))?;

        let active_model: book::ActiveModel = book.into();
        active_model
            .delete(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(())
    }
}

