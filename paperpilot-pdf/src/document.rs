use lopdf::Document as LopdfInnerDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfDocument;
use std::any::Any;
use std::path::Path;

pub struct LopdfDocument {
    pub inner: LopdfInnerDocument,
}

impl LopdfDocument {
    pub fn new(inner: LopdfInnerDocument) -> Self {
        Self { inner }
    }
}

impl PdfDocument for LopdfDocument {
    fn page_count(&self) -> OperationResult<u32> {
        Ok(self.inner.get_pages().len() as u32)
    }

    fn save(&self, path: &Path) -> OperationResult<()> {
        let mut inner = self.inner.clone();
        inner.save(path).map_err(PdfError::IoError)?;
        Ok(())
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
