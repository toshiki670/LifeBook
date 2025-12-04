// Asset Infrastructure - Category Repository Implementation

use crate::domain::entities::category::Category;
use crate::domain::errors::DomainError;
use crate::domain::repositories::category::CategoryRepository;
use asset_entity::{category, product_category};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult, NotSet, QueryOrder,
    Set, Statement, entity::prelude::*,
};

/// CategoryRepository の SeaORM実装
pub struct CategoryRepositoryImpl {
    db: DatabaseConnection,
}

impl CategoryRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// ドメインモデルをDBモデルに変換
    fn domain_to_active_model(category: &Category) -> category::ActiveModel {
        if let Some(id) = category.id() {
            category::ActiveModel {
                id: Set(id),
                name: Set(category.name().to_string()),
                parent_id: Set(category.parent_id()),
                depth: Set(category.depth()),
                created_at: NotSet,
                updated_at: Set(chrono::Utc::now()),
            }
        } else {
            category::ActiveModel {
                id: NotSet,
                name: Set(category.name().to_string()),
                parent_id: Set(category.parent_id()),
                depth: Set(category.depth()),
                created_at: Set(chrono::Utc::now()),
                updated_at: Set(chrono::Utc::now()),
            }
        }
    }

    /// DBモデルをドメインモデルに変換
    fn db_to_domain(model: category::Model) -> Category {
        Category::reconstruct(model.id, model.name, model.parent_id, model.depth)
    }
}

#[async_trait]
impl CategoryRepository for CategoryRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Category>, DomainError> {
        let category = category::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(category.map(Self::db_to_domain))
    }

    async fn find_all(&self) -> Result<Vec<Category>, DomainError> {
        let categories = category::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(categories.into_iter().map(Self::db_to_domain).collect())
    }

    async fn find_roots(&self) -> Result<Vec<Category>, DomainError> {
        let categories = category::Entity::find()
            .filter(category::Column::ParentId.is_null())
            .order_by_asc(category::Column::Name)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(categories.into_iter().map(Self::db_to_domain).collect())
    }

    async fn find_children(&self, parent_id: i32) -> Result<Vec<Category>, DomainError> {
        let categories = category::Entity::find()
            .filter(category::Column::ParentId.eq(parent_id))
            .order_by_asc(category::Column::Name)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(categories.into_iter().map(Self::db_to_domain).collect())
    }

    async fn find_descendants(&self, id: i32) -> Result<Vec<Category>, DomainError> {
        // WITH RECURSIVE を使用して子孫を取得
        let sql = format!(
            r#"
            WITH RECURSIVE descendants AS (
                SELECT id, name, parent_id, depth
                FROM category
                WHERE id = {}
                
                UNION ALL
                
                SELECT c.id, c.name, c.parent_id, c.depth
                FROM category c
                JOIN descendants d ON c.parent_id = d.id
            )
            SELECT id, name, parent_id, depth FROM descendants
            WHERE id != {}
            ORDER BY depth, name
            "#,
            id, id
        );

        #[derive(FromQueryResult)]
        struct CategoryResult {
            id: i32,
            name: String,
            parent_id: Option<i32>,
            depth: i32,
        }

        let results = CategoryResult::find_by_statement(Statement::from_sql_and_values(
            self.db.get_database_backend(),
            &sql,
            vec![],
        ))
        .all(&self.db)
        .await
        .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(results
            .into_iter()
            .map(|r| Category::reconstruct(r.id, r.name, r.parent_id, r.depth))
            .collect())
    }

    async fn find_tree(&self) -> Result<Vec<Category>, DomainError> {
        let categories = category::Entity::find()
            .order_by_asc(category::Column::Depth)
            .order_by_asc(category::Column::Name)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(categories.into_iter().map(Self::db_to_domain).collect())
    }

    async fn save(&self, mut category: Category) -> Result<Category, DomainError> {
        let active_model = Self::domain_to_active_model(&category);

        let result = if category.id().is_some() {
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
        } else {
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
        };

        category.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }

    async fn delete(&self, id: i32) -> Result<(), DomainError> {
        let category = category::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
            .ok_or_else(|| DomainError::NotFound(format!("Category with id {} not found", id)))?;

        let active_model: category::ActiveModel = category.into();
        active_model
            .delete(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn has_products(&self, id: i32) -> Result<bool, DomainError> {
        let count = product_category::Entity::find()
            .filter(product_category::Column::CategoryId.eq(id))
            .count(&self.db)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(count > 0)
    }

    async fn is_descendant_of(
        &self,
        category_id: i32,
        ancestor_id: i32,
    ) -> Result<bool, DomainError> {
        let descendants = self.find_descendants(ancestor_id).await?;
        Ok(descendants.iter().any(|c| c.id() == Some(category_id)))
    }

    async fn update_subtree_depth(&self, id: i32, depth_diff: i32) -> Result<(), DomainError> {
        let sql = format!(
            r#"
            WITH RECURSIVE descendants AS (
                SELECT id FROM category WHERE id = {}
                UNION ALL
                SELECT c.id FROM category c
                JOIN descendants d ON c.parent_id = d.id
            )
            UPDATE category 
            SET depth = depth + {}
            WHERE id IN (SELECT id FROM descendants)
            "#,
            id, depth_diff
        );

        self.db
            .execute(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                &sql,
                vec![],
            ))
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn get_parent_depth(&self, parent_id: Option<i32>) -> Result<i32, DomainError> {
        if let Some(pid) = parent_id {
            let parent = self.find_by_id(pid).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Parent category {} not found", pid))
            })?;
            Ok(parent.depth())
        } else {
            Ok(-1) // ルートカテゴリの場合、depth は 0
        }
    }

    async fn find_by_id_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
    ) -> Result<Option<Category>, DomainError> {
        let category = category::Entity::find_by_id(id)
            .one(tx)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(category.map(Self::db_to_domain))
    }

    async fn find_children_with_tx(
        &self,
        tx: &DatabaseTransaction,
        parent_id: i32,
    ) -> Result<Vec<Category>, DomainError> {
        let categories = category::Entity::find()
            .filter(category::Column::ParentId.eq(parent_id))
            .all(tx)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(categories.into_iter().map(Self::db_to_domain).collect())
    }

    async fn save_with_tx(
        &self,
        tx: &DatabaseTransaction,
        mut category: Category,
    ) -> Result<Category, DomainError> {
        let active_model = Self::domain_to_active_model(&category);

        let result = if category.id().is_some() {
            active_model.update(tx).await
        } else {
            active_model.insert(tx).await
        }
        .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        category.set_id(result.id);

        Ok(Self::db_to_domain(result))
    }

    async fn delete_with_tx(&self, tx: &DatabaseTransaction, id: i32) -> Result<(), DomainError> {
        let category = category::Entity::find_by_id(id)
            .one(tx)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?
            .ok_or_else(|| DomainError::NotFound(format!("Category with id {} not found", id)))?;

        let active_model: category::ActiveModel = category.into();
        active_model
            .delete(tx)
            .await
            .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn update_subtree_depth_with_tx(
        &self,
        tx: &DatabaseTransaction,
        id: i32,
        depth_diff: i32,
    ) -> Result<(), DomainError> {
        let sql = format!(
            r#"
            WITH RECURSIVE descendants AS (
                SELECT id FROM category WHERE id = {}
                UNION ALL
                SELECT c.id FROM category c
                JOIN descendants d ON c.parent_id = d.id
            )
            UPDATE category 
            SET depth = depth + {}
            WHERE id IN (SELECT id FROM descendants)
            "#,
            id, depth_diff
        );

        tx.execute(Statement::from_sql_and_values(
            self.db.get_database_backend(),
            &sql,
            vec![],
        ))
        .await
        .map_err(|e| DomainError::InvalidState(format!("Database error: {}", e)))?;

        Ok(())
    }
}
