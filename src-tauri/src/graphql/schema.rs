use async_graphql::{EmptySubscription, Schema};
use crate::graphql::resolvers::{QueryRoot, MutationRoot};
use sea_orm::DatabaseConnection;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(db: DatabaseConnection) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish()
}

