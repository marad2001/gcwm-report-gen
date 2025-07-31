// driven/doc_generator.rs
use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

pub mod lambda;

#[derive(Debug, Error)]
pub enum DocGenError {
    #[error("AWS Lambda invocation failed: {0}")]
    InvocationError(String),
    #[error("Invalid response payload: {0}")]
    ResponseParseError(String),
}

#[async_trait]
pub trait DocumentGenerator {
    /// Sends the JSON instructions to the docx‐generator Lambda
    /// and returns the download URL it responses with.
    async fn generate(&self, instructions: &Value) -> Result<String, DocGenError>;
}