// Library Context - 図書管理の境界づけられたコンテキスト

pub(crate) mod application;
pub(crate) mod domain;
pub(crate) mod infrastructure;
pub(crate) mod presentation;

// Public API - Presentation層のみ公開
pub use presentation::graphql::{BookMutation, BookQuery};
pub use presentation::integration::build_book_service;

// Type exports for type annotations (opaque to external users)
pub use application::services::book::BookService;
