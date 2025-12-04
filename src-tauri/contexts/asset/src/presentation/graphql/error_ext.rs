// Asset Presentation - GraphQL Error Extensions

use crate::application::errors::ApplicationError;
use async_graphql::ErrorExtensions;

impl ErrorExtensions for ApplicationError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string()).extend_with(|_err, e| match self {
            ApplicationError::NotFound(_) => {
                e.set("code", "NOT_FOUND");
            }
            ApplicationError::CannotDelete(_) => {
                e.set("code", "CANNOT_DELETE");
            }
            ApplicationError::Domain(_) => {
                e.set("code", "DOMAIN_ERROR");
            }
            ApplicationError::DatabaseError(_) => {
                e.set("code", "DATABASE_ERROR");
            }
        })
    }
}
