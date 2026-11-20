use paperpilot_core::error::PdfError;
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
            // Can be single rect "x,y,w,h" or semicolon/newline separated "x1,y1,w1,h1;x2,y2,w2,h2"
            let parsed = paperpilot_pdf::operations::img_redact::parse_rects_str(item);
            if !parsed.is_empty() {
                rects.extend(parsed);
            } else if let Some(single) = paperpilot_pdf::operations::img_redact::parse_rect_str(item) {
                rects.push(single);
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
