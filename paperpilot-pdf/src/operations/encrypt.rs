use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use crate::document::LopdfDocument;
use lopdf;
use lopdf::{EncryptionState, EncryptionVersion, Permissions};
use std::convert::TryFrom;

#[derive(Default)]
pub struct EncryptOperation {
    pub user_password: Option<String>,
    pub owner_password: Option<String>,
}

impl EncryptOperation {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PdfOperation for EncryptOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        // Dynamically inject a placeholder ID array if missing
        if lopdf_doc.inner.trailer.get(b"ID").is_err() {
            let id_str = lopdf::Object::String(b"default_id_placeholder".to_vec(), lopdf::StringFormat::Literal);
            lopdf_doc.inner.trailer.set("ID", lopdf::Object::Array(vec![id_str.clone(), id_str]));
        }

        let user_pwd = self.user_password.as_deref().unwrap_or("");
        let owner_pwd = self.owner_password.as_deref().unwrap_or("");

        let version = EncryptionVersion::V2 {
            document: &lopdf_doc.inner,
            owner_password: owner_pwd,
            user_password: user_pwd,
            key_length: 128,
            permissions: Permissions::all(),
        };

        let state = EncryptionState::try_from(version)
            .map_err(|e| PdfError::Other(format!("Encryption failed: {}", e)))?;

        lopdf_doc.inner.encrypt(&state)
            .map_err(|e| PdfError::Other(format!("Encryption failed: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;

    #[test]
    fn test_encrypt_operation() {
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

        let mut op = EncryptOperation::new();
        op.user_password = Some("userpass".to_string());
        op.owner_password = Some("ownerpass".to_string());

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        // Ensure ID was added
        assert!(doc.inner.trailer.get(b"ID").is_ok());
        // Ensure Encrypt dict was added
        assert!(doc.inner.trailer.get(b"Encrypt").is_ok());
    }
}
