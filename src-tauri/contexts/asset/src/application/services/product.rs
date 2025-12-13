// Asset Application - Product Service
// プレースホルダー実装

use crate::application::dto::product::ProductDto;
use crate::application::errors::ApplicationError;
use crate::domain::product::{MeasurementUnit, Product, ProductRepository};

use std::sync::Arc;

pub struct ProductService {
    repository: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_product(
        &self,
        name: String,
        description: Option<String>,
        unit: MeasurementUnit,
        manufacturer_id: i32,
    ) -> Result<ProductDto, ApplicationError> {
        let product = Product::new(name, description, unit, manufacturer_id)?;
        let saved = self.repository.save(product).await?;
        Ok(ProductDto::from(saved))
    }

    pub async fn get_all_products(
        &self,
        include_deleted: bool,
    ) -> Result<Vec<ProductDto>, ApplicationError> {
        let products = self.repository.find_all(include_deleted).await?;
        Ok(products.into_iter().map(ProductDto::from).collect())
    }

    pub async fn get_product(&self, id: i32) -> Result<Option<ProductDto>, ApplicationError> {
        let product = self.repository.find_by_id(id).await?;
        Ok(product.map(ProductDto::from))
    }

    pub async fn update_product(
        &self,
        id: i32,
        name: Option<String>,
        description: Option<String>,
        unit: Option<MeasurementUnit>,
    ) -> Result<ProductDto, ApplicationError> {
        let mut product = self.repository.find_by_id(id).await?.ok_or_else(|| {
            ApplicationError::NotFound(format!("Product with id {} not found", id))
        })?;

        product.update_details(name, description, unit)?;
        let updated = self.repository.save(product).await?;
        Ok(ProductDto::from(updated))
    }

    pub async fn delete_product(&self, id: i32) -> Result<(), ApplicationError> {
        self.repository.find_by_id(id).await?.ok_or_else(|| {
            ApplicationError::NotFound(format!("Product with id {} not found", id))
        })?;

        self.repository.soft_delete(id).await?;
        Ok(())
    }

    pub async fn assign_categories(
        &self,
        product_id: i32,
        category_ids: Vec<i32>,
    ) -> Result<(), ApplicationError> {
        self.repository
            .assign_categories(product_id, category_ids)
            .await?;
        Ok(())
    }
}
