// Settings Infrastructure Layer - Settings Repository Implementation

use crate::domain::{entities::Settings, repositories::SettingsRepository};
use async_trait::async_trait;
use shared::domain::errors::DomainError;
use std::path::PathBuf;
use tokio::fs;

const SETTINGS_FILE_NAME: &str = "settings.json";

/// ファイルシステムベースの設定リポジトリ実装
pub struct SettingsRepositoryImpl {
    config_dir: PathBuf,
    default_db_dir: PathBuf,
}

impl SettingsRepositoryImpl {
    /// 新しいリポジトリインスタンスを作成
    pub fn new(config_dir: PathBuf, default_db_dir: PathBuf) -> Self {
        Self {
            config_dir,
            default_db_dir,
        }
    }

    /// 設定ファイルのパスを取得
    fn settings_file_path(&self) -> PathBuf {
        self.config_dir.join(SETTINGS_FILE_NAME)
    }
}

#[async_trait]
impl SettingsRepository for SettingsRepositoryImpl {
    async fn load(&self) -> Result<Settings, DomainError> {
        let file_path = self.settings_file_path();

        // ファイルが存在しない場合はデフォルト設定を返す（デフォルトDBディレクトリを注入）
        if !file_path.exists() {
            let mut settings = Settings::default();
            settings.database.database_directory = self.default_db_dir.clone();
            return Ok(settings);
        }

        // ファイルを読み込む
        let content = fs::read_to_string(&file_path).await.map_err(|e| {
            DomainError::IoError(format!("Failed to read settings file: {}", e))
        })?;

        // JSONをパース
        let settings: Settings = serde_json::from_str(&content).map_err(|e| {
            DomainError::SerializationError(format!("Failed to parse settings file: {}", e))
        })?;

        Ok(settings)
    }

    async fn save(&self, settings: &Settings) -> Result<(), DomainError> {
        // 設定ディレクトリが存在しない場合は作成
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir).await.map_err(|e| {
                DomainError::IoError(format!("Failed to create config directory: {}", e))
            })?;
        }

        let file_path = self.settings_file_path();

        // 設定をJSONに変換（pretty print）
        let content = serde_json::to_string_pretty(settings).map_err(|e| {
            DomainError::SerializationError(format!("Failed to serialize settings: {}", e))
        })?;

        // ファイルに書き込む
        fs::write(&file_path, content).await.map_err(|e| {
            DomainError::IoError(format!("Failed to write settings file: {}", e))
        })?;

        Ok(())
    }

    async fn delete(&self) -> Result<(), DomainError> {
        let file_path = self.settings_file_path();

        if file_path.exists() {
            fs::remove_file(&file_path).await.map_err(|e| {
                DomainError::IoError(format!("Failed to delete settings file: {}", e))
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::Language;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_load_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repo =
            SettingsRepositoryImpl::new(temp_dir.path().to_path_buf(), default_db_dir.clone());

        let settings = repo.load().await.unwrap();
        assert_eq!(settings.general.language, Language::Japanese);
        assert_eq!(settings.database.database_directory, default_db_dir);
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repo = SettingsRepositoryImpl::new(temp_dir.path().to_path_buf(), default_db_dir);

        let mut settings = Settings::default();
        settings.general.language = Language::English;

        repo.save(&settings).await.unwrap();

        let loaded_settings = repo.load().await.unwrap();
        assert_eq!(loaded_settings.general.language, Language::English);
    }

    #[tokio::test]
    async fn test_delete() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repo = SettingsRepositoryImpl::new(temp_dir.path().to_path_buf(), default_db_dir);

        let settings = Settings::default();
        repo.save(&settings).await.unwrap();

        assert!(repo.settings_file_path().exists());

        repo.delete().await.unwrap();

        assert!(!repo.settings_file_path().exists());
    }
}
