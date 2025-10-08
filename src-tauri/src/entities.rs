// Book Entity
use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub mod book {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
    #[sea_orm(table_name = "books")]
    #[graphql(concrete(name = "Book", params()))]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,

        pub title: String,
        pub author: Option<String>,
        pub description: Option<String>,
        pub published_year: Option<i32>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
