// Settings Domain Layer - General Settings Entity

use crate::domain::value_objects::Language;
use serde::{Deserialize, Serialize};

/// 一般設定
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralSettings {
    pub language: Language,
}
