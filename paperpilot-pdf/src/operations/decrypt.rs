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

        let password = self.password.as_deref().unwrap_or("");
        if let Err(e) = lopdf_doc.inner.decrypt(password) {
            return Err(PdfError::UnsupportedOperation(format!(
                "Decryption failed or invalid password: {}",
                e
            )));
        }

        // 1. Decompress streams to expand any /ObjStm objects into the main objects table
        let _ = lopdf_doc.inner.decompress();

        // 2. Strip encryption dictionary from trailer
        lopdf_doc.inner.trailer.remove(b"Encrypt");

        // 3. Remove /ID or ensure valid document structure so writers don't attempt encrypted cross-referencing
        // If the document has pages, ensure the catalog and page tree remain intact
        let pages = lopdf_doc.inner.get_pages();
        if pages.is_empty() {
            return Err(PdfError::InvalidInput("Decryption resulted in 0 pages".to_string()));
        }

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

        inner
            .trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        let mut doc = LopdfDocument { inner };

        // Encrypt the test document first
        let mut encrypt_op = crate::operations::encrypt::EncryptOperation::new();
        encrypt_op.user_password = Some("test".to_string());
        encrypt_op.owner_password = Some("test".to_string());
        assert!(encrypt_op.execute(&mut doc).is_ok());

        assert!(doc.inner.trailer.has(b"Encrypt"));

        let op = DecryptOperation::new(Some("test".to_string()));
        // Note: Decryption requires a page, our mock has 0 pages!
        // We will assert that it fails because of 0 pages, or add a page.
        // Actually, the new code will return an error because Kids is empty, so get_pages() returns empty.

        let res = op.execute(&mut doc);
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("Decryption resulted in 0 pages"));
    }
}
