use crate::document::LopdfDocument;
use crate::operations::extract_text::ExtractTextOperation;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::path::PathBuf;

#[derive(Default)]
pub struct CompareOperation {
    pub input_b: Option<String>,
}

impl CompareOperation {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PdfOperation for CompareOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc_a = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        let input_b_path = self.input_b.as_ref().ok_or_else(|| {
            PdfError::InvalidInput("Second document path (input_b) is required".into())
        })?;

        // Load document B
        let mut doc_b = LopdfDocument::load(&PathBuf::from(input_b_path))?;

        // Compare page counts
        let pages_a = lopdf_doc_a.inner.get_pages();
        let pages_b = doc_b.inner.get_pages();
        if pages_a.len() != pages_b.len() {
            return Err(PdfError::InvalidInput(format!(
                "Documents differ in page count: A has {}, B has {}",
                pages_a.len(),
                pages_b.len()
            )));
        }

        // Extract and compare text
        let extract_op = ExtractTextOperation::new(None);

        extract_op.execute(document)?;
        let text_a_mutex = extract_op.extracted_text.clone();
        let text_a_opt = text_a_mutex.lock().unwrap().clone();

        // Use a clean operation instance for document B to avoid mingling states
        let extract_op_b = ExtractTextOperation::new(None);
        let mut dyn_doc_b: &mut dyn PdfDocument = &mut doc_b;
        extract_op_b.execute(dyn_doc_b)?;
        let text_b_mutex = extract_op_b.extracted_text.clone();
        let text_b_opt = text_b_mutex.lock().unwrap().clone();

        if text_a_opt != text_b_opt {
            return Err(PdfError::InvalidInput(
                "Documents differ in extracted text content".into(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;
    use lopdf::dictionary;
    use tempfile::tempdir;

    fn create_test_pdf(path: &PathBuf, text_content: &str) {
        let mut inner = LopdfInnerDocument::with_version("1.5");
        let pages_id = inner.new_object_id();
        let font_id = inner.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });

        let resources_id = inner.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });

        let content = lopdf::content::Content {
            operations: vec![
                lopdf::content::Operation::new("BT", vec![]),
                lopdf::content::Operation::new("Tf", vec!["F1".into(), 12.into()]),
                lopdf::content::Operation::new("Td", vec![10.into(), 10.into()]),
                lopdf::content::Operation::new(
                    "Tj",
                    vec![lopdf::Object::string_literal(text_content)],
                ),
                lopdf::content::Operation::new("ET", vec![]),
            ],
        };
        let content_id = inner.add_object(lopdf::Stream::new(
            lopdf::Dictionary::new(),
            content.encode().unwrap(),
        ));

        let page_id = inner.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
        });

        inner.set_object(
            pages_id,
            dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
            },
        );

        let catalog_id = inner.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        inner.trailer.set("Root", catalog_id);

        inner.save(path).unwrap();
    }

    #[test]
    fn test_compare_operation() {
        let temp_dir = tempdir().unwrap();

        let path_a = temp_dir.path().join("doc_a.pdf");
        let path_b = temp_dir.path().join("doc_b.pdf");
        let path_c = temp_dir.path().join("doc_c.pdf");

        create_test_pdf(&path_a, "Hello World");
        create_test_pdf(&path_b, "Hello World");
        create_test_pdf(&path_c, "Different Text");

        let mut doc_a = LopdfDocument::load(&path_a).unwrap();

        // Compare same documents
        let mut op1 = CompareOperation::new();
        op1.input_b = Some(path_b.to_string_lossy().to_string());
        assert!(op1.execute(&mut doc_a).is_ok());

        // Compare different documents
        let mut op2 = CompareOperation::new();
        op2.input_b = Some(path_c.to_string_lossy().to_string());
        assert!(op2.execute(&mut doc_a).is_err());
    }
}
