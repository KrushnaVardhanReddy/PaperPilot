use paperpilot_pdf::operations::delete::DeletePagesOperation;
use paperpilot_pdf::operations::extract::ExtractPagesOperation;
use paperpilot_pdf::operations::reorder::ReorderPagesOperation;
use paperpilot_pdf::operations::rotate::RotatePagesOperation;
use paperpilot_pdf::operations::split::SplitOperation;
use proptest::prelude::*;

proptest! {
    /// Splitting a 3-page PDF at page N should always produce two valid outputs or an error, never panic.
    #[test]
    fn test_split_never_panics(page_num in 1u32..5u32) {
        use paperpilot_pdf::document::LopdfDocument;
        use paperpilot_core::traits::PdfOperation;
        let mut path = std::env::current_dir().unwrap();
        if path.file_name().unwrap() == "paperpilot-pdf" {
            path.pop();
        }
        path.push("tests/fixtures/multi_page.pdf");

        if !path.exists() { return Ok(()); }
        let mut doc = LopdfDocument::load(&path).unwrap();
        let output_dir = std::env::temp_dir();
        let op = SplitOperation::new(output_dir, vec![page_num]);
        let _ = op.execute(&mut doc);
    }

    /// Extracting pages from a 3-page PDF with any range should not panic.
    #[test]
    fn test_extract_page_range_invariant(start in 1u32..4u32, end in 1u32..5u32) {
        use paperpilot_pdf::document::LopdfDocument;
        use paperpilot_core::traits::PdfOperation;
        let mut path = std::env::current_dir().unwrap();
        if path.file_name().unwrap() == "paperpilot-pdf" {
            path.pop();
        }
        path.push("tests/fixtures/multi_page.pdf");

        if !path.exists() { return Ok(()); }
        let mut doc = LopdfDocument::load(&path).unwrap();
        // Generate a range based on start and end
        let start = std::cmp::min(start, end);
        let end = std::cmp::max(start, end);
        let pages: Vec<u32> = (start..=end).collect();

        let op = ExtractPagesOperation::new(pages);
        let _ = op.execute(&mut doc);
    }

    /// Rotating any page in a 3-page PDF by valid and invalid angles should not panic.
    #[test]
    fn test_rotate_never_panics(page_num in 1u32..5u32, angle in proptest::sample::select(vec![0, 90, 180, 270, 360, 45, 450])) {
        use paperpilot_pdf::document::LopdfDocument;
        use paperpilot_core::traits::PdfOperation;
        let mut path = std::env::current_dir().unwrap();
        if path.file_name().unwrap() == "paperpilot-pdf" {
            path.pop();
        }
        path.push("tests/fixtures/multi_page.pdf");

        if !path.exists() { return Ok(()); }
        let mut doc = LopdfDocument::load(&path).unwrap();
        let op = RotatePagesOperation::new(angle as u16, Some(vec![page_num]));
        let _ = op.execute(&mut doc);
    }

    /// Deleting any page from a 3-page PDF should not panic.
    #[test]
    fn test_delete_never_panics(page_num in 1u32..5u32) {
        use paperpilot_pdf::document::LopdfDocument;
        use paperpilot_core::traits::PdfOperation;
        let mut path = std::env::current_dir().unwrap();
        if path.file_name().unwrap() == "paperpilot-pdf" {
            path.pop();
        }
        path.push("tests/fixtures/multi_page.pdf");

        if !path.exists() { return Ok(()); }
        let mut doc = LopdfDocument::load(&path).unwrap();
        let op = DeletePagesOperation::new(vec![page_num]);
        let _ = op.execute(&mut doc);
    }

    /// Reordering pages of a 3-page PDF to any order (including duplicates or missing pages) should not panic.
    #[test]
    fn test_reorder_never_panics(order in proptest::collection::vec(1u32..5u32, 0..5)) {
        use paperpilot_pdf::document::LopdfDocument;
        use paperpilot_core::traits::PdfOperation;
        let mut path = std::env::current_dir().unwrap();
        if path.file_name().unwrap() == "paperpilot-pdf" {
            path.pop();
        }
        path.push("tests/fixtures/multi_page.pdf");

        if !path.exists() { return Ok(()); }
        let mut doc = LopdfDocument::load(&path).unwrap();
        let op = ReorderPagesOperation::new(order);
        let _ = op.execute(&mut doc);
    }
}
