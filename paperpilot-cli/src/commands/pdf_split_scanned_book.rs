use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_split_scanned_book_command(
    input: &PathBuf,
    output: &PathBuf,
    reading_direction: &str,
    gutter_overlap: f32,
    split_horizontal: bool,
) -> Result<(), PdfError> {
    let input_bytes = std::fs::read(input).map_err(PdfError::IoError)?;

    let options = paperpilot_pdf::operations::pdf_split_scanned_book::PdfSplitScannedBookOptions {
        reading_direction: Some(reading_direction.to_string()),
        gutter_overlap: Some(gutter_overlap),
        split_horizontal: Some(split_horizontal),
    };

    let output_bytes =
        paperpilot_pdf::operations::pdf_split_scanned_book::pdf_split_scanned_book_bytes(
            &input_bytes,
            &options,
        )
        .map_err(|e| PdfError::Other(e.to_string()))?;

    std::fs::write(output, output_bytes).map_err(PdfError::IoError)?;

    println!("PdfSplitScannedBook complete.");
    Ok(())
}
