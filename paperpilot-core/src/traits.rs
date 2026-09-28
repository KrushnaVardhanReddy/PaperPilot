use crate::error::OperationResult;
use std::any::Any;
use std::path::Path;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Represents a PDF document and operations that can be performed on it.
/// This acts as an architectural boundary separating the core logic from specific PDF engine implementations.
pub trait PdfDocument: AsAny {
    /// Returns the number of pages in the document.
    fn page_count(&self) -> OperationResult<u32>;

    /// Saves the document to the specified path.
    fn save(&self, path: &Path) -> OperationResult<()>;
}

/// Represents an operation that can be executed against a PDF document.
pub trait PdfOperation {
    /// Executes the operation on the given document.
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PdfError;

    struct MockDocument {
        page_count: u32,
    }

    impl PdfDocument for MockDocument {
        fn page_count(&self) -> OperationResult<u32> {
            Ok(self.page_count)
        }

        fn save(&self, _path: &Path) -> OperationResult<()> {
            Ok(())
        }
    }

    struct DummyOperation;

    impl PdfOperation for DummyOperation {
        fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
            let count = document.page_count()?;
            if count == 0 {
                return Err(PdfError::UnsupportedOperation("Empty document".to_string()));
            }
            Ok(())
        }
    }

    #[test]
    fn test_mock_document_and_operation() {
        let mut doc = MockDocument { page_count: 5 };
        let op = DummyOperation;

        // Test normal execution
        assert!(op.execute(&mut doc).is_ok());

        // Test error propagation
        let mut empty_doc = MockDocument { page_count: 0 };
        let result = op.execute(&mut empty_doc);
        assert!(result.is_err());
        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert_eq!(msg, "Empty document");
        } else {
            panic!("Expected UnsupportedOperation error");
        }
    }
}
