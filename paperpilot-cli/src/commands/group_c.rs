use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::{
    bookmarks::BookmarksOperation, compare::CompareOperation,
    extract_images::ExtractImagesOperation, extract_text::ExtractTextOperation,
    images_to_pdf::ImagesToPdfOperation, ocr::OcrOperation, render::RenderOperation,
    search::SearchOperation,
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub fn handle_extract_text(input: PathBuf, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let result = Arc::new(Mutex::new(Option::<String>::None));
    let op = ExtractTextOperation {
        pages: None,
        extracted_text: result.clone(),
    };
    op.execute(&mut doc)?;
    let text = result.lock().unwrap().clone().unwrap_or_default();
    std::fs::write(&output, text).map_err(paperpilot_core::error::PdfError::IoError)?;
    Ok(())
}

pub fn handle_extract_images(input: PathBuf, output_dir: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = ExtractImagesOperation { output_dir };
    op.execute(&mut doc)
}

pub fn handle_images_to_pdf(images: Vec<PathBuf>, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::new();
    let op = ImagesToPdfOperation {
        image_paths: images,
    };
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_search(input: PathBuf, query: String) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let results = Arc::new(Mutex::new(Vec::new()));
    let op = SearchOperation {
        search_query: query,
        match_pages: results.clone(),
    };
    op.execute(&mut doc)?;
    let res = results.lock().unwrap();
    if res.is_empty() {
        println!("No results found.");
    } else {
        for m in res.iter() {
            println!("Found on page {}", m);
        }
    }
    Ok(())
}

pub fn handle_compare(input1: PathBuf, _input2: PathBuf) -> OperationResult<()> {
    let mut doc1 = LopdfDocument::load(&input1)?;
    let op = CompareOperation;
    op.execute(&mut doc1)
}

pub fn handle_render(input: PathBuf, _output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = RenderOperation;
    op.execute(&mut doc)
}

pub fn handle_ocr(input: PathBuf, output: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = OcrOperation;
    op.execute(&mut doc)?;
    doc.save(&output)
}

pub fn handle_bookmarks(input: PathBuf) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(&input)?;
    let op = BookmarksOperation {};
    op.execute(&mut doc)
}
