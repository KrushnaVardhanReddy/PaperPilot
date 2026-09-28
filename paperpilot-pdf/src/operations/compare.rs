use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct CompareOperation;

impl PdfOperation for CompareOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Compare operation is too complex for basic structural diff".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;

    #[test]
    fn test_compare_operation_stub() {
        let inner = LopdfInnerDocument::with_version("1.5");
        let mut doc = LopdfDocument { inner };
        let op = CompareOperation;

        let result = op.execute(&mut doc);
        assert!(result.is_err());
        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert_eq!(
                msg,
                "Compare operation is too complex for basic structural diff"
            );
        } else {
            panic!("Expected UnsupportedOperation error");
        }
    }
}
