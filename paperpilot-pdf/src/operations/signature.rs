use crate::document::LopdfDocument;
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
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        // Add a /SigFlags entry to the AcroForm to indicate signature fields exist.
        // This is a "placeholder sign" that reserves space without actual PKI signing.
        let mut acroform = lopdf::Dictionary::new();
        acroform.set("SigFlags", lopdf::Object::Integer(3));
        let acroform_id = lopdf_doc
            .inner
            .add_object(lopdf::Object::Dictionary(acroform));

        // Attach to catalog
        let catalog_ref = lopdf_doc
            .inner
            .trailer
            .get(b"Root")
            .and_then(|r| r.as_reference())
            .map_err(|e| PdfError::Other(format!("Missing Root reference: {:?}", e)))?;

        if let Ok(catalog) = lopdf_doc.inner.get_object_mut(catalog_ref)
            && let Ok(dict) = catalog.as_dict_mut() {
                dict.set("AcroForm", lopdf::Object::Reference(acroform_id));
            }

        Ok(())
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

        inner
            .trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        LopdfDocument { inner }
    }

    #[test]
    fn test_signature_operation() {
        let mut doc = create_test_document();
        let op = SignatureOperation::new();

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        let catalog_id = doc
            .inner
            .trailer
            .get(b"Root")
            .unwrap()
            .as_reference()
            .unwrap();
        let catalog = doc.inner.get_object(catalog_id).unwrap().as_dict().unwrap();

        let acroform_id = catalog.get(b"AcroForm").unwrap().as_reference().unwrap();
        let acroform = doc
            .inner
            .get_object(acroform_id)
            .unwrap()
            .as_dict()
            .unwrap();

        assert_eq!(acroform.get(b"SigFlags").unwrap().as_i64().unwrap(), 3);
    }
}
