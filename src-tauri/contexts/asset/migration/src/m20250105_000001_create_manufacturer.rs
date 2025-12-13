use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Manufacturer::Table)
                    .if_not_exists()
                    .col(pk_auto(Manufacturer::Id))
                    .col(string_len(Manufacturer::Name, 200).not_null().unique_key())
                    .col(string_len_null(Manufacturer::Website, 500))
                    .col(string_len_null(Manufacturer::ContactEmail, 255))
                    .col(timestamp_with_time_zone_null(Manufacturer::DeletedAt))
                    .col(
                        timestamp_with_time_zone(Manufacturer::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Manufacturer::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // インデックス作成
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_manufacturer_deleted_at")
                    .table(Manufacturer::Table)
                    .col(Manufacturer::DeletedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Manufacturer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Manufacturer {
    Table,
    Id,
    Name,
    Website,
    ContactEmail,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
