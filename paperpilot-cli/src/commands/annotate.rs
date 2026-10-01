use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfDocument;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::annotate::{AnnotateOperation, AnnotationParams};
use std::fs;
use std::path::Path;

pub fn handle_annotate(input: &Path, data: &Path, output: &Path) -> OperationResult<()> {
    let data_str = fs::read_to_string(data).map_err(PdfError::IoError)?;

    let annotations: Vec<AnnotationParams> =
        serde_json::from_str(&data_str).map_err(|e| PdfError::InvalidInput(e.to_string()))?;

    let mut doc = LopdfDocument::load(input)?;
    let op = AnnotateOperation::new().with_annotations(annotations);

    op.execute(&mut doc.inner)?;

    doc.save(output)
}
