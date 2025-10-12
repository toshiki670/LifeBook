// LifeBook Library

mod app_state;
mod database;
mod infrastructure;
mod migration;
mod modules;
mod presentation;

use app_state::AppState;
use database::setup_database;
use presentation::schema::build_schema;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // データベース接続を確立
            let db = tauri::async_runtime::block_on(setup_database())?;

            // アプリケーションステートを初期化
            let app_state = AppState::new(db);

            // GraphQLスキーマを構築
            let schema = build_schema(app_state);

            // スキーマをアプリの状態として管理
            app.manage(schema);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![graphql_request, get_db_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// GraphQLリクエストを処理するTauriコマンド
#[tauri::command]
async fn graphql_request(
    schema: tauri::State<'_, presentation::schema::AppSchema>,
    request: String,
) -> Result<String, String> {
    use async_graphql::Request;

    // リクエストをパース
    let gql_request: Request =
        serde_json::from_str(&request).map_err(|e| format!("Failed to parse request: {}", e))?;

    // リクエストを実行
    let response = schema.execute(gql_request).await;

    // レスポンスをJSON文字列に変換
    serde_json::to_string(&response).map_err(|e| format!("Failed to serialize response: {}", e))
}

/// データベース接続状態を確認するTauriコマンド
#[tauri::command]
fn get_db_status() -> String {
    // セットアップ時にデータベース接続が成功しているため、常にConnectedを返す
    "Connected".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_setup() {
        // 基本的なセットアップテスト
        // 実際のテストケースはここに追加
    }
}
