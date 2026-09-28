use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct ExtractPagesOperation {
    pub page_indices: Vec<u32>,
}

impl ExtractPagesOperation {
    pub fn new(page_indices: Vec<u32>) -> Self {
        Self { page_indices }
    }
}

impl PdfOperation for ExtractPagesOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let mut inner = lopdf_doc.inner.borrow_mut();

        let all_pages = inner.get_pages();
        let max_page = all_pages.len() as u32;

        for &idx in &self.page_indices {
            if idx == 0 || idx > max_page {
                return Err(PdfError::ParseError(format!("Invalid page index: {}", idx)));
            }
        }

        let pages_to_delete: Vec<u32> = (1..=max_page)
            .filter(|idx| !self.page_indices.contains(idx))
            .collect();

        inner.delete_pages(&pages_to_delete);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, Document, Object};

    fn create_dummy_doc(pages: u32) -> LopdfDocument {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut page_ids = vec![];

        for _ in 0..pages {
            let mut page_dict = Dictionary::new();
            page_dict.set("Type", Object::Name(b"Page".to_vec()));
            page_dict.set("Parent", Object::Reference(pages_id));
            let page_id = doc.add_object(page_dict);
            page_ids.push(Object::Reference(page_id));
        }

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", Object::Integer(pages as i64));
        pages_dict.set("Kids", Object::Array(page_ids));

        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
        doc.trailer
            .set("Root", Object::Dictionary(Dictionary::new()));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        LopdfDocument::new(doc)
    }

    #[test]
    fn test_extract_pages() {
        let mut doc = create_dummy_doc(5);
        let op = ExtractPagesOperation::new(vec![2, 4]);
        op.execute(&mut doc).unwrap();

        assert_eq!(doc.page_count().unwrap(), 2);
    }

    #[test]
    fn test_extract_invalid_page() {
        let mut doc = create_dummy_doc(3);
        let op = ExtractPagesOperation::new(vec![4]);
        let res = op.execute(&mut doc);
        assert!(res.is_err());
    }
}
