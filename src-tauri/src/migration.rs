use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250108_000001_create_book_table::Migration)]
    }
}

// Migration: Create Book Table
mod m20250108_000001_create_book_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Book::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Book::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Book::Title).string().not_null())
                        .col(ColumnDef::new(Book::Author).string())
                        .col(ColumnDef::new(Book::Description).string())
                        .col(ColumnDef::new(Book::PublishedYear).integer())
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Book::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum Book {
        #[sea_orm(iden = "books")]
        Table,
        #[sea_orm(iden = "id")]
        Id,
        #[sea_orm(iden = "title")]
        Title,
        #[sea_orm(iden = "author")]
        Author,
        #[sea_orm(iden = "description")]
        Description,
        #[sea_orm(iden = "published_year")]
        PublishedYear,
    }
}
