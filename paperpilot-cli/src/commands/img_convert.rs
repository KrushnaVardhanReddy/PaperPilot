use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_img_convert_command(
    input: &PathBuf,
    output: &PathBuf,
    format: &Option<String>,
    quality: Option<u8>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    // If format is not explicitly passed, try inferring from output extension
    let target_format = format.clone().or_else(|| {
        output
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    });

    let options = paperpilot_pdf::operations::img_convert::ImgConvertOptions {
        format: target_format,
        quality,
    };

    let output_bytes = paperpilot_pdf::operations::img_convert::img_convert_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("ImgConvert complete.");
    Ok(())
}
