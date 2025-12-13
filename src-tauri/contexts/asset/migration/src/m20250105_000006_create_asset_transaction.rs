use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AssetTransaction::Table)
                    .if_not_exists()
                    .col(pk_auto(AssetTransaction::Id))
                    .col(string_len(AssetTransaction::Type, 20).not_null())
                    .col(date(AssetTransaction::TransactionDate).not_null())
                    .col(
                        ColumnDef::new(AssetTransaction::Quantity)
                            .decimal_len(15, 4)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AssetTransaction::UnitPrice)
                            .decimal_len(15, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(AssetTransaction::UnitCostAtTime)
                            .decimal_len(15, 2)
                            .null(),
                    )
                    .col(text_null(AssetTransaction::Note))
                    .col(integer(AssetTransaction::AssetId).not_null())
                    .col(
                        timestamp_with_time_zone(AssetTransaction::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_asset")
                            .from(AssetTransaction::Table, AssetTransaction::AssetId)
                            .to(Asset::Table, Asset::Id)
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
                    .from(AssetTransaction::Table)
                    .and_where(Expr::cust("quantity > 0"))
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::select()
                    .expr(Expr::cust("1"))
                    .from(AssetTransaction::Table)
                    .and_where(Expr::cust(
                        "type IN ('PURCHASE', 'SALE', 'DISPOSAL', 'ADJUSTMENT')",
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::select()
                    .expr(Expr::cust("1"))
                    .from(AssetTransaction::Table)
                    .and_where(Expr::cust("unit_price IS NULL OR unit_price >= 0"))
                    .to_owned(),
            )
            .await?;

        // インデックス作成
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_asset_id")
                    .table(AssetTransaction::Table)
                    .col(AssetTransaction::AssetId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_date")
                    .table(AssetTransaction::Table)
                    .col(AssetTransaction::TransactionDate)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_transaction_type")
                    .table(AssetTransaction::Table)
                    .col(AssetTransaction::Type)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AssetTransaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AssetTransaction {
    Table,
    Id,
    Type,
    TransactionDate,
    Quantity,
    UnitPrice,
    UnitCostAtTime,
    Note,
    AssetId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Asset {
    Table,
    Id,
}
