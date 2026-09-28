use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct RepairOperation;

impl RepairOperation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RepairOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for RepairOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let _lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        // `lopdf` handles basic cross-reference repairs automatically upon `load()`.
        // For this operation, we do a no-op that relies on the save cycle to rewrite a clean XRef table.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document;

    #[test]
    fn test_repair_operation_success() {
        let doc = Document::with_version("1.5");
        let mut lopdf_doc = LopdfDocument { inner: doc };
        let op = RepairOperation::new();

        let result = op.execute(&mut lopdf_doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_repair_operation_wrong_document_type() {
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
        let op = RepairOperation::new();

        let result = op.execute(&mut mock_doc);
        assert!(result.is_err());
        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert_eq!(msg, "Document is not a LopdfDocument");
        } else {
            panic!("Expected UnsupportedOperation error");
        }
    }
}
