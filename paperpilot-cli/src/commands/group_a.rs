use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use std::path::PathBuf;

pub fn handle_merge(input: &[PathBuf], output: &std::path::Path) -> OperationResult<()> {
    if input.is_empty() {
        return Err(PdfError::InvalidInput(
            "No input files provided".to_string(),
        ));
    }

    let mut doc = LopdfDocument::load(&input[0])?;
    let op = paperpilot_pdf::operations::merge::MergeOperation {
        paths: input[1..].iter().map(|p| p.to_path_buf()).collect(),
    };
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_split(
    input: &std::path::Path,
    pages: &str,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::split::SplitOperation {
        split_points: pages.split(',').filter_map(|s| s.parse().ok()).collect(),
        output_dir: output.to_path_buf(),
    };
    op.execute(&mut doc)
}

pub fn handle_extract(
    input: &std::path::Path,
    pages: &str,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::extract::ExtractPagesOperation {
        page_indices: pages.split(',').filter_map(|s| s.parse().ok()).collect(),
    };
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_delete(
    input: &std::path::Path,
    pages: &str,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::delete::DeletePagesOperation {
        page_indices: pages.split(',').filter_map(|s| s.parse().ok()).collect(),
    };
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_reorder(
    input: &std::path::Path,
    order: &str,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::reorder::ReorderPagesOperation {
        new_order: order.split(',').filter_map(|s| s.parse().ok()).collect(),
    };
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_rotate(
    input: &std::path::Path,
    _pages: &str,
    degrees: i32,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;

    let op = paperpilot_pdf::operations::rotate::RotatePagesOperation::new(
        degrees.try_into().unwrap_or(0),
        None,
    );
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_crop(
    input: &std::path::Path,
    _pages: &str,
    rect: &str,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;

    let parts: Vec<&str> = rect.split(',').collect();
    if parts.len() != 4 {
        return Err(PdfError::InvalidInput(
            "Rect must be 4 comma-separated values".to_string(),
        ));
    }

    let left = parts[0]
        .parse()
        .map_err(|_| PdfError::InvalidInput("Invalid left".to_string()))?;
    let bottom = parts[1]
        .parse()
        .map_err(|_| PdfError::InvalidInput("Invalid bottom".to_string()))?;
    let right = parts[2]
        .parse()
        .map_err(|_| PdfError::InvalidInput("Invalid right".to_string()))?;
    let top = parts[3]
        .parse()
        .map_err(|_| PdfError::InvalidInput("Invalid top".to_string()))?;

    let op = paperpilot_pdf::operations::crop::CropPagesOperation::new(
        paperpilot_pdf::operations::crop::CropBox {
            left,
            bottom,
            right,
            top,
        },
    );
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_burst(input: &std::path::Path, output: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    if !output.exists() {
        std::fs::create_dir_all(output).map_err(PdfError::IoError)?;
    }
    let op = paperpilot_pdf::operations::burst::BurstOperation {
        output_dir: output.to_path_buf(),
    };
    op.execute(&mut doc)
}

pub fn handle_remove_blank(
    input: &std::path::Path,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::cleanup::RemoveBlankPagesOperation::new(90);
    op.execute(&mut doc)?;
    doc.save(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_handle_burst_creates_dir() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("input.pdf");
        let output_dir = dir.path().join("burst_out");

        let fixture_path = std::path::Path::new("../tests/e2e_fixtures/multi_page.pdf");
        let fixture_path2 = std::path::Path::new("tests/e2e_fixtures/multi_page.pdf");
        let actual_fixture = if fixture_path.exists() {
            fixture_path
        } else if fixture_path2.exists() {
            fixture_path2
        } else {
            return;
        };

        fs::copy(actual_fixture, &input_path).unwrap();

        assert!(!output_dir.exists());
        let res = handle_burst(&input_path, &output_dir);
        assert!(res.is_ok());
        assert!(output_dir.exists());
        assert!(output_dir.join("page_1.pdf").exists());
    }
}
