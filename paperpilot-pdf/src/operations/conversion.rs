use crate::document::LopdfDocument;
use docx_rs::{Docx, Paragraph, Run};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use rust_xlsxwriter::Workbook;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

pub const CSS_PRESET_GITHUB: &str = r#"
@page { margin: 20mm; size: A4; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    font-size: 14px;
    line-height: 1.6;
    color: #24292e;
    background-color: #ffffff;
}
h1, h2, h3, h4, h5, h6 {
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
    line-height: 1.25;
    color: #1f2328;
}
h1 { font-size: 2em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
h2 { font-size: 1.5em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
code {
    background-color: rgba(175, 184, 193, 0.2);
    border-radius: 4px;
    padding: 0.2em 0.4em;
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;
    font-size: 85%;
}
pre {
    background-color: #f6f8fa;
    border-radius: 6px;
    padding: 16px;
    overflow: auto;
    line-height: 1.45;
}
table {
    border-collapse: collapse;
    width: 100%;
    margin: 16px 0;
}
table th, table td {
    padding: 8px 12px;
    border: 1px solid #d0d7de;
}
table th {
    background-color: #f6f8fa;
    font-weight: 600;
}
table tr:nth-child(2n) {
    background-color: #f6f8fa;
}
thead { display: table-header-group; }
tr { page-break-inside: avoid; }
"#;

pub const CSS_PRESET_ELEGANT: &str = r#"
@page { margin: 25mm; size: A4; }
body {
    font-family: "Georgia", "Merriweather", serif;
    font-size: 15px;
    line-height: 1.7;
    color: #2c3e50;
    background-color: #ffffff;
}
h1, h2, h3 {
    font-family: "Palatino Linotype", "Book Antiqua", Palatino, serif;
    color: #1a252f;
    letter-spacing: 0.02em;
}
h1 { font-size: 2.2em; text-align: center; margin-bottom: 1.5em; }
h2 { font-size: 1.6em; border-bottom: 2px solid #8e44ad; padding-bottom: 6px; margin-top: 1.8em; }
table {
    border-collapse: collapse;
    width: 100%;
    margin: 20px 0;
}
table th {
    border-bottom: 2px solid #2c3e50;
    padding: 10px 14px;
    font-size: 14px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}
table td {
    border-bottom: 1px solid #ecf0f1;
    padding: 10px 14px;
}
"#;

pub const CSS_PRESET_MINIMAL: &str = r#"
@page { margin: 18mm; size: A4; }
body {
    font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
    font-size: 13px;
    line-height: 1.5;
    color: #111111;
}
h1, h2, h3 { font-weight: 700; color: #000000; }
h1 { font-size: 1.8em; }
h2 { font-size: 1.4em; }
table { width: 100%; border-collapse: collapse; margin: 16px 0; }
table th, table td { padding: 6px 10px; border-bottom: 1px solid #000; text-align: left; }
table th { font-weight: 700; border-top: 1px solid #000; }
"#;

pub const CSS_PRESET_BRANDED: &str = r#"
@page { margin: 20mm; size: A4; }
body {
    font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 13.5px;
    line-height: 1.6;
    color: #1e293b;
}
h1 { font-size: 2em; color: #4361ee; border-left: 4px solid #4361ee; padding-left: 12px; }
h2 { font-size: 1.4em; color: #3a0ca3; margin-top: 1.5em; }
table { width: 100%; border-collapse: separate; border-spacing: 0; border-radius: 8px; overflow: hidden; border: 1px solid #e2e8f0; margin: 16px 0; }
table th { background: #4361ee; color: #ffffff; padding: 10px 14px; font-weight: 600; text-align: left; }
table td { padding: 8px 14px; border-bottom: 1px solid #f1f5f9; }
table tr:nth-child(even) td { background-color: #f8fafc; }
"#;

pub const CSS_PRESET_COMPACT: &str = r#"
@page { margin: 12mm; size: A4; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 11px;
    line-height: 1.35;
    color: #1f2937;
}
h1 { font-size: 1.4em; margin-bottom: 8px; }
h2 { font-size: 1.2em; margin-top: 12px; margin-bottom: 6px; }
table { width: 100%; border-collapse: collapse; margin: 8px 0; font-size: 10.5px; }
table th, table td { padding: 4px 6px; border: 1px solid #d1d5db; }
table th { background: #f3f4f6; font-weight: 600; }
thead { display: table-header-group; }
tr { page-break-inside: avoid; }
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlToPdfOptions {
    pub preset: Option<String>, // "github", "elegant", "minimal", "branded", "compact"
    pub custom_css: Option<String>, // inline CSS string
    pub page_size: Option<String>, // "A4", "Letter", etc.
    pub margin_mm: Option<f32>, // default 20mm
}

impl Default for HtmlToPdfOptions {
    fn default() -> Self {
        Self {
            preset: Some("github".to_string()),
            custom_css: None,
            page_size: Some("A4".to_string()),
            margin_mm: Some(20.0),
        }
    }
}

pub fn resolve_css(preset: Option<&str>, custom_css: Option<&str>) -> String {
    let preset_css = match preset.unwrap_or("github") {
        "elegant" => CSS_PRESET_ELEGANT,
        "minimal" => CSS_PRESET_MINIMAL,
        "branded" => CSS_PRESET_BRANDED,
        "compact" => CSS_PRESET_COMPACT,
        _ => CSS_PRESET_GITHUB,
    };

    if let Some(custom) = custom_css
        && !custom.trim().is_empty()
    {
        return format!("{}\n/* Custom CSS */\n{}", preset_css, custom);
    }
    preset_css.to_string()
}

pub fn inject_css_into_html(html: &str, css: &str) -> String {
    let style_tag = format!("<style id=\"paperpilot-injected-css\">\n{}\n</style>", css);
    if let Some(pos) = html.find("</head>") {
        let mut out = html[..pos].to_string();
        out.push_str(&style_tag);
        out.push_str(&html[pos..]);
        out
    } else {
        format!(
            "<!DOCTYPE html><html><head>{}</head><body>{}</body></html>",
            style_tag, html
        )
    }
}

pub struct HtmlToPdfOperation {
    pub input_html: String,
    pub output_path: PathBuf,
    pub options: HtmlToPdfOptions,
}

impl HtmlToPdfOperation {
    pub fn new(input_html: String, output_path: PathBuf, options: HtmlToPdfOptions) -> Self {
        Self {
            input_html,
            output_path,
            options,
        }
    }

    pub fn render(&self) -> OperationResult<()> {
        let css = resolve_css(
            self.options.preset.as_deref(),
            self.options.custom_css.as_deref(),
        );
        let styled_html = inject_css_into_html(&self.input_html, &css);

        let engine = fulgur::Engine::builder().build();
        let pdf_data = engine
            .render(&styled_html)
            .map_err(|e| PdfError::Other(format!("Failed to render pdf: {:?}", e)))?;

        if pdf_data.is_empty() || !pdf_data.starts_with(b"%PDF-") {
            return Err(PdfError::Other(
                "Fulgur rendered invalid or empty PDF bytes".to_string(),
            ));
        }

        std::fs::write(&self.output_path, pdf_data)
            .map_err(|e| PdfError::Other(format!("Failed to write pdf file: {}", e)))?;

        Ok(())
    }
}

pub struct MarkdownToPdfOperation {
    pub input_md: String,
    pub output_path: PathBuf,
    pub options: HtmlToPdfOptions,
}

impl MarkdownToPdfOperation {
    pub fn new(input_md: String, output_path: PathBuf, options: HtmlToPdfOptions) -> Self {
        Self {
            input_md,
            output_path,
            options,
        }
    }

    pub fn render(&self) -> OperationResult<()> {
        use pulldown_cmark::{Parser, html};
        let parser = Parser::new(&self.input_md);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let full_html = format!(
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head><body>{}</body></html>",
            html_output
        );

        let html_op =
            HtmlToPdfOperation::new(full_html, self.output_path.clone(), self.options.clone());
        html_op.render()
    }
}

pub struct ExcelToStyledHtmlOperation {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub options: HtmlToPdfOptions,
}

impl ExcelToStyledHtmlOperation {
    pub fn new(input_path: PathBuf, output_path: PathBuf, options: HtmlToPdfOptions) -> Self {
        Self {
            input_path,
            output_path,
            options,
        }
    }

    pub fn render(&self) -> OperationResult<()> {
        use calamine::{Data, Reader, open_workbook_auto};

        let mut html =
            String::from("<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head><body>");

        if self.input_path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let mut rdr = csv::Reader::from_path(&self.input_path)
                .map_err(|e| PdfError::Other(format!("Failed to parse CSV: {}", e)))?;

            html.push_str("<table>");
            if let Ok(headers) = rdr.headers() {
                html.push_str("<tr>");
                for h in headers {
                    html.push_str(&format!("<th>{}</th>", html_escape::encode_text(h)));
                }
                html.push_str("</tr>");
            }

            for record in rdr.records().flatten() {
                html.push_str("<tr>");
                for field in record.iter() {
                    html.push_str(&format!("<td>{}</td>", html_escape::encode_text(field)));
                }
                html.push_str("</tr>");
            }
            html.push_str("</table>");
        } else {
            let mut workbook = open_workbook_auto(&self.input_path)
                .map_err(|e| PdfError::Other(format!("Failed to open workbook: {}", e)))?;

            let sheets = workbook.sheet_names().to_owned();
            for sheet_name in sheets {
                html.push_str(&format!(
                    "<h2>{}</h2>",
                    html_escape::encode_text(&sheet_name)
                ));
                if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                    html.push_str("<table>");
                    let mut is_first_row = true;
                    for row in range.rows() {
                        html.push_str("<tr>");
                        for cell in row {
                            let cell_tag = if is_first_row { "th" } else { "td" };
                            let cell_val = match cell {
                                Data::String(s) => s.to_string(),
                                Data::Float(f) => f.to_string(),
                                Data::Int(i) => i.to_string(),
                                Data::Bool(b) => b.to_string(),
                                Data::Error(e) => format!("Error: {:?}", e),
                                Data::Empty => String::new(),
                                Data::DateTime(d) => d.as_f64().to_string(),
                                Data::DateTimeIso(d) => d.to_string(),
                                Data::DurationIso(d) => d.to_string(),
                            };
                            html.push_str(&format!(
                                "<{}>{}</{}>",
                                cell_tag,
                                html_escape::encode_text(&cell_val),
                                cell_tag
                            ));
                        }
                        html.push_str("</tr>");
                        is_first_row = false;
                    }
                    html.push_str("</table>");
                }
            }
        }
        html.push_str("</body></html>");

        let html_op = HtmlToPdfOperation::new(html, self.output_path.clone(), self.options.clone());
        html_op.render()
    }
}

#[derive(Default)]
pub struct PdfToHtmlOperation {
    pub extracted_html: Arc<Mutex<Option<String>>>,
}

impl PdfToHtmlOperation {
    pub fn new() -> Self {
        Self {
            extracted_html: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToHtmlOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut html = String::from(
            "<!DOCTYPE html>\n<html>\n<head><title>PDF Export</title></head>\n<body>\n",
        );

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    html.push_str(&format!("<div class=\"page\" id=\"page-{}\">\n", page_id));
                    html.push_str(&format!("<h2>Page {}</h2>\n", page_id));
                    for line in text.lines() {
                        if !line.trim().is_empty() {
                            html.push_str(&format!("<p>{}</p>\n", line.trim()));
                        }
                    }
                    html.push_str("</div>\n");
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        html.push_str("</body>\n</html>");

        if let Ok(mut lock) = self.extracted_html.lock() {
            *lock = Some(html);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct PdfToMarkdownOperation {
    pub extracted_markdown: Arc<Mutex<Option<String>>>,
}

impl PdfToMarkdownOperation {
    pub fn new() -> Self {
        Self {
            extracted_markdown: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToMarkdownOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut md = String::new();

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    md.push_str(&format!("# Page {}\n\n", page_id));
                    md.push_str(text.trim());
                    md.push_str("\n\n");
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        if let Ok(mut lock) = self.extracted_markdown.lock() {
            *lock = Some(md);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageMetadata {
    pub page_number: u32,
    pub char_count: usize,
    pub word_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmExportMetadata {
    pub total_pages: usize,
    pub total_chars: usize,
    pub total_words: usize,
    pub pages: Vec<PageMetadata>,
}

#[derive(Default)]
pub struct PdfToLlmExportOperation {
    pub extracted_markdown: Arc<Mutex<Option<String>>>,
    pub metadata: Arc<Mutex<Option<LlmExportMetadata>>>,
}

impl PdfToLlmExportOperation {
    pub fn new() -> Self {
        Self {
            extracted_markdown: Arc::new(Mutex::new(None)),
            metadata: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToLlmExportOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut md = String::new();
        let mut pages_meta = Vec::new();
        let mut total_chars = 0;
        let mut total_words = 0;

        for page_id in &all_pages {
            match inner.extract_text(&[*page_id]) {
                Ok(text) => {
                    let trimmed = text.trim();
                    let chars = trimmed.chars().count();
                    let words = trimmed.split_whitespace().count();

                    pages_meta.push(PageMetadata {
                        page_number: *page_id,
                        char_count: chars,
                        word_count: words,
                    });

                    total_chars += chars;
                    total_words += words;

                    md.push_str(&format!("## Page {}\n\n", page_id));
                    md.push_str(trimmed);
                    md.push_str("\n\n---\n\n");
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        let meta = LlmExportMetadata {
            total_pages: all_pages.len(),
            total_chars,
            total_words,
            pages: pages_meta,
        };

        if let Ok(mut lock) = self.extracted_markdown.lock() {
            *lock = Some(md);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        if let Ok(mut lock) = self.metadata.lock() {
            *lock = Some(meta);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonPage {
    pub page_number: u32,
    pub text: String,
    pub char_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonDocument {
    pub document_name: String,
    pub pages: Vec<JsonPage>,
}

pub struct PdfToJsonOperation {
    pub document_name: String,
    pub extracted_json: Arc<Mutex<Option<String>>>,
}

impl PdfToJsonOperation {
    pub fn new(document_name: String) -> Self {
        Self {
            document_name,
            extracted_json: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for PdfToJsonOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();

        let mut pages_data = Vec::new();

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    let chars = text.chars().count();
                    pages_data.push(JsonPage {
                        page_number: page_id,
                        text,
                        char_count: chars,
                    });
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        let doc_struct = JsonDocument {
            document_name: self.document_name.clone(),
            pages: pages_data,
        };

        let json_str = serde_json::to_string_pretty(&doc_struct)
            .map_err(|e| PdfError::Other(format!("Failed to serialize JSON: {}", e)))?;

        if let Ok(mut lock) = self.extracted_json.lock() {
            *lock = Some(json_str);
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

pub struct PdfToDocxOperation {
    pub output_path: std::path::PathBuf,
}

impl PdfToDocxOperation {
    pub fn new(output_path: std::path::PathBuf) -> Self {
        Self { output_path }
    }
}

impl PdfOperation for PdfToDocxOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let mut all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();
        all_pages.sort_unstable();

        let mut docx = Docx::new();

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    for line in text.lines() {
                        let para = Paragraph::new().add_run(Run::new().add_text(line));
                        docx = docx.add_paragraph(para);
                    }
                    docx = docx.add_paragraph(
                        Paragraph::new().add_run(Run::new().add_text("--- PAGE BREAK ---")),
                    );
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        let file = std::fs::File::create(&self.output_path)
            .map_err(|e| PdfError::Other(format!("Failed to create DOCX file: {}", e)))?;
        docx.build()
            .pack(file)
            .map_err(|e| PdfError::Other(format!("Failed to pack DOCX file: {}", e)))?;

        Ok(())
    }
}

pub struct PdfToXlsxOperation {
    pub output_path: std::path::PathBuf,
}

impl PdfToXlsxOperation {
    pub fn new(output_path: std::path::PathBuf) -> Self {
        Self { output_path }
    }
}

impl PdfOperation for PdfToXlsxOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let mut all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();
        all_pages.sort_unstable();

        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        let mut current_row: u32 = 0;
        let ws_regex = regex::Regex::new(r"\s{3,}").unwrap();

        for page_id in all_pages {
            match inner.extract_text(&[page_id]) {
                Ok(text) => {
                    for line in text.lines() {
                        let line_trimmed = line.trim();
                        if line_trimmed.is_empty() {
                            continue;
                        }

                        let cols: Vec<&str> = ws_regex.split(line_trimmed).collect();

                        for (col_idx, val) in cols.iter().enumerate() {
                            worksheet
                                .write_string(current_row, col_idx as u16, *val)
                                .map_err(|e| {
                                    PdfError::Other(format!("Failed to write XLSX cell: {}", e))
                                })?;
                        }
                        current_row += 1;
                    }
                }
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_id, e
                    )));
                }
            }
        }

        workbook
            .save(&self.output_path)
            .map_err(|e| PdfError::Other(format!("Failed to save XLSX: {}", e)))?;

        Ok(())
    }
}

pub struct PdfToPptxOperation {
    pub output_path: std::path::PathBuf,
}

impl PdfToPptxOperation {
    pub fn new(output_path: std::path::PathBuf) -> Self {
        Self { output_path }
    }
}

impl PdfOperation for PdfToPptxOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;
        let mut all_pages: Vec<u32> = inner.get_pages().keys().copied().collect();
        all_pages.sort_unstable();

        let file = std::fs::File::create(&self.output_path)
            .map_err(|e| PdfError::Other(format!("Failed to create PPTX file: {}", e)))?;
        let mut zip = ZipWriter::new(file);

        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        for (i, page_id) in all_pages.iter().enumerate() {
            let slide_name = format!("ppt/slides/slide{}.xml", i + 1);
            zip.start_file(&slide_name, options)
                .map_err(|e| PdfError::Other(format!("Failed to start ZIP file: {}", e)))?;

            let text = inner.extract_text(&[*page_id]).unwrap_or_default();
            let slide_xml = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
    <p:cSld>
        <p:spTree>
            <p:sp>
                <p:txBody>
                    <a:p>
                        <a:r>
                            <a:t>{}</a:t>
                        </a:r>
                    </a:p>
                </p:txBody>
            </p:sp>
        </p:spTree>
    </p:cSld>
</p:sld>"#,
                text.replace("&", "&amp;")
                    .replace("<", "&lt;")
                    .replace(">", "&gt;")
            );

            zip.write_all(slide_xml.as_bytes())
                .map_err(|e| PdfError::Other(format!("Failed to write slide XML: {}", e)))?;
        }

        zip.start_file("[Content_Types].xml", options)
            .map_err(|e| PdfError::Other(format!("Failed to start ZIP file: {}", e)))?;
        let content_types = r#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
    <Default Extension="xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
</Types>"#;
        zip.write_all(content_types.as_bytes())
            .map_err(|e| PdfError::Other(format!("Failed to write content types: {}", e)))?;

        zip.finish()
            .map_err(|e| PdfError::Other(format!("Failed to finish ZIP file: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, Document, Object};

    fn create_dummy_doc(pages: u32) -> LopdfDocument {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut page_ids = vec![];

        for _ in 0..pages {
            let mut page_dict = Dictionary::new();
            page_dict.set("Type", Object::Name(b"Page".to_vec()));
            page_dict.set("Parent", Object::Reference(pages_id));

            let content = "BT /F1 12 Tf 0 0 Td (Test Content) Tj ET";
            let content_stream = lopdf::Stream::new(Dictionary::new(), content.as_bytes().to_vec());
            let content_id = doc.add_object(Object::Stream(content_stream));
            page_dict.set("Contents", Object::Reference(content_id));

            let mut resources_dict = Dictionary::new();
            let mut font_dict = Dictionary::new();
            let mut f1_dict = Dictionary::new();
            f1_dict.set("Type", Object::Name(b"Font".to_vec()));
            f1_dict.set("Subtype", Object::Name(b"Type1".to_vec()));
            f1_dict.set("BaseFont", Object::Name(b"Helvetica".to_vec()));
            let f1_id = doc.add_object(Object::Dictionary(f1_dict));
            font_dict.set("F1", Object::Reference(f1_id));
            resources_dict.set("Font", Object::Dictionary(font_dict));

            page_dict.set("Resources", Object::Dictionary(resources_dict));

            let page_id = doc.add_object(page_dict);
            page_ids.push(Object::Reference(page_id));
        }

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", Object::Integer(pages as i64));
        pages_dict.set("Kids", Object::Array(page_ids));

        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
        doc.trailer
            .set("Root", Object::Dictionary(Dictionary::new()));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        LopdfDocument { inner: doc }
    }

    #[test]
    fn test_css_preset_resolution() {
        let css = resolve_css(Some("elegant"), None);
        assert!(css.contains("Georgia"));
        assert!(css.contains("margin: 25mm"));

        let css2 = resolve_css(Some("github"), None);
        assert!(css2.contains("-apple-system"));
        assert!(css2.contains("margin: 20mm"));

        let css3 = resolve_css(None, None);
        assert!(css3.contains("-apple-system"));
    }

    #[test]
    fn test_custom_css_concatenation() {
        let css = resolve_css(Some("minimal"), Some("body { font-size: 20px; }"));
        assert!(css.contains("margin: 18mm"));
        assert!(css.contains("/* Custom CSS */"));
        assert!(css.contains("body { font-size: 20px; }"));
    }

    #[test]
    fn test_inject_css_into_html() {
        let html = "<!DOCTYPE html><html><head><title>Test</title></head><body><h1>Hello</h1></body></html>";
        let css = "body { color: red; }";
        let injected = inject_css_into_html(html, css);

        assert!(
            injected
                .contains("<style id=\"paperpilot-injected-css\">\nbody { color: red; }\n</style>")
        );
        assert!(injected.contains("</head>"));

        let html_no_head = "<body><h1>Hello</h1></body>";
        let injected_no_head = inject_css_into_html(html_no_head, css);
        assert!(
            injected_no_head
                .starts_with("<!DOCTYPE html><html><head><style id=\"paperpilot-injected-css\">")
        );
        assert!(injected_no_head.contains("<body><body><h1>Hello</h1></body></body></html>"));
    }

    #[test]
    fn test_pdf_to_html() {
        let mut doc = create_dummy_doc(2);
        let op = PdfToHtmlOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_html.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("<!DOCTYPE html>"));
        assert!(extracted.contains("<div class=\"page\" id=\"page-1\">"));
    }

    #[test]
    fn test_pdf_to_markdown() {
        let mut doc = create_dummy_doc(2);
        let op = PdfToMarkdownOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_markdown.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("# Page 1"));
    }

    #[test]
    fn test_pdf_to_llm_export() {
        let mut doc = create_dummy_doc(1);
        let op = PdfToLlmExportOperation::new();
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_markdown.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("## Page 1"));

        let meta = op.metadata.lock().unwrap();
        assert!(meta.is_some());
        let extracted_meta = meta.as_ref().unwrap();
        assert_eq!(extracted_meta.total_pages, 1);
        assert_eq!(extracted_meta.pages.len(), 1);
    }

    #[test]
    fn test_pdf_to_json() {
        let mut doc = create_dummy_doc(1);
        let op = PdfToJsonOperation::new("test_doc.pdf".to_string());
        assert!(op.execute(&mut doc).is_ok());

        let text = op.extracted_json.lock().unwrap();
        assert!(text.is_some());
        let extracted = text.as_ref().unwrap();
        assert!(extracted.contains("\"document_name\": \"test_doc.pdf\""));
    }

    #[test]
    fn test_pdf_to_docx() {
        let mut doc = create_dummy_doc(1);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out_test.docx");

        let op = PdfToDocxOperation::new(path.clone());
        assert!(op.execute(&mut doc).is_ok());

        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_pdf_to_xlsx() {
        let mut doc = create_dummy_doc(1);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out_test.xlsx");

        let op = PdfToXlsxOperation::new(path.clone());
        assert!(op.execute(&mut doc).is_ok());

        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_pdf_to_pptx() {
        let mut doc = create_dummy_doc(1);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out_test.pptx");

        let op = PdfToPptxOperation::new(path.clone());
        assert!(op.execute(&mut doc).is_ok());

        assert!(path.exists());
        assert!(path.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_excel_to_html_csv() {
        let dir = tempfile::tempdir().unwrap();
        let csv_path = dir.path().join("test.csv");
        let out_path = dir.path().join("out.pdf");
        std::fs::write(&csv_path, "Name,Age\nJohn,30\nJane,25").unwrap();

        let options = HtmlToPdfOptions {
            preset: None,
            custom_css: None,
            page_size: None,
            margin_mm: None,
        };

        let op = ExcelToStyledHtmlOperation::new(csv_path.clone(), out_path.clone(), options);
        assert!(op.render().is_ok());

        assert!(out_path.exists());
        assert!(out_path.metadata().unwrap().len() > 0);
        let pdf_bytes = std::fs::read(&out_path).unwrap();
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_html_to_pdf_operation_fulgur() {
        let dir = tempfile::tempdir().unwrap();
        let out_path = dir.path().join("out_fulgur.pdf");
        let options = HtmlToPdfOptions {
            preset: Some("minimal".to_string()),
            custom_css: None,
            page_size: None,
            margin_mm: None,
        };

        let html =
            "<html><body><h1>Test HTML</h1><p>Rendering via fulgur.</p></body></html>".to_string();
        let op = HtmlToPdfOperation::new(html, out_path.clone(), options);
        let res = op.render();

        assert!(res.is_ok());
        assert!(out_path.exists());
        assert!(out_path.metadata().unwrap().len() > 0);

        let pdf_bytes = std::fs::read(&out_path).unwrap();
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }

    #[test]
    fn test_markdown_to_pdf_operation_fulgur() {
        let dir = tempfile::tempdir().unwrap();
        let out_path = dir.path().join("out_md_fulgur.pdf");
        let options = HtmlToPdfOptions {
            preset: Some("github".to_string()),
            custom_css: None,
            page_size: None,
            margin_mm: None,
        };

        let md = "# Test Markdown\n\nRendering via **fulgur**.".to_string();
        let op = MarkdownToPdfOperation::new(md, out_path.clone(), options);
        let res = op.render();

        assert!(res.is_ok());
        assert!(out_path.exists());
        assert!(out_path.metadata().unwrap().len() > 0);

        let pdf_bytes = std::fs::read(&out_path).unwrap();
        assert!(pdf_bytes.starts_with(b"%PDF-"));
    }
}
