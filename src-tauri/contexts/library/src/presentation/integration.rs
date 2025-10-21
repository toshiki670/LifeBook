// Presentation Layer - 統合ヘルパー
// 他のクレートからの依存性注入を簡素化する

use crate::application::services::book::BookService;
use crate::infrastructure::repositories::book::BookRepositoryImpl;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// BookServiceを構築する統合関数
///
/// # Arguments
/// * `db` - データベース接続
///
/// # Returns
/// 依存性が注入されたBookServiceのArcポインタ
pub fn build_book_service(db: DatabaseConnection) -> Arc<BookService> {
    let book_repo = Arc::new(BookRepositoryImpl::new(db));
    Arc::new(BookService::new(book_repo))
}
