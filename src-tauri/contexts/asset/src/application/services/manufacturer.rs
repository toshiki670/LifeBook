// Asset Application - Manufacturer Service

use crate::application::dto::manufacturer::ManufacturerDto;
use crate::application::errors::ApplicationError;
use crate::domain::manufacturer::{Manufacturer, ManufacturerRepository};

use std::sync::Arc;

pub struct ManufacturerService {
    repository: Arc<dyn ManufacturerRepository>,
}

impl ManufacturerService {
    pub fn new(repository: Arc<dyn ManufacturerRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_manufacturer(
        &self,
        name: String,
        website: Option<String>,
        contact_email: Option<String>,
    ) -> Result<ManufacturerDto, ApplicationError> {
        let manufacturer = Manufacturer::new(name, website, contact_email)?;
        let saved = self.repository.save(manufacturer).await?;
        Ok(ManufacturerDto::from(saved))
    }

    pub async fn get_all_manufacturers(
        &self,
        include_deleted: bool,
    ) -> Result<Vec<ManufacturerDto>, ApplicationError> {
        let manufacturers = self.repository.find_all(include_deleted).await?;
        Ok(manufacturers
            .into_iter()
            .map(ManufacturerDto::from)
            .collect())
    }

    pub async fn get_manufacturer(
        &self,
        id: i32,
    ) -> Result<Option<ManufacturerDto>, ApplicationError> {
        let manufacturer = self.repository.find_by_id(id).await?;
        Ok(manufacturer.map(ManufacturerDto::from))
    }

    pub async fn update_manufacturer(
        &self,
        id: i32,
        name: Option<String>,
        website: Option<String>,
        contact_email: Option<String>,
    ) -> Result<ManufacturerDto, ApplicationError> {
        let mut manufacturer = self.repository.find_by_id(id).await?.ok_or_else(|| {
            ApplicationError::NotFound(format!("Manufacturer with id {} not found", id))
        })?;

        manufacturer.update_details(name, website, contact_email)?;
        let updated = self.repository.save(manufacturer).await?;
        Ok(ManufacturerDto::from(updated))
    }

    pub async fn delete_manufacturer(&self, id: i32) -> Result<(), ApplicationError> {
        self.repository.find_by_id(id).await?.ok_or_else(|| {
            ApplicationError::NotFound(format!("Manufacturer with id {} not found", id))
        })?;

        self.repository.soft_delete(id).await?;
        Ok(())
    }
}
