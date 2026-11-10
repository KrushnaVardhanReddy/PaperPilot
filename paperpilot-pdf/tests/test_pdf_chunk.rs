use paperpilot_core::document_source::DocumentSource;
use paperpilot_pdf::operations::pdf_chunk::{
    chunk_markdown, chunk_pages, estimate_tokens, ChunkOptions, PdfChunkOperation,
};
use std::path::Path;

#[test]
fn test_basic_chunking_with_heading_preservation() {
    let md = r#"# Section 1: Executive Overview

This is the primary introduction to the system architecture.
It details the foundational components and design parameters.

## Subsection 1.1: Component Layout

The layout contains five decoupled micro-services handling distinct workflows.
Each service communicates over lightweight binary IPC.

# Section 2: Implementation Details

Here we dive deep into the memory mapping and zero-copy data pipelines.
Performance is strictly optimized for low latency.
"#;

    let options = ChunkOptions {
        target_tokens: 150,
        overlap_tokens: 20,
        strip_boilerplate: true,
        break_on_headings: true,
    };

    let result = chunk_markdown(md, options);

    assert!(
        result.total_chunks >= 2,
        "Expected at least 2 chunks across distinct headings, got {}",
        result.total_chunks
    );

    // Verify first chunk starts with Section 1 and has Section 1 in headings_context
    assert_eq!(result.chunks[0].headings_context, vec!["Section 1: Executive Overview"]);
    assert!(result.chunks[0].text.contains("Section 1: Executive Overview"));

    // Verify chunk corresponding to Section 2 starts fresh
    let section2_chunk = result
        .chunks
        .iter()
        .find(|c| c.text.contains("Section 2: Implementation Details"))
        .expect("Section 2 chunk must exist");

    assert_eq!(
        section2_chunk.headings_context,
        vec!["Section 2: Implementation Details"]
    );
}

#[test]
fn test_token_bounds_and_overlap_validation() {
    // Generate text with multiple sentences that will span several chunks
    let mut text = String::new();
    for i in 1..=30 {
        text.push_str(&format!(
            "Sentence number {} provides detailed information about system performance and benchmarks. ",
            i
        ));
        if i % 5 == 0 {
            text.push_str("\n\n");
        }
    }

    let target_tokens = 60;
    let overlap_tokens = 20;

    let options = ChunkOptions {
        target_tokens,
        overlap_tokens,
        strip_boilerplate: false,
        break_on_headings: false,
    };

    let result = chunk_markdown(&text, options);

    assert!(result.total_chunks > 1, "Should create multiple chunks");

    for (idx, chunk) in result.chunks.iter().enumerate() {
        assert_eq!(chunk.chunk_index, idx);
        assert!(!chunk.text.trim().is_empty());
        assert_eq!(chunk.estimated_tokens, estimate_tokens(&chunk.text));
    }

    // Verify overlap between contiguous chunks
    for i in 1..result.chunks.len() {
        let prev_text = &result.chunks[i - 1].text;
        let curr_text = &result.chunks[i].text;

        // The current chunk should contain some words or sentence from the tail of the previous chunk
        let prev_words: Vec<&str> = prev_text.split_whitespace().collect();
        let last_few_prev = prev_words[prev_words.len().saturating_sub(4)..].join(" ");

        assert!(
            curr_text.contains(&last_few_prev) || curr_text.contains(prev_words.last().unwrap()),
            "Chunk {} should contain overlap from chunk {}",
            i,
            i - 1
        );
    }
}

#[test]
fn test_boilerplate_suppression_across_multiple_pages() {
    let pages = vec![
        (
            1,
            "Confidential - PaperPilot Internal\n\nPage 1 body text discussing initial requirements.\n\nPage 1 of 5"
                .to_string(),
        ),
        (
            2,
            "Confidential - PaperPilot Internal\n\nPage 2 body text detailing architecture decisions.\n\nPage 2 of 5"
                .to_string(),
        ),
        (
            3,
            "Confidential - PaperPilot Internal\n\nPage 3 body text analyzing verification results.\n\nPage 3 of 5"
                .to_string(),
        ),
        (
            4,
            "Confidential - PaperPilot Internal\n\nPage 4 body text describing deployment plans.\n\nPage 4 of 5"
                .to_string(),
        ),
        (
            5,
            "Confidential - PaperPilot Internal\n\nPage 5 body text with conclusions and sign-off.\n\nPage 5 of 5"
                .to_string(),
        ),
    ];

    let options = ChunkOptions {
        target_tokens: 500,
        overlap_tokens: 50,
        strip_boilerplate: true,
        break_on_headings: true,
    };

    let result = chunk_pages(pages, options);

    for chunk in &result.chunks {
        assert!(
            !chunk.text.contains("Confidential - PaperPilot Internal"),
            "Header boilerplate should be stripped from chunk: {}",
            chunk.text
        );
        assert!(
            !chunk.text.contains("Page 1 of 5") && !chunk.text.contains("Page 2 of 5"),
            "Footer boilerplate should be stripped from chunk: {}",
            chunk.text
        );
    }
}

#[test]
fn test_table_preservation() {
    let table_md = r#"# Financial Overview

Here is the revenue table for the past four quarters:

| Quarter | Revenue ($M) | Operating Income ($M) | Margin (%) |
| --- | --- | --- | --- |
| Q1 2026 | 124.5 | 32.1 | 25.8% |
| Q2 2026 | 145.2 | 41.6 | 28.7% |
| Q3 2026 | 168.0 | 50.4 | 30.0% |
| Q4 2026 | 192.3 | 61.5 | 32.0% |

All figures are verified under GAAP standards.
"#;

    let options = ChunkOptions {
        target_tokens: 500,
        overlap_tokens: 50,
        strip_boilerplate: true,
        break_on_headings: true,
    };

    let result = chunk_markdown(table_md, options);

    assert_eq!(result.total_chunks, 1);
    let chunk_text = &result.chunks[0].text;
    assert!(chunk_text.contains("| Quarter | Revenue ($M) |"));
    assert!(chunk_text.contains("| Q4 2026 | 192.3 | 61.5 | 32.0% |"));
}

#[test]
fn test_edge_cases_empty_single_page_and_long_sentence() {
    let options = ChunkOptions::default();

    // 1. Edge Case: 0-length documents
    let empty_res = chunk_pages(vec![], options.clone());
    assert_eq!(empty_res.total_chunks, 0);
    assert_eq!(empty_res.total_tokens, 0);
    assert!(empty_res.chunks.is_empty());

    let empty_bytes_res = PdfChunkOperation::chunk_bytes(&[], options.clone()).unwrap();
    assert_eq!(empty_bytes_res.total_chunks, 0);
    assert_eq!(empty_bytes_res.total_tokens, 0);

    // 2. Edge Case: 1-page document
    let single_page = vec![(1, "A single concise page of notes.".to_string())];
    let single_res = chunk_pages(single_page, options.clone());
    assert_eq!(single_res.total_chunks, 1);
    assert_eq!(single_res.chunks[0].page_start, 1);
    assert_eq!(single_res.chunks[0].page_end, 1);

    // 3. Edge Case: Single sentence exceeding target tokens
    let mut huge_sentence = String::from("Supercalifragilisticexpialidocious");
    for _ in 0..100 {
        huge_sentence.push_str(" continuously extending words without any punctuation whatsoever");
    }
    huge_sentence.push('.');

    let small_token_options = ChunkOptions {
        target_tokens: 60,
        overlap_tokens: 10,
        strip_boilerplate: false,
        break_on_headings: false,
    };

    let long_sentence_res = chunk_markdown(&huge_sentence, small_token_options);
    assert!(
        long_sentence_res.total_chunks > 1,
        "Sentence exceeding target tokens must be split into multiple chunks without panic"
    );
    for chunk in &long_sentence_res.chunks {
        assert!(!chunk.text.trim().is_empty());
    }
}

#[test]
fn test_integration_with_document_source_open() {
    let fixture_candidates = [
        "tests/e2e_fixtures/real/single_page.pdf",
        "../tests/e2e_fixtures/real/single_page.pdf",
        "../../tests/e2e_fixtures/real/single_page.pdf",
    ];

    let fixture_path = fixture_candidates
        .iter()
        .map(Path::new)
        .find(|p| p.exists())
        .expect("single_page.pdf fixture must exist");

    let doc_source = DocumentSource::open(fixture_path, None)
        .expect("DocumentSource::open failed on single_page.pdf");

    let options = ChunkOptions {
        target_tokens: 200,
        overlap_tokens: 30,
        strip_boilerplate: true,
        break_on_headings: true,
    };

    let result = PdfChunkOperation::chunk_source(&doc_source, options)
        .expect("PdfChunkOperation::chunk_source failed on real PDF fixture");

    assert!(result.total_chunks >= 1);
    assert!(result.total_tokens > 0);
    assert_eq!(result.chunks[0].page_start, 1);
    assert!(result.execution_time_ms >= 0.0);
}
