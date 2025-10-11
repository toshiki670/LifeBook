// Application State - 依存性注入コンテナ

use crate::application::services::BookService;
use crate::infrastructure::repositories::BookRepositoryImpl;
use std::sync::Arc;

/// アプリケーション全体の状態と依存性を管理
#[derive(Clone)]
pub struct AppState {
    pub book_service: Arc<BookService<BookRepositoryImpl>>,
    // 将来のサービスをここに追加
    // 例: pub author_service: Arc<AuthorService<AuthorRepositoryImpl>>,
    // 例: pub category_service: Arc<CategoryService<CategoryRepositoryImpl>>,
}

impl AppState {
    pub fn new(book_service: Arc<BookService<BookRepositoryImpl>>) -> Self {
        Self { book_service }
    }
}
