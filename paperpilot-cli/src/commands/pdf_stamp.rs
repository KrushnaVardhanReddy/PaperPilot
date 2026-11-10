use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_stamp_command(
    input: &PathBuf,
    output: &PathBuf,
    image: Option<&PathBuf>,
    x: Option<f32>,
    y: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    opacity: Option<f32>,
    pages: Option<&str>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut options = paperpilot_pdf::operations::pdf_stamp::PdfStampOptions::default();
    if let Some(img) = image {
        options.image_path = Some(img.to_string_lossy().to_string());
    }
    if let Some(val) = x {
        options.x = val;
    }
    if let Some(val) = y {
        options.y = val;
    }
    if let Some(val) = width {
        options.width = val;
    }
    if let Some(val) = height {
        options.height = val;
    }
    if let Some(val) = opacity {
        options.opacity = val;
    }
    if let Some(val) = pages {
        options.pages = val.to_string();
    }

    let output_bytes =
        paperpilot_pdf::operations::pdf_stamp::pdf_stamp_bytes(&input_bytes, &options)
            .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfStamp complete.");
    Ok(())
}
