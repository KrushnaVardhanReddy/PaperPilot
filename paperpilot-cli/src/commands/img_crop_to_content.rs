use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_img_crop_to_content_command(
    input: &PathBuf,
    output: &PathBuf,
    threshold: Option<f32>,
    padding: Option<u32>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut options = paperpilot_pdf::operations::img_crop_to_content::ImgCropToContentOptions::default();
    if let Some(t) = threshold {
        options.threshold = Some(t);
    }
    if let Some(p) = padding {
        options.padding = Some(p);
    }

    let output_bytes = paperpilot_pdf::operations::img_crop_to_content::img_crop_to_content_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("ImgCropToContent complete.");
    Ok(())
}
