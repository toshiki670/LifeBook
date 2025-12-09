// Asset Domain - AssetTransaction Entity

use super::transaction_type::TransactionType;
use crate::domain::errors::DomainError;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// AssetTransaction エンティティ（ビジネスルールを持つドメインモデル）
#[derive(Debug, Clone, PartialEq)]
pub struct AssetTransaction {
    id: Option<i32>,
    type_: TransactionType,
    transaction_date: NaiveDate,
    quantity: Decimal,
    unit_price: Option<Decimal>,
    unit_cost_at_time: Option<Decimal>,
    note: Option<String>,
    asset_id: i32,
}

impl AssetTransaction {
    /// 新しいトランザクションを作成（バリデーション付き）
    pub fn new(
        type_: TransactionType,
        transaction_date: NaiveDate,
        quantity: Decimal,
        unit_price: Option<Decimal>,
        unit_cost_at_time: Option<Decimal>,
        note: Option<String>,
        asset_id: i32,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: 数量は0より大きい
        if quantity <= dec!(0) {
            return Err(DomainError::ValidationError(
                "Quantity must be greater than zero".to_string(),
            ));
        }

        // ビジネスルール: PURCHASE/SALE時はunit_priceが必須
        if matches!(type_, TransactionType::Purchase | TransactionType::Sale) {
            if unit_price.is_none() {
                return Err(DomainError::ValidationError(
                    "Unit price is required for purchase/sale".to_string(),
                ));
            }
            if let Some(price) = unit_price
                && price < dec!(0)
            {
                return Err(DomainError::ValidationError(
                    "Unit price must be non-negative".to_string(),
                ));
            }
        }

        // ビジネスルール: 未来日不可
        let today = chrono::Utc::now().date_naive();
        if transaction_date > today {
            return Err(DomainError::ValidationError(
                "Transaction date cannot be in the future".to_string(),
            ));
        }

        Ok(Self {
            id: None,
            type_,
            transaction_date,
            quantity,
            unit_price,
            unit_cost_at_time,
            note: note.map(|n| n.trim().to_string()).filter(|n| !n.is_empty()),
            asset_id,
        })
    }

    /// 既存のトランザクションを再構築（DB から取得した場合など）
    pub fn reconstruct(
        id: i32,
        type_: TransactionType,
        transaction_date: NaiveDate,
        quantity: Decimal,
        unit_price: Option<Decimal>,
        unit_cost_at_time: Option<Decimal>,
        note: Option<String>,
        asset_id: i32,
    ) -> Self {
        Self {
            id: Some(id),
            type_,
            transaction_date,
            quantity,
            unit_price,
            unit_cost_at_time,
            note,
            asset_id,
        }
    }

    /// 売却損益を計算（売却時のみ）
    pub fn profit_loss(&self) -> Option<Decimal> {
        if self.type_ == TransactionType::Sale
            && let (Some(price), Some(cost)) = (self.unit_price, self.unit_cost_at_time)
        {
            return Some(price - cost);
        }
        None
    }

    // Getters
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn type_(&self) -> TransactionType {
        self.type_
    }

    pub fn transaction_date(&self) -> NaiveDate {
        self.transaction_date
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }

    pub fn unit_price(&self) -> Option<Decimal> {
        self.unit_price
    }

    pub fn unit_cost_at_time(&self) -> Option<Decimal> {
        self.unit_cost_at_time
    }

    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }

    pub fn asset_id(&self) -> i32 {
        self.asset_id
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
    fn test_create_valid_purchase() {
        let today = chrono::Utc::now().date_naive();
        let transaction = AssetTransaction::new(
            TransactionType::Purchase,
            today,
            dec!(2),
            Some(dec!(5000)),
            Some(dec!(5000)),
            None,
            1,
        );
        assert!(transaction.is_ok());
    }

    #[test]
    fn test_zero_quantity_fails() {
        let today = chrono::Utc::now().date_naive();
        let transaction = AssetTransaction::new(
            TransactionType::Purchase,
            today,
            dec!(0),
            Some(dec!(5000)),
            None,
            None,
            1,
        );
        assert!(transaction.is_err());
    }

    #[test]
    fn test_purchase_without_price_fails() {
        let today = chrono::Utc::now().date_naive();
        let transaction = AssetTransaction::new(
            TransactionType::Purchase,
            today,
            dec!(2),
            None,
            None,
            None,
            1,
        );
        assert!(transaction.is_err());
    }

    #[test]
    fn test_future_date_fails() {
        let future = chrono::Utc::now().date_naive() + chrono::Duration::days(1);
        let transaction = AssetTransaction::new(
            TransactionType::Purchase,
            future,
            dec!(2),
            Some(dec!(5000)),
            None,
            None,
            1,
        );
        assert!(transaction.is_err());
    }

    #[test]
    fn test_profit_loss_calculation() {
        let today = chrono::Utc::now().date_naive();
        let transaction = AssetTransaction::new(
            TransactionType::Sale,
            today,
            dec!(1),
            Some(dec!(3500)),
            Some(dec!(4400)),
            None,
            1,
        )
        .unwrap();

        assert_eq!(transaction.profit_loss(), Some(dec!(-900)));
    }
}
