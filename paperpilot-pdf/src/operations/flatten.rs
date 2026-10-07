use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct FlattenOperation;

impl FlattenOperation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FlattenOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for FlattenOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        let catalog_ref = lopdf_doc
            .inner
            .trailer
            .get(b"Root")
            .and_then(|r| r.as_reference())
            .map_err(|e| PdfError::Other(format!("Missing Root reference: {:?}", e)))?;

        if let Ok(catalog) = lopdf_doc.inner.get_object_mut(catalog_ref)
            && let Ok(dict) = catalog.as_dict_mut()
        {
            dict.remove(b"AcroForm");
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

        let mut acroform_dict = lopdf::Dictionary::new();
        acroform_dict.set("Fields", lopdf::Object::Array(vec![]));
        let acroform_id = inner.add_object(acroform_dict);

        catalog_dict.set("AcroForm", lopdf::Object::Reference(acroform_id));

        let catalog_id = inner.add_object(catalog_dict);

        inner
            .trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        LopdfDocument { inner }
    }

    #[test]
    fn test_flatten_operation() {
        let mut doc = create_test_document();
        let op = FlattenOperation::new();

        // Check AcroForm exists before
        let catalog_id = doc
            .inner
            .trailer
            .get(b"Root")
            .unwrap()
            .as_reference()
            .unwrap();
        let catalog = doc.inner.get_object(catalog_id).unwrap().as_dict().unwrap();
        assert!(catalog.has(b"AcroForm"));

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        // Check AcroForm does not exist after
        let catalog = doc.inner.get_object(catalog_id).unwrap().as_dict().unwrap();
        assert!(!catalog.has(b"AcroForm"));
    }
}
