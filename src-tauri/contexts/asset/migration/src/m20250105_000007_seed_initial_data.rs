use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // デフォルトカテゴリを挿入
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Category::Table)
                    .columns([Category::Name, Category::ParentId, Category::Depth])
                    .values_panic(["家電".into(), Option::<i32>::None.into(), 0.into()])
                    .values_panic(["衣類".into(), Option::<i32>::None.into(), 0.into()])
                    .values_panic(["書籍".into(), Option::<i32>::None.into(), 0.into()])
                    .values_panic(["家具".into(), Option::<i32>::None.into(), 0.into()])
                    .values_panic(["その他".into(), Option::<i32>::None.into(), 0.into()])
                    .to_owned(),
            )
            .await?;

        // デフォルトメーカーを挿入
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Manufacturer::Table)
                    .columns([Manufacturer::Name])
                    .values_panic(["不明".into()])
                    .values_panic(["ノーブランド".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // データを削除
        manager
            .exec_stmt(
                Query::delete()
                    .from_table(Category::Table)
                    .and_where(Expr::col(Category::Name).is_in([
                        "家電",
                        "衣類",
                        "書籍",
                        "家具",
                        "その他",
                    ]))
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::delete()
                    .from_table(Manufacturer::Table)
                    .and_where(Expr::col(Manufacturer::Name).is_in(["不明", "ノーブランド"]))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Name,
    ParentId,
    Depth,
}

#[derive(DeriveIden)]
enum Manufacturer {
    Table,
    Name,
}
