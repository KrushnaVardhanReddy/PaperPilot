use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_overlay_underlay_command(
    input: &PathBuf,
    output: &PathBuf,
    template: Option<&PathBuf>,
    mode: Option<&str>,
    pages: Option<&str>,
    opacity: Option<f32>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut options =
        paperpilot_pdf::operations::pdf_overlay_underlay::PdfOverlayUnderlayOptions::default();

    if let Some(tpl) = template {
        options.template_path = Some(tpl.to_string_lossy().to_string());
    }
    if let Some(m) = mode {
        options.mode = m.to_string();
    }
    if let Some(p) = pages {
        options.pages = p.to_string();
    }
    if let Some(op) = opacity {
        options.opacity = op;
    }

    let output_bytes =
        paperpilot_pdf::operations::pdf_overlay_underlay::pdf_overlay_underlay_bytes(
            &input_bytes,
            &options,
        )
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfOverlayUnderlay complete.");
    Ok(())
}
