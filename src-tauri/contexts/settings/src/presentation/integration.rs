// Presentation Layer - 統合ヘルパー
// 他のクレートからの依存性注入を簡素化する

use crate::application::services::SettingsService;
use crate::infrastructure::repositories::SettingsRepositoryImpl;
use std::path::PathBuf;
use std::sync::Arc;

/// SettingsServiceを構築する統合関数
///
/// # Arguments
/// * `config_dir` - 設定ファイルを保存するディレクトリ
/// * `default_db_dir` - データベースディレクトリのデフォルト値
///
/// # Returns
/// 依存性が注入されたSettingsServiceのArcポインタ
pub fn build_settings_service(
    config_dir: PathBuf,
    default_db_dir: PathBuf,
) -> Arc<SettingsService> {
    let settings_repo = Arc::new(SettingsRepositoryImpl::new(config_dir, default_db_dir));
    Arc::new(SettingsService::new(settings_repo))
}

