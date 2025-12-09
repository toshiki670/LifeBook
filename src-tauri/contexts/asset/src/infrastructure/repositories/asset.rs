// Asset Infrastructure - Asset Repository Implementation

use crate::domain::asset::{Asset, AssetRepository};
use crate::domain::errors::DomainError;

use asset_entity::asset;
use async_trait::async_trait;
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, NotSet, Set, entity::prelude::*,
};

pub struct AssetRepositoryImpl {
    db: DatabaseConnection,
}

impl AssetRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn domain_to_active_model(asset: &Asset) -> asset::ActiveModel {
        if let Some(id) = asset.id() {
            asset::ActiveModel {
                id: Set(id),
                name: Set(asset.name().to_string()),
                description: Set(asset.description().map(String::from)),
                product_id: Set(asset.product_id()),
                current_quantity: Set(asset.current_quantity()),
                total_acquisition_cost: Set(asset.total_acquisition_cost()),
                created_at: NotSet,
                updated_at: Set(chrono::Utc::now()),
            }
        } else {
            asset::ActiveModel {
                id: NotSet,
                name: Set(asset.name().to_string()),
                description: Set(asset.description().map(String::from)),
                product_id: Set(asset.product_id()),
                current_quantity: Set(asset.current_quantity()),
                total_acquisition_cost: Set(asset.total_acquisition_cost()),
                created_at: Set(chrono::Utc::now()),
                updated_at: Set(chrono::Utc::now()),
            }
        }
    }

    fn db_to_domain(model: asset::Model) -> Asset {
        Asset::reconstruct(
            model.id,
            model.name,
            model.description,
            model.product_id,
            model.current_quantity,
            model.total_acquisition_cost,
        )
    }
}

#[async_trait]
impl AssetRepository for AssetRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Asset>, DomainError> {
        let asset = asset::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(asset.map(Self::db_to_domain))
    }

    async fn find_all(&self, owned_only: bool) -> Result<Vec<Asset>, DomainError> {
        let mut query = asset::Entity::find();

        if owned_only {
            query = query.filter(asset::Column::CurrentQuantity.gt(Decimal::ZERO));
        }

        let assets = query
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(assets.into_iter().map(Self::db_to_domain).collect())
    }

    async fn find_by_product_id(&self, product_id: i32) -> Result<Option<Asset>, DomainError> {
        let asset = asset::Entity::find()
            .filter(asset::Column::ProductId.eq(product_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(asset.map(Self::db_to_domain))
    }

    async fn save(&self, mut asset: Asset) -> Result<Asset, DomainError> {
        let active_model = Self::domain_to_active_model(&asset);

        let result = if asset.id().is_some() {
            active_model.update(&self.db).await
        } else {
            active_model.insert(&self.db).await
        }
        .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        asset.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }

    async fn find_by_id_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
    ) -> Result<Option<Asset>, DomainError> {
        let asset = asset::Entity::find_by_id(id)
            .one(tx)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(asset.map(Self::db_to_domain))
    }

    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        mut asset: Asset,
    ) -> Result<Asset, DomainError> {
        let active_model = Self::domain_to_active_model(&asset);

        let result = if asset.id().is_some() {
            active_model.update(tx).await
        } else {
            active_model.insert(tx).await
        }
        .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        asset.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }
}
