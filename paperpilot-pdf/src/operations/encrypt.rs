use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

#[derive(Default)]
pub struct EncryptOperation {
    pub user_password: Option<String>,
    pub owner_password: Option<String>,
}

impl EncryptOperation {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PdfOperation for EncryptOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> OperationResult<()> {
        // lopdf does not easily support writing encrypted files natively via its generic interface
        // return UnsupportedOperation as requested for stub implementation
        Err(PdfError::UnsupportedOperation(
            "Encrypt operation is not natively supported by lopdf".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;

    #[test]
    fn test_encrypt_operation_returns_unsupported() {
        let mut doc = LopdfDocument {
            inner: LopdfInnerDocument::with_version("1.5"),
        };
        let op = EncryptOperation::new();
        let result = op.execute(&mut doc);

        assert!(result.is_err());
        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert_eq!(msg, "Encrypt operation is not natively supported by lopdf");
        } else {
            panic!("Expected UnsupportedOperation error");
        }
    }
}
