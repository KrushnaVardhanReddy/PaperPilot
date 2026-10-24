use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_comic_archive_command(input: &PathBuf, output: &PathBuf) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let options = paperpilot_pdf::operations::pdf_comic_archive::PdfComicArchiveOptions::default();

    let output_bytes = paperpilot_pdf::operations::pdf_comic_archive::pdf_comic_archive_bytes(
        &input_bytes,
        &options,
    )
    .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfComicArchive complete.");
    Ok(())
}
