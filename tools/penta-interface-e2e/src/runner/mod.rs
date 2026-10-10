pub mod api;
pub mod cli;
pub mod gateway;
pub mod mcp;

use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Cli,
    Mcp,
    Api,
    Wasm,
    CloudflareEdge,
}

impl InterfaceType {
    pub fn label(&self) -> &'static str {
        match self {
            InterfaceType::Cli => "💻 CLI",
            InterfaceType::Mcp => "🤖 MCP",
            InterfaceType::Api => "🌐 REST API",
            InterfaceType::Wasm => "⚡ WASM (Browser)",
            InterfaceType::CloudflareEdge => "☁️ Cloudflare Edge",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityTier {
    Simple,
    Medium,
    Complex,
    Negative,
}

impl ComplexityTier {
    pub fn label(&self) -> &'static str {
        match self {
            ComplexityTier::Simple => "Simple (Tier 1)",
            ComplexityTier::Medium => "Medium (Tier 2)",
            ComplexityTier::Complex => "Complex (Tier 3)",
            ComplexityTier::Negative => "Negative",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestExecutionResult {
    pub tool_id: String,
    pub interface: InterfaceType,
    pub tier: ComplexityTier,
    pub data_sent: String,
    pub assertion_checked: String,
    pub expected_result: String,
    pub actual_result: String,
    pub passed: bool,
    pub latency_ms: f64,
}

impl TestExecutionResult {
    pub fn get_invocation(&self) -> String {
        let sent = self.data_sent.trim();
        if sent.starts_with("curl ") {
            return sent.to_string();
        }
        if sent.starts_with("CLI args: [") {
            let inner = sent.trim_start_matches("CLI args: [").trim_end_matches(']');
            let parts: Vec<String> = inner
                .split(", ")
                .map(|s| s.trim_matches('"').to_string())
                .collect();
            return format!("paperpilot {}", parts.join(" "));
        }
        if sent.starts_with("paperpilot ") {
            return sent.to_string();
        }

        let cmd = self.tool_id.strip_prefix("pdf_").unwrap_or(&self.tool_id);
        let endpoint = cmd.replace('_', "-");

        let (canonical_args, mcp_args, wasm_call, edge_call) = match self.tool_id.as_str() {
            "pdf_merge" => (
                match self.tier {
                    ComplexityTier::Medium => "merge --input tests/e2e_fixtures/real/merge_a.pdf tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/merge_cli_medium.pdf --json",
                    ComplexityTier::Complex => "merge --input tests/e2e_fixtures/real/merge_a.pdf tests/e2e_fixtures/real/merge_b.pdf tests/e2e_fixtures/real/merge_c.pdf --output tests/e2e_fixtures/out/penta_e2e/merge_cli_complex.pdf --json",
                    ComplexityTier::Negative => "merge --input tests/e2e_fixtures/real/missing_file.pdf tests/e2e_fixtures/real/merge_a.pdf --output tests/e2e_fixtures/out/penta_e2e/merge_cli_neg.pdf --json",
                    _ => "merge --input tests/e2e_fixtures/real/merge_a.pdf tests/e2e_fixtures/real/merge_b.pdf --output tests/e2e_fixtures/out/penta_e2e/merge_cli_simple.pdf --json",
                },
                match self.tier {
                    ComplexityTier::Medium => r#"{"inputs": ["tests/e2e_fixtures/real/merge_a.pdf", "tests/e2e_fixtures/real/multi_page.pdf"], "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_medium.pdf"}"#,
                    ComplexityTier::Complex => r#"{"inputs": ["tests/e2e_fixtures/real/merge_a.pdf", "tests/e2e_fixtures/real/merge_b.pdf", "tests/e2e_fixtures/real/merge_c.pdf"], "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_complex.pdf"}"#,
                    ComplexityTier::Negative => r#"{"inputs": ["tests/e2e_fixtures/real/missing.pdf", "tests/e2e_fixtures/real/merge_a.pdf"], "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_neg.pdf"}"#,
                    _ => r#"{"inputs": ["tests/e2e_fixtures/real/merge_a.pdf", "tests/e2e_fixtures/real/merge_b.pdf"], "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf"}"#,
                },
                "WasmPdfEngine.merge([file1, file2])",
                "POST /api/v1/merge (multipart/form-data)"
            ),
            "pdf_split" => (
                match self.tier {
                    ComplexityTier::Medium => "split --input tests/e2e_fixtures/real/multi_page.pdf --pages 2,3,4 --output tests/e2e_fixtures/out/penta_e2e/pdf_split_medium --json",
                    ComplexityTier::Complex => "split --input tests/e2e_fixtures/real/multi_page.pdf --pages 5,1,2 --output tests/e2e_fixtures/out/penta_e2e/pdf_split_complex --json",
                    ComplexityTier::Negative => "split --input tests/e2e_fixtures/real/missing.pdf --pages 1,2 --output tests/e2e_fixtures/out/penta_e2e/pdf_split_neg --json",
                    _ => "split --input tests/e2e_fixtures/real/multi_page.pdf --pages 1,3 --output tests/e2e_fixtures/out/penta_e2e/pdf_split_simple --json",
                },
                match self.tier {
                    ComplexityTier::Medium => r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_medium", "pages": "2,3,4"}"#,
                    ComplexityTier::Complex => r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_complex", "pages": "5,1,2"}"#,
                    ComplexityTier::Negative => r#"{"input": "tests/e2e_fixtures/real/missing.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_neg"}"#,
                    _ => r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_simple", "pages": "1,3"}"#,
                },
                "WasmPdfEngine.split(pdfBytes, '1,3')",
                "POST /api/v1/split?ranges=1,3"
            ),
            "pdf_extract_pages" => (
                "extract --input tests/e2e_fixtures/real/multi_page.pdf --pages 1,3 --output tests/e2e_fixtures/out/penta_e2e/extracted.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/extracted.pdf", "pages": "1,3"}"#,
                "WasmPdfEngine.extract_pages(pdfBytes, '1,3')",
                "POST /api/v1/extract-pages?pages=1,3"
            ),
            "pdf_delete_pages" => (
                "delete --input tests/e2e_fixtures/real/multi_page.pdf --pages 2,4 --output tests/e2e_fixtures/out/penta_e2e/deleted.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/deleted.pdf", "pages": "2,4"}"#,
                "WasmPdfEngine.delete_pages(pdfBytes, '2,4')",
                "POST /api/v1/delete-pages?pages=2,4"
            ),
            "pdf_reorder_pages" => (
                "reorder --input tests/e2e_fixtures/real/multi_page.pdf --order 2,1,3,4,5 --output tests/e2e_fixtures/out/penta_e2e/reordered.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "order": "2,1,3,4,5", "output": "tests/e2e_fixtures/out/penta_e2e/reordered.pdf"}"#,
                "WasmPdfEngine.reorder(pdfBytes, [2,1,3,4,5])",
                "POST /api/v1/reorder?order=2,1,3,4,5"
            ),
            "pdf_rotate" => (
                "rotate --input tests/e2e_fixtures/real/single_page.pdf --degrees 90 --pages 1 --output tests/e2e_fixtures/out/penta_e2e/rotated.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "angle": 90, "pages": "1", "output": "tests/e2e_fixtures/out/penta_e2e/rotated.pdf"}"#,
                "WasmPdfEngine.rotate(pdfBytes, 90, '1')",
                "POST /api/v1/rotate?angle=90"
            ),
            "pdf_crop" => (
                "crop --input tests/e2e_fixtures/real/single_page.pdf --rect 10,10,200,200 --output tests/e2e_fixtures/out/penta_e2e/cropped.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "box": "10,10,200,200", "output": "tests/e2e_fixtures/out/penta_e2e/cropped.pdf"}"#,
                "WasmPdfEngine.crop(pdfBytes, 10, 10, 200, 200)",
                "POST /api/v1/crop?left=10&bottom=10&right=200&top=200"
            ),
            "pdf_burst" => (
                "burst --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/burst --json",
                r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/burst"}"#,
                "WasmPdfEngine.split(pdfBytes, 'each')",
                "POST /api/v1/split?ranges=each"
            ),
            "pdf_remove_blank" => (
                "remove-blank --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/noblank.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/noblank.pdf"}"#,
                "WasmPdfEngine.delete_pages(pdfBytes, blankPages)",
                "POST /api/v1/delete-pages"
            ),
            "pdf_compress" => (
                "compress --input tests/e2e_fixtures/real/single_page.pdf --quality medium --output tests/e2e_fixtures/out/penta_e2e/compressed.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "quality": "medium", "output": "tests/e2e_fixtures/out/penta_e2e/compressed.pdf"}"#,
                "WasmPdfEngine.compress(pdfBytes, 'medium')",
                "POST /api/v1/compress"
            ),
            "pdf_repair" => (
                "repair --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/repaired.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"}"#,
                "WasmPdfEngine.repair(pdfBytes)",
                "POST /api/v1/repair"
            ),
            "pdf_linearize" => (
                "linearize --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/linearized.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"}"#,
                "WasmPdfEngine.linearize(pdfBytes)",
                "POST /api/v1/linearize"
            ),
            "pdf_encrypt" => (
                "encrypt --input tests/e2e_fixtures/real/single_page.pdf --user-password testpass --output tests/e2e_fixtures/out/penta_e2e/encrypted.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "password": "testpass", "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf"}"#,
                "WasmPdfEngine.encrypt(pdfBytes, 'testpass')",
                "POST /api/v1/encrypt?password=testpass"
            ),
            "pdf_decrypt" => (
                "decrypt --input tests/e2e_fixtures/real/encrypted.pdf --password testpass --output tests/e2e_fixtures/out/penta_e2e/decrypted.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/encrypted.pdf", "password": "testpass", "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"}"#,
                "WasmPdfEngine.decrypt(pdfBytes, 'testpass')",
                "POST /api/v1/decrypt?password=testpass"
            ),
            "pdf_watermark" => (
                "watermark --input tests/e2e_fixtures/real/single_page.pdf --text CONFIDENTIAL --output tests/e2e_fixtures/out/penta_e2e/watermarked.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "text": "CONFIDENTIAL", "output": "tests/e2e_fixtures/out/penta_e2e/watermarked.pdf"}"#,
                "WasmPdfEngine.watermark(pdfBytes, 'CONFIDENTIAL')",
                "POST /api/v1/watermark?text=CONFIDENTIAL"
            ),
            "pdf_redact" => (
                "redact --input tests/e2e_fixtures/real/single_page.pdf --pages 1 --rect 50,50,200,50 --output tests/e2e_fixtures/out/penta_e2e/redacted.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "page": 1, "rect": [50, 50, 200, 50], "output": "tests/e2e_fixtures/out/penta_e2e/redacted.pdf"}"#,
                "WasmPdfEngine.redact(pdfBytes, 1, 50, 50, 200, 50)",
                "POST /api/v1/redact"
            ),
            "pdf_sign" => (
                "signature --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/signed.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/signed.pdf"}"#,
                "WasmPdfEngine.sign(pdfBytes)",
                "POST /api/v1/sign"
            ),
            "pdf_metadata" => (
                "metadata --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/metadata.json --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf"}"#,
                "WasmPdfEngine.metadata(pdfBytes)",
                "POST /api/v1/metadata"
            ),
            "pdf_flatten" => (
                "flatten --input tests/e2e_fixtures/real/form.pdf --output tests/e2e_fixtures/out/penta_e2e/flattened.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/form.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/flattened.pdf"}"#,
                "WasmPdfEngine.flatten(pdfBytes)",
                "POST /api/v1/flatten"
            ),
            "pdf_header_footer" => (
                "header-footer --input tests/e2e_fixtures/real/multi_page.pdf --text Confidential --output tests/e2e_fixtures/out/penta_e2e/header.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "text": "Confidential", "output": "tests/e2e_fixtures/out/penta_e2e/header.pdf"}"#,
                "WasmPdfEngine.header_footer(pdfBytes, 'Confidential')",
                "POST /api/v1/header-footer"
            ),
            "pdf_bates" => (
                "bates --input tests/e2e_fixtures/real/multi_page.pdf --prefix CONF- --start 1 --output tests/e2e_fixtures/out/penta_e2e/bates.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "prefix": "CONF-", "start": 1, "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf"}"#,
                "WasmPdfEngine.bates(pdfBytes, 'CONF-', 1)",
                "POST /api/v1/bates"
            ),
            "pdf_page_numbers" => (
                "page-numbers --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/numbers.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/numbers.pdf"}"#,
                "WasmPdfEngine.page_numbers(pdfBytes, '{page}/{total}')",
                "POST /api/v1/page-numbers"
            ),
            "pdf_annotate" => (
                "annotate --input tests/e2e_fixtures/real/single_page.pdf --data '[]' --output tests/e2e_fixtures/out/penta_e2e/annotated.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "annotations": [], "output": "tests/e2e_fixtures/out/penta_e2e/annotated.pdf"}"#,
                "WasmPdfEngine.annotate(pdfBytes, [])",
                "POST /api/v1/annotate"
            ),
            "pdf_read_form" => (
                "form read tests/e2e_fixtures/real/form.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/form.pdf"}"#,
                "WasmPdfEngine.read_form(pdfBytes)",
                "POST /api/v1/form/read"
            ),
            "pdf_fill_form" => (
                "form fill tests/e2e_fixtures/real/form.pdf --data '{}' --output tests/e2e_fixtures/out/penta_e2e/filled.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/form.pdf", "data": {}, "output": "tests/e2e_fixtures/out/penta_e2e/filled.pdf"}"#,
                "WasmPdfEngine.fill_form(pdfBytes, {})",
                "POST /api/v1/form/fill"
            ),
            "pdf_create_form_field" => (
                "form add-field tests/e2e_fixtures/real/single_page.pdf --name f1 --rect 0,0,10,10 --output tests/e2e_fixtures/out/penta_e2e/added_field.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "name": "f1", "rect": [0,0,10,10], "output": "tests/e2e_fixtures/out/penta_e2e/added_field.pdf"}"#,
                "WasmPdfEngine.add_field(pdfBytes, 'f1')",
                "POST /api/v1/form/add-field"
            ),
            "pdf_extract_text" => (
                "extract-text --input tests/e2e_fixtures/real/single_page.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf"}"#,
                "WasmPdfEngine.extract_text(pdfBytes)",
                "POST /api/v1/extract-text"
            ),
            "pdf_extract_images" => (
                "extract-images --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/images --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/images"}"#,
                "WasmPdfEngine.extract_images(pdfBytes)",
                "POST /api/v1/extract-images"
            ),
            "pdf_images_to_pdf" => (
                "images-to-pdf --images tests/e2e_fixtures/real/test.png --output tests/e2e_fixtures/out/penta_e2e/img2pdf.pdf --json",
                r#"{"images": ["tests/e2e_fixtures/real/test.png"], "output": "tests/e2e_fixtures/out/penta_e2e/img2pdf.pdf"}"#,
                "WasmPdfEngine.images_to_pdf([img1, img2])",
                "POST /api/v1/images-to-pdf"
            ),
            "pdf_search" => (
                "search --input tests/e2e_fixtures/real/single_page.pdf --query test --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "query": "test"}"#,
                "WasmPdfEngine.search(pdfBytes, 'test')",
                "POST /api/v1/search"
            ),
            "pdf_render" => (
                "render --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/rendered.png --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/rendered.png"}"#,
                "WasmPdfEngine.render(pdfBytes, 1)",
                "POST /api/v1/render"
            ),
            "pdf_compare" => (
                "compare --input tests/e2e_fixtures/real/merge_a.pdf --input-b tests/e2e_fixtures/real/merge_b.pdf --json",
                r#"{"file1": "tests/e2e_fixtures/real/merge_a.pdf", "file2": "tests/e2e_fixtures/real/merge_b.pdf"}"#,
                "WasmPdfEngine.compare(pdf1, pdf2)",
                "POST /api/v1/compare"
            ),
            "pdf_ocr" => (
                "ocr --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/ocr.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/ocr.pdf"}"#,
                "WasmPdfEngine.ocr(pdfBytes)",
                "POST /api/v1/ocr"
            ),
            "pdf_bookmarks" => (
                "bookmarks --input tests/e2e_fixtures/real/multi_page.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/multi_page.pdf"}"#,
                "WasmPdfEngine.bookmarks(pdfBytes)",
                "POST /api/v1/bookmarks"
            ),
            "pdf_classify_type" => (
                "classify --input tests/e2e_fixtures/real/single_page.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf"}"#,
                "WasmPdfEngine.classify(pdfBytes)",
                "POST /api/v1/classify"
            ),
            "pdf_validate" => (
                "validate --input tests/e2e_fixtures/real/single_page.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf"}"#,
                "WasmPdfEngine.validate(pdfBytes)",
                "POST /api/v1/validate"
            ),
            "pdf_hash" => (
                "hash --input tests/e2e_fixtures/real/single_page.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf"}"#,
                "WasmPdfEngine.hash(pdfBytes)",
                "POST /api/v1/hash"
            ),
            "pdf_to_docx" => (
                "convert --input tests/e2e_fixtures/real/single_page.pdf --format docx --output tests/e2e_fixtures/out/penta_e2e/out.docx --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.docx"}"#,
                "WasmPdfEngine.convert(pdfBytes, 'docx')",
                "POST /api/v1/convert/docx"
            ),
            "pdf_to_xlsx" => (
                "convert --input tests/e2e_fixtures/real/single_page.pdf --format xlsx --output tests/e2e_fixtures/out/penta_e2e/out.xlsx --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.xlsx"}"#,
                "WasmPdfEngine.convert(pdfBytes, 'xlsx')",
                "POST /api/v1/convert/xlsx"
            ),
            "pdf_to_pptx" => (
                "convert --input tests/e2e_fixtures/real/single_page.pdf --format pptx --output tests/e2e_fixtures/out/penta_e2e/out.pptx --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pptx"}"#,
                "WasmPdfEngine.convert(pdfBytes, 'pptx')",
                "POST /api/v1/convert/pptx"
            ),
            "pdf_to_pdf_a" => (
                "pdf-a --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/pdf_a.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/pdf_a.pdf"}"#,
                "WasmPdfEngine.pdf_a(pdfBytes)",
                "POST /api/v1/pdf-a"
            ),
            "pdf_convert_html" => (
                "convert --input tests/e2e_fixtures/real/test.html --format pdf --output tests/e2e_fixtures/out/penta_e2e/out_html.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/test.html", "output": "tests/e2e_fixtures/out/penta_e2e/out_html.pdf"}"#,
                "WasmPdfEngine.convert_html(htmlBytes)",
                "POST /api/v1/convert/html"
            ),
            "pdf_convert_markdown" => (
                "convert --input tests/e2e_fixtures/real/test.md --format pdf --output tests/e2e_fixtures/out/penta_e2e/out_md.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/test.md", "output": "tests/e2e_fixtures/out/penta_e2e/out_md.pdf"}"#,
                "WasmPdfEngine.convert_md(mdBytes)",
                "POST /api/v1/convert/markdown"
            ),
            "pdf_convert_excel" => (
                "convert --input tests/e2e_fixtures/real/test.csv --format pdf --output tests/e2e_fixtures/out/penta_e2e/out_csv.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/test.csv", "output": "tests/e2e_fixtures/out/penta_e2e/out_csv.pdf"}"#,
                "WasmPdfEngine.convert_csv(csvBytes)",
                "POST /api/v1/convert/excel"
            ),
            _ => (
                "process --input tests/e2e_fixtures/real/single_page.pdf --output tests/e2e_fixtures/out/penta_e2e/out.pdf --json",
                r#"{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/out.pdf"}"#,
                "WasmPdfEngine.process(pdfBytes)",
                "POST /api/v1/process"
            )
        };

        match self.interface {
            InterfaceType::Cli => {
                if canonical_args.starts_with(cmd)
                    || canonical_args.contains(" ") && !canonical_args.starts_with('-')
                {
                    format!("paperpilot {}", canonical_args)
                } else {
                    format!("paperpilot {} {}", cmd, canonical_args)
                }
            }
            InterfaceType::Mcp => {
                format!(
                    "tools/call {{\"name\": \"{}\", \"arguments\": {}}}",
                    self.tool_id, mcp_args
                )
            }
            InterfaceType::Api => {
                format!("curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/{} -H 'Content-Type: application/json' -d '{}'", endpoint, mcp_args)
            }
            InterfaceType::Wasm => wasm_call.to_string(),
            InterfaceType::CloudflareEdge => edge_call.to_string(),
        }
    }
}

pub fn hash_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let bytes = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn get_cli_bin() -> PathBuf {
    let release = PathBuf::from("/app/target/release/paperpilot");
    if release.exists() {
        release
    } else {
        PathBuf::from("/app/target/debug/paperpilot")
    }
}

pub fn get_mcp_bin() -> PathBuf {
    let release = PathBuf::from("/app/target/release/paperpilot-mcp");
    if release.exists() {
        release
    } else {
        PathBuf::from("/app/target/debug/paperpilot-mcp")
    }
}
