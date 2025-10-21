// Application State - 依存性注入コンテナ

use library::{BookService, build_book_service};
use sea_orm::DatabaseConnection;
use settings::{SettingsService, build_settings_service};
use std::path::PathBuf;
use std::sync::Arc;

/// アプリケーション全体の状態を保持する構造体
pub struct AppState {
    pub book_service: Arc<BookService>,
    pub settings_service: Arc<SettingsService>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config_dir: PathBuf, default_db_dir: PathBuf) -> Self {
        // Library Context（統合ヘルパー関数）
        let book_service = build_book_service(db);

        // Settings Context（統合ヘルパー関数）
        let settings_service = build_settings_service(config_dir, default_db_dir);

        Self {
            book_service,
            settings_service,
        }
    }
}
