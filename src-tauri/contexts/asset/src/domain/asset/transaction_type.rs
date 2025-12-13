// Asset Domain - Transaction Type Value Object

use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PURCHASE" => Ok(Self::Purchase),
            "SALE" => Ok(Self::Sale),
            "DISPOSAL" => Ok(Self::Disposal),
            "ADJUSTMENT" => Ok(Self::Adjustment),
            _ => Err(format!("Invalid transaction type: {}", s)),
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
            "PURCHASE".parse::<TransactionType>(),
            Ok(TransactionType::Purchase)
        );
    }
}
