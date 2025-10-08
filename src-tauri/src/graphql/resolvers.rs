use async_graphql::*;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::entities::book;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// すべての本を取得
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<book::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let books = book::Entity::find()
            .all(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        Ok(books)
    }

    /// IDで本を取得
    async fn book(&self, ctx: &Context<'_>, id: i32) -> Result<Option<book::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let book = book::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        Ok(book)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// 新しい本を作成
    async fn create_book(
        &self,
        ctx: &Context<'_>,
        title: String,
        author: Option<String>,
        description: Option<String>,
        published_year: Option<i32>,
    ) -> Result<book::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        let new_book = book::ActiveModel {
            title: Set(title),
            author: Set(author),
            description: Set(description),
            published_year: Set(published_year),
            ..Default::default()
        };
        
        let result = new_book.insert(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        
        Ok(result)
    }

    /// 本を更新
    async fn update_book(
        &self,
        ctx: &Context<'_>,
        id: i32,
        title: Option<String>,
        author: Option<String>,
        description: Option<String>,
        published_year: Option<i32>,
    ) -> Result<book::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        let book_to_update = book::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?
            .ok_or_else(|| Error::new("Book not found"))?;
        
        let mut active_model: book::ActiveModel = book_to_update.into();
        
        if let Some(title) = title {
            active_model.title = Set(title);
        }
        if let Some(author) = author {
            active_model.author = Set(Some(author));
        }
        if let Some(description) = description {
            active_model.description = Set(Some(description));
        }
        if let Some(published_year) = published_year {
            active_model.published_year = Set(Some(published_year));
        }
        
        let result = active_model.update(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        
        Ok(result)
    }

    /// 本を削除
    async fn delete_book(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        
        let book_to_delete = book::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?
            .ok_or_else(|| Error::new("Book not found"))?;
        
        let active_model: book::ActiveModel = book_to_delete.into();
        active_model.delete(db)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        
        Ok(true)
    }
}

