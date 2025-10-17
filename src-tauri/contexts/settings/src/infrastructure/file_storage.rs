// Settings Infrastructure Layer - File Storage

use crate::domain::entities::AppSettings;
use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::fs;

const SETTINGS_FILE_NAME: &str = "settings.json";

/// 設定ファイルのストレージ
pub struct SettingsFileStorage {
    config_dir: PathBuf,
}

impl SettingsFileStorage {
    /// 新しいストレージインスタンスを作成
    pub fn new(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    /// 設定ファイルのパスを取得
    fn settings_file_path(&self) -> PathBuf {
        self.config_dir.join(SETTINGS_FILE_NAME)
    }

    /// 設定をファイルから読み込む
    pub async fn load(&self) -> Result<AppSettings> {
        let file_path = self.settings_file_path();

        // ファイルが存在しない場合はデフォルト設定を返す
        if !file_path.exists() {
            return Ok(AppSettings::default());
        }

        // ファイルを読み込む
        let content = fs::read_to_string(&file_path)
            .await
            .context("Failed to read settings file")?;

        // JSONをパース
        let settings: AppSettings = serde_json::from_str(&content).context(
            "Failed to parse settings file. The file may be corrupted. Using default settings.",
        )?;

        Ok(settings)
    }

    /// 設定をファイルに保存
    pub async fn save(&self, settings: &AppSettings) -> Result<()> {
        // 設定ディレクトリが存在しない場合は作成
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir)
                .await
                .context("Failed to create config directory")?;
        }

        let file_path = self.settings_file_path();

        // 設定をJSONに変換（pretty print）
        let content =
            serde_json::to_string_pretty(settings).context("Failed to serialize settings")?;

        // ファイルに書き込む
        fs::write(&file_path, content)
            .await
            .context("Failed to write settings file")?;

        Ok(())
    }

    /// 設定ファイルを削除（リセット用）
    pub async fn delete(&self) -> Result<()> {
        let file_path = self.settings_file_path();

        if file_path.exists() {
            fs::remove_file(&file_path)
                .await
                .context("Failed to delete settings file")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_load_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let storage = SettingsFileStorage::new(temp_dir.path().to_path_buf());

        let settings = storage.load().await.unwrap();
        assert_eq!(
            settings.general.language,
            crate::domain::value_objects::Language::Japanese
        );
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let storage = SettingsFileStorage::new(temp_dir.path().to_path_buf());

        let mut settings = AppSettings::default();
        settings.general.language = crate::domain::value_objects::Language::English;

        storage.save(&settings).await.unwrap();

        let loaded_settings = storage.load().await.unwrap();
        assert_eq!(
            loaded_settings.general.language,
            crate::domain::value_objects::Language::English
        );
    }

    #[tokio::test]
    async fn test_delete() {
        let temp_dir = TempDir::new().unwrap();
        let storage = SettingsFileStorage::new(temp_dir.path().to_path_buf());

        let settings = AppSettings::default();
        storage.save(&settings).await.unwrap();

        assert!(storage.settings_file_path().exists());

        storage.delete().await.unwrap();

        assert!(!storage.settings_file_path().exists());
    }
}
