use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_sanitize_command(
    input: &PathBuf,
    output: &PathBuf,
    remove_metadata: bool,
    remove_attachments: bool,
    remove_scripts: bool,
) -> Result<(), PdfError> {
    let doc_source = super::open_document_source(input)?;

    let options = paperpilot_pdf::operations::sanitize::SanitizeOptions {
        remove_metadata,
        remove_attachments,
        remove_scripts,
        remove_thumbnails: true,
        remove_private_data: true,
    };

    let (output_bytes, report) =
        paperpilot_pdf::operations::sanitize::sanitize_pdf(&doc_source, &options)
            .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("Sanitization complete:");
    println!("  Metadata stripped: {}", report.metadata_stripped);
    println!("  Attachments removed: {}", report.attachments_removed);
    println!("  Scripts neutralized: {}", report.scripts_removed);
    println!("  Thumbnails removed: {}", report.thumbnails_removed);
    println!("  Output size: {} bytes", report.output_bytes);

    Ok(())
}
