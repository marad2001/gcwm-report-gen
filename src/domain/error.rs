use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Deserialize, Serialize)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Conflict error: {0}")]
    Conflict(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

}