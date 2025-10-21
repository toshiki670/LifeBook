// Presentation Layer - Library Book GraphQL Mutation

use crate::application::{dto::book::BookDto, services::book::BookService};
use crate::presentation::graphql::to_graphql_error;
use async_graphql::*;
use std::sync::Arc;

#[derive(Default)]
pub struct BookMutation;

#[Object]
impl BookMutation {
    /// 新しい本を作成
    async fn create_book(
        &self,
        ctx: &Context<'_>,
        title: String,
        author: Option<String>,
        description: Option<String>,
        published_year: Option<i32>,
    ) -> Result<BookDto> {
        let book_service = ctx
            .data::<Arc<BookService>>()
            .map_err(|_| Error::new("BookService not found"))?;

        book_service
            .create_book(title, author, description, published_year)
            .await
            .map_err(to_graphql_error)
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
        let book_service = ctx
            .data::<Arc<BookService>>()
            .map_err(|_| Error::new("BookService not found"))?;

        book_service
            .update_book(id, title, author, description, published_year)
            .await
            .map_err(to_graphql_error)
    }

    /// 本を削除
    async fn delete_book(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let book_service = ctx
            .data::<Arc<BookService>>()
            .map_err(|_| Error::new("BookService not found"))?;

        book_service
            .delete_book(id)
            .await
            .map_err(to_graphql_error)?;

        Ok(true)
    }
}
