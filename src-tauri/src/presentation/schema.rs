// Presentation Layer - GraphQL Schema

use super::{mutation::MutationRoot, query::QueryRoot};
use crate::app_state::AppState;
use async_graphql::*;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// GraphQLスキーマを構築
pub fn build_schema(app_state: AppState) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(app_state)
        .finish()
}
