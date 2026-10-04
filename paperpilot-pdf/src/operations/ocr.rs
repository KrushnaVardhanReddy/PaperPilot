use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use std::path::PathBuf;

pub struct OcrOperationImpl {
    pub output_path: Option<PathBuf>,
}

impl OcrOperationImpl {
    pub fn new() -> Self {
        Self { output_path: None }
    }
}

impl Default for OcrOperationImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for OcrOperationImpl {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let text_op = crate::operations::extract_text::ExtractTextOperation::new(None);
        let result = text_op.execute(document);

        if let Some(path) = &self.output_path {
            document.save(path)?;
        }

        result
    }
}

pub type OcrOperation = OcrOperationImpl;
#[allow(non_upper_case_globals)]
pub const OcrOperation: OcrOperationImpl = OcrOperationImpl { output_path: None };

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
    fn test_ocr_operation() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("doc.pdf");
        create_test_pdf(&doc_path, "Hello World");

        let mut doc = LopdfDocument::load(&doc_path).unwrap();
        let mut op = OcrOperationImpl::new();

        let output_path = temp_dir.path().join("out.pdf");
        op.output_path = Some(output_path.clone());

        let result = op.execute(&mut doc);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }
}
