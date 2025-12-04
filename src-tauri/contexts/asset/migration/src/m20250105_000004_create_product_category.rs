use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductCategory::Table)
                    .if_not_exists()
                    .col(pk_auto(ProductCategory::Id))
                    .col(integer(ProductCategory::ProductId).not_null())
                    .col(integer(ProductCategory::CategoryId).not_null())
                    .col(
                        timestamp_with_time_zone(ProductCategory::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category_product")
                            .from(ProductCategory::Table, ProductCategory::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category_category")
                            .from(ProductCategory::Table, ProductCategory::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // ユニーク制約
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("uq_product_category")
                    .table(ProductCategory::Table)
                    .col(ProductCategory::ProductId)
                    .col(ProductCategory::CategoryId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // インデックス作成
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_category_product_id")
                    .table(ProductCategory::Table)
                    .col(ProductCategory::ProductId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_product_category_category_id")
                    .table(ProductCategory::Table)
                    .col(ProductCategory::CategoryId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductCategory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProductCategory {
    Table,
    Id,
    ProductId,
    CategoryId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
}
