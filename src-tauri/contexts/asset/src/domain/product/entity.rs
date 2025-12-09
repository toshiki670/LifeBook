// Asset Domain - Product Entity

use super::measurement_unit::MeasurementUnit;
use crate::domain::errors::DomainError;

/// Product エンティティ（ビジネスルールを持つドメインモデル）
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    id: Option<i32>,
    name: String,
    description: Option<String>,
    unit: MeasurementUnit,
    manufacturer_id: i32,
}

impl Product {
    /// 新しい商品を作成（バリデーション付き）
    pub fn new(
        name: String,
        description: Option<String>,
        unit: MeasurementUnit,
        manufacturer_id: i32,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: 名前は必須で、空文字列は不可
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Product name cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: 名前は200文字以内
        if name.len() > 200 {
            return Err(DomainError::ValidationError(
                "Product name must be 200 characters or less".to_string(),
            ));
        }

        Ok(Self {
            id: None,
            name: name.trim().to_string(),
            description: description
                .map(|d| d.trim().to_string())
                .filter(|d| !d.is_empty()),
            unit,
            manufacturer_id,
        })
    }

    /// 既存の商品を再構築（DB から取得した場合など）
    pub fn reconstruct(
        id: i32,
        name: String,
        description: Option<String>,
        unit: MeasurementUnit,
        manufacturer_id: i32,
    ) -> Self {
        Self {
            id: Some(id),
            name,
            description,
            unit,
            manufacturer_id,
        }
    }

    /// 商品の詳細を更新（バリデーション付き）
    pub fn update_details(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        unit: Option<MeasurementUnit>,
    ) -> Result<(), DomainError> {
        // 名前更新
        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(DomainError::ValidationError(
                    "Product name cannot be empty".to_string(),
                ));
            }
            if new_name.len() > 200 {
                return Err(DomainError::ValidationError(
                    "Product name must be 200 characters or less".to_string(),
                ));
            }
            self.name = new_name.trim().to_string();
        }

        // 説明更新
        if let Some(new_description) = description {
            self.description = if new_description.trim().is_empty() {
                None
            } else {
                Some(new_description.trim().to_string())
            };
        }

        // 単位更新
        if let Some(new_unit) = unit {
            self.unit = new_unit;
        }

        Ok(())
    }

    // Getters
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn unit(&self) -> MeasurementUnit {
        self.unit
    }

    pub fn manufacturer_id(&self) -> i32 {
        self.manufacturer_id
    }

    // ID をセット（リポジトリで保存後に使用）
    pub(crate) fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_product() {
        let product = Product::new(
            "MacBook Pro".to_string(),
            Some("14-inch".to_string()),
            MeasurementUnit::Piece,
            1,
        );
        assert!(product.is_ok());
    }

    #[test]
    fn test_empty_name_fails() {
        let product = Product::new("".to_string(), None, MeasurementUnit::Piece, 1);
        assert!(product.is_err());
    }

    #[test]
    fn test_long_name_fails() {
        let long_name = "a".repeat(201);
        let product = Product::new(long_name, None, MeasurementUnit::Piece, 1);
        assert!(product.is_err());
    }
}
