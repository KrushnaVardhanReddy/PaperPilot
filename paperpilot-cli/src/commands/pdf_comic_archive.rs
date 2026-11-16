use paperpilot_core::error::PdfError;
use std::path::PathBuf;

pub fn handle_pdf_comic_archive_command(input: &PathBuf, output: &PathBuf) -> Result<(), PdfError> {
    let options = paperpilot_pdf::operations::pdf_comic_archive::PdfComicArchiveOptions::default();

    paperpilot_pdf::operations::pdf_comic_archive::pdf_comic_archive_path(
        input,
        output,
        &options,
    )?;

    println!("PdfComicArchive complete.");
    Ok(())
}
