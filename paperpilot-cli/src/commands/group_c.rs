use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use std::path::PathBuf;

pub fn handle_extract_text(input: &std::path::Path, _output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::extract_text::ExtractTextOperation::new(None);
    op.execute(&mut doc)?;
    Ok(())
}

pub fn handle_extract_images(input: &std::path::Path, output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::extract_images::ExtractImagesOperation::new(output.to_path_buf());
    op.execute(&mut doc)
}

pub fn handle_search(input: &std::path::Path, query: &str) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::search::SearchOperation::new(query.to_string());
    op.execute(&mut doc)
}

pub fn handle_render(input: &std::path::Path, _output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::render::RenderOperation;
    op.execute(&mut doc)
}

pub fn handle_compare(input: &std::path::Path, _input_b: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::compare::CompareOperation;
    op.execute(&mut doc)
}

pub fn handle_ocr(input: &std::path::Path, _output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::ocr::OcrOperation;
    op.execute(&mut doc)
}

pub fn handle_bookmarks(input: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::bookmarks::BookmarksOperation::new();
    op.execute(&mut doc)
}

pub fn handle_images_to_pdf(images: &[PathBuf], output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::new();
    let op = paperpilot_pdf::operations::images_to_pdf::ImagesToPdfOperation::new(images.iter().map(|p| p.to_path_buf()).collect());
    op.execute(&mut doc)?;
    doc.save(output)
}
