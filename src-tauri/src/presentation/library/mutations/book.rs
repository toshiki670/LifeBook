// Presentation Layer - Library Book GraphQL Mutation

use crate::app_state::AppState;
use crate::modules::library::application::dto::book::BookDto;
use async_graphql::*;

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
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not found"))?;

        app_state
            .book_service
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
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not found"))?;

        app_state
            .book_service
            .update_book(id, title, author, description, published_year)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }

    /// 本を削除
    async fn delete_book(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not found"))?;

        app_state
            .book_service
            .delete_book(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(true)
    }
}

