use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use paperpilot_nlp::resolver::OfflineNlpResolver;
use paperpilot_nlp::traits::NlpResolver;

fn generate_queries() -> Vec<String> {
    let simple_commands = vec![
        "merge",
        "split",
        "compress",
        "watermark",
        "encrypt",
        "decrypt",
        "extract text",
        "ocr",
        "render",
        "delete pages",
    ];
    let medium_commands = vec![
        "please compress this report.pdf",
        "merge a.pdf and b.pdf",
        "extract text from document.pdf",
        "watermark with 'CONFIDENTIAL'",
        "encrypt doc.pdf with password 'secret'",
    ];
    let complex_commands = vec![
        "Extract pages 3-7 from the Q3 report and save to output.pdf with password 'abc123'",
        "merge doc1.pdf, doc2.pdf, and doc3.pdf into final_report.pdf and encrypt it",
        "please split my large document.pdf at pages 10 and 20 into multiple files",
        "watermark contract.pdf with 'DRAFT' and rotate all pages 90 degrees",
        "ocr the scanned_invoice.pdf and then extract all text to a json file",
    ];

    let mut queries = Vec::with_capacity(10000);
    for _ in 0..(10000 / 150 + 1) {
        for _ in 0..5 {
            for q in &simple_commands {
                queries.push(q.to_string());
            }
        }
        for _ in 0..10 {
            for q in &medium_commands {
                queries.push(q.to_string());
            }
        }
        for _ in 0..10 {
            for q in &complex_commands {
                queries.push(q.to_string());
            }
        }
    }
    queries.truncate(10000);
    queries
}

fn bench_nlp_resolver(c: &mut Criterion) {
    let resolver = OfflineNlpResolver::new();
    let queries = generate_queries();

    let mut group = c.benchmark_group("NlpResolver");
    group.sample_size(10);
    group.throughput(Throughput::Elements(queries.len() as u64));

    group.bench_function("10000_queries", |b| {
        b.iter_batched(
            || queries.clone(),
            |queries| {
                for q in queries {
                    let _ = resolver.resolve(&q);
                }
            },
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_nlp_resolver);
criterion_main!(benches);
