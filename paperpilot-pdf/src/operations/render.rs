use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct RenderOperation;

impl RenderOperation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RenderOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for RenderOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        // Fallback: extract text as plain-text "render" when no rendering engine is available
        let text_op = crate::operations::extract_text::ExtractTextOperation::new(None);
        text_op.execute(document)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use lopdf::dictionary;

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
                lopdf::content::Operation::new("Tj", vec![lopdf::Object::string_literal(text_content)]),
                lopdf::content::Operation::new("ET", vec![]),
            ],
        };
        let content_id = inner.add_object(lopdf::Stream::new(lopdf::Dictionary::new(), content.encode().unwrap()));

        let page_id = inner.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
        });

        inner.set_object(pages_id, dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        });

        let catalog_id = inner.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        inner.trailer.set("Root", catalog_id);

        inner.save(path).unwrap();
    }

    #[test]
    fn test_render_operation() {
        let temp_dir = tempdir().unwrap();
        let doc_path = temp_dir.path().join("doc.pdf");
        create_test_pdf(&doc_path, "Hello World");

        let mut doc = LopdfDocument::load(&doc_path).unwrap();
        let op = RenderOperation::new();

        let result = op.execute(&mut doc);
        assert!(result.is_ok());
    }
}
