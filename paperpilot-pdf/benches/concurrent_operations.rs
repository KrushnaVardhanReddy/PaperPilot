use criterion::{criterion_group, criterion_main, Criterion};
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::merge::MergeOperation;
use paperpilot_pdf::operations::extract_text::ExtractTextOperation;
use paperpilot_pdf::operations::compress::CompressOperation;
use paperpilot_pdf::operations::watermark::WatermarkOperation;
use std::path::PathBuf;
use lopdf::Document;
use std::time::Instant;

fn get_workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap().to_path_buf()
}

fn bench_concurrent(_c: &mut Criterion) {
    let workspace = get_workspace_root();

    let large_pdf_path = workspace.join("tests").join("bench_fixtures").join("large_100page.pdf");
    let medium_pdf_path = workspace.join("tests").join("bench_fixtures").join("medium_50page.pdf");
    let small_pdf_path = workspace.join("tests").join("bench_fixtures").join("small_10page.pdf");
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
        let mut doc = LopdfDocument { inner: Document::load(&large_pdf_path).unwrap() };
        let extract_op = ExtractTextOperation::new(None);
        extract_op.execute(&mut doc).unwrap();
    }

    // C: Compress
    {
        let mut doc = LopdfDocument { inner: Document::load(&medium_pdf_path).unwrap() };
        let compress_op = CompressOperation::new();
        compress_op.execute(&mut doc).unwrap();
    }

    // D: Watermark
    {
        let mut doc = LopdfDocument { inner: Document::load(&small_pdf_path).unwrap() };
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
            let mut doc = LopdfDocument { inner: Document::load(&large_pdf_path).unwrap() };
            let extract_op = ExtractTextOperation::new(None);
            extract_op.execute(&mut doc).unwrap();
        }
    });

    let t3 = std::thread::spawn({
        let medium_pdf_path = medium_pdf_path.clone();
        move || {
            let mut doc = LopdfDocument { inner: Document::load(&medium_pdf_path).unwrap() };
            let compress_op = CompressOperation::new();
            compress_op.execute(&mut doc).unwrap();
        }
    });

    let t4 = std::thread::spawn({
        let small_pdf_path = small_pdf_path.clone();
        move || {
            let mut doc = LopdfDocument { inner: Document::load(&small_pdf_path).unwrap() };
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
