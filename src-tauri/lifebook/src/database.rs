use migration::Migrator;
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;

pub async fn setup_database() -> Result<DatabaseConnection, DbErr> {
    // SQLiteデータベースを使用（アプリのデータディレクトリに保存）
    let database_url = "sqlite://lifebook.db?mode=rwc";
    let db = Database::connect(database_url).await?;

    // マイグレーションを実行
    Migrator::up(&db, None).await?;

    Ok(db)
}
