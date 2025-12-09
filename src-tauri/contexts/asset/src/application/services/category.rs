// Asset Application - Category Service
// プレースホルダー実装

use crate::application::dto::category::CategoryDto;
use crate::application::errors::ApplicationError;
use crate::domain::category::{Category, CategoryRepository};

use sea_orm::{DatabaseConnection, TransactionTrait};
use std::sync::Arc;

pub struct CategoryService {
    db: DatabaseConnection,
    repository: Arc<dyn CategoryRepository>,
}

impl CategoryService {
    pub fn new(db: DatabaseConnection, repository: Arc<dyn CategoryRepository>) -> Self {
        Self { db, repository }
    }

    pub async fn create_category(
        &self,
        name: String,
        parent_id: Option<i32>,
    ) -> Result<CategoryDto, ApplicationError> {
        let mut category = Category::new(name, parent_id)?;
        let depth = self.repository.get_parent_depth(parent_id).await? + 1;
        category.set_depth(depth);
        let saved = self.repository.save(category).await?;
        Ok(CategoryDto::from(saved))
    }

    pub async fn get_all_categories(&self) -> Result<Vec<CategoryDto>, ApplicationError> {
        let categories = self.repository.find_all().await?;
        Ok(categories.into_iter().map(CategoryDto::from).collect())
    }

    pub async fn get_category(&self, id: i32) -> Result<Option<CategoryDto>, ApplicationError> {
        let category = self.repository.find_by_id(id).await?;
        Ok(category.map(CategoryDto::from))
    }

    pub async fn update_category(
        &self,
        id: i32,
        name: Option<String>,
    ) -> Result<CategoryDto, ApplicationError> {
        let mut category = self.repository.find_by_id(id).await?.ok_or_else(|| {
            ApplicationError::NotFound(format!("Category with id {} not found", id))
        })?;

        if let Some(new_name) = name {
            category.update_name(new_name)?;
        }

        let updated = self.repository.save(category).await?;
        Ok(CategoryDto::from(updated))
    }

    pub async fn delete_category(&self, id: i32) -> Result<(), ApplicationError> {
        // Product紐付きチェック
        if self.repository.has_products(id).await? {
            return Err(ApplicationError::CannotDelete(
                "Category has associated products".to_string(),
            ));
        }

        // トランザクション開始
        let tx = self.db.begin().await.map_err(|e| {
            ApplicationError::DatabaseError(format!("Failed to begin transaction: {}", e))
        })?;

        let category = self
            .repository
            .find_by_id_with_tx(&tx, id)
            .await?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Category with id {} not found", id))
            })?;

        // 子カテゴリを取得
        let children = self.repository.find_children_with_tx(&tx, id).await?;

        // 子カテゴリを親に昇格
        for mut child in children {
            child.change_parent(category.parent_id(), category.depth() - 1)?;
            self.repository.save_with_tx(&tx, child.clone()).await?;
            // 子孫のdepthを-1
            self.repository
                .update_subtree_depth_with_tx(&tx, child.id().unwrap(), -1)
                .await?;
        }

        // カテゴリを削除
        self.repository.delete_with_tx(&tx, id).await?;

        tx.commit().await.map_err(|e| {
            ApplicationError::DatabaseError(format!("Failed to commit transaction: {}", e))
        })?;

        Ok(())
    }
}
