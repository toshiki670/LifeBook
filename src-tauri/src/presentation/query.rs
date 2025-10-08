// Presentation Layer - GraphQL Query

use crate::application::{dto::BookDto, services::BookService};
use crate::infrastructure::repositories::BookRepositoryImpl;
use async_graphql::*;
use std::sync::Arc;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// すべての本を取得
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<BookDto>> {
        let service = ctx
            .data::<Arc<BookService<BookRepositoryImpl>>>()
            .map_err(|_| Error::new("Service not found"))?;

        service
            .get_all_books()
            .await
            .map_err(|e| Error::new(e.to_string()))
    }

    /// IDで本を取得
    async fn book(&self, ctx: &Context<'_>, id: i32) -> Result<Option<BookDto>> {
        let service = ctx
            .data::<Arc<BookService<BookRepositoryImpl>>>()
            .map_err(|_| Error::new("Service not found"))?;

        service
            .get_book(id)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }
}
