use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use std::path::PathBuf;

pub fn handle_extract_text(
    input: &std::path::Path,
    output: &std::path::Path,
    format: &str,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::extract_text::ExtractTextOperation::new(None);
    op.execute(&mut doc)?;

    let text_lock = op.extracted_text.lock().unwrap();
    let text_pages = text_lock.as_ref().unwrap();

    let result_str = if format.eq_ignore_ascii_case("json") {
        serde_json::to_string_pretty(text_pages).unwrap()
    } else {
        text_pages.join("\n")
    };

    std::fs::write(output, result_str).map_err(paperpilot_core::error::PdfError::IoError)?;

    Ok(())
}

pub fn handle_extract_images(
    input: &std::path::Path,
    output: &std::path::Path,
) -> OperationResult<()> {
    std::fs::create_dir_all(output).map_err(paperpilot_core::error::PdfError::IoError)?;
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::extract_images::ExtractImagesOperation::new(
        output.to_path_buf(),
    );
    op.execute(&mut doc)
}

pub fn handle_search(input: &std::path::Path, query: &str) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::search::SearchOperation::new(query.to_string());
    op.execute(&mut doc)
}

pub fn handle_render(input: &std::path::Path, output: &std::path::Path) -> OperationResult<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).map_err(paperpilot_core::error::PdfError::IoError)?;
    }
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::render::RenderOperation::new();
    op.execute(&mut doc)
}

pub fn handle_compare(input: &std::path::Path, input_b: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let mut op = paperpilot_pdf::operations::compare::CompareOperation::new();
    op.input_b = Some(input_b.to_string_lossy().to_string());
    op.execute(&mut doc)
}

pub fn handle_ocr(input: &std::path::Path, _output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::ocr::OcrOperation::new();
    op.execute(&mut doc)
}

pub fn handle_bookmarks(input: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::bookmarks::BookmarksOperation::new();
    op.execute(&mut doc)
}

pub fn handle_images_to_pdf(images: &[PathBuf], output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::new();
    let op = paperpilot_pdf::operations::images_to_pdf::ImagesToPdfOperation::new(
        images.iter().map(|p| p.to_path_buf()).collect(),
    );
    op.execute(&mut doc)?;
    doc.save(output)
}
