use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_img_deskew_command(
    input: &PathBuf,
    output: &PathBuf,
    angle: Option<f32>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut options = paperpilot_pdf::operations::img_deskew::ImgDeskewOptions::default();
    if let Some(a) = angle {
        options.angle = Some(a);
        options.auto_detect = Some(false);
    }

    let output_bytes = paperpilot_pdf::operations::img_deskew::img_deskew_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("ImgDeskew complete.");
    Ok(())
}
