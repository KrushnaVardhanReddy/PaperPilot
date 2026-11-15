use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_scale_pages_command(
    input: &PathBuf,
    output: &PathBuf,
    target_size: Option<String>,
    scale_factor: Option<f32>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut options = paperpilot_pdf::operations::pdf_scale_pages::PdfScalePagesOptions::default();
    if let Some(size) = target_size {
        options.target_size = size;
    }
    if let Some(factor) = scale_factor {
        options.scale_factor = factor;
    }

    let output_bytes =
        paperpilot_pdf::operations::pdf_scale_pages::pdf_scale_pages_bytes(&input_bytes, &options)
            .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfScalePages complete.");
    Ok(())
}
