use assert_cmd::Command;
use lopdf::dictionary;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn make_pdf_bytes(page_count: u32, payload_comment_bytes: usize) -> Vec<u8> {
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

    let mut kids = Vec::with_capacity(page_count as usize);
    for page in 1..=page_count {
        let text = format!("BT /F1 14 Tf 40 720 Td (CLI Mmap Page {} Keyword) Tj ET", page);
        let content_id = doc.add_object(lopdf::Stream::new(dictionary! {}, text.into_bytes()));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        });
        kids.push(page_id.into());
    }

    doc.set_object(
        pages_id,
        dictionary! {
            "Type" => "Pages",
            "Kids" => kids,
            "Count" => page_count as i64,
        },
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    if payload_comment_bytes > 0 {
        let pad_stream = lopdf::Stream::new(dictionary! {}, vec![b'A'; payload_comment_bytes]);
        doc.add_object(pad_stream);
    }

    let mut out = Vec::new();
    doc.save_to(&mut out).unwrap();
    out
}

fn write_pdf(path: &Path, page_count: u32, padding: usize) {
    let bytes = make_pdf_bytes(page_count, padding);
    fs::write(path, bytes).unwrap();
}

#[test]
fn test_cli_mmap_flag_info_extract_convert_repair() {
    let dir = tempdir().unwrap();
    let input_pdf = dir.path().join("cli_mmap_input.pdf");
    write_pdf(&input_pdf, 5, 0);

    // 1. info --mmap writes JSON confirming mmap: true on native
    let info_out = dir.path().join("info_mmap.json");
    Command::cargo_bin("paperpilot-cli")
        .unwrap()
        .args([
            "--mmap",
            "info",
            "--input",
            input_pdf.to_str().unwrap(),
            "--output",
            info_out.to_str().unwrap(),
        ])
        .assert()
        .success();

    let info_json: Value =
        serde_json::from_str(&fs::read_to_string(&info_out).unwrap()).unwrap();
    assert_eq!(info_json["page_count"], 5);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(info_json["mmap"], true);

    // 2. Without --mmap on a small file (<10MB), mmap should remain false
    let info_small_out = dir.path().join("info_small.json");
    Command::cargo_bin("paperpilot-cli")
        .unwrap()
        .args([
            "info",
            "--input",
            input_pdf.to_str().unwrap(),
            "--output",
            info_small_out.to_str().unwrap(),
        ])
        .assert()
        .success();
    let info_small_json: Value =
        serde_json::from_str(&fs::read_to_string(&info_small_out).unwrap()).unwrap();
    assert_eq!(info_small_json["mmap"], false);

    // 3. extract-text --mmap
    let text_out = dir.path().join("extracted.txt");
    Command::cargo_bin("paperpilot-cli")
        .unwrap()
        .args([
            "extract-text",
            "--input",
            input_pdf.to_str().unwrap(),
            "--output",
            text_out.to_str().unwrap(),
            "--mmap",
        ])
        .assert()
        .success();
    let text_content = fs::read_to_string(&text_out).unwrap();
    assert!(text_content.contains("CLI Mmap Page 1 Keyword"));
    assert!(text_content.contains("CLI Mmap Page 5 Keyword"));

    // 4. convert --format md --mmap
    let md_out = dir.path().join("converted.md");
    Command::cargo_bin("paperpilot-cli")
        .unwrap()
        .args([
            "--mmap",
            "convert",
            "--format",
            "md",
            "--input",
            input_pdf.to_str().unwrap(),
            "--output",
            md_out.to_str().unwrap(),
        ])
        .assert()
        .success();
    let md_content = fs::read_to_string(&md_out).unwrap();
    assert!(md_content.contains("CLI Mmap Page 3 Keyword"));

    // 5. repair --mmap on prefix-corrupted PDF
    let raw = fs::read(&input_pdf).unwrap();
    let mut prefixed = b"JUNK_PREFIX_BYTES\r\n\r\n".to_vec();
    prefixed.extend_from_slice(&raw);
    let broken_pdf = dir.path().join("broken_prefix.pdf");
    let repaired_pdf = dir.path().join("repaired.pdf");
    fs::write(&broken_pdf, prefixed).unwrap();

    Command::cargo_bin("paperpilot-cli")
        .unwrap()
        .args([
            "--mmap",
            "repair",
            "--input",
            broken_pdf.to_str().unwrap(),
            "--output",
            repaired_pdf.to_str().unwrap(),
        ])
        .assert()
        .success();
    let reopened = lopdf::Document::load(&repaired_pdf).unwrap();
    assert_eq!(reopened.get_pages().len(), 5);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn test_cli_auto_detects_mmap_over_10mb_threshold() {
    let dir = tempdir().unwrap();
    let large_pdf = dir.path().join("over_10mb.pdf");
    // 10.5 MB payload so DEFAULT_MMAP_THRESHOLD_BYTES (10 MB) triggers automatically without --mmap
    write_pdf(&large_pdf, 2, 10_600_000);
    assert!(fs::metadata(&large_pdf).unwrap().len() >= 10 * 1024 * 1024);

    let info_out = dir.path().join("large_info.json");
    Command::cargo_bin("paperpilot-cli")
        .unwrap()
        .args([
            "info",
            "--input",
            large_pdf.to_str().unwrap(),
            "--output",
            info_out.to_str().unwrap(),
        ])
        .assert()
        .success();

    let info_json: Value =
        serde_json::from_str(&fs::read_to_string(&info_out).unwrap()).unwrap();
    assert_eq!(info_json["page_count"], 2);
    assert_eq!(
        info_json["mmap"], true,
        "Files >= 10MB should auto-select memmap2 without --mmap"
    );
}
