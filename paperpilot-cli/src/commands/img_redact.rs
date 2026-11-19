use paperpilot_core::error::PdfError;
use paperpilot_pdf::operations::img_redact::RedactRect;
use std::path::PathBuf;

pub fn handle_img_redact_command(
    input: &PathBuf,
    output: &PathBuf,
    rect: &Option<Vec<String>>,
    color: &Option<String>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut rects = Vec::new();
    if let Some(r_list) = rect {
        for item in r_list {
            // Can be comma or whitespace separated: x,y,w,h
            let parts: Vec<&str> = item.split(|c| c == ',' || c == 'x' || c == ':').map(|s| s.trim()).collect();
            if parts.len() == 4 {
                if let (Ok(x), Ok(y), Ok(w), Ok(h)) = (
                    parts[0].parse::<u32>(),
                    parts[1].parse::<u32>(),
                    parts[2].parse::<u32>(),
                    parts[3].parse::<u32>(),
                ) {
                    rects.push(RedactRect { x, y, width: w, height: h });
                }
            }
        }
    }

    let mut options = paperpilot_pdf::operations::img_redact::ImgRedactOptions::default();
    options.rects = rects;
    if let Some(c) = color {
        options.color = Some(c.clone());
    }

    let output_bytes = paperpilot_pdf::operations::img_redact::img_redact_bytes(&input_bytes, &options)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("ImgRedact complete.");
    Ok(())
}
