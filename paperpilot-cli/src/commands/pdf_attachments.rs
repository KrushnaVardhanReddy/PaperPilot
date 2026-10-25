use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_attachments_command(
    input: &PathBuf,
    output: &PathBuf,
    action: &Option<String>,
    file_name: &Option<String>,
    file: &Option<PathBuf>,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let mut options = paperpilot_pdf::operations::pdf_attachments::PdfAttachmentsOptions::default();
    if let Some(a) = action {
        options.action = a.clone();
    }
    if let Some(fn_name) = file_name {
        options.file_name = Some(fn_name.clone());
    }

    if options.action == "add" {
        if let Some(f) = file {
            let file_bytes = std::fs::read(f).map_err(PdfError::IoError)?;
            use base64::{Engine, engine::general_purpose::STANDARD as b64_standard};
            options.file_data_base64 = Some(b64_standard.encode(&file_bytes));
        }
    }

    let output_bytes =
        paperpilot_pdf::operations::pdf_attachments::pdf_attachments_bytes(&input_bytes, &options)
            .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfAttachments complete.");
    Ok(())
}
