use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

#[derive(Default)]
pub struct BookmarksOperation {}

impl BookmarksOperation {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PdfOperation for BookmarksOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Bookmarks operation is not supported yet".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document;

    #[test]
    fn test_bookmarks_operation_unsupported() {
        let mut doc = LopdfDocument {
            inner: Document::with_version("1.5"),
        };
        let op = BookmarksOperation::new();
        let result = op.execute(&mut doc);

        assert!(result.is_err());
        match result {
            Err(PdfError::UnsupportedOperation(msg)) => {
                assert_eq!(msg, "Bookmarks operation is not supported yet");
            }
            _ => panic!("Expected UnsupportedOperation error"),
        }
    }
}
