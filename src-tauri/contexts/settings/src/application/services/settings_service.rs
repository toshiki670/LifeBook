// Settings Application Layer - Settings Service

use crate::{
    application::dto::{AppearanceSettingsDto, DatabaseSettingsDto, GeneralSettingsDto},
    domain::{
        entities::AppSettings,
        value_objects::{Language, Theme},
    },
    infrastructure::SettingsFileStorage,
};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 設定管理サービス
pub struct SettingsService {
    storage: Arc<SettingsFileStorage>,
    // メモリキャッシュ（頻繁な読み込みを避けるため）
    cache: Arc<RwLock<Option<AppSettings>>>,
}

impl SettingsService {
    /// 新しいサービスインスタンスを作成
    pub fn new(storage: Arc<SettingsFileStorage>) -> Self {
        Self {
            storage,
            cache: Arc::new(RwLock::new(None)),
        }
    }

    /// 設定を読み込む（キャッシュを使用）
    async fn load_settings(&self) -> Result<AppSettings> {
        // キャッシュをチェック
        {
            let cache = self.cache.read().await;
            if let Some(settings) = cache.as_ref() {
                return Ok(settings.clone());
            }
        }

        // キャッシュがない場合はファイルから読み込む
        let settings = self.storage.load().await?;

        // キャッシュを更新
        {
            let mut cache = self.cache.write().await;
            *cache = Some(settings.clone());
        }

        Ok(settings)
    }

    /// 設定を保存（キャッシュも更新）
    async fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        // ファイルに保存
        self.storage.save(settings).await?;

        // キャッシュを更新
        {
            let mut cache = self.cache.write().await;
            *cache = Some(settings.clone());
        }

        Ok(())
    }

    /// 一般設定を取得
    pub async fn get_general_settings(&self) -> Result<GeneralSettingsDto> {
        let settings = self.load_settings().await?;
        Ok(settings.general.into())
    }

    /// 表示設定を取得
    pub async fn get_appearance_settings(&self) -> Result<AppearanceSettingsDto> {
        let settings = self.load_settings().await?;
        Ok(settings.appearance.into())
    }

    /// データベース設定を取得
    pub async fn get_database_settings(&self) -> Result<DatabaseSettingsDto> {
        let settings = self.load_settings().await?;
        Ok(settings.database.into())
    }

    /// 一般設定を更新
    pub async fn update_general_settings(
        &self,
        language: Option<String>,
    ) -> Result<GeneralSettingsDto> {
        let mut settings = self.load_settings().await?;

        // 言語を更新
        if let Some(lang_str) = language {
            let language = Language::from_str(&lang_str)
                .map_err(|e| anyhow::anyhow!("Invalid language: {}", e))?;
            settings.general.language = language;
        }

        self.save_settings(&settings).await?;
        Ok(settings.general.into())
    }

    /// 表示設定を更新
    pub async fn update_appearance_settings(
        &self,
        theme: Option<String>,
    ) -> Result<AppearanceSettingsDto> {
        let mut settings = self.load_settings().await?;

        // テーマを更新
        if let Some(theme_str) = theme {
            let theme =
                Theme::from_str(&theme_str).map_err(|e| anyhow::anyhow!("Invalid theme: {}", e))?;
            settings.appearance.theme = theme;
        }

        self.save_settings(&settings).await?;
        Ok(settings.appearance.into())
    }

    /// データベース設定を更新
    pub async fn update_database_settings(
        &self,
        database_directory: Option<String>,
    ) -> Result<DatabaseSettingsDto> {
        let mut settings = self.load_settings().await?;

        // データベースディレクトリを更新
        if let Some(dir_str) = database_directory {
            let path = PathBuf::from(dir_str);

            // ディレクトリのバリデーション（親ディレクトリが存在するか）
            if let Some(parent) = path.parent()
                && !parent.exists() && parent != std::path::Path::new("") {
                    return Err(anyhow::anyhow!(
                        "Parent directory does not exist: {}",
                        parent.display()
                    ));
                }

            settings.database.database_directory = path;
        }

        self.save_settings(&settings).await?;
        Ok(settings.database.into())
    }

    /// すべての設定をリセット
    pub async fn reset_all_settings(&self) -> Result<()> {
        // 設定ファイルを削除
        self.storage.delete().await?;

        // キャッシュをクリア
        {
            let mut cache = self.cache.write().await;
            *cache = None;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_get_default_settings() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Arc::new(SettingsFileStorage::new(temp_dir.path().to_path_buf()));
        let service = SettingsService::new(storage);

        let general = service.get_general_settings().await.unwrap();
        assert_eq!(general.language, "ja");

        let appearance = service.get_appearance_settings().await.unwrap();
        assert_eq!(appearance.theme, "system");
    }

    #[tokio::test]
    async fn test_update_general_settings() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Arc::new(SettingsFileStorage::new(temp_dir.path().to_path_buf()));
        let service = SettingsService::new(storage);

        let updated = service
            .update_general_settings(Some("en".to_string()))
            .await
            .unwrap();
        assert_eq!(updated.language, "en");

        // 再取得して確認
        let general = service.get_general_settings().await.unwrap();
        assert_eq!(general.language, "en");
    }

    #[tokio::test]
    async fn test_update_appearance_settings() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Arc::new(SettingsFileStorage::new(temp_dir.path().to_path_buf()));
        let service = SettingsService::new(storage);

        let updated = service
            .update_appearance_settings(Some("dark".to_string()))
            .await
            .unwrap();
        assert_eq!(updated.theme, "dark");
    }

    #[tokio::test]
    async fn test_reset_settings() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Arc::new(SettingsFileStorage::new(temp_dir.path().to_path_buf()));
        let service = SettingsService::new(storage);

        // 設定を変更
        service
            .update_general_settings(Some("en".to_string()))
            .await
            .unwrap();

        // リセット
        service.reset_all_settings().await.unwrap();

        // デフォルトに戻っているか確認
        let general = service.get_general_settings().await.unwrap();
        assert_eq!(general.language, "ja");
    }
}
