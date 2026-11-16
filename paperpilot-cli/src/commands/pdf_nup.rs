use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_nup_command(
    input: &PathBuf,
    output: &PathBuf,
    nup: u32,
    ordering: String,
    margin_pt: f32,
    spacing_pt: f32,
    draw_borders: bool,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let options = paperpilot_pdf::operations::pdf_nup::PdfNupOptions {
        nup,
        ordering,
        margin_pt,
        spacing_pt,
        draw_borders,
    };

    let output_bytes = paperpilot_pdf::operations::pdf_nup::pdf_nup_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfNup complete.");
    Ok(())
}
