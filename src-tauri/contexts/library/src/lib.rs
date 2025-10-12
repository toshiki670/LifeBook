// Library Context - 図書管理の境界づけられたコンテキスト

#[path = "application/application.rs"]
pub mod application;

#[path = "domain/domain.rs"]
pub mod domain;

#[path = "infrastructure/infrastructure.rs"]
pub mod infrastructure;

#[path = "presentation/presentation.rs"]
pub mod presentation;

// Re-export for convenience
pub use application::dto::book::BookDto;
pub use application::services::book::BookService;
pub use domain::entities::book::Book;
pub use domain::repositories::book::BookRepository;
pub use infrastructure::repositories::book::BookRepositoryImpl;
pub use presentation::graphql::{BookMutation, BookQuery};
