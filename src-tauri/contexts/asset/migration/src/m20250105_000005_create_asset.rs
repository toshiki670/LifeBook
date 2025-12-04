use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Asset::Table)
                    .if_not_exists()
                    .col(pk_auto(Asset::Id))
                    .col(string_len(Asset::Name, 200).not_null())
                    .col(text_null(Asset::Description))
                    .col(integer(Asset::ProductId).not_null().unique_key())
                    .col(
                        ColumnDef::new(Asset::CurrentQuantity)
                            .decimal_len(15, 4)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Asset::TotalAcquisitionCost)
                            .decimal_len(15, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        timestamp_with_time_zone(Asset::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Asset::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_asset_product")
                            .from(Asset::Table, Asset::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // CHECK制約
        manager
            .exec_stmt(
                Query::select()
                    .expr(Expr::cust("1"))
                    .from(Asset::Table)
                    .and_where(Expr::cust("current_quantity >= 0"))
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::select()
                    .expr(Expr::cust("1"))
                    .from(Asset::Table)
                    .and_where(Expr::cust("total_acquisition_cost >= 0"))
                    .to_owned(),
            )
            .await?;

        // インデックス作成
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_asset_product_id")
                    .table(Asset::Table)
                    .col(Asset::ProductId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Asset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Asset {
    Table,
    Id,
    Name,
    Description,
    ProductId,
    CurrentQuantity,
    TotalAcquisitionCost,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
}
