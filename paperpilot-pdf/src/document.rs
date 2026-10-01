use lopdf::Document;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfDocument;
use std::any::Any;
use std::path::Path;

pub struct LopdfDocument {
    pub inner: Document,
}

impl Default for LopdfDocument {
    fn default() -> Self {
        Self::new()
    }
}

impl LopdfDocument {
    pub fn new() -> Self {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", lopdf::Object::Integer(0));
        pages_dict.set("Kids", lopdf::Object::Array(vec![]));

        doc.objects
            .insert(pages_id, lopdf::Object::Dictionary(pages_dict));

        let mut catalog = lopdf::Dictionary::new();
        catalog.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);

        doc.trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        Self { inner: doc }
    }

    pub fn load(path: &Path) -> OperationResult<Self> {
        let doc = Document::load(path).map_err(|e| PdfError::ParseError(e.to_string()))?;
        Ok(Self { inner: doc })
    }
}

impl PdfDocument for LopdfDocument {
    fn page_count(&self) -> OperationResult<u32> {
        Ok(self.inner.get_pages().len() as u32)
    }

    fn save(&self, path: &Path) -> OperationResult<()> {
        let mut inner = self.inner.clone();
        inner
            .save(path)
            .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
        Ok(())
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_new_document() {
        let doc = LopdfDocument::new();
        assert_eq!(doc.page_count().unwrap(), 0);
    }

    #[test]
    fn test_save_and_load_document() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.pdf");

        let doc = LopdfDocument::new();
        doc.save(&file_path).unwrap();

        assert!(file_path.exists());

        let loaded_doc = LopdfDocument::load(&file_path).unwrap();
        assert_eq!(loaded_doc.page_count().unwrap(), 0);
    }
}
