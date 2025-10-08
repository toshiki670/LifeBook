use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type DbState = Arc<RwLock<Option<DatabaseConnection>>>;

pub async fn init_database() -> Result<DatabaseConnection, DbErr> {
    // SQLiteデータベースを使用（アプリのデータディレクトリに保存）
    let database_url = "sqlite://lifebook.db?mode=rwc";
    let db = Database::connect(database_url).await?;
    Ok(db)
}
