// GraphQL Schema - スキーマ統合

use crate::app_state::AppState;
use async_graphql::*;
use library::{BookMutation, BookQuery};
use settings::{SettingsMutation, SettingsQuery};

/// クエリのルート
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Libraryコンテキストへのアクセス
    async fn library(&self) -> BookQuery {
        BookQuery
    }

    /// Settingsコンテキストへのアクセス
    async fn settings(&self) -> SettingsQuery {
        SettingsQuery
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

    /// Settingsコンテキストのミューテーション
    async fn settings(&self) -> SettingsMutation {
        SettingsMutation
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// GraphQLスキーマを構築
pub fn build_schema(app_state: AppState) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(app_state.book_service)
        .data(app_state.settings_service)
        .finish()
}

/// スキーマ定義のみを構築（データなし）
/// スキーマエクスポート用 - データベース接続や設定ディレクトリ不要
pub fn build_schema_without_data() -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}
