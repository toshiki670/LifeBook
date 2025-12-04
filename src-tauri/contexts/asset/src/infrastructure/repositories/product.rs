// Asset Infrastructure - Product Repository Implementation
// プレースホルダー実装 - 実際の実装は後で追加

use crate::domain::entities::{MeasurementUnit, product::Product};
use crate::domain::errors::DomainError;
use crate::domain::repositories::product::ProductRepository;
use asset_entity::{asset, product, product_category};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, NotSet, Set, entity::prelude::*,
};

pub struct ProductRepositoryImpl {
    db: DatabaseConnection,
}

impl ProductRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn domain_to_active_model(product: &Product) -> product::ActiveModel {
        if let Some(id) = product.id() {
            product::ActiveModel {
                id: Set(id),
                name: Set(product.name().to_string()),
                description: Set(product.description().map(String::from)),
                unit: Set(product.unit().as_str().to_string()),
                manufacturer_id: Set(product.manufacturer_id()),
                deleted_at: NotSet,
                created_at: NotSet,
                updated_at: Set(chrono::Utc::now()),
            }
        } else {
            product::ActiveModel {
                id: NotSet,
                name: Set(product.name().to_string()),
                description: Set(product.description().map(String::from)),
                unit: Set(product.unit().as_str().to_string()),
                manufacturer_id: Set(product.manufacturer_id()),
                deleted_at: NotSet,
                created_at: Set(chrono::Utc::now()),
                updated_at: Set(chrono::Utc::now()),
            }
        }
    }

    fn db_to_domain(model: product::Model) -> Product {
        let unit = MeasurementUnit::from_str(&model.unit).unwrap_or(MeasurementUnit::Piece);
        Product::reconstruct(
            model.id,
            model.name,
            model.description,
            unit,
            model.manufacturer_id,
        )
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Product>, DomainError> {
        let product = product::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(product.map(Self::db_to_domain))
    }

    async fn find_all(&self, include_deleted: bool) -> Result<Vec<Product>, DomainError> {
        let mut query = product::Entity::find();

        if !include_deleted {
            query = query.filter(product::Column::DeletedAt.is_null());
        }

        let products = query
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(products.into_iter().map(Self::db_to_domain).collect())
    }

    async fn find_by_category(
        &self,
        category_id: i32,
        include_deleted: bool,
    ) -> Result<Vec<Product>, DomainError> {
        let product_ids: Vec<i32> = product_category::Entity::find()
            .filter(product_category::Column::CategoryId.eq(category_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
            .into_iter()
            .map(|pc| pc.product_id)
            .collect();

        if product_ids.is_empty() {
            return Ok(vec![]);
        }

        let mut query = product::Entity::find().filter(product::Column::Id.is_in(product_ids));

        if !include_deleted {
            query = query.filter(product::Column::DeletedAt.is_null());
        }

        let products = query
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(products.into_iter().map(Self::db_to_domain).collect())
    }

    async fn save(&self, mut product: Product) -> Result<Product, DomainError> {
        let active_model = Self::domain_to_active_model(&product);

        let result = if product.id().is_some() {
            active_model.update(&self.db).await
        } else {
            active_model.insert(&self.db).await
        }
        .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        product.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }

    async fn soft_delete(&self, id: i32) -> Result<(), DomainError> {
        if self.has_assets(id).await? {
            return Err(DomainError::CannotDelete(
                "Product has associated assets".to_string(),
            ));
        }

        let product = product::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
            .ok_or_else(|| DomainError::NotFound(format!("Product with id {} not found", id)))?;

        let mut active_model: product::ActiveModel = product.into();
        active_model.deleted_at = Set(Some(chrono::Utc::now()));

        active_model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn has_assets(&self, id: i32) -> Result<bool, DomainError> {
        let count = asset::Entity::find()
            .filter(asset::Column::ProductId.eq(id))
            .count(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(count > 0)
    }

    async fn assign_categories(
        &self,
        product_id: i32,
        category_ids: Vec<i32>,
    ) -> Result<(), DomainError> {
        // 既存の紐付けを削除
        product_category::Entity::delete_many()
            .filter(product_category::Column::ProductId.eq(product_id))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        // 新しい紐付けを作成
        for category_id in category_ids {
            let pc = product_category::ActiveModel {
                id: NotSet,
                product_id: Set(product_id),
                category_id: Set(category_id),
                created_at: Set(chrono::Utc::now()),
            };

            pc.insert(&self.db)
                .await
                .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;
        }

        Ok(())
    }

    async fn get_categories(&self, product_id: i32) -> Result<Vec<i32>, DomainError> {
        let categories = product_category::Entity::find()
            .filter(product_category::Column::ProductId.eq(product_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(categories.into_iter().map(|pc| pc.category_id).collect())
    }

    async fn find_by_id_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
    ) -> Result<Option<Product>, DomainError> {
        let product = product::Entity::find_by_id(id)
            .one(tx)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(product.map(Self::db_to_domain))
    }

    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        mut product: Product,
    ) -> Result<Product, DomainError> {
        let active_model = Self::domain_to_active_model(&product);

        let result = if product.id().is_some() {
            active_model.update(tx).await
        } else {
            active_model.insert(tx).await
        }
        .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        product.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }
}
