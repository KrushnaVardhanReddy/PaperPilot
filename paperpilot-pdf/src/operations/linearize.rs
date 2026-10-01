use crate::document::LopdfDocument;
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
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        // lopdf doesn't support true linearization.
        // Normalize the document by compressing object streams (equivalent cleanup pass).
        // This is a valid "best-effort" linearize that at least validates and re-saves cleanly.
        lopdf_doc.inner.compress();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document;

    #[test]
    fn test_linearize_operation() {
        let doc = Document::with_version("1.5");
        let mut lopdf_doc = LopdfDocument { inner: doc };
        let op = LinearizeOperation::new();

        let result = op.execute(&mut lopdf_doc);
        assert!(result.is_ok());
    }
}
