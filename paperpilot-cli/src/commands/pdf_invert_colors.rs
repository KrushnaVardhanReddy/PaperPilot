#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::needless_borrow)]
use paperpilot_core::error::OperationResult;
use paperpilot_pdf::operations::pdf_invert_colors::{invert_pdf_colors, InvertColorsOptions, InvertMode};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_core::traits::PdfDocument;
use std::path::PathBuf;

pub fn handle_invert_colors(
    input: &PathBuf,
    output: &PathBuf,
    mode_str: &Option<String>,
    pages: &Option<String>,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;

    let mode = match mode_str.as_deref() {
        Some("true_invert") => Some(InvertMode::TrueInvert),
        Some("high_contrast") => Some(InvertMode::HighContrast),
        Some("sepia") => Some(InvertMode::Sepia),
        Some("dark_mode") => Some(InvertMode::DarkMode),
        Some(_) => Some(InvertMode::DarkMode), // default fallback
        None => Some(InvertMode::DarkMode),
    };

    let options = InvertColorsOptions {
        mode,
        pages: pages.clone(),
    };

    invert_pdf_colors(&mut doc.inner, &options)?;
    doc.save(output)?;
    Ok(())
}
