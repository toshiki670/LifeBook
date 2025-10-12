// Infrastructure Layer - Book SeaORMモデル（DBスキーマ）

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Book テーブルのSeaORMモデル
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "books")]
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
