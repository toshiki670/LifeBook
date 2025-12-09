// Asset Infrastructure - Manufacturer Repository Implementation

use crate::domain::errors::DomainError;
use crate::domain::manufacturer::{Manufacturer, ManufacturerRepository};

use asset_entity::manufacturer;
use asset_entity::product;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, NotSet, Set, entity::prelude::*,
};

/// ManufacturerRepository の SeaORM実装
pub struct ManufacturerRepositoryImpl {
    db: DatabaseConnection,
}

impl ManufacturerRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// ドメインモデルをDBモデルに変換
    fn domain_to_active_model(manufacturer: &Manufacturer) -> manufacturer::ActiveModel {
        if let Some(id) = manufacturer.id() {
            // 既存のメーカー（更新）
            manufacturer::ActiveModel {
                id: Set(id),
                name: Set(manufacturer.name().to_string()),
                website: Set(manufacturer.website().map(String::from)),
                contact_email: Set(manufacturer.contact_email().map(String::from)),
                deleted_at: NotSet,
                created_at: NotSet,
                updated_at: Set(chrono::Utc::now()),
            }
        } else {
            // 新しいメーカー（作成）
            manufacturer::ActiveModel {
                id: NotSet,
                name: Set(manufacturer.name().to_string()),
                website: Set(manufacturer.website().map(String::from)),
                contact_email: Set(manufacturer.contact_email().map(String::from)),
                deleted_at: NotSet,
                created_at: Set(chrono::Utc::now()),
                updated_at: Set(chrono::Utc::now()),
            }
        }
    }

    /// DBモデルをドメインモデルに変換
    fn db_to_domain(model: manufacturer::Model) -> Manufacturer {
        Manufacturer::reconstruct(model.id, model.name, model.website, model.contact_email)
    }
}

#[async_trait]
impl ManufacturerRepository for ManufacturerRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Manufacturer>, DomainError> {
        let manufacturer = manufacturer::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(manufacturer.map(Self::db_to_domain))
    }

    async fn find_all(&self, include_deleted: bool) -> Result<Vec<Manufacturer>, DomainError> {
        let mut query = manufacturer::Entity::find();

        if !include_deleted {
            query = query.filter(manufacturer::Column::DeletedAt.is_null());
        }

        let manufacturers = query
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(manufacturers.into_iter().map(Self::db_to_domain).collect())
    }

    async fn save(&self, mut manufacturer: Manufacturer) -> Result<Manufacturer, DomainError> {
        let active_model = Self::domain_to_active_model(&manufacturer);

        let result = if manufacturer.id().is_some() {
            // 更新
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
        } else {
            // 新規作成
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
        };

        // 保存後のIDをドメインモデルにセット
        manufacturer.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }

    async fn soft_delete(&self, id: i32) -> Result<(), DomainError> {
        // Product紐付きチェック
        if self.has_products(id).await? {
            return Err(DomainError::CannotDelete(
                "Manufacturer has associated products".to_string(),
            ));
        }

        let manufacturer = manufacturer::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Manufacturer with id {} not found", id))
            })?;

        let mut active_model: manufacturer::ActiveModel = manufacturer.into();
        active_model.deleted_at = Set(Some(chrono::Utc::now()));

        active_model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn has_products(&self, id: i32) -> Result<bool, DomainError> {
        let count = product::Entity::find()
            .filter(product::Column::ManufacturerId.eq(id))
            .filter(product::Column::DeletedAt.is_null())
            .count(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(count > 0)
    }

    async fn find_by_id_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
    ) -> Result<Option<Manufacturer>, DomainError> {
        let manufacturer = manufacturer::Entity::find_by_id(id)
            .one(tx)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(manufacturer.map(Self::db_to_domain))
    }

    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        mut manufacturer: Manufacturer,
    ) -> Result<Manufacturer, DomainError> {
        let active_model = Self::domain_to_active_model(&manufacturer);

        let result = if manufacturer.id().is_some() {
            active_model
                .update(tx)
                .await
                .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
        } else {
            active_model
                .insert(tx)
                .await
                .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
        };

        manufacturer.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }
}
