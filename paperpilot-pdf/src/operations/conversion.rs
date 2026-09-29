use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct PdfToHtmlOperation {
    pub extracted_html: Arc<Mutex<Option<String>>>,
}

impl PdfToHtmlOperation {
    pub fn new() -> Self {
        Self {
            extracted_html: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToHtmlOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head><title>PDF Export</title></head>\n<body>\n");

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    html.push_str(&format!("<div class=\"page\" id=\"page-{}\">\n", page_id));
                    html.push_str(&format!("<h2>Page {}</h2>\n", page_id));
                    for line in text.lines() {
                        if !line.trim().is_empty() {
                            html.push_str(&format!("<p>{}</p>\n", line.trim()));
                        }
                    }
                    html.push_str("</div>\n");
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        html.push_str("</body>\n</html>");

        if let Ok(mut lock) = self.extracted_html.lock() {
            *lock = Some(html);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct PdfToMarkdownOperation {
    pub extracted_markdown: Arc<Mutex<Option<String>>>,
}

impl PdfToMarkdownOperation {
    pub fn new() -> Self {
        Self {
            extracted_markdown: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToMarkdownOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut md = String::new();

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    md.push_str(&format!("# Page {}\n\n", page_id));
                    md.push_str(text.trim());
                    md.push_str("\n\n");
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        if let Ok(mut lock) = self.extracted_markdown.lock() {
            *lock = Some(md);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageMetadata {
    pub page_number: u32,
    pub char_count: usize,
    pub word_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmExportMetadata {
    pub total_pages: usize,
    pub total_chars: usize,
    pub total_words: usize,
    pub pages: Vec<PageMetadata>,
}

#[derive(Default)]
pub struct PdfToLlmExportOperation {
    pub extracted_markdown: Arc<Mutex<Option<String>>>,
    pub metadata: Arc<Mutex<Option<LlmExportMetadata>>>,
}

impl PdfToLlmExportOperation {
    pub fn new() -> Self {
        Self {
            extracted_markdown: Arc::new(Mutex::new(None)),
            metadata: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToLlmExportOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut md = String::new();
        let mut pages_meta = Vec::new();
        let mut total_chars = 0;
        let mut total_words = 0;

        for page_id in &all_pages {
            match inner.extract_text(&[*page_id]) {
                Ok(text) => {
                    let trimmed = text.trim();
                    let chars = trimmed.chars().count();
                    let words = trimmed.split_whitespace().count();

                    pages_meta.push(PageMetadata {
                        page_number: *page_id,
                        char_count: chars,
                        word_count: words,
                    });

                    total_chars += chars;
                    total_words += words;

                    md.push_str(&format!("## Page {}\n\n", page_id));
                    md.push_str(trimmed);
                    md.push_str("\n\n---\n\n");
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        let meta = LlmExportMetadata {
            total_pages: all_pages.len(),
            total_chars,
            total_words,
            pages: pages_meta,
        };

        if let Ok(mut lock) = self.extracted_markdown.lock() {
            *lock = Some(md);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        if let Ok(mut lock) = self.metadata.lock() {
            *lock = Some(meta);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonPage {
    pub page_number: u32,
    pub text: String,
    pub char_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonDocument {
    pub document_name: String,
    pub pages: Vec<JsonPage>,
}

pub struct PdfToJsonOperation {
    pub document_name: String,
    pub extracted_json: Arc<Mutex<Option<String>>>,
}

impl PdfToJsonOperation {
    pub fn new(document_name: String) -> Self {
        Self {
            document_name,
            extracted_json: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToJsonOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut pages_data = Vec::new();

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    let chars = text.chars().count();
                    pages_data.push(JsonPage {
                        page_number: page_id,
                        text,
                        char_count: chars,
                    });
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        let doc_struct = JsonDocument {
            document_name: self.document_name.clone(),
            pages: pages_data,
        };

        let json_str = serde_json::to_string_pretty(&doc_struct)
            .map_err(|e| PdfError::Other(format!("Failed to serialize JSON: {}", e)))?;

        if let Ok(mut lock) = self.extracted_json.lock() {
            *lock = Some(json_str);
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

            let content = "BT /F1 12 Tf 0 0 Td (Test Content) Tj ET";
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

        LopdfDocument { inner: doc }
    }

    #[test]
    fn test_pdf_to_html() {
        let mut doc = create_dummy_doc(2);
        let op = PdfToHtmlOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_html.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("<!DOCTYPE html>"));
        assert!(extracted.contains("<div class=\"page\" id=\"page-1\">"));
    }

    #[test]
    fn test_pdf_to_markdown() {
        let mut doc = create_dummy_doc(2);
        let op = PdfToMarkdownOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_markdown.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("# Page 1"));
    }

    #[test]
    fn test_pdf_to_llm_export() {
        let mut doc = create_dummy_doc(1);
        let op = PdfToLlmExportOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_markdown.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("## Page 1"));

        let meta = op.metadata.lock().unwrap();
        assert!(meta.is_some());
        let extracted_meta = meta.as_ref().unwrap();
        assert_eq!(extracted_meta.total_pages, 1);
        assert_eq!(extracted_meta.pages.len(), 1);
    }

    #[test]
    fn test_pdf_to_json() {
        let mut doc = create_dummy_doc(1);
        let op = PdfToJsonOperation::new("test_doc.pdf".to_string());
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_json.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("\"document_name\": \"test_doc.pdf\""));
    }
}
