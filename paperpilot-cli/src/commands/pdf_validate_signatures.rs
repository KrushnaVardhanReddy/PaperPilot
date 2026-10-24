use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_validate_signatures_command(
    input: &PathBuf,
    output: &PathBuf,
    baseline: &Option<PathBuf>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut options =
        paperpilot_pdf::operations::pdf_validate_signatures::PdfValidateSignaturesOptions::default(
        );
    if let Some(baseline_path) = baseline {
        options.baseline_path = Some(baseline_path.to_string_lossy().to_string());
    }

    let output_bytes =
        paperpilot_pdf::operations::pdf_validate_signatures::pdf_validate_signatures_bytes(
            &input_bytes,
            &options,
        )
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfValidateSignatures complete.");

    Ok(())
}
