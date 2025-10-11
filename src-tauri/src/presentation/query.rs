// Presentation Layer - GraphQL Query

use crate::app_state::AppState;
use crate::application::dto::BookDto;
use async_graphql::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// すべての本を取得
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<BookDto>> {
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not found"))?;

        app_state
            .book_service
            .get_all_books()
            .await
            .map_err(|e| Error::new(e.to_string()))
    }

    /// IDで本を取得
    async fn book(&self, ctx: &Context<'_>, id: i32) -> Result<Option<BookDto>> {
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not found"))?;

        app_state
            .book_service
            .get_book(id)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }
}
