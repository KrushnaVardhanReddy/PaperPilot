use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RagChunk {
    pub page: u32,
    pub section_title: String,
    pub content: String,
}

pub struct PdfRagChunksOperation {
    pub chunks: Arc<Mutex<Vec<RagChunk>>>,
}

impl PdfRagChunksOperation {
    pub fn new() -> Self {
        Self {
            chunks: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for PdfRagChunksOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for PdfRagChunksOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let bytes = lopdf_doc.original_bytes.as_deref().unwrap_or(&[]);
        if bytes.is_empty() {
            if let Ok(mut lock) = self.chunks.lock() {
                *lock = Vec::new();
            }
            return Ok(());
        }

        let mut pdf = pdf_oxide::api::Pdf::from_bytes(bytes.to_vec())
            .map_err(|e| PdfError::Other(format!("pdf_oxide error: {:?}", e)))?;

        let mut all_chunks = Vec::new();
        let page_count = pdf.page_count().unwrap_or(0);
        let mut current_section = String::new();

        for i in 0..page_count {
            if let Ok(page_md) = pdf.to_markdown(i) {
                // simple split by headers
                let lines = page_md.lines();
                let mut current_content = String::new();

                for line in lines {
                    if line.starts_with("#") {
                        if !current_content.trim().is_empty() {
                            all_chunks.push(RagChunk {
                                page: (i + 1) as u32,
                                section_title: current_section.clone(),
                                content: current_content.trim().to_string(),
                            });
                        }
                        current_section = line.trim_start_matches('#').trim().to_string();
                        current_content.clear();
                    } else {
                        current_content.push_str(line);
                        current_content.push('\n');
                    }
                }

                if !current_content.trim().is_empty() {
                    all_chunks.push(RagChunk {
                        page: (i + 1) as u32,
                        section_title: current_section.clone(),
                        content: current_content.trim().to_string(),
                    });
                }
            }
        }

        if let Ok(mut lock) = self.chunks.lock() {
            *lock = all_chunks;
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rag_chunks() {
        let dummy_pdf = include_bytes!("../../tests/fixtures/sample.pdf");
        let mut doc = LopdfDocument::new();
        doc.original_bytes = Some(dummy_pdf.to_vec());

        let op = PdfRagChunksOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let chunks = op.chunks.lock().unwrap();
        assert!(!chunks.is_empty());
        assert!(chunks[0].page > 0);
    }
}
