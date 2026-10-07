use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfDocument;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::annotate::{AnnotateOperation, AnnotationParams};
use std::fs;
use std::path::Path;

pub fn handle_annotate(input: &Path, data: &Path, output: &Path) -> OperationResult<()> {
    let data_str = if data.exists() {
        fs::read_to_string(data).map_err(PdfError::IoError)?
    } else {
        let s = data.to_string_lossy().to_string();
        if s.starts_with('[') || s.starts_with('{') {
            s
        } else {
            return Err(PdfError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found or invalid inline JSON string",
            )));
        }
    };

    let annotations: Vec<AnnotationParams> =
        serde_json::from_str(&data_str).map_err(|e| PdfError::InvalidInput(e.to_string()))?;

    let mut doc = LopdfDocument::load(input)?;
    let op = AnnotateOperation::new().with_annotations(annotations);

    op.execute(&mut doc.inner)?;

    doc.save(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn test_handle_annotate_inline_json() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("input.pdf");
        let output_path = dir.path().join("output.pdf");

        // Use the existing multi_page fixture which is valid.
        // Paths in cargo tests are relative to the crate root.
        let fixture_path = Path::new("../tests/e2e_fixtures/single_page.pdf");
        let fixture_path2 = Path::new("tests/e2e_fixtures/single_page.pdf");
        let actual_fixture = if fixture_path.exists() {
            fixture_path
        } else if fixture_path2.exists() {
            fixture_path2
        } else {
            return; // skip if not found
        };

        std::fs::copy(actual_fixture, &input_path).unwrap();

        let json_str = "[{\"id\": \"test-id-1\",\"type\": \"note\",\"page\": 1,\"x\": 50.0,\"y\": 50.0,\"content\": \"Test Annotation\",\"color\": \"#FF0000\"}]";

        let result = handle_annotate(&input_path, Path::new(json_str), &output_path);

        if let Err(e) = &result {
            println!("Error from handle_annotate: {:?}", e);
        }
        assert!(result.is_ok(), "Failed to handle inline JSON");

        let out_doc = LopdfDocument::load(&output_path).unwrap();
        let pages = out_doc.page_count().unwrap();
        assert_eq!(pages, 1);
    }
}
