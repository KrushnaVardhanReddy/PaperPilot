use crate::document::LopdfDocument;
use lopdf::{Dictionary, Object};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct LinearizeOperation;

impl LinearizeOperation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinearizeOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for LinearizeOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        // Fast Web View / Linearized structure:
        // Set /Linearized dictionary in document trailer and create Linearized parameter dictionary.
        let mut lin_dict = Dictionary::new();
        lin_dict.set("Linearized", Object::Real(1.0));
        let file_len = lopdf_doc
            .original_bytes
            .as_ref()
            .map(|b| b.len() as i64)
            .unwrap_or(4096);
        lin_dict.set("L", Object::Integer(file_len));
        lin_dict.set("O", Object::Integer(1));
        lin_dict.set("E", Object::Integer(1024));
        lin_dict.set("N", Object::Integer(lopdf_doc.inner.get_pages().len() as i64));
        lin_dict.set("T", Object::Integer(file_len - 100));

        let lin_id = lopdf_doc.inner.add_object(lin_dict.clone());
        lopdf_doc.inner.trailer.set("Linearized", Object::Reference(lin_id));

        lopdf_doc.inner.compress();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document;

    #[test]
    fn test_linearize_operation() {
        let doc = Document::with_version("1.5");
        let mut lopdf_doc = LopdfDocument {
            inner: doc,
            original_bytes: None,
        };
        let op = LinearizeOperation::new();

        let result = op.execute(&mut lopdf_doc);
        assert!(result.is_ok());
    }
}
