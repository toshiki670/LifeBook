// Asset Domain - Asset Entity

use crate::domain::errors::DomainError;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Asset エンティティ（ビジネスルールを持つドメインモデル）
#[derive(Debug, Clone, PartialEq)]
pub struct Asset {
    id: Option<i32>,
    name: String,
    description: Option<String>,
    product_id: i32,
    current_quantity: Decimal,
    total_acquisition_cost: Decimal,
}

impl Asset {
    /// 新しい資産を作成（バリデーション付き）
    pub fn new(
        name: String,
        description: Option<String>,
        product_id: i32,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: 名前は必須で、空文字列は不可
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Asset name cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: 名前は200文字以内
        if name.len() > 200 {
            return Err(DomainError::ValidationError(
                "Asset name must be 200 characters or less".to_string(),
            ));
        }

        Ok(Self {
            id: None,
            name: name.trim().to_string(),
            description: description
                .map(|d| d.trim().to_string())
                .filter(|d| !d.is_empty()),
            product_id,
            current_quantity: dec!(0),
            total_acquisition_cost: dec!(0),
        })
    }

    /// 既存の資産を再構築（DB から取得した場合など）
    pub fn reconstruct(
        id: i32,
        name: String,
        description: Option<String>,
        product_id: i32,
        current_quantity: Decimal,
        total_acquisition_cost: Decimal,
    ) -> Self {
        Self {
            id: Some(id),
            name,
            description,
            product_id,
            current_quantity,
            total_acquisition_cost,
        }
    }

    /// 資産名・説明を更新（バリデーション付き）
    pub fn update_details(
        &mut self,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<(), DomainError> {
        // 名前更新
        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(DomainError::ValidationError(
                    "Asset name cannot be empty".to_string(),
                ));
            }
            if new_name.len() > 200 {
                return Err(DomainError::ValidationError(
                    "Asset name must be 200 characters or less".to_string(),
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

        Ok(())
    }

    /// 購入を追加（移動平均法）
    pub fn add_purchase(
        &mut self,
        quantity: Decimal,
        unit_price: Decimal,
    ) -> Result<(), DomainError> {
        // バリデーション
        if quantity <= dec!(0) {
            return Err(DomainError::ValidationError(
                "Quantity must be greater than zero".to_string(),
            ));
        }
        if unit_price < dec!(0) {
            return Err(DomainError::ValidationError(
                "Unit price must be non-negative".to_string(),
            ));
        }

        // checked演算で安全に計算
        let purchase_cost = quantity
            .checked_mul(unit_price)
            .ok_or(DomainError::ArithmeticError)?;

        self.current_quantity = self
            .current_quantity
            .checked_add(quantity)
            .ok_or(DomainError::ArithmeticError)?;

        self.total_acquisition_cost = self
            .total_acquisition_cost
            .checked_add(purchase_cost)
            .ok_or(DomainError::ArithmeticError)?;

        Ok(())
    }

    /// 売却を追加（移動平均法で原価を按分）
    pub fn add_sale(&mut self, quantity: Decimal) -> Result<Decimal, DomainError> {
        // バリデーション
        if quantity <= dec!(0) {
            return Err(DomainError::ValidationError(
                "Quantity must be greater than zero".to_string(),
            ));
        }
        if quantity > self.current_quantity {
            return Err(DomainError::InsufficientQuantity);
        }

        // 平均取得単価で按分
        let avg_unit_cost = self.average_unit_cost();
        let cost_to_remove = quantity
            .checked_mul(avg_unit_cost)
            .ok_or(DomainError::ArithmeticError)?;

        self.current_quantity = self
            .current_quantity
            .checked_sub(quantity)
            .ok_or(DomainError::ArithmeticError)?;

        self.total_acquisition_cost = self
            .total_acquisition_cost
            .checked_sub(cost_to_remove)
            .ok_or(DomainError::ArithmeticError)?;

        Ok(avg_unit_cost)
    }

    /// 廃棄を追加（売却と同じロジック）
    pub fn add_disposal(&mut self, quantity: Decimal) -> Result<Decimal, DomainError> {
        self.add_sale(quantity)
    }

    /// 平均取得単価を計算
    pub fn average_unit_cost(&self) -> Decimal {
        if self.current_quantity.is_zero() {
            dec!(0)
        } else {
            self.total_acquisition_cost / self.current_quantity
        }
    }

    /// 所有中かどうか
    pub fn is_owned(&self) -> bool {
        self.current_quantity > dec!(0)
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

    pub fn product_id(&self) -> i32 {
        self.product_id
    }

    pub fn current_quantity(&self) -> Decimal {
        self.current_quantity
    }

    pub fn total_acquisition_cost(&self) -> Decimal {
        self.total_acquisition_cost
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
    fn test_create_valid_asset() {
        let asset = Asset::new("MacBook Pro 2024".to_string(), None, 1);
        assert!(asset.is_ok());
    }

    #[test]
    fn test_purchase_updates_quantity_and_cost() {
        let mut asset = Asset::new("Test".to_string(), None, 1).unwrap();
        asset.add_purchase(dec!(2), dec!(5000)).unwrap();

        assert_eq!(asset.current_quantity(), dec!(2));
        assert_eq!(asset.total_acquisition_cost(), dec!(10000));
        assert_eq!(asset.average_unit_cost(), dec!(5000));
    }

    #[test]
    fn test_moving_average() {
        let mut asset = Asset::new("Test".to_string(), None, 1).unwrap();

        // 購入1: 2個 @ 5,000円
        asset.add_purchase(dec!(2), dec!(5000)).unwrap();
        assert_eq!(asset.average_unit_cost(), dec!(5000));

        // 購入2: 3個 @ 4,000円
        asset.add_purchase(dec!(3), dec!(4000)).unwrap();
        assert_eq!(asset.current_quantity(), dec!(5));
        assert_eq!(asset.total_acquisition_cost(), dec!(22000));
        assert_eq!(asset.average_unit_cost(), dec!(4400));
    }

    #[test]
    fn test_sale_reduces_quantity_and_cost() {
        let mut asset = Asset::new("Test".to_string(), None, 1).unwrap();
        asset.add_purchase(dec!(5), dec!(4400)).unwrap();

        let avg_cost = asset.add_sale(dec!(1)).unwrap();

        assert_eq!(avg_cost, dec!(4400));
        assert_eq!(asset.current_quantity(), dec!(4));
        assert_eq!(asset.total_acquisition_cost(), dec!(17600));
    }

    #[test]
    fn test_insufficient_quantity_fails() {
        let mut asset = Asset::new("Test".to_string(), None, 1).unwrap();
        asset.add_purchase(dec!(2), dec!(5000)).unwrap();

        let result = asset.add_sale(dec!(3));
        assert!(result.is_err());
    }
}
