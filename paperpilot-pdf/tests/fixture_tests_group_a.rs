use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::{
    burst::BurstOperation,
    crop::{CropBox, CropPagesOperation},
    delete::DeletePagesOperation,
    extract::ExtractPagesOperation,
    merge::MergeOperation,
    reorder::ReorderPagesOperation,
    rotate::RotatePagesOperation,
    split::SplitOperation,
};
use std::path::PathBuf;
use tempfile::tempdir;

fn get_fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn test_merge_fixture() {
    let simple = get_fixture_path("simple.pdf");
    let multi = get_fixture_path("multi_page.pdf");
    let dir = tempdir().unwrap();
    let out_path = dir.path().join("out.pdf");

    let mut doc = LopdfDocument::load(&simple).unwrap();
    let op = MergeOperation::new(vec![multi]);
    op.execute(&mut doc).unwrap();
    doc.save(&out_path).unwrap();

    let result_doc = LopdfDocument::load(&out_path).unwrap();
    assert_eq!(result_doc.page_count().unwrap(), 4);
}

#[test]
fn test_split_fixture() {
    let multi = get_fixture_path("multi_page.pdf");
    let dir = tempdir().unwrap();

    let mut doc = LopdfDocument::load(&multi).unwrap();
    let op = SplitOperation::new(dir.path().to_path_buf(), vec![1, 2]);
    op.execute(&mut doc).unwrap();

    let split_1 = dir.path().join("split_1.pdf");
    let split_2 = dir.path().join("split_2.pdf");
    let split_3 = dir.path().join("split_3.pdf");

    assert!(split_1.exists());
    assert!(split_2.exists());
    assert!(split_3.exists());

    let doc1 = LopdfDocument::load(&split_1).unwrap();
    assert_eq!(doc1.page_count().unwrap(), 1);
}

#[test]
fn test_extract_fixture() {
    let multi = get_fixture_path("multi_page.pdf");
    let dir = tempdir().unwrap();
    let out_path = dir.path().join("out.pdf");

    let mut doc = LopdfDocument::load(&multi).unwrap();
    let op = ExtractPagesOperation::new(vec![1, 3]);
    op.execute(&mut doc).unwrap();
    doc.save(&out_path).unwrap();

    let result_doc = LopdfDocument::load(&out_path).unwrap();
    assert_eq!(result_doc.page_count().unwrap(), 2);
}

#[test]
fn test_delete_fixture() {
    let multi = get_fixture_path("multi_page.pdf");
    let dir = tempdir().unwrap();
    let out_path = dir.path().join("out.pdf");

    let mut doc = LopdfDocument::load(&multi).unwrap();
    let op = DeletePagesOperation::new(vec![2]);
    op.execute(&mut doc).unwrap();
    doc.save(&out_path).unwrap();

    let result_doc = LopdfDocument::load(&out_path).unwrap();
    assert_eq!(result_doc.page_count().unwrap(), 2);
}

#[test]
fn test_reorder_fixture() {
    let multi = get_fixture_path("multi_page.pdf");
    let dir = tempdir().unwrap();
    let out_path = dir.path().join("out.pdf");

    let mut doc = LopdfDocument::load(&multi).unwrap();
    let op = ReorderPagesOperation::new(vec![3, 1, 2]);
    op.execute(&mut doc).unwrap();
    doc.save(&out_path).unwrap();

    let result_doc = LopdfDocument::load(&out_path).unwrap();
    assert_eq!(result_doc.page_count().unwrap(), 3);
}

#[test]
fn test_rotate_fixture() {
    let simple = get_fixture_path("simple.pdf");
    let dir = tempdir().unwrap();
    let out_path = dir.path().join("out.pdf");

    let mut doc = LopdfDocument::load(&simple).unwrap();
    let op = RotatePagesOperation::new(90, None);
    op.execute(&mut doc).unwrap();
    doc.save(&out_path).unwrap();

    let result_doc = LopdfDocument::load(&out_path).unwrap();
    assert_eq!(result_doc.page_count().unwrap(), 1);

    let page_id = *result_doc.inner.get_pages().get(&1).unwrap();
    let dict = result_doc
        .inner
        .get_object(page_id)
        .unwrap()
        .as_dict()
        .unwrap();
    let rotate = dict.get(b"Rotate").unwrap().as_i64().unwrap();
    assert_eq!(rotate, 90);
}

#[test]
fn test_crop_fixture() {
    let simple = get_fixture_path("simple.pdf");
    let dir = tempdir().unwrap();
    let out_path = dir.path().join("out.pdf");

    let mut doc = LopdfDocument::load(&simple).unwrap();
    let op = CropPagesOperation::new(CropBox {
        left: 10.0,
        bottom: 20.0,
        right: 100.0,
        top: 200.0,
    });
    op.execute(&mut doc).unwrap();
    doc.save(&out_path).unwrap();

    let result_doc = LopdfDocument::load(&out_path).unwrap();
    let page_id = *result_doc.inner.get_pages().get(&1).unwrap();
    let dict = result_doc
        .inner
        .get_object(page_id)
        .unwrap()
        .as_dict()
        .unwrap();
    let crop_box = dict.get(b"CropBox").unwrap().as_array().unwrap();

    let get_val = |obj: &lopdf::Object| {
        if let lopdf::Object::Real(r) = obj {
            *r
        } else if let lopdf::Object::Integer(i) = obj {
            *i as f32
        } else {
            0.0
        }
    };

    assert_eq!(crop_box.len(), 4);
    assert_eq!(get_val(&crop_box[0]), 10.0);
    assert_eq!(get_val(&crop_box[1]), 20.0);
    assert_eq!(get_val(&crop_box[2]), 100.0);
    assert_eq!(get_val(&crop_box[3]), 200.0);
}

#[test]
fn test_burst_fixture() {
    let multi = get_fixture_path("multi_page.pdf");
    let dir = tempdir().unwrap();

    let mut doc = LopdfDocument::load(&multi).unwrap();
    let op = BurstOperation::new(dir.path().to_path_buf());
    op.execute(&mut doc).unwrap();

    let page_1 = dir.path().join("page_1.pdf");
    let page_2 = dir.path().join("page_2.pdf");
    let page_3 = dir.path().join("page_3.pdf");

    assert!(page_1.exists());
    assert!(page_2.exists());
    assert!(page_3.exists());

    let doc1 = LopdfDocument::load(&page_1).unwrap();
    assert_eq!(doc1.page_count().unwrap(), 1);

    let doc2 = LopdfDocument::load(&page_2).unwrap();
    assert_eq!(doc2.page_count().unwrap(), 1);

    let doc3 = LopdfDocument::load(&page_3).unwrap();
    assert_eq!(doc3.page_count().unwrap(), 1);
}
