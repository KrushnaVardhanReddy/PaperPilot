use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

#[derive(Default)]
pub struct BookmarksOperation {}

impl BookmarksOperation {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PdfOperation for BookmarksOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        // Access the Outline (bookmarks) from the Catalog dictionary
        let catalog = lopdf_doc
            .inner
            .catalog()
            .map_err(|e| PdfError::Other(format!("Cannot read catalog: {}", e)))?;

        // Check if Outlines entry exists
        if catalog.get(b"Outlines").is_err() {
            // No bookmarks - this is valid, not an error
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document;

    #[test]
    fn test_bookmarks_operation() {
        let mut inner = Document::with_version("1.5");
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

        let op = BookmarksOperation::new();
        let result = op.execute(&mut doc);

        assert!(result.is_ok());
    }
}
