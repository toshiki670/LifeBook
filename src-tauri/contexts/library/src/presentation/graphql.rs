// Presentation Layer - Library GraphQL Module

mod error_ext;
pub mod mutations;
pub mod queries;

// Re-export for convenience
pub use mutations::book::BookMutation;
pub use queries::book::BookQuery;

// Re-export error conversion function for internal use
pub(crate) use error_ext::to_graphql_error;
