// Application State - 依存性注入コンテナ

use crate::infrastructure::repositories::book::BookRepositoryImpl;
use crate::modules::library::application::services::book::BookService;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// アプリケーション全体の状態を保持する構造体
pub struct AppState {
    pub book_service: Arc<BookService>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        // Repositories
        let book_repo = Arc::new(BookRepositoryImpl::new(db));

        // Services
        let book_service = Arc::new(BookService::new(book_repo));

        Self { book_service }
    }
}
