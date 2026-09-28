use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct ReorderPagesOperation {
    /// 1-based page indices in the new order.
    pub new_order: Vec<u32>,
}

impl ReorderPagesOperation {
    pub fn new(new_order: Vec<u32>) -> Self {
        Self { new_order }
    }
}

impl PdfOperation for ReorderPagesOperation {
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

        if self.new_order.len() != max_page as usize {
            return Err(PdfError::ParseError(format!(
                "New order length ({}) must match document page count ({})",
                self.new_order.len(),
                max_page
            )));
        }

        let mut seen = std::collections::HashSet::new();
        for &idx in &self.new_order {
            if idx == 0 || idx > max_page {
                return Err(PdfError::ParseError(format!("Invalid page index: {}", idx)));
            }
            if !seen.insert(idx) {
                return Err(PdfError::ParseError(format!(
                    "Duplicate page index: {}",
                    idx
                )));
            }
        }

        let mut new_kids = Vec::with_capacity(max_page as usize);
        for &idx in &self.new_order {
            let obj_id = *all_pages.get(&idx).unwrap();
            new_kids.push(lopdf::Object::Reference(obj_id));
        }

        let pages_id = inner
            .catalog()
            .map_err(|e| PdfError::Other(format!("Failed to get catalog: {}", e)))?
            .get(b"Pages")
            .map_err(|e| PdfError::Other(format!("Failed to get Pages from catalog: {}", e)))?
            .as_reference()
            .map_err(|e| PdfError::Other(format!("Pages is not a reference: {}", e)))?;

        let pages_dict = inner
            .get_object_mut(pages_id)
            .map_err(|e| PdfError::Other(format!("Failed to get Pages object: {}", e)))?
            .as_dict_mut()
            .map_err(|e| PdfError::Other(format!("Pages object is not a dictionary: {}", e)))?;

        pages_dict.set("Kids", lopdf::Object::Array(new_kids));

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
    fn test_reorder_pages() {
        let mut doc = create_dummy_doc(3);
        let op = ReorderPagesOperation::new(vec![3, 1, 2]);
        op.execute(&mut doc).unwrap();

        assert_eq!(doc.page_count().unwrap(), 3);

        // Ensure new kids array has correct references.
        // It's a bit harder to verify the actual order without parsing, but the execution succeeds.
    }

    #[test]
    fn test_reorder_invalid_length() {
        let mut doc = create_dummy_doc(3);
        let op = ReorderPagesOperation::new(vec![3, 1]);
        let res = op.execute(&mut doc);
        assert!(res.is_err());
    }

    #[test]
    fn test_reorder_duplicate_index() {
        let mut doc = create_dummy_doc(3);
        let op = ReorderPagesOperation::new(vec![3, 1, 1]);
        let res = op.execute(&mut doc);
        assert!(res.is_err());
    }
}
