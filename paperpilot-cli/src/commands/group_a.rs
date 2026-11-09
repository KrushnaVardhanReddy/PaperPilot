#![cfg(feature = "pdf")]
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
    every: Option<u32>,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = if let Some(n) = every {
        let total_pages = doc.page_count()?;
        paperpilot_pdf::operations::split::SplitOperation::from_every(
            output.to_path_buf(),
            n,
            total_pages,
        )
    } else {
        let mut pts = Vec::new();
        for part in pages.split(',') {
            let part = part.trim();
            if let Some((_start, end)) = part.split_once('-') {
                if let Ok(end_num) = end.trim().parse::<u32>() {
                    if !pts.contains(&end_num) {
                        pts.push(end_num);
                    }
                }
            } else if let Ok(pt) = part.parse::<u32>() {
                if !pts.contains(&pt) {
                    pts.push(pt);
                }
            }
        }
        pts.sort_unstable();
        paperpilot_pdf::operations::split::SplitOperation {
            split_points: pts,
            output_dir: output.to_path_buf(),
        }
    };
    op.execute(&mut doc)
}

pub fn parse_page_indices(pages_str: &str, total_pages: Option<u32>) -> Vec<u32> {
    if pages_str.trim().eq_ignore_ascii_case("all") {
        if let Some(total) = total_pages {
            return (1..=total).collect();
        }
    }
    let mut result = Vec::new();
    for part in pages_str.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some((start_str, end_str)) = part.split_once('-') {
            if let (Ok(start), Ok(end)) = (start_str.trim().parse::<u32>(), end_str.trim().parse::<u32>()) {
                if start <= end {
                    for p in start..=end {
                        if !result.contains(&p) {
                            result.push(p);
                        }
                    }
                }
            }
        } else if let Ok(p) = part.parse::<u32>() {
            if !result.contains(&p) {
                result.push(p);
            }
        }
    }
    result
}

pub fn parse_page_order(order_str: &str) -> Vec<u32> {
    order_str
        .split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect()
}

pub fn handle_extract(
    input: &std::path::Path,
    pages: &str,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let total_pages = doc.page_count().ok();
    let op = paperpilot_pdf::operations::extract::ExtractPagesOperation {
        page_indices: parse_page_indices(pages, total_pages),
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
    let total_pages = doc.page_count().ok();
    let op = paperpilot_pdf::operations::delete::DeletePagesOperation {
        page_indices: parse_page_indices(pages, total_pages),
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
        new_order: parse_page_order(order),
    };
    op.execute(&mut doc)?;
    doc.save(output)
}

pub fn handle_rotate(
    input: &std::path::Path,
    pages: &str,
    degrees: i32,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let total_pages = doc.page_count().ok();

    let target_pages = if pages.trim().is_empty() || pages.trim().eq_ignore_ascii_case("all") {
        None
    } else {
        let parsed = parse_page_indices(pages, total_pages);
        if parsed.is_empty() {
            None
        } else {
            Some(parsed)
        }
    };

    let op = paperpilot_pdf::operations::rotate::RotatePagesOperation::new(
        degrees.try_into().unwrap_or(0),
        target_pages,
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
    sensitivity: u8,
    output: &std::path::Path,
) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::cleanup::RemoveBlankPagesOperation::new(sensitivity);
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
