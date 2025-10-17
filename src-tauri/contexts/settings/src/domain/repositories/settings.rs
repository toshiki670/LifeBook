// Settings Domain Layer - Settings Repository Interface

use crate::domain::entities::AppSettings;
use async_trait::async_trait;
use shared::domain::errors::DomainError;

/// Settings リポジトリのインターフェース
/// インフラ層がこのtraitを実装する
#[async_trait]
pub trait SettingsRepository: Send + Sync {
    /// 設定を読み込む
    async fn load(&self) -> Result<AppSettings, DomainError>;

    /// 設定を保存
    async fn save(&self, settings: &AppSettings) -> Result<(), DomainError>;

    /// 設定を削除（リセット用）
    async fn delete(&self) -> Result<(), DomainError>;
}
