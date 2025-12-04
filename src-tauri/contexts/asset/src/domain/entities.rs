// Asset Context - Domain Entities

pub mod asset;
pub mod asset_transaction;
pub mod category;
pub mod manufacturer;
pub mod product;

// Value Objects (Enums)
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

/// Measurement Unit - 測定単位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeasurementUnit {
    Piece,  // 個
    Box,    // 箱
    Pack,   // パック
    Weight, // 重量
    Volume, // 容量
}

impl MeasurementUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Piece => "PIECE",
            Self::Box => "BOX",
            Self::Pack => "PACK",
            Self::Weight => "WEIGHT",
            Self::Volume => "VOLUME",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PIECE" => Some(Self::Piece),
            "BOX" => Some(Self::Box),
            "PACK" => Some(Self::Pack),
            "WEIGHT" => Some(Self::Weight),
            "VOLUME" => Some(Self::Volume),
            _ => None,
        }
    }
}
