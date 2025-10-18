// Settings Domain Layer - Appearance Settings Entity

use crate::domain::value_objects::Theme;
use serde::{Deserialize, Serialize};

/// 表示設定
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppearanceSettings {
    pub theme: Theme,
}

