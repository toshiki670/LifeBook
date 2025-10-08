mod database;
mod entities;
mod graphql;
mod migration;

use database::DbState;
use graphql::{AppSchema, build_schema};
use std::sync::Arc;
use tokio::sync::RwLock;

// GraphQLクエリを実行するTauriコマンド
#[tauri::command]
async fn execute_graphql(
    query: String,
    schema: tauri::State<'_, AppSchema>,
) -> Result<String, String> {
    let result = schema.execute(&query).await;
    Ok(serde_json::to_string(&result).map_err(|e| e.to_string())?)
}

// データベース接続状態を取得
#[tauri::command]
async fn get_db_status(db_state: tauri::State<'_, DbState>) -> Result<String, String> {
    let db = db_state.read().await;
    Ok(if db.is_some() {
        "Connected".to_string()
    } else {
        "Disconnected".to_string()
    })
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Tauriビルダー構築前に非同期初期化を実行
    let (schema, db_state) = tauri::async_runtime::block_on(async {
        // データベース初期化
        let db = database::init_database()
            .await
            .expect("Failed to initialize database");

        // マイグレーション実行
        use sea_orm_migration::MigratorTrait;
        migration::Migrator::up(&db, None)
            .await
            .expect("Failed to run migrations");

        // GraphQLスキーマ構築
        let schema = build_schema(db.clone());

        // データベース状態の管理
        let db_state: DbState = Arc::new(RwLock::new(Some(db)));

        (schema, db_state)
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(schema)
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            execute_graphql,
            get_db_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
