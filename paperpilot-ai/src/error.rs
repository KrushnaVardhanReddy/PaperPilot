use thiserror::Error;

#[derive(Error, Debug)]
pub enum AiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    Parse(String),
    #[error("API returned error: {0}")]
    Api(String),
    #[error("Model failed to call tools or returned ambiguous response: {0}")]
    AmbiguousIntent(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatting() {
        let err = AiError::Parse("invalid JSON".to_string());
        assert_eq!(err.to_string(), "Failed to parse response: invalid JSON");
    }
}
