use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_merge_alternate_command(
    input_a: &PathBuf,
    input_b: &PathBuf,
    output: &PathBuf,
    reverse_a: bool,
    reverse_b: bool,
    step: usize,
    step_b: usize,
) -> Result<(), PdfError> {
    let a_bytes = std::fs::read(input_a).map_err(PdfError::IoError)?;
    let b_bytes = std::fs::read(input_b).map_err(PdfError::IoError)?;

    let options = paperpilot_pdf::operations::pdf_merge_alternate::PdfMergeAlternateOptions {
        step,
        step_b,
        reverse_a,
        reverse_b,
    };

    let output_bytes = paperpilot_pdf::operations::pdf_merge_alternate::pdf_merge_alternate_bytes(
        &a_bytes, &b_bytes, &options,
    )
    .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfMergeAlternate complete.");
    Ok(())
}
