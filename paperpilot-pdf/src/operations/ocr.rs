use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct OcrOperation;

impl PdfOperation for OcrOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "OCR requires tesseract".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;

    #[test]
    fn test_ocr_operation_stub() {
        let inner = LopdfInnerDocument::with_version("1.5");
        let mut doc = LopdfDocument { inner };
        let op = OcrOperation;

        let result = op.execute(&mut doc);
        assert!(result.is_err());
        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert_eq!(msg, "OCR requires tesseract");
        } else {
            panic!("Expected UnsupportedOperation error");
        }
    }
}
