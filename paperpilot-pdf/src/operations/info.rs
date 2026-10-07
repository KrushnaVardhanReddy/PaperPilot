use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfInfoResult {
    pub page_count: u32,
    pub version: String,
}

#[derive(Default)]
pub struct InfoOperation {
    pub result: Arc<Mutex<Option<PdfInfoResult>>>,
}

impl InfoOperation {
    pub fn new() -> Self {
        Self {
            result: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for InfoOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        let pages = lopdf_doc.inner.get_pages();
        let version = lopdf_doc.inner.version.clone();

        let res = PdfInfoResult {
            page_count: pages.len() as u32,
            version,
        };

        if let Ok(mut lock) = self.result.lock() {
            *lock = Some(res);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document as LopdfInnerDocument;

    fn create_empty_document() -> LopdfDocument {
        LopdfDocument {
            inner: LopdfInnerDocument::with_version("1.5"),
        }
    }

    #[test]
    fn test_info_operation() {
        let mut doc = create_empty_document();
        let op = InfoOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let res = op.result.lock().unwrap().clone().unwrap();
        assert_eq!(res.page_count, 0);
        assert_eq!(res.version, "1.5");
    }
}
