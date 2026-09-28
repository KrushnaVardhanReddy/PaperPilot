use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;

pub fn handle_validate(input: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::validate::ValidateOperation::new();
    op.execute(&mut doc)
}
