#[macro_use]
extern crate lopdf;

use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::conversion::{PdfToJsonOperation, PdfToMarkdownOperation};
use paperpilot_pdf::operations::docast::LayoutExtractor;
use paperpilot_pdf::operations::extract_text::{ExtractTextOperation, stream_extract_text_batches};
use paperpilot_pdf::operations::info::InfoOperation;
use paperpilot_pdf::operations::render::RenderOperation;
use paperpilot_pdf::operations::search::SearchOperation;
use paperpilot_pdf::operations::table::{TableExtractOptions, extract_tables_source};
use paperpilot_pdf::RepairOperation;
use std::path::Path;
use std::sync::Arc;
use tempfile::tempdir;

fn make_multipage_pdf_bytes(page_count: u32) -> Vec<u8> {
    let mut doc = lopdf::Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    });

    let mut page_ids = Vec::with_capacity(page_count as usize);
    for idx in 1..=page_count {
        let content_str = format!(
            "BT /F1 14 Tf 50 720 Td (Page {:03} Mmap Parallel Marker) Tj ET",
            idx
        );
        let content_id = doc.add_object(lopdf::Stream::new(
            dictionary! {},
            content_str.into_bytes(),
        ));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        });
        page_ids.push(page_id.into());
    }

    doc.set_object(
        pages_id,
        dictionary! {
            "Type" => "Pages",
            "Kids" => page_ids,
            "Count" => page_count as i64,
        },
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    let mut bytes = Vec::new();
    doc.save_to(&mut bytes)
        .expect("Failed to serialize multi-page PDF");
    bytes
}

fn write_multipage_pdf(path: &Path, page_count: u32) -> usize {
    let bytes = make_multipage_pdf_bytes(page_count);
    let len = bytes.len();
    std::fs::write(path, bytes).expect("Failed writing PDF fixture");
    len
}

#[test]
fn test_from_file_mmap_and_from_mmap_zero_copy() {
    let dir = tempdir().unwrap();
    let pdf_path = dir.path().join("twelve_pages.pdf");
    let len = write_multipage_pdf(&pdf_path, 12);

    let doc = LopdfDocument::from_file_mmap(&pdf_path).expect("from_file_mmap should succeed");
    assert_eq!(doc.page_count().unwrap(), 12);
    assert_eq!(doc.source_len(), Some(len));
    assert!(doc.shared_source().is_some());
    assert!(
        doc.original_bytes.is_none(),
        "Zero-copy mmap should not allocate original_bytes heap vector"
    );

    #[cfg(not(target_arch = "wasm32"))]
    {
        assert!(doc.is_mmap());
        let file = std::fs::File::open(&pdf_path).unwrap();
        let mmap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        let direct = LopdfDocument::from_mmap(mmap).expect("from_mmap should succeed");
        assert!(direct.is_mmap());
        assert!(direct.original_bytes.is_none());
        assert_eq!(direct.page_count().unwrap(), 12);
    }
}

#[test]
fn test_load_with_threshold_switches_mmap_behavior() {
    let dir = tempdir().unwrap();
    let pdf_path = dir.path().join("threshold_test.pdf");
    write_multipage_pdf(&pdf_path, 4);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mmap_doc = LopdfDocument::load_with_threshold(&pdf_path, Some(1))
            .expect("Threshold=1 should succeed");
        assert!(mmap_doc.is_mmap());
        assert!(mmap_doc.original_bytes.is_none());

        let heap_doc = LopdfDocument::load_with_threshold(&pdf_path, Some(1024 * 1024))
            .expect("High threshold should load into DocumentSource::Bytes");
        assert!(!heap_doc.is_mmap());
        assert!(heap_doc.source_bytes().is_some());
        assert_eq!(heap_doc.page_count().unwrap(), 4);
    }
}

#[test]
fn test_from_file_mmap_error_handling_no_panic() {
    let dir = tempdir().unwrap();

    // Missing file
    let missing = dir.path().join("does_not_exist.pdf");
    let res_missing = LopdfDocument::from_file_mmap(&missing);
    assert!(matches!(res_missing, Err(PdfError::IoError(_))));

    // Empty 0-byte file
    let empty_path = dir.path().join("empty.pdf");
    std::fs::write(&empty_path, b"").unwrap();
    let res_empty = LopdfDocument::from_file_mmap(&empty_path);
    assert!(matches!(res_empty, Err(PdfError::ParseError(_))));

    // Invalid non-PDF binary bytes
    let invalid_path = dir.path().join("invalid.pdf");
    std::fs::write(&invalid_path, b"Not a PDF! Random binary garbage \x00\xff\xfe\xfa").unwrap();
    let res_invalid = LopdfDocument::from_file_mmap(&invalid_path);
    assert!(res_invalid.is_err());

    // repair_file_mmap error paths also fail cleanly without panic
    let repaired_out = dir.path().join("repaired_out.pdf");
    assert!(RepairOperation::repair_file_mmap(&empty_path, &repaired_out).is_err());
    assert!(RepairOperation::repair_file_mmap(&invalid_path, &repaired_out).is_err());
}

#[test]
fn test_from_file_mmap_resilient_loading_and_repair() {
    let dir = tempdir().unwrap();
    let raw = make_multipage_pdf_bytes(3);

    // Leading garbage before %PDF header
    let mut prefixed = b"HTTP/1.1 200 OK\r\nContent-Type: application/pdf\r\n\r\n".to_vec();
    prefixed.extend_from_slice(&raw);
    let prefixed_path = dir.path().join("prefixed.pdf");
    std::fs::write(&prefixed_path, &prefixed).unwrap();

    let doc = LopdfDocument::from_file_mmap(&prefixed_path)
        .expect("from_file_mmap should self-heal prefix noise");
    assert_eq!(doc.page_count().unwrap(), 3);

    // Damaged xref table + missing %%EOF
    let xref_pos = raw.windows(4).position(|w| w == b"xref").unwrap();
    let mut damaged = raw[..xref_pos].to_vec();
    damaged.extend_from_slice(b"trailer << /Corrupted true >>\nstartxref\n999999\n");
    let damaged_path = dir.path().join("damaged.pdf");
    std::fs::write(&damaged_path, &damaged).unwrap();

    let healed = LopdfDocument::from_file_mmap(&damaged_path)
        .expect("from_file_mmap should rescue damaged xref via resilient loader");
    assert_eq!(healed.page_count().unwrap(), 3);

    let repaired_path = dir.path().join("repaired.pdf");
    RepairOperation::repair_file_mmap(&damaged_path, &repaired_path)
        .expect("repair_file_mmap should emit a valid repaired PDF");
    let standard = lopdf::Document::load(&repaired_path)
        .expect("Standard lopdf should open file produced by repair_file_mmap");
    assert_eq!(standard.get_pages().len(), 3);
}

#[test]
fn test_mmap_parallel_operations_preserve_order_and_zero_panic() {
    let dir = tempdir().unwrap();
    let pdf_path = dir.path().join("parallel_ops.pdf");
    write_multipage_pdf(&pdf_path, 12);

    let mut doc = LopdfDocument::from_file_mmap(&pdf_path).unwrap();

    // 1. Parallel plain-text extraction via lopdf + rayon
    let par_pages = doc.extract_text_parallel(None).unwrap();
    assert_eq!(par_pages.len(), 12);
    assert!(par_pages[0].contains("Page 001"));
    assert!(par_pages[11].contains("Page 012"));

    let subset = doc.extract_text_parallel(Some(&[1, 6, 12])).unwrap();
    assert_eq!(subset.len(), 3);
    assert!(subset[0].contains("Page 001"));
    assert!(subset[1].contains("Page 006"));
    assert!(subset[2].contains("Page 012"));

    // 2. Bounded-batch streaming extraction via PageBatchIterator + rayon
    let batched = stream_extract_text_batches(&doc, 5).unwrap();
    assert_eq!(batched, par_pages);

    // 3. ExtractTextOperation on mmap doc
    let extract_op = ExtractTextOperation::new(None);
    extract_op.execute(&mut doc).unwrap();
    let op_text = extract_op.extracted_text.lock().unwrap().clone().unwrap();
    assert_eq!(op_text.len(), 12);
    assert!(op_text[0].contains("Page 001"));
    assert!(op_text[11].contains("Page 012"));

    // 4. Parallel layout AST & Markdown via docast
    let ast_pages = LayoutExtractor::parse_pages_parallel(&par_pages);
    assert_eq!(ast_pages.len(), 12);
    assert!(ast_pages[0].to_markdown().contains("Page 001"));
    assert!(ast_pages[11].to_markdown().contains("Page 012"));
    let md_pages = LayoutExtractor::render_pages_markdown_parallel(&par_pages);
    assert_eq!(md_pages.len(), 12);
    assert!(md_pages[0].contains("Page 001"));
    assert!(md_pages[11].contains("Page 012"));

    // 5. Zero-copy parallel RenderOperation directly from shared mmap source
    let render_op = RenderOperation::new();
    render_op.execute(&mut doc).unwrap();
    let images = render_op.rendered_images.lock().unwrap();
    assert_eq!(images.len(), 12);
    for png in images.iter() {
        assert_eq!(&png[0..4], &[0x89, 0x50, 0x4E, 0x47]);
    }
    drop(images);

    // 6. Parallel batched SearchOperation
    let search_op = SearchOperation::new("Page 011".to_string());
    search_op.execute_batched(&mut doc, 4).unwrap();
    assert_eq!(*search_op.match_pages.lock().unwrap(), vec![11]);

    // 7. Info, Markdown, JSON, and zero-copy table extraction on mmap doc
    let info_op = InfoOperation::new();
    info_op.execute(&mut doc).unwrap();
    assert_eq!(info_op.result.lock().unwrap().as_ref().unwrap().page_count, 12);

    let md_op = PdfToMarkdownOperation::new();
    md_op.execute(&mut doc).unwrap();
    let md_out = md_op.extracted_markdown.lock().unwrap().clone().unwrap();
    assert!(md_out.contains("Page 001") && md_out.contains("Page 012"));

    let json_op = PdfToJsonOperation::new("parallel_ops.pdf".to_string());
    json_op.execute(&mut doc).unwrap();
    let json_out = json_op.extracted_json.lock().unwrap().clone().unwrap();
    assert!(json_out.contains("Page 001") && json_out.contains("Page 012"));

    let source_arc = Arc::clone(doc.shared_source().unwrap());
    let table_res = extract_tables_source(source_arc, &TableExtractOptions::default()).unwrap();
    assert_eq!(table_res.total_tables, 0);
}

#[test]
fn test_high_volume_150_page_mmap_streaming() {
    let dir = tempdir().unwrap();
    let pdf_path = dir.path().join("high_volume_150.pdf");
    write_multipage_pdf(&pdf_path, 150);

    let mut doc = LopdfDocument::from_file_mmap(&pdf_path).unwrap();
    assert_eq!(doc.page_count().unwrap(), 150);

    let streamed = stream_extract_text_batches(&doc, 25).unwrap();
    assert_eq!(streamed.len(), 150);
    assert!(streamed[0].contains("Page 001"));
    assert!(streamed[74].contains("Page 075"));
    assert!(streamed[149].contains("Page 150"));

    let search_op = SearchOperation::new("Page 149".to_string());
    search_op.execute(&mut doc).unwrap();
    assert_eq!(*search_op.match_pages.lock().unwrap(), vec![149]);
}
