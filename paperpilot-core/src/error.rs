#[derive(Debug)]
pub enum PdfError {
    IoError(std::io::Error),
    ParseError(String),
    CorruptFile(String),
    UnsupportedOperation(String),
    Other(String),
}

impl std::fmt::Display for PdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfError::IoError(e) => write!(f, "IO Error: {}", e),
            PdfError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            PdfError::CorruptFile(msg) => write!(f, "Corrupt File: {}", msg),
            PdfError::UnsupportedOperation(msg) => write!(f, "Unsupported Operation: {}", msg),
            PdfError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl std::error::Error for PdfError {}

pub type OperationResult<T> = Result<T, PdfError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = PdfError::ParseError("Invalid format".to_string());
        assert_eq!(format!("{}", err), "Parse Error: Invalid format");
    }
}
