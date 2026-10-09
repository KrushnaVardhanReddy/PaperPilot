use criterion::{Criterion, criterion_group, criterion_main};
use lopdf::Document;
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::compress::CompressOperation;
use paperpilot_pdf::operations::extract_text::ExtractTextOperation;
use paperpilot_pdf::operations::merge::MergeOperation;
use paperpilot_pdf::operations::watermark::WatermarkOperation;
use std::path::PathBuf;
use std::time::Instant;

fn get_workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap().to_path_buf()
}

fn create_pdf_with_pages(path: &std::path::Path, num_pages: usize) {
    if path.exists() {
        return;
    }
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    use lopdf::content::{Content, Operation};
    use lopdf::{Document, Object, Stream, dictionary};

    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });

    let mut page_ids = Vec::new();
    for i in 1..=num_pages {
        let text = format!("Bench Document Page {}", i);
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 14.into()]),
                Operation::new("Td", vec![50.into(), 700.into()]),
                Operation::new("Tj", vec![Object::string_literal(text)]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_bytes = content.encode().unwrap();
        let content_id = doc.add_object(Stream::new(dictionary! {}, content_bytes));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Contents" => content_id,
            "Resources" => dictionary! {
                "Font" => dictionary! {
                    "F1" => font_id,
                }
            }
        });
        page_ids.push(page_id.into());
    }

    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => page_ids,
            "Count" => num_pages as i64,
        }),
    );

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    let _ = doc.save(path);
}

fn ensure_bench_fixtures(workspace: &std::path::Path) {
    let bench_dir = workspace.join("tests").join("bench_fixtures");
    let batch_dir = bench_dir.join("batch");
    let _ = std::fs::create_dir_all(&batch_dir);

    create_pdf_with_pages(&bench_dir.join("tiny_1page.pdf"), 1);
    create_pdf_with_pages(&bench_dir.join("single_1page.pdf"), 1);
    create_pdf_with_pages(&bench_dir.join("small_10page.pdf"), 10);
    create_pdf_with_pages(&bench_dir.join("medium_50page.pdf"), 50);
    create_pdf_with_pages(&bench_dir.join("large_100page.pdf"), 100);
    create_pdf_with_pages(&bench_dir.join("xlarge_500page.pdf"), 500);

    for i in 1..=20 {
        create_pdf_with_pages(&batch_dir.join(format!("doc_{:03}.pdf", i)), 1);
    }
}

fn bench_concurrent(_c: &mut Criterion) {
    let workspace = get_workspace_root();
    ensure_bench_fixtures(&workspace);

    let large_pdf_path = workspace
        .join("tests")
        .join("bench_fixtures")
        .join("large_100page.pdf");
    let medium_pdf_path = workspace
        .join("tests")
        .join("bench_fixtures")
        .join("medium_50page.pdf");
    let small_pdf_path = workspace
        .join("tests")
        .join("bench_fixtures")
        .join("small_10page.pdf");
    let batch_dir = workspace.join("tests").join("bench_fixtures").join("batch");

    let mut batch_paths = Vec::new();
    for i in 1..=20 {
        batch_paths.push(batch_dir.join(format!("doc_{:03}.pdf", i)));
    }

    // 1. Measure sequential execution time
    let seq_start = Instant::now();

    // A: Merge
    {
        let mut main_doc = LopdfDocument::new();
        let merge_op = MergeOperation::new(batch_paths.clone());
        merge_op.execute(&mut main_doc).unwrap();
    }

    // B: Extract Text
    {
        let mut doc = LopdfDocument {
            inner: Document::load(&large_pdf_path).unwrap(),
        };
        let extract_op = ExtractTextOperation::new(None);
        extract_op.execute(&mut doc).unwrap();
    }

    // C: Compress
    {
        let mut doc = LopdfDocument {
            inner: Document::load(&medium_pdf_path).unwrap(),
        };
        let compress_op = CompressOperation::default();
        compress_op.execute(&mut doc).unwrap();
    }

    // D: Watermark
    {
        let mut doc = LopdfDocument {
            inner: Document::load(&small_pdf_path).unwrap(),
        };
        let watermark_op = WatermarkOperation::new("CONFIDENTIAL".to_string());
        watermark_op.execute(&mut doc).unwrap();
    }
    let seq_time = seq_start.elapsed().as_micros();

    // 2. Measure concurrent execution time
    let conc_start = Instant::now();

    let t1 = std::thread::spawn({
        let batch_paths = batch_paths.clone();
        move || {
            let mut main_doc = LopdfDocument::new();
            let merge_op = MergeOperation::new(batch_paths);
            merge_op.execute(&mut main_doc).unwrap();
        }
    });

    let t2 = std::thread::spawn({
        let large_pdf_path = large_pdf_path.clone();
        move || {
            let mut doc = LopdfDocument {
                inner: Document::load(&large_pdf_path).unwrap(),
            };
            let extract_op = ExtractTextOperation::new(None);
            extract_op.execute(&mut doc).unwrap();
        }
    });

    let t3 = std::thread::spawn({
        let medium_pdf_path = medium_pdf_path.clone();
        move || {
            let mut doc = LopdfDocument {
                inner: Document::load(&medium_pdf_path).unwrap(),
            };
            let compress_op = CompressOperation::default();
            compress_op.execute(&mut doc).unwrap();
        }
    });

    let t4 = std::thread::spawn({
        let small_pdf_path = small_pdf_path.clone();
        move || {
            let mut doc = LopdfDocument {
                inner: Document::load(&small_pdf_path).unwrap(),
            };
            let watermark_op = WatermarkOperation::new("CONFIDENTIAL".to_string());
            watermark_op.execute(&mut doc).unwrap();
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();

    let conc_time = conc_start.elapsed().as_micros();

    println!("SYSTEM_BENCHMARK_CONCURRENT|Sequential|{}", seq_time);
    println!("SYSTEM_BENCHMARK_CONCURRENT|Concurrent|{}", conc_time);
}

criterion_group!(benches, bench_concurrent);
criterion_main!(benches);
