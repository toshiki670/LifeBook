// Settings Context - Library Entry Point

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

// Public API - Presentation層のみ公開
pub use presentation::graphql::{mutations::SettingsMutation, queries::SettingsQuery};
pub use presentation::integration::build_settings_service;

// Type exports for type annotations (opaque to external users)
pub use application::services::SettingsService;
