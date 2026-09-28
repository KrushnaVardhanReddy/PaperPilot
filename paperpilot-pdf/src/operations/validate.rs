use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct ValidateOperation;

impl ValidateOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ValidateOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for ValidateOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;

        // Check if document is encrypted
        if inner.trailer.has(b"Encrypt") {
            return Err(PdfError::InvalidInput("Document is encrypted".to_string()));
        }

        // Validate page count > 0
        if inner.get_pages().is_empty() {
            return Err(PdfError::InvalidInput("Document has 0 pages".to_string()));
        }

        // Check for broken cross-references (verify that all objects referenced in the trailer/pages actually exist)
        // lopdf performs lazy loading or might retain references to non-existent objects.
        // We iterate over all object references we can find, but it's simpler to just check if `max_id` is reasonable
        // and if critical objects like Pages/Root exist.

        let _catalog_id = inner
            .catalog()
            .map_err(|_| PdfError::InvalidInput("Broken or missing Catalog".to_string()))?;

        // Also check if any referenced page object actually exists in the object map
        for (_page_number, object_id) in inner.get_pages() {
            if inner.get_object(object_id).is_err() {
                return Err(PdfError::InvalidInput(format!(
                    "Broken cross-reference: missing page object {:?}",
                    object_id
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document as LopdfInnerDocument;

    fn create_valid_test_document() -> LopdfDocument {
        let mut inner = LopdfInnerDocument::with_version("1.5");
        let pages_id = inner.new_object_id();
        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", lopdf::Object::Reference(pages_id));
        let page_id = inner.add_object(page_dict);

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set(
            "Kids",
            lopdf::Object::Array(vec![lopdf::Object::Reference(page_id)]),
        );
        pages_dict.set("Count", lopdf::Object::Integer(1));
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
    fn test_valid_document() {
        let mut doc = create_valid_test_document();
        let op = ValidateOperation::new();
        assert!(op.execute(&mut doc).is_ok());
    }

    #[test]
    fn test_invalid_page_count() {
        let mut doc = create_valid_test_document();

        // Remove the page reference from Kids array in Pages to simulate 0 pages
        let pages_id = {
            let root_dict = doc.inner.catalog().unwrap();
            root_dict.get(b"Pages").unwrap().as_reference().unwrap()
        };

        let pages_dict = doc
            .inner
            .get_object_mut(pages_id)
            .unwrap()
            .as_dict_mut()
            .unwrap();
        pages_dict.set("Kids", lopdf::Object::Array(vec![]));
        pages_dict.set("Count", lopdf::Object::Integer(0));

        let op = ValidateOperation::new();
        let result = op.execute(&mut doc);
        assert!(result.is_err());
        if let Err(PdfError::InvalidInput(msg)) = result {
            assert!(msg.contains("0 pages"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_encrypted_document() {
        let mut doc = create_valid_test_document();
        doc.inner.trailer.set(
            "Encrypt",
            lopdf::Object::Dictionary(lopdf::Dictionary::new()),
        );

        let op = ValidateOperation::new();
        let result = op.execute(&mut doc);
        assert!(result.is_err());
        if let Err(PdfError::InvalidInput(msg)) = result {
            assert!(msg.contains("encrypted"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_broken_cross_references_missing_catalog() {
        let mut doc = create_valid_test_document();
        doc.inner.trailer.remove(b"Root");

        let op = ValidateOperation::new();
        let result = op.execute(&mut doc);
        assert!(result.is_err());
        if let Err(PdfError::InvalidInput(_msg)) = result {
            // Success
        } else {
            panic!("Expected InvalidInput error");
        }
    }
}
