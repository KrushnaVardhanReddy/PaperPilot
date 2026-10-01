use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

#[derive(Default)]
pub struct DecryptOperation {
    pub password: Option<String>,
}

impl DecryptOperation {
    pub fn new(password: Option<String>) -> Self {
        Self { password }
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

        if let Some(password) = &self.password
            && let Err(e) = lopdf_doc.inner.decrypt(password) {
                return Err(PdfError::UnsupportedOperation(format!("Decryption failed or unsupported format: {}", e)));
            }

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

        let pages_id = inner.new_object_id();
        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", lopdf::Object::Array(vec![]));
        pages_dict.set("Count", lopdf::Object::Integer(0));
        inner.set_object(pages_id, pages_dict);

        let mut catalog_dict = lopdf::Dictionary::new();
        catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = inner.add_object(catalog_dict);

        inner.trailer.set("Root", lopdf::Object::Reference(catalog_id));

        let mut doc = LopdfDocument { inner };

        // Encrypt the test document first
        let mut encrypt_op = crate::operations::encrypt::EncryptOperation::new();
        encrypt_op.user_password = Some("test".to_string());
        encrypt_op.owner_password = Some("test".to_string());
        assert!(encrypt_op.execute(&mut doc).is_ok());

        assert!(doc.inner.trailer.has(b"Encrypt"));

        let op = DecryptOperation::new(Some("test".to_string()));
        assert!(op.execute(&mut doc).is_ok());

        assert!(!doc.inner.trailer.has(b"Encrypt"));
    }
}
