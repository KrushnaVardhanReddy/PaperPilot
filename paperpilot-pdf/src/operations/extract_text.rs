use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::sync::{Arc, Mutex};

/// Extract page-ordered Markdown (including detected tables) without disk I/O.
/// Shared by document operations and binary-upload ingestion adapters.
pub fn run_extract_text_bytes(
    input_bytes: &[u8],
    pages: Option<&[u32]>,
) -> OperationResult<Vec<String>> {
    let mut pdf = pdf_oxide::api::Pdf::from_bytes(input_bytes.to_vec())
        .map_err(|error| PdfError::ParseError(error.to_string()))?;
    if pdf
        .editor()
        .is_some_and(|editor| editor.source().is_encrypted() && !editor.source().is_authenticated())
    {
        return Err(PdfError::InvalidInput(
            "Unlock the encrypted PDF before extracting text".to_owned(),
        ));
    }
    let page_count = pdf
        .page_count()
        .map_err(|error| PdfError::ParseError(error.to_string()))?;
    let target_pages: Vec<u32> = match pages {
        Some(pages) => pages.to_vec(),
        None => (1..=page_count as u32).collect(),
    };

    target_pages
        .into_iter()
        .filter(|&page| page >= 1 && page as usize <= page_count)
        .map(|page| {
            pdf.to_markdown((page - 1) as usize).map_err(|error| {
                PdfError::Other(format!("Failed to extract text from page {page}: {error}"))
            })
        })
        .collect()
}

pub struct ExtractTextOperation {
    pub pages: Option<Vec<u32>>,
    pub extracted_text: Arc<Mutex<Option<Vec<String>>>>,
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

        let mut pages_text = Vec::new();

        if let Some(bytes) = lopdf_doc.original_bytes.as_deref()
            && !bytes.is_empty()
            && let Ok(text) = run_extract_text_bytes(bytes, self.pages.as_deref())
        {
            pages_text = text;
        }

        if pages_text.is_empty() {
            let inner = &mut lopdf_doc.inner;

            let target_pages = match &self.pages {
                Some(pages) => pages.clone(),
                None => {
                    let all_pages = inner.get_pages();
                    all_pages.keys().copied().collect()
                }
            };

            for page_id in target_pages {
                match inner.extract_text(&[page_id]) {
                    Ok(text) => {
                        pages_text.push(text);
                    }
                    Err(e) => {
                        return Err(PdfError::Other(format!(
                            "Failed to extract text from page {}: {}",
                            page_id, e
                        )));
                    }
                }
            }
        }

        if let Ok(mut lock) = self.extracted_text.lock() {
            *lock = Some(pages_text);
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

        for page_number in 1..=pages {
            let mut page_dict = Dictionary::new();
            page_dict.set("Type", Object::Name(b"Page".to_vec()));
            page_dict.set("Parent", Object::Reference(pages_id));
            page_dict.set("MediaBox", vec![0.into(), 0.into(), 612.into(), 792.into()]);

            // Basic content stream with text to extract
            let content = format!("BT /F1 12 Tf 72 720 Td (Test page {page_number}) Tj ET");
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

        LopdfDocument {
            inner: doc,
            original_bytes: None,
        }
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
        assert_eq!(extracted.len(), 2);
    }

    #[test]
    fn test_extract_text_specific_pages() {
        let mut doc = create_dummy_doc(3);
        let op = ExtractTextOperation::new(Some(vec![2, 3]));
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_text.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert_eq!(extracted.len(), 2);
    }

    #[test]
    fn test_extract_text_bytes_page_selection_and_legacy_parity() {
        let mut document = create_dummy_doc(3);
        let mut bytes = Vec::new();
        document.inner.save_to(&mut bytes).unwrap();
        let pages = run_extract_text_bytes(&bytes, Some(&[3, 1])).unwrap();
        assert_eq!(pages.len(), 2);
        assert!(pages[0].contains("Test page 3"));
        assert!(pages[1].contains("Test page 1"));

        document.original_bytes = Some(bytes);
        let operation = ExtractTextOperation::new(Some(vec![3, 1]));
        operation.execute(&mut document).unwrap();
        assert_eq!(
            operation.extracted_text.lock().unwrap().as_ref().unwrap(),
            &pages
        );
    }

    #[test]
    fn test_extract_text_bytes_rejects_corrupt_pdf() {
        assert!(run_extract_text_bytes(b"%PDF-1.7\ncorrupt", None).is_err());
    }

    #[test]
    fn test_extract_text_bytes_requires_unlocked_pdf() {
        let mut document = create_dummy_doc(1);
        let encrypt = crate::operations::encrypt::EncryptOperation {
            user_password: Some("user-password".to_owned()),
            owner_password: Some("owner-password".to_owned()),
        };
        encrypt.execute(&mut document).unwrap();
        let mut bytes = Vec::new();
        document.inner.save_to(&mut bytes).unwrap();
        assert!(run_extract_text_bytes(&bytes, None).is_err());
    }
}
