use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(pk_auto(Category::Id))
                    .col(string_len(Category::Name, 100).not_null())
                    .col(integer_null(Category::ParentId))
                    .col(integer(Category::Depth).not_null().default(0))
                    .col(
                        timestamp_with_time_zone(Category::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Category::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_parent")
                            .from(Category::Table, Category::ParentId)
                            .to(Category::Table, Category::Id)
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
                    .from(Category::Table)
                    .and_where(Expr::cust("depth >= 0"))
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::select()
                    .expr(Expr::cust("1"))
                    .from(Category::Table)
                    .and_where(Expr::cust("depth <= 10"))
                    .to_owned(),
            )
            .await?;

        // インデックス作成
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_category_parent_id")
                    .table(Category::Table)
                    .col(Category::ParentId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_category_depth")
                    .table(Category::Table)
                    .col(Category::Depth)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    Name,
    ParentId,
    Depth,
    CreatedAt,
    UpdatedAt,
}
