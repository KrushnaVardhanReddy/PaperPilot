use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use crate::document::LopdfDocument;

pub struct CompressOperation;

impl CompressOperation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CompressOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for CompressOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        lopdf_doc.inner.compress();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document;

    #[test]
    fn test_compress_operation_success() {
        let doc = Document::with_version("1.5");
        // We just need an empty or dummy document to test compress
        let mut lopdf_doc = LopdfDocument { inner: doc };
        let op = CompressOperation::new();

        let result = op.execute(&mut lopdf_doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compress_operation_wrong_document_type() {
        use std::any::Any;
        use std::path::Path;

        struct MockDocument;
        impl PdfDocument for MockDocument {
            fn page_count(&self) -> OperationResult<u32> {
                Ok(0)
            }
            fn save(&self, _path: &Path) -> OperationResult<()> {
                Ok(())
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }

        let mut mock_doc = MockDocument;
        let op = CompressOperation::new();

        let result = op.execute(&mut mock_doc);
        assert!(result.is_err());
        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert_eq!(msg, "Document is not a LopdfDocument");
        } else {
            panic!("Expected UnsupportedOperation error");
        }
    }
}
