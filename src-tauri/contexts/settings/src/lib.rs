// Settings Context - Library Entry Point

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

// 便利な re-export
pub use application::errors::SettingsError;
pub use application::services::SettingsService;
pub use domain::repositories::SettingsRepository;
pub use infrastructure::repositories::SettingsRepositoryImpl;
pub use presentation::graphql::{mutations::SettingsMutation, queries::SettingsQuery};
