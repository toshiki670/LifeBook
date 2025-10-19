// Application State - 依存性注入コンテナ

use library::{BookRepositoryImpl, BookService};
use sea_orm::DatabaseConnection;
use settings::{build_settings_service, SettingsService};
use std::path::PathBuf;
use std::sync::Arc;

/// アプリケーション全体の状態を保持する構造体
pub struct AppState {
    pub book_service: Arc<BookService>,
    pub settings_service: Arc<SettingsService>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config_dir: PathBuf, default_db_dir: PathBuf) -> Self {
        // Library Context
        let book_repo = Arc::new(BookRepositoryImpl::new(db));
        let book_service = Arc::new(BookService::new(book_repo));

        // Settings Context
        let settings_service = build_settings_service(config_dir, default_db_dir);

        Self {
            book_service,
            settings_service,
        }
    }
}
