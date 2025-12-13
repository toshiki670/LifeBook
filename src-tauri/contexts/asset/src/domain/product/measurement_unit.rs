// Product Domain - Measurement Unit Value Object

use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
}

impl FromStr for MeasurementUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PIECE" => Ok(Self::Piece),
            "BOX" => Ok(Self::Box),
            "PACK" => Ok(Self::Pack),
            "WEIGHT" => Ok(Self::Weight),
            "VOLUME" => Ok(Self::Volume),
            _ => Err(format!("Invalid measurement unit: {}", s)),
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
            "PIECE".parse::<MeasurementUnit>(),
            Ok(MeasurementUnit::Piece)
        );
    }
}
