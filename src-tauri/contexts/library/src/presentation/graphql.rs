// Presentation Layer - Library GraphQL Module

pub mod mutations;
pub mod queries;

// Re-export for convenience
pub use mutations::book::BookMutation;
pub use queries::book::BookQuery;
