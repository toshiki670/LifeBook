// Asset Domain - Transaction Type Value Object

use serde::{Deserialize, Serialize};

/// Transaction Type - トランザクション種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Purchase,   // 購入
    Sale,       // 売却
    Disposal,   // 廃棄
    Adjustment, // 調整
}

impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Purchase => "PURCHASE",
            Self::Sale => "SALE",
            Self::Disposal => "DISPOSAL",
            Self::Adjustment => "ADJUSTMENT",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PURCHASE" => Some(Self::Purchase),
            "SALE" => Some(Self::Sale),
            "DISPOSAL" => Some(Self::Disposal),
            "ADJUSTMENT" => Some(Self::Adjustment),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_type_conversion() {
        assert_eq!(TransactionType::Purchase.as_str(), "PURCHASE");
        assert_eq!(
            TransactionType::from_str("PURCHASE"),
            Some(TransactionType::Purchase)
        );
    }
}
