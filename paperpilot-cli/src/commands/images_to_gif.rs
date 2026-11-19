use paperpilot_core::error::PdfError;
use paperpilot_pdf::operations::images_to_gif::{images_to_gif_from_paths, ImagesToGifOptions};
use std::path::PathBuf;

pub fn handle_images_to_gif_command(
    input: &[PathBuf],
    inputs: Option<&[PathBuf]>,
    output: &PathBuf,
    fps: Option<u8>,
    delay: Option<u16>,
    width: Option<u32>,
) -> Result<(), PdfError> {
    let mut resolved_inputs: Vec<PathBuf> = Vec::new();
    if !input.is_empty() {
        resolved_inputs.extend_from_slice(input);
    }
    if let Some(extra) = inputs {
        resolved_inputs.extend_from_slice(extra);
    }

    if resolved_inputs.is_empty() {
        return Err(PdfError::ParseError("No input images specified for images-to-gif".to_string()));
    }

    let options = ImagesToGifOptions {
        fps,
        delay_centisecs: delay,
        width,
        loop_count: Some(0),
    };

    let output_bytes = images_to_gif_from_paths(&resolved_inputs, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("ImagesToGif complete.");
    Ok(())
}
