use paperpilot_nlp::rag::DocumentationRagEngine;
use std::time::Instant;

#[test]
fn test_query_encrypt() {
    let engine = DocumentationRagEngine::new();
    let start = Instant::now();
    let ans = engine.query("How to encrypt with password?");
    let duration = start.elapsed();

    assert!(ans.is_some(), "Should find an answer for encrypt query");
    let ans = ans.unwrap();
    assert_eq!(ans.tool_name, "pdf_encrypt");
    assert!(ans.cli_example.contains("encrypt"));
    assert!(ans.confidence_score >= 0.39);
    assert!(duration.as_millis() < 25, "Query latency should be < 25ms");
}

#[test]
fn test_query_watermark() {
    let engine = DocumentationRagEngine::new();
    let start = Instant::now();
    let ans = engine.query("How to add watermark?");
    let duration = start.elapsed();

    assert!(ans.is_some(), "Should find an answer for watermark query");
    let ans = ans.unwrap();
    assert_eq!(ans.tool_name, "pdf_watermark");
    assert!(ans.cli_example.contains("watermark"));
    assert!(duration.as_millis() < 25, "Query latency should be < 25ms");
}

#[test]
fn test_query_bates() {
    let engine = DocumentationRagEngine::new();
    let start = Instant::now();
    let ans = engine.query("What is Bates numbering?");
    let duration = start.elapsed();

    assert!(ans.is_some(), "Should find an answer for Bates query");
    let ans = ans.unwrap();
    assert_eq!(ans.tool_name, "pdf_bates");
    assert!(ans.cli_example.contains("bates"));
    assert!(duration.as_millis() < 25, "Query latency should be < 25ms");
}

#[test]
fn test_query_no_match() {
    let engine = DocumentationRagEngine::new();
    let ans = engine.query("What is the recipe for chocolate cake?");
    assert!(ans.is_none(), "Should not match irrelevant queries");
}

#[test]
fn test_query_rotate() {
    let engine = DocumentationRagEngine::new();
    let ans = engine.query("rotate pdf pages 90 degrees");
    assert!(ans.is_some());
    assert_eq!(ans.unwrap().tool_name, "pdf_rotate");
}

#[test]
fn test_query_extract_pages() {
    let engine = DocumentationRagEngine::new();
    let ans = engine.query("extract pages 1,3,5");
    assert!(ans.is_some());
    assert_eq!(ans.unwrap().tool_name, "pdf_extract_pages");
}
