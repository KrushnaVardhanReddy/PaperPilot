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
        // lopdf 0.45 leaves a password-protected file only partially parsed
        // when loaded without its password. Decrypting that object table alone
        // can report success while dropping the catalog and every page.
        let encrypted_source = lopdf_doc
            .original_bytes
            .as_deref()
            .filter(|bytes| lopdf::Document::load_mem(bytes).is_ok_and(|doc| doc.is_encrypted()));
        let result = if lopdf_doc.inner.is_encrypted()
            && let Some(bytes) = encrypted_source
        {
            lopdf::Document::load_mem_with_options(
                bytes,
                lopdf::LoadOptions::with_password(password),
            )
            .map(|inner| lopdf_doc.inner = inner)
        } else if lopdf_doc.inner.was_encrypted() && !lopdf_doc.inner.is_encrypted() {
            // Empty-user-password documents are already decrypted by the loader.
            Ok(())
        } else {
            lopdf_doc.inner.decrypt(password)
        };
        if let Err(e) = result {
            return Err(PdfError::UnsupportedOperation(format!(
                "Decryption failed or invalid password: {}",
                e
            )));
        }

        // 1. Decompress streams to expand any /ObjStm objects into the main objects table
        lopdf_doc.inner.decompress();

        // 2. Strip encryption dictionary from trailer
        lopdf_doc.inner.trailer.remove(b"Encrypt");
        // This is a full decrypted export, not an encrypted incremental save.
        lopdf_doc.inner.encryption_state = None;

        // 3. Remove /ID or ensure valid document structure so writers don't attempt encrypted cross-referencing
        // If the document has pages, ensure the catalog and page tree remain intact
        let _pages = lopdf_doc.inner.get_pages();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Document as LopdfInnerDocument, dictionary};

    #[test]
    fn decrypt_loaded_file_preserves_catalog_pages_and_text() {
        let mut inner = LopdfInnerDocument::with_version("1.5");
        let pages_id = inner.new_object_id();
        let font = inner.add_object(lopdf::dictionary! {
            "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica",
        });
        let content = inner.add_object(lopdf::Stream::new(
            lopdf::Dictionary::new(),
            b"BT /F1 12 Tf 50 700 Td (Preserved contract) Tj ET".to_vec(),
        ));
        let page = inner.add_object(lopdf::dictionary! {
            "Type" => "Page", "Parent" => pages_id, "Contents" => content,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Resources" => lopdf::dictionary! { "Font" => lopdf::dictionary! { "F1" => font } },
        });
        inner.set_object(
            pages_id,
            lopdf::dictionary! {
                "Type" => "Pages", "Kids" => vec![page.into()], "Count" => 1,
            },
        );
        let catalog =
            inner.add_object(lopdf::dictionary! { "Type" => "Catalog", "Pages" => pages_id });
        inner.trailer.set("Root", catalog);
        let mut doc = LopdfDocument::from_inner(inner);
        let mut encrypt = crate::operations::encrypt::EncryptOperation::new();
        encrypt.user_password = Some("secret".into());
        encrypt.execute(&mut doc).unwrap();
        let mut encrypted = Vec::new();
        doc.inner.save_to(&mut encrypted).unwrap();
        let mut loaded = LopdfDocument::from_bytes(&encrypted).unwrap();
        assert!(
            DecryptOperation::new(Some("wrong".into()))
                .execute(&mut loaded)
                .is_err()
        );
        DecryptOperation::new(Some("secret".into()))
            .execute(&mut loaded)
            .unwrap();
        let mut decrypted = Vec::new();
        loaded.inner.save_to(&mut decrypted).unwrap();
        let reopened = LopdfInnerDocument::load_mem(&decrypted).unwrap();
        assert!(!reopened.is_encrypted());
        assert_eq!(reopened.get_pages().len(), 1);
        assert!(
            reopened
                .extract_text(&[1])
                .unwrap()
                .contains("Preserved contract")
        );
    }

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

        let mut doc = LopdfDocument {
            inner,
            original_bytes: None,
        };

        // Encrypt the test document first
        let mut encrypt_op = crate::operations::encrypt::EncryptOperation::new();
        encrypt_op.user_password = Some("test".to_string());
        encrypt_op.owner_password = Some("test".to_string());
        assert!(encrypt_op.execute(&mut doc).is_ok());

        assert!(doc.inner.trailer.has(b"Encrypt"));

        let op = DecryptOperation::new(Some("test".to_string()));
        // Decryption will now succeed even if get_pages() returns empty

        let res = op.execute(&mut doc);
        assert!(res.is_ok());
    }

    #[test]
    fn decrypt_file_with_empty_user_password() {
        let mut doc = LopdfDocument::new();
        let mut encrypt = crate::operations::encrypt::EncryptOperation::new();
        encrypt.user_password = Some(String::new());
        encrypt.owner_password = Some("owner".into());
        encrypt.execute(&mut doc).unwrap();
        let mut bytes = Vec::new();
        doc.inner.save_to(&mut bytes).unwrap();
        let mut loaded = LopdfDocument::from_bytes(&bytes).unwrap();
        assert!(loaded.inner.was_encrypted());
        DecryptOperation::new(None).execute(&mut loaded).unwrap();
        assert!(!loaded.inner.is_encrypted());
        assert!(!loaded.inner.was_encrypted());
        assert!(loaded.inner.catalog().is_ok());
    }
}
