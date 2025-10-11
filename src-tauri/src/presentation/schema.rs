// Presentation Layer - GraphQL Schema

use super::library::{mutations::book::BookMutation, queries::book::BookQuery};
use crate::app_state::AppState;
use async_graphql::*;

/// クエリのルート
#[derive(MergedObject, Default)]
pub struct QueryRoot(BookQuery);

/// ミューテーションのルート
#[derive(MergedObject, Default)]
pub struct MutationRoot(BookMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// GraphQLスキーマを構築
pub fn build_schema(app_state: AppState) -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(app_state)
        .finish()
}
