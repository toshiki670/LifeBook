// Presentation Layer - GraphQL Schema

use super::{mutation::MutationRoot, query::QueryRoot};
use crate::application::services::BookService;
use crate::infrastructure::repositories::BookRepositoryImpl;
use async_graphql::*;
use std::sync::Arc;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// GraphQLスキーマを構築
pub fn build_schema(service: Arc<BookService<BookRepositoryImpl>>) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(service)
        .finish()
}

