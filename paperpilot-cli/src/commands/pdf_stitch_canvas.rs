use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_stitch_canvas_command(
    input: &PathBuf,
    output: &PathBuf,
    direction: &String,
    margin: &f32,
    spacing: &f32,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let options = paperpilot_pdf::operations::pdf_stitch_canvas::PdfStitchCanvasOptions {
        direction: direction.clone(),
        margin_pt: *margin,
        spacing_pt: *spacing,
    };

    let output_bytes = paperpilot_pdf::operations::pdf_stitch_canvas::pdf_stitch_canvas_bytes(
        &input_bytes,
        &options,
    )
    .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfStitchCanvas complete.");
    Ok(())
}
