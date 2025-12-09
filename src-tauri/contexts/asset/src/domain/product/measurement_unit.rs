// Product Domain - Measurement Unit Value Object

use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement_unit_conversion() {
        assert_eq!(MeasurementUnit::Piece.as_str(), "PIECE");
        assert_eq!(
            MeasurementUnit::from_str("PIECE"),
            Some(MeasurementUnit::Piece)
        );
    }
}
