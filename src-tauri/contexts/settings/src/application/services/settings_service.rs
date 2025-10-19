// Settings Application Layer - Settings Service

use crate::{
    application::{
        dto::{AppearanceSettingsDto, DatabaseSettingsDto, GeneralSettingsDto},
        errors::ApplicationError,
    },
    domain::{
        entities::Settings,
        repositories::SettingsRepository,
        value_objects::{Language, Theme},
    },
};
use std::path::{Component, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 設定管理サービス
pub struct SettingsService {
    repository: Arc<dyn SettingsRepository>,
    // メモリキャッシュ（頻繁な読み込みを避けるため）
    cache: Arc<RwLock<Option<Settings>>>,
}

impl SettingsService {
    /// 新しいサービスインスタンスを作成
    pub fn new(repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            repository,
            cache: Arc::new(RwLock::new(None)),
        }
    }

    /// 設定を読み込む（キャッシュを使用）
    async fn load_settings(&self) -> Result<Settings, ApplicationError> {
        // キャッシュをチェック
        {
            let cache = self.cache.read().await;
            if let Some(settings) = cache.as_ref() {
                return Ok(settings.clone());
            }
        }

        // キャッシュがない場合はリポジトリから読み込む
        let settings = self.repository.load().await?;

        // キャッシュを更新
        {
            let mut cache = self.cache.write().await;
            *cache = Some(settings.clone());
        }

        Ok(settings)
    }

    /// 設定を保存（キャッシュも更新）
    async fn save_settings(&self, settings: &Settings) -> Result<(), ApplicationError> {
        // リポジトリに保存
        self.repository.save(settings).await?;

        // キャッシュを更新
        {
            let mut cache = self.cache.write().await;
            *cache = Some(settings.clone());
        }

        Ok(())
    }

    /// 一般設定を取得
    pub async fn get_general_settings(&self) -> Result<GeneralSettingsDto, ApplicationError> {
        let settings = self.load_settings().await?;
        Ok(settings.general.into())
    }

    /// 表示設定を取得
    pub async fn get_appearance_settings(&self) -> Result<AppearanceSettingsDto, ApplicationError> {
        let settings = self.load_settings().await?;
        Ok(settings.appearance.into())
    }

    /// データベース設定を取得
    pub async fn get_database_settings(&self) -> Result<DatabaseSettingsDto, ApplicationError> {
        let settings = self.load_settings().await?;
        Ok(settings.database.into())
    }

    /// 一般設定を更新
    pub async fn update_general_settings(
        &self,
        language: Option<String>,
    ) -> Result<GeneralSettingsDto, ApplicationError> {
        let mut settings = self.load_settings().await?;

        // 言語を更新
        if let Some(lang_str) = language {
            let language = Language::from_str(&lang_str)
                .map_err(|e| ApplicationError::InvalidLanguage(e.to_string()))?;
            settings.general.language = language;
        }

        self.save_settings(&settings).await?;
        Ok(settings.general.into())
    }

    /// 表示設定を更新
    pub async fn update_appearance_settings(
        &self,
        theme: Option<String>,
    ) -> Result<AppearanceSettingsDto, ApplicationError> {
        let mut settings = self.load_settings().await?;

        // テーマを更新
        if let Some(theme_str) = theme {
            let theme = Theme::from_str(&theme_str)
                .map_err(|e| ApplicationError::InvalidTheme(e.to_string()))?;
            settings.appearance.theme = theme;
        }

        self.save_settings(&settings).await?;
        Ok(settings.appearance.into())
    }

    /// データベースディレクトリのバリデーション（セキュリティ強化版）
    fn validate_database_directory(path_str: &str) -> Result<PathBuf, ApplicationError> {
        // 空文字列チェック
        if path_str.trim().is_empty() {
            return Err(ApplicationError::InvalidDatabaseDirectory(
                "Database directory cannot be empty".to_string(),
            ));
        }

        let path = PathBuf::from(path_str);

        // 絶対パスチェック
        if !path.is_absolute() {
            return Err(ApplicationError::InvalidDatabaseDirectory(
                "Database directory must be an absolute path".to_string(),
            ));
        }

        // パストラバーサル検出（../ の使用を禁止）
        for component in path.components() {
            if matches!(component, Component::ParentDir) {
                return Err(ApplicationError::InvalidDatabaseDirectory(
                    "Path traversal detected: '..' is not allowed".to_string(),
                ));
            }
        }

        // シンボリックリンクの検出（存在する場合のみチェック）
        if path.exists() && path.is_symlink() {
            return Err(ApplicationError::InvalidDatabaseDirectory(
                "Symbolic links are not allowed for security reasons".to_string(),
            ));
        }

        // 親ディレクトリの存在チェック
        if let Some(parent) = path.parent() {
            // 空パスでない場合のみチェック
            if parent != std::path::Path::new("") && !parent.exists() {
                return Err(ApplicationError::InvalidDatabaseDirectory(format!(
                    "Parent directory does not exist: {}",
                    parent.display()
                )));
            }

            // 親ディレクトリへの書き込み権限チェック（Unix系のみ）
            #[cfg(unix)]
            if parent.exists() {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = parent.metadata() {
                    let permissions = metadata.permissions();
                    // 書き込み権限があるかチェック (owner write = 0o200)
                    if permissions.mode() & 0o200 == 0 {
                        return Err(ApplicationError::InvalidDatabaseDirectory(
                            "No write permission for parent directory".to_string(),
                        ));
                    }
                }
            }
        }

        Ok(path)
    }

    /// データベース設定を更新
    pub async fn update_database_settings(
        &self,
        database_directory: Option<String>,
    ) -> Result<DatabaseSettingsDto, ApplicationError> {
        let mut settings = self.load_settings().await?;

        // データベースディレクトリを更新
        if let Some(dir_str) = database_directory {
            let validated_path = Self::validate_database_directory(&dir_str)?;
            settings.database.database_directory = validated_path;
        }

        self.save_settings(&settings).await?;
        Ok(settings.database.into())
    }

    /// すべての設定をリセット
    pub async fn reset_all_settings(&self) -> Result<(), ApplicationError> {
        // 設定ファイルを削除
        self.repository.delete().await?;

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
    use crate::infrastructure::repositories::SettingsRepositoryImpl;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_get_default_settings() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

        let general = service.get_general_settings().await.unwrap();
        assert_eq!(general.language, "ja");

        let appearance = service.get_appearance_settings().await.unwrap();
        assert_eq!(appearance.theme, "system");
    }

    #[tokio::test]
    async fn test_update_general_settings() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

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
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

        let updated = service
            .update_appearance_settings(Some("dark".to_string()))
            .await
            .unwrap();
        assert_eq!(updated.theme, "dark");
    }

    #[tokio::test]
    async fn test_reset_settings() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

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

    // ============================================
    // エラーケースのテスト
    // ============================================

    #[tokio::test]
    async fn test_invalid_language_code() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

        // 無効な言語コード
        let result = service
            .update_general_settings(Some("invalid".to_string()))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::InvalidLanguage(_msg) => {
                // エラーが返されれば成功
            }
            _ => panic!("Expected InvalidLanguage error"),
        }
    }

    #[tokio::test]
    async fn test_invalid_theme() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

        // 無効なテーマ
        let result = service
            .update_appearance_settings(Some("invalid".to_string()))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::InvalidTheme(_msg) => {
                // エラーが返されれば成功
            }
            _ => panic!("Expected InvalidTheme error"),
        }
    }

    #[tokio::test]
    async fn test_empty_database_directory() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

        // 空文字列
        let result = service
            .update_database_settings(Some("".to_string()))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::InvalidDatabaseDirectory(msg) => {
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected InvalidDatabaseDirectory error"),
        }
    }

    #[tokio::test]
    async fn test_relative_path_rejected() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

        // 相対パス
        let result = service
            .update_database_settings(Some("./relative/path".to_string()))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::InvalidDatabaseDirectory(msg) => {
                assert!(msg.contains("absolute path"));
            }
            _ => panic!("Expected InvalidDatabaseDirectory error"),
        }
    }

    #[tokio::test]
    async fn test_path_traversal_rejected() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

        // パストラバーサルを含むパス
        let result = service
            .update_database_settings(Some("/tmp/test/../../../etc".to_string()))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::InvalidDatabaseDirectory(msg) => {
                assert!(msg.contains("Path traversal") || msg.contains("'..'"));
            }
            _ => panic!("Expected InvalidDatabaseDirectory error"),
        }
    }

    #[tokio::test]
    async fn test_nonexistent_parent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir,
        ));
        let service = SettingsService::new(repository);

        // 存在しない親ディレクトリ
        let result = service
            .update_database_settings(Some("/nonexistent/parent/databases".to_string()))
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::InvalidDatabaseDirectory(msg) => {
                assert!(msg.contains("does not exist"));
            }
            _ => panic!("Expected InvalidDatabaseDirectory error"),
        }
    }

    #[tokio::test]
    async fn test_valid_database_directory_accepted() {
        let temp_dir = TempDir::new().unwrap();
        let default_db_dir = temp_dir.path().join("databases");
        let repository = Arc::new(SettingsRepositoryImpl::new(
            temp_dir.path().to_path_buf(),
            default_db_dir.clone(),
        ));
        let service = SettingsService::new(repository);

        // 有効なパス（存在する親ディレクトリを持つ）
        let valid_path = temp_dir.path().join("my_databases");
        let result = service
            .update_database_settings(Some(valid_path.to_str().unwrap().to_string()))
            .await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.database_directory, valid_path.to_str().unwrap());
    }
}
