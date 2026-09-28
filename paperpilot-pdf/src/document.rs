use lopdf::Document;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfDocument;
use std::cell::RefCell;
use std::path::Path;

pub struct LopdfDocument {
    pub inner: RefCell<Document>,
}

impl LopdfDocument {
    pub fn new(inner: Document) -> Self {
        Self {
            inner: RefCell::new(inner),
        }
    }
}

impl PdfDocument for LopdfDocument {
    fn page_count(&self) -> OperationResult<u32> {
        Ok(self.inner.borrow().get_pages().len() as u32)
    }

    fn save(&self, path: &Path) -> OperationResult<()> {
        self.inner
            .borrow_mut()
            .save(path)
            .map(|_| ())
            .map_err(PdfError::IoError)
    }
}
