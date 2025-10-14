// Library Context - 図書管理の境界づけられたコンテキスト

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

// Re-export for convenience
pub use application::dto::book::BookDto;
pub use application::services::book::BookService;
pub use domain::entities::book::Book;
pub use domain::repositories::book::BookRepository;
pub use infrastructure::repositories::book::BookRepositoryImpl;
pub use presentation::graphql::{BookMutation, BookQuery};
