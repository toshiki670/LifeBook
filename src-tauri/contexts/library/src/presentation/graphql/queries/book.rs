// Presentation Layer - Library Book GraphQL Query

use crate::application::{dto::book::BookDto, services::book::BookService};
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

        book_service
            .get_all_books()
            .await
            .map_err(|e| Error::new(e.to_string()))
    }

    /// IDで本を取得
    async fn book(&self, ctx: &Context<'_>, id: i32) -> Result<Option<BookDto>> {
        let book_service = ctx
            .data::<Arc<BookService>>()
            .map_err(|_| Error::new("BookService not found"))?;

        book_service
            .get_book(id)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }
}
