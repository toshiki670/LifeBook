// Settings Domain Layer - Database Settings Entity

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// データベース設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    pub database_directory: PathBuf,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            // プレースホルダー。実際のデフォルト値はRepositoryから注入される
            database_directory: PathBuf::new(),
        }
    }
}

// PathBufのシリアライズヘルパー
fn serialize_pathbuf<S>(path: &std::path::Path, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&path.to_string_lossy())
}

// PathBufのデシリアライズヘルパー
fn deserialize_pathbuf<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(PathBuf::from(s))
}

