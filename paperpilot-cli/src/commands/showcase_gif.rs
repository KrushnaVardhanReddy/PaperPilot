use paperpilot_core::error::PdfError;
use paperpilot_pdf::operations::showcase_gif::ShowcaseGifOptions;
use std::path::PathBuf;

pub fn handle_showcase_gif_command(
    input: &PathBuf,
    output: &PathBuf,
    fps: Option<u8>,
    delay: Option<u16>,
    width: Option<u32>,
    transition_frames: Option<u8>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let options = ShowcaseGifOptions {
        fps,
        delay_centisecs: delay,
        width,
        transition_frames,
    };

    let output_bytes = paperpilot_pdf::operations::showcase_gif::showcase_gif_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(PdfError::IoError)?;
        }
    }

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("Generated Showcase GIF successfully: {}", output.display());
    Ok(())
}
