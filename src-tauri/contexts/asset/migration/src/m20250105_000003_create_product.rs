use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(pk_auto(Product::Id))
                    .col(string_len(Product::Name, 200).not_null())
                    .col(text_null(Product::Description))
                    .col(string_len(Product::Unit, 50).not_null())
                    .col(integer(Product::ManufacturerId).not_null())
                    .col(timestamp_with_time_zone_null(Product::DeletedAt))
                    .col(
                        timestamp_with_time_zone(Product::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Product::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_manufacturer")
                            .from(Product::Table, Product::ManufacturerId)
                            .to(Manufacturer::Table, Manufacturer::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // インデックス作成
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_manufacturer_id")
                    .table(Product::Table)
                    .col(Product::ManufacturerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_deleted_at")
                    .table(Product::Table)
                    .col(Product::DeletedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    Description,
    Unit,
    ManufacturerId,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Manufacturer {
    Table,
    Id,
}
