use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct LinearizeOperation;

impl LinearizeOperation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinearizeOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for LinearizeOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Linearization not yet supported by lopdf backend".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document;

    #[test]
    fn test_linearize_operation_unsupported() {
        let doc = Document::with_version("1.5");
        let mut lopdf_doc = LopdfDocument { inner: doc };
        let op = LinearizeOperation::new();

        let result = op.execute(&mut lopdf_doc);
        assert!(result.is_err());

        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert_eq!(msg, "Linearization not yet supported by lopdf backend");
        } else {
            panic!("Expected UnsupportedOperation error");
        }
    }
}
