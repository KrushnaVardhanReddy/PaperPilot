use crate::document::LopdfDocument;
use aho_corasick::AhoCorasick;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassificationResult {
    #[serde(rename = "type")]
    pub category: String,
    pub confidence: f32,
}

pub struct PdfClassifyOperation {
    pub classification: Arc<Mutex<Option<ClassificationResult>>>,
}

impl PdfClassifyOperation {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            classification: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfClassifyOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let mut all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();
        all_pages.sort_unstable();

        // Extract first 2-3 pages
        let pages_to_extract = all_pages.into_iter().take(3).collect::<Vec<_>>();
        let mut text = String::new();

        for page_id in pages_to_extract {
            if let Ok(extracted) = inner.extract_text(&[page_id]) {
                text.push_str(&extracted);
                text.push(' ');
            }
        }

        let text_lower = text.to_lowercase();

        let categories = vec![
            ("Invoice", vec!["invoice", "total", "tax", "due date", "balance"]),
            ("Contract", vec!["agreement", "hereby", "party of the first part", "signatures"]),
            ("Research Paper", vec!["abstract", "references", "methodology"]),
            ("Form", vec!["application", "date of birth", "signature"]),
        ];

        let mut best_category = "Unknown".to_string();
        let mut best_score = 0;
        let mut total_score = 0;

        for (category, keywords) in &categories {
            let ac = AhoCorasick::builder()
                .ascii_case_insensitive(true)
                .build(keywords)
                .unwrap();
            let matches = ac.find_iter(&text_lower).count();

            total_score += matches;

            if matches > best_score {
                best_score = matches;
                best_category = category.to_string();
            }
        }

        let confidence = if total_score > 0 {
            (best_score as f32) / (total_score as f32)
        } else {
            0.0
        };

        if let Ok(mut lock) = self.classification.lock() {
            *lock = Some(ClassificationResult {
                category: best_category,
                confidence,
            });
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

    fn create_dummy_doc(content_str: &str) -> LopdfDocument {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let mut page_dict = Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", Object::Reference(pages_id));

        let content = format!("BT /F1 12 Tf 0 0 Td ({}) Tj ET", content_str);
        let content_stream = lopdf::Stream::new(Dictionary::new(), content.as_bytes().to_vec());
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

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", Object::Integer(1));
        pages_dict.set("Kids", Object::Array(vec![Object::Reference(page_id)]));

        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
        doc.trailer.set("Root", Object::Dictionary(Dictionary::new()));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        LopdfDocument { inner: doc }
    }

    #[test]
    fn test_pdf_classify() {
        let mut doc = create_dummy_doc("Invoice Total Tax Balance");
        let op = PdfClassifyOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let res = op.classification.lock().unwrap();
        assert!(res.is_some());
        let class = res.as_ref().unwrap();
        assert_eq!(class.category, "Invoice");
        assert!(class.confidence > 0.0);
    }
}
