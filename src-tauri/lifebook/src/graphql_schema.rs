// GraphQL Schema - スキーマ統合

use crate::app_state::AppState;
use async_graphql::*;
use library::{BookMutation, BookQuery};

/// クエリのルート
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Libraryコンテキストへのアクセス
    async fn library(&self) -> BookQuery {
        BookQuery
    }
}

/// ミューテーションのルート
#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Libraryコンテキストのミューテーション
    async fn library(&self) -> BookMutation {
        BookMutation
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// GraphQLスキーマを構築
pub fn build_schema(app_state: AppState) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(app_state.book_service)
        .finish()
}
