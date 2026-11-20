use paperpilot_core::error::PdfError;
use paperpilot_pdf::operations::pdf_to_gif::PdfToGifOptions;
use std::path::PathBuf;

pub fn handle_pdf_to_gif_command(
    input: &PathBuf,
    output: &PathBuf,
    fps: Option<u8>,
    delay: Option<u16>,
    max_pages: Option<usize>,
    width: Option<u32>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let options = PdfToGifOptions {
        fps,
        delay_centisecs: delay,
        max_pages,
        width,
    };

    let output_bytes = paperpilot_pdf::operations::pdf_to_gif::pdf_to_gif_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(PdfError::IoError)?;
        }
    }

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("Converted PDF to animated GIF successfully: {}", output.display());
    Ok(())
}
