// DDD Architecture Entry Point
// 依存性注入とアプリケーション起動

// ============================================================================
// モジュール宣言（mod.rsを使わない方針）
// ============================================================================

// Domain Layer
mod domain {
    pub mod entities;
    pub mod errors;
    pub mod repositories;
}

// Application Layer
mod application {
    pub mod dto;
    pub mod errors;
    pub mod services;
}

// Infrastructure Layer
mod infrastructure {
    pub mod models;
    pub mod repositories;
}

// Presentation Layer
mod presentation {
    pub mod mutation;
    pub mod query;
    pub mod schema;
}

// Other modules
mod database;
mod migration;

// ============================================================================
// Imports
// ============================================================================

use application::services::BookService;
use database::DbState;
use infrastructure::repositories::BookRepositoryImpl;
use presentation::schema::{AppSchema, build_schema};
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// Tauri Commands
// ============================================================================

/// GraphQLクエリを実行するTauriコマンド
#[tauri::command]
async fn execute_graphql(
    query: String,
    schema: tauri::State<'_, AppSchema>,
) -> Result<String, String> {
    let result = schema.execute(&query).await;
    Ok(serde_json::to_string(&result).map_err(|e| e.to_string())?)
}

/// データベース接続状態を取得
#[tauri::command]
async fn get_db_status(db_state: tauri::State<'_, DbState>) -> Result<String, String> {
    let db = db_state.read().await;
    Ok(if db.is_some() {
        "Connected".to_string()
    } else {
        "Disconnected".to_string()
    })
}

// ============================================================================
// Application Entry Point
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Tauriビルダー構築前に非同期初期化を実行
    let (schema, db_state) = tauri::async_runtime::block_on(async {
        // 1. データベース初期化
        let db = database::init_database()
            .await
            .expect("Failed to initialize database");

        // 2. マイグレーション実行
        use sea_orm_migration::MigratorTrait;
        migration::Migrator::up(&db, None)
            .await
            .expect("Failed to run migrations");

        // 3. Infrastructure層: Repository実装
        let book_repository = Arc::new(BookRepositoryImpl::new(db.clone()));

        // 4. Application層: サービス構築
        let book_service = Arc::new(BookService::new(book_repository));

        // 5. Presentation層: GraphQLスキーマ構築
        let schema = build_schema(book_service);

        // 6. データベース状態の管理
        let db_state: DbState = Arc::new(RwLock::new(Some(db)));

        (schema, db_state)
    });

    // Tauriアプリケーション起動
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(schema)
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![execute_graphql, get_db_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
