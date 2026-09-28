use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct SignatureOperation;

impl SignatureOperation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SignatureOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for SignatureOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Digital signatures not yet supported natively".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;

    fn create_test_document() -> LopdfDocument {
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

        LopdfDocument { inner }
    }

    #[test]
    fn test_signature_unsupported() {
        let mut doc = create_test_document();
        let op = SignatureOperation::new();

        let result = op.execute(&mut doc);
        assert!(result.is_err());
        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert_eq!(msg, "Digital signatures not yet supported natively");
        } else {
            panic!("Expected UnsupportedOperation error");
        }
    }
}
