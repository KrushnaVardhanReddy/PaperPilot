use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use lopdf::Document;
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::compress::CompressOperation;
use paperpilot_pdf::operations::encrypt::EncryptOperation;
use paperpilot_pdf::operations::extract_text::ExtractTextOperation;
use paperpilot_pdf::operations::merge::MergeOperation;
use paperpilot_pdf::operations::split::SplitOperation;
use paperpilot_pdf::operations::watermark::WatermarkOperation;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use sysinfo::{ProcessesToUpdate, RefreshKind, System};

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

fn measure_peak_rss<F: FnOnce()>(f: F) -> u64 {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::everything()),
    );
    let pid = sysinfo::get_current_pid().unwrap();

    sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
    let start_mem = sys.process(pid).map(|p| p.memory()).unwrap_or(0);

    let is_running = Arc::new(Mutex::new(true));
    let is_running_clone = is_running.clone();

    let monitor = std::thread::spawn(move || {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::everything()),
        );
        let mut peak: u64 = 0;
        while *is_running_clone.lock().unwrap() {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
            if let Some(p) = sys.process(pid)
                && p.memory() > peak
            {
                peak = p.memory();
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        peak
    });

    f();

    *is_running.lock().unwrap() = false;
    let peak = monitor.join().unwrap();
    peak.saturating_sub(start_mem)
}

fn get_peak_rss() -> u64 {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::everything()),
    );
    let pid = sysinfo::get_current_pid().unwrap();
    sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
    sys.process(pid).map(|p| p.memory()).unwrap_or(0)
}

fn bench_merge(c: &mut Criterion) {
    let workspace = get_workspace_root();
    ensure_bench_fixtures(&workspace);
    let batch_dir = workspace.join("tests").join("bench_fixtures").join("batch");

    let mut paths = Vec::new();
    for i in 1..=20 {
        paths.push(batch_dir.join(format!("doc_{:03}.pdf", i)));
    }

    let peak_mem = measure_peak_rss(|| {
        let mut main_doc = LopdfDocument::new();
        let merge_op = MergeOperation::new(paths.clone());
        merge_op.execute(&mut main_doc).unwrap();
    });
    println!("SYSTEM_BENCHMARK_MEMORY|Merge_100|{}", peak_mem);

    let mut group = c.benchmark_group("Merge");
    group.sample_size(10);
    group.throughput(Throughput::Elements(100));
    group.bench_function("merge_20_docs_100_pages", |b| {
        b.iter(|| {
            let mut main_doc = LopdfDocument::new();
            let merge_op = MergeOperation::new(paths.clone());
            merge_op.execute(&mut main_doc).unwrap();
        });
    });
    group.finish();
}

fn bench_split(c: &mut Criterion) {
    let workspace = get_workspace_root();
    let fixtures = vec![
        ("tiny_1page.pdf", 1),
        ("small_10page.pdf", 10),
        ("medium_50page.pdf", 50),
        ("large_100page.pdf", 100),
        ("xlarge_500page.pdf", 500),
    ];
    let bench_dir = workspace.join("tests").join("bench_fixtures");

    let mut group = c.benchmark_group("Split");
    group.sample_size(10);
    for (filename, pages) in fixtures {
        let path = bench_dir.join(filename);
        let peak_mem = measure_peak_rss(|| {
            let mut doc = LopdfDocument {
                inner: Document::load(&path).unwrap(),
            };
            let out_dir = tempfile::tempdir().unwrap();
            let split_op = SplitOperation::new(out_dir.path().to_path_buf(), vec![]);
            split_op.execute(&mut doc).unwrap();
        });
        println!("SYSTEM_BENCHMARK_MEMORY|Split_{}|{}", pages, peak_mem);

        group.throughput(Throughput::Elements(pages as u64));
        group.bench_with_input(BenchmarkId::from_parameter(pages), &path, |b, path| {
            b.iter(|| {
                let mut doc = LopdfDocument {
                    inner: Document::load(path).unwrap(),
                };
                let out_dir = tempfile::tempdir().unwrap();
                let split_op = SplitOperation::new(out_dir.path().to_path_buf(), vec![]);
                split_op.execute(&mut doc).unwrap();
            });
        });
    }
    group.finish();
}

fn bench_extract_text(c: &mut Criterion) {
    let workspace = get_workspace_root();
    let fixtures = vec![
        ("tiny_1page.pdf", 1),
        ("small_10page.pdf", 10),
        ("medium_50page.pdf", 50),
        ("large_100page.pdf", 100),
        ("xlarge_500page.pdf", 500),
    ];
    let bench_dir = workspace.join("tests").join("bench_fixtures");

    let mut group = c.benchmark_group("ExtractText");
    group.sample_size(10);
    for (filename, pages) in fixtures {
        let path = bench_dir.join(filename);
        let peak_mem = measure_peak_rss(|| {
            let mut doc = LopdfDocument {
                inner: Document::load(&path).unwrap(),
            };
            let extract_op = ExtractTextOperation::new(None);
            extract_op.execute(&mut doc).unwrap();
        });
        println!("SYSTEM_BENCHMARK_MEMORY|ExtractText_{}|{}", pages, peak_mem);

        group.throughput(Throughput::Elements(pages as u64));
        group.bench_with_input(BenchmarkId::from_parameter(pages), &path, |b, path| {
            b.iter(|| {
                let mut doc = LopdfDocument {
                    inner: Document::load(path).unwrap(),
                };
                let extract_op = ExtractTextOperation::new(None);
                extract_op.execute(&mut doc).unwrap();
            });
        });
    }
    group.finish();
}

fn bench_compress(c: &mut Criterion) {
    let workspace = get_workspace_root();
    let fixtures = vec![
        ("tiny_1page.pdf", 1),
        ("small_10page.pdf", 10),
        ("medium_50page.pdf", 50),
        ("large_100page.pdf", 100),
        ("xlarge_500page.pdf", 500),
    ];
    let bench_dir = workspace.join("tests").join("bench_fixtures");

    let mut group = c.benchmark_group("Compress");
    group.sample_size(10);
    for (filename, pages) in fixtures {
        let path = bench_dir.join(filename);
        let peak_mem = measure_peak_rss(|| {
            let mut doc = LopdfDocument {
                inner: Document::load(&path).unwrap(),
            };
            let compress_op = CompressOperation::default();
            compress_op.execute(&mut doc).unwrap();
        });
        println!("SYSTEM_BENCHMARK_MEMORY|Compress_{}|{}", pages, peak_mem);

        group.throughput(Throughput::Elements(pages as u64));
        group.bench_with_input(BenchmarkId::from_parameter(pages), &path, |b, path| {
            b.iter(|| {
                let mut doc = LopdfDocument {
                    inner: Document::load(path).unwrap(),
                };
                let compress_op = CompressOperation::default();
                compress_op.execute(&mut doc).unwrap();
            });
        });
    }
    group.finish();
}

fn bench_watermark(c: &mut Criterion) {
    let workspace = get_workspace_root();
    let fixtures = vec![
        ("tiny_1page.pdf", 1),
        ("small_10page.pdf", 10),
        ("medium_50page.pdf", 50),
        ("large_100page.pdf", 100),
        ("xlarge_500page.pdf", 500),
    ];
    let bench_dir = workspace.join("tests").join("bench_fixtures");

    let mut group = c.benchmark_group("Watermark");
    group.sample_size(10);
    for (filename, pages) in fixtures {
        let path = bench_dir.join(filename);
        let peak_mem = measure_peak_rss(|| {
            let mut doc = LopdfDocument {
                inner: Document::load(&path).unwrap(),
            };
            let watermark_op = WatermarkOperation::new("CONFIDENTIAL".to_string());
            watermark_op.execute(&mut doc).unwrap();
        });
        println!("SYSTEM_BENCHMARK_MEMORY|Watermark_{}|{}", pages, peak_mem);

        group.throughput(Throughput::Elements(pages as u64));
        group.bench_with_input(BenchmarkId::from_parameter(pages), &path, |b, path| {
            b.iter(|| {
                let mut doc = LopdfDocument {
                    inner: Document::load(path).unwrap(),
                };
                let watermark_op = WatermarkOperation::new("CONFIDENTIAL".to_string());
                watermark_op.execute(&mut doc).unwrap();
            });
        });
    }
    group.finish();
}

fn bench_encrypt(c: &mut Criterion) {
    let workspace = get_workspace_root();
    let fixtures = vec![
        ("tiny_1page.pdf", 1),
        ("small_10page.pdf", 10),
        ("medium_50page.pdf", 50),
        ("large_100page.pdf", 100),
        ("xlarge_500page.pdf", 500),
    ];
    let bench_dir = workspace.join("tests").join("bench_fixtures");

    let mut group = c.benchmark_group("Encrypt");
    group.sample_size(10);
    for (filename, pages) in fixtures {
        let path = bench_dir.join(filename);
        let peak_mem = measure_peak_rss(|| {
            let mut doc = LopdfDocument {
                inner: Document::load(&path).unwrap(),
            };
            let encrypt_op = EncryptOperation::new();
            let _ = encrypt_op.execute(&mut doc);
        });
        println!("SYSTEM_BENCHMARK_MEMORY|Encrypt_{}|{}", pages, peak_mem);

        group.throughput(Throughput::Elements(pages as u64));
        group.bench_with_input(BenchmarkId::from_parameter(pages), &path, |b, path| {
            b.iter(|| {
                let mut doc = LopdfDocument {
                    inner: Document::load(path).unwrap(),
                };
                let encrypt_op = EncryptOperation::new();
                let _ = encrypt_op.execute(&mut doc);
            });
        });
    }
    group.finish();
}

fn bench_ocr(c: &mut Criterion) {
    let workspace = get_workspace_root();
    let path = workspace
        .join("tests")
        .join("bench_fixtures")
        .join("large_100page.pdf");

    let peak_mem = measure_peak_rss(|| {
        let mut doc = LopdfDocument {
            inner: Document::load(&path).unwrap(),
        };
        let ocr_op = paperpilot_pdf::operations::ocr::OcrOperation;
        let _ = ocr_op.execute(&mut doc);
    });
    println!("SYSTEM_BENCHMARK_MEMORY|OCR_100|{}", peak_mem);

    let mut group = c.benchmark_group("OCR");
    group.sample_size(10);
    group.throughput(Throughput::Elements(100));
    group.bench_function("ocr_large_100page", |b| {
        b.iter(|| {
            let mut doc = LopdfDocument {
                inner: Document::load(&path).unwrap(),
            };
            let ocr_op = paperpilot_pdf::operations::ocr::OcrOperation;
            let _ = ocr_op.execute(&mut doc);
        });
    });
    group.finish();
}

fn bench_pdf_to_image(c: &mut Criterion) {
    let workspace = get_workspace_root();
    let path = workspace
        .join("tests")
        .join("bench_fixtures")
        .join("medium_50page.pdf");

    let peak_mem = measure_peak_rss(|| {
        let mut doc = LopdfDocument {
            inner: Document::load(&path).unwrap(),
        };
        let render_op = paperpilot_pdf::operations::render::RenderOperation::new();
        let _ = render_op.execute(&mut doc);
    });
    println!("SYSTEM_BENCHMARK_MEMORY|Render_50|{}", peak_mem);

    let mut group = c.benchmark_group("Render");
    group.sample_size(10);
    group.throughput(Throughput::Elements(50));
    group.bench_function("render_medium_50page", |b| {
        b.iter(|| {
            let mut doc = LopdfDocument {
                inner: Document::load(&path).unwrap(),
            };
            let render_op = paperpilot_pdf::operations::render::RenderOperation::new();
            let _ = render_op.execute(&mut doc);
        });
    });
    group.finish();
}

fn bench_cold_warm_and_leak(_c: &mut Criterion) {
    let workspace = get_workspace_root();
    let large_pdf_path = workspace
        .join("tests")
        .join("bench_fixtures")
        .join("large_100page.pdf");
    let batch_dir = workspace.join("tests").join("bench_fixtures").join("batch");

    let mut batch_paths = Vec::new();
    for i in 1..=20 {
        batch_paths.push(batch_dir.join(format!("doc_{:03}.pdf", i)));
    }

    let cold_extract: u128;
    let warm_extract: u128;
    {
        let start = std::time::Instant::now();
        let mut doc = LopdfDocument {
            inner: Document::load(&large_pdf_path).unwrap(),
        };
        let extract_op = ExtractTextOperation::new(None);
        extract_op.execute(&mut doc).unwrap();
        cold_extract = start.elapsed().as_micros();

        let start = std::time::Instant::now();
        for _ in 0..10 {
            let mut doc = LopdfDocument {
                inner: Document::load(&large_pdf_path).unwrap(),
            };
            let extract_op = ExtractTextOperation::new(None);
            extract_op.execute(&mut doc).unwrap();
        }
        warm_extract = start.elapsed().as_micros() / 10;
    }
    println!(
        "SYSTEM_BENCHMARK_COLDWARM|ExtractText|{}|{}",
        cold_extract, warm_extract
    );

    let cold_merge: u128;
    let warm_merge: u128;
    {
        let start = std::time::Instant::now();
        let mut main_doc = LopdfDocument::new();
        let merge_op = MergeOperation::new(batch_paths.clone());
        merge_op.execute(&mut main_doc).unwrap();
        cold_merge = start.elapsed().as_micros();

        let start = std::time::Instant::now();
        for _ in 0..10 {
            let mut main_doc = LopdfDocument::new();
            let merge_op = MergeOperation::new(batch_paths.clone());
            merge_op.execute(&mut main_doc).unwrap();
        }
        warm_merge = start.elapsed().as_micros() / 10;
    }
    println!(
        "SYSTEM_BENCHMARK_COLDWARM|Merge|{}|{}",
        cold_merge, warm_merge
    );

    let small_batch_paths = vec![batch_dir.join("doc_001.pdf"), batch_dir.join("doc_002.pdf")];

    for i in 1..=1000 {
        let mut main_doc = LopdfDocument::new();
        let merge_op = MergeOperation::new(small_batch_paths.clone());
        merge_op.execute(&mut main_doc).unwrap();

        if i % 100 == 0 {
            let rss = get_peak_rss();
            println!("SYSTEM_BENCHMARK_LEAK|Merge|{}|{}", i, rss);
        }
    }

    for i in 1..=1000 {
        let mut doc = LopdfDocument {
            inner: Document::load(&small_batch_paths[0]).unwrap(),
        };
        let extract_op = ExtractTextOperation::new(None);
        extract_op.execute(&mut doc).unwrap();

        if i % 100 == 0 {
            let rss = get_peak_rss();
            println!("SYSTEM_BENCHMARK_LEAK|ExtractText|{}|{}", i, rss);
        }
    }
}

criterion_group!(
    benches,
    bench_merge,
    bench_split,
    bench_extract_text,
    bench_compress,
    bench_watermark,
    bench_encrypt,
    bench_ocr,
    bench_pdf_to_image,
    bench_cold_warm_and_leak
);
criterion_main!(benches);
