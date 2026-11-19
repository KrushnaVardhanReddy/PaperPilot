use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_img_optimize_for_llm_command(
    input: &PathBuf,
    output: &PathBuf,
    max_dim: Option<u32>,
    quality: Option<u8>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut options = paperpilot_pdf::operations::img_optimize_for_llm::ImgOptimizeForLlmOptions::default();
    if let Some(m) = max_dim {
        options.max_dim = Some(m);
    }
    if let Some(q) = quality {
        options.quality = Some(q);
    }

    let output_bytes = paperpilot_pdf::operations::img_optimize_for_llm::img_optimize_for_llm_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("ImgOptimizeForLlm complete.");
    Ok(())
}
