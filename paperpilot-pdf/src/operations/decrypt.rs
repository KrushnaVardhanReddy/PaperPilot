use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

#[derive(Default)]
pub struct DecryptOperation;

impl DecryptOperation {
    pub fn new() -> Self {
        Self
    }
}

impl PdfOperation for DecryptOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        // Remove the 'Encrypt' key from the trailer dictionary to strip encryption
        lopdf_doc.inner.trailer.remove(b"Encrypt");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document as LopdfInnerDocument;

    #[test]
    fn test_decrypt_operation() {
        let mut inner = LopdfInnerDocument::with_version("1.5");
        // Add a dummy Encrypt dictionary
        let encrypt_dict = lopdf::Dictionary::new();
        let encrypt_id = inner.add_object(encrypt_dict);
        inner
            .trailer
            .set("Encrypt", lopdf::Object::Reference(encrypt_id));

        let mut doc = LopdfDocument { inner };

        assert!(doc.inner.trailer.has(b"Encrypt"));

        let op = DecryptOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        assert!(!doc.inner.trailer.has(b"Encrypt"));
    }
}
