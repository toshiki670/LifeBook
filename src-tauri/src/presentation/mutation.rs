// Presentation Layer - GraphQL Mutation

use crate::application::{dto::BookDto, services::BookService};
use crate::infrastructure::repositories::BookRepositoryImpl;
use async_graphql::*;
use std::sync::Arc;

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
    ) -> Result<BookDto> {
        let service = ctx
            .data::<Arc<BookService<BookRepositoryImpl>>>()
            .map_err(|_| Error::new("Service not found"))?;

        service
            .create_book(title, author, description, published_year)
            .await
            .map_err(|e| Error::new(e.to_string()))
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
    ) -> Result<BookDto> {
        let service = ctx
            .data::<Arc<BookService<BookRepositoryImpl>>>()
            .map_err(|_| Error::new("Service not found"))?;

        service
            .update_book(id, title, author, description, published_year)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }

    /// 本を削除
    async fn delete_book(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let service = ctx
            .data::<Arc<BookService<BookRepositoryImpl>>>()
            .map_err(|_| Error::new("Service not found"))?;

        service
            .delete_book(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(true)
    }
}
