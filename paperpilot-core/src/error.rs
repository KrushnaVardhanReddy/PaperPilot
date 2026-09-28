use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Corrupt file: {0}")]
    CorruptFile(String),
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Other error: {0}")]
    Other(String),
}

pub type OperationResult<T> = Result<T, PdfError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let pdf_error: PdfError = io_error.into();
        assert!(matches!(pdf_error, PdfError::IoError(_)));
    }

    #[test]
    fn test_parse_error() {
        let error = PdfError::ParseError("invalid syntax".to_string());
        assert_eq!(error.to_string(), "Parse error: invalid syntax");
    }
}
