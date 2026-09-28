use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::sync::{Arc, Mutex};

pub struct ExtractTextOperation {
    pub pages: Option<Vec<u32>>,
    pub extracted_text: Arc<Mutex<Option<String>>>,
}

impl ExtractTextOperation {
    pub fn new(pages: Option<Vec<u32>>) -> Self {
        Self {
            pages,
            extracted_text: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for ExtractTextOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;

        let target_pages = match &self.pages {
            Some(pages) => pages.clone(),
            None => {
                let all_pages = inner.get_pages();
                all_pages.keys().copied().collect()
            }
        };

        let mut full_text = String::new();

        for page_id in target_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    full_text.push_str(&text);
                    full_text.push('\n');
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        if let Ok(mut lock) = self.extracted_text.lock() {
            *lock = Some(full_text);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

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

            // Basic content stream with text to extract
            let content = "BT /F1 12 Tf 0 0 Td (Test) Tj ET";
            let content_stream =
                lopdf::Stream::new(Dictionary::new(), content.as_bytes().to_vec());
            let content_id = doc.add_object(Object::Stream(content_stream));
            page_dict.set("Contents", Object::Reference(content_id));

            let mut resources_dict = Dictionary::new();
            let mut font_dict = Dictionary::new();
            let mut f1_dict = Dictionary::new();
            f1_dict.set("Type", Object::Name(b"Font".to_vec()));
            f1_dict.set("Subtype", Object::Name(b"Type1".to_vec()));
            f1_dict.set("BaseFont", Object::Name(b"Helvetica".to_vec()));
            let f1_id = doc.add_object(Object::Dictionary(f1_dict));
            font_dict.set("F1", Object::Reference(f1_id));
            resources_dict.set("Font", Object::Dictionary(font_dict));

            page_dict.set("Resources", Object::Dictionary(resources_dict));

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

        LopdfDocument { inner: doc }
    }

    #[test]
    fn test_extract_text_all_pages() {
        let mut doc = create_dummy_doc(2);
        let op = ExtractTextOperation::new(None);
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_text.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        // The extraction might not be perfect with this dummy font, but it shouldn't fail
        assert!(!extracted.is_empty());
    }

    #[test]
    fn test_extract_text_specific_pages() {
        let mut doc = create_dummy_doc(3);
        let op = ExtractTextOperation::new(Some(vec![2, 3]));
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_text.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(!extracted.is_empty());
    }
}
