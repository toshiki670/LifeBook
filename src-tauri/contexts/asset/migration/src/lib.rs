pub use sea_orm_migration::prelude::*;

mod m20250105_000001_create_manufacturer;
mod m20250105_000002_create_category;
mod m20250105_000003_create_product;
mod m20250105_000004_create_product_category;
mod m20250105_000005_create_asset;
mod m20250105_000006_create_asset_transaction;
mod m20250105_000007_seed_initial_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250105_000001_create_manufacturer::Migration),
            Box::new(m20250105_000002_create_category::Migration),
            Box::new(m20250105_000003_create_product::Migration),
            Box::new(m20250105_000004_create_product_category::Migration),
            Box::new(m20250105_000005_create_asset::Migration),
            Box::new(m20250105_000006_create_asset_transaction::Migration),
            Box::new(m20250105_000007_seed_initial_data::Migration),
        ]
    }
}
