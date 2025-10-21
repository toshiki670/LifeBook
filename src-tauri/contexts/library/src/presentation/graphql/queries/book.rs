// Presentation Layer - Library Book GraphQL Query

use crate::application::{dto::book::BookDto, services::book::BookService};
use crate::presentation::graphql::to_graphql_error;
use async_graphql::*;
use std::sync::Arc;

#[derive(Default)]
pub struct BookQuery;

#[Object]
impl BookQuery {
    /// すべての本を取得
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<BookDto>> {
        let book_service = ctx
            .data::<Arc<BookService>>()
            .map_err(|_| Error::new("BookService not found"))?;

        book_service.get_all_books().await.map_err(to_graphql_error)
    }

    /// IDで本を取得
    async fn book(&self, ctx: &Context<'_>, id: i32) -> Result<Option<BookDto>> {
        let book_service = ctx
            .data::<Arc<BookService>>()
            .map_err(|_| Error::new("BookService not found"))?;

        book_service.get_book(id).await.map_err(to_graphql_error)
    }
}
