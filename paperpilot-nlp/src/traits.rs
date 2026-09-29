/// Represents a sequence of PDF operations to execute.
#[derive(Debug, Default, Clone)]
pub struct OperationPlan {
    // TODO: implement Phase 4.2.1
}

/// Error type for NLP resolution failures
#[derive(Debug, thiserror::Error)]
pub enum NlpError {
    #[error("Could not determine intent from the query: {0}")]
    AmbiguousIntent(String),
    #[error("Failed to extract required parameters for intent: {0}")]
    MissingParameters(String),
    #[error("Internal NLP engine error: {0}")]
    InternalError(String),
}

/// The core abstraction for natural language command processing.
/// This trait is implemented by both the embedded Offline NLP engine
/// and the cloud/local LLM providers (via `paperpilot-ai`).
pub trait NlpResolver: Send + Sync {
    /// Takes a natural language string and attempts to resolve it into a structured
    /// `OperationPlan` that the PaperPilot engine can execute.
    fn resolve(&self, query: &str) -> Result<OperationPlan, NlpError>;
}
