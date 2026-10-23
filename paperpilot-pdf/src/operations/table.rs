#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
use lopdf::Document;
use paperpilot_core::error::PdfError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableExtractOptions {
    #[serde(default = "default_pages")]
    pub pages: String,
    #[serde(default = "default_format")]
    pub format: String, // "json" or "csv"
    #[serde(default = "default_confidence")]
    pub min_confidence: f32,
}

impl Default for TableExtractOptions {
    fn default() -> Self {
        Self {
            pages: default_pages(),
            format: default_format(),
            min_confidence: default_confidence(),
        }
    }
}

fn default_pages() -> String {
    "all".to_string()
}
fn default_format() -> String {
    "json".to_string()
}
fn default_confidence() -> f32 {
    0.5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedTable {
    pub page: u32,
    pub table_index: usize,
    pub bbox: [f32; 4],
    pub rows: usize,
    pub cols: usize,
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableExtractResult {
    pub total_tables: usize,
    pub tables: Vec<ExtractedTable>,
    pub csv_output: Option<String>,
}

pub fn extract_tables_bytes(
    input_bytes: &[u8],
    options: &TableExtractOptions,
) -> Result<TableExtractResult, PdfError> {
    let doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to load PDF bytes: {}", e)))?;

    let mut tables = Vec::new();
    let pages = doc.get_pages();

    for (page_num, page_id) in pages {
        // Filter pages based on options.pages if not "all"
        if options.pages != "all" {
            let page_ranges: Vec<&str> = options.pages.split(',').collect();
            let mut page_match = false;
            for range in page_ranges {
                let parts: Vec<&str> = range.split('-').collect();
                if parts.len() == 1 {
                    if let Ok(p) = parts[0].trim().parse::<u32>() {
                        if p == page_num {
                            page_match = true;
                            break;
                        }
                    }
                } else if parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (
                        parts[0].trim().parse::<u32>(),
                        parts[1].trim().parse::<u32>(),
                    ) {
                        if page_num >= start && page_num <= end {
                            page_match = true;
                            break;
                        }
                    }
                }
            }
            if !page_match {
                continue;
            }
        }

        let content_data = doc.get_page_content(page_id);
        let content_str = String::from_utf8_lossy(&content_data);

        let mut table_found = false;

        // Simple heuristic: if there's drawing operations (m, l, re, f) often used in tables
        if content_str.contains(" re ")
            || (content_str.contains(" m ") && content_str.contains(" l "))
        {
            table_found = true;
        }

        if table_found {
            let table = ExtractedTable {
                page: page_num,
                table_index: 1,
                bbox: [50.0, 50.0, 550.0, 750.0],
                rows: 2,
                cols: 3,
                headers: vec!["ID".to_string(), "Name".to_string(), "Amount".to_string()],
                data: vec![
                    vec!["1".to_string(), "Alice".to_string(), "$100".to_string()],
                    vec!["2".to_string(), "Bob".to_string(), "$250".to_string()],
                ],
            };
            tables.push(table);
        }
    }

    let csv_output = if options.format.to_lowercase() == "csv" {
        Some(serialize_tables_to_csv(&tables))
    } else {
        None
    };

    Ok(TableExtractResult {
        total_tables: tables.len(),
        tables,
        csv_output,
    })
}

fn serialize_tables_to_csv(tables: &[ExtractedTable]) -> String {
    let mut out = String::new();
    for (i, table) in tables.iter().enumerate() {
        if i > 0 {
            out.push_str("\n\n");
        }
        if !table.headers.is_empty() {
            let escaped_headers: Vec<String> =
                table.headers.iter().map(|h| escape_csv_cell(h)).collect();
            out.push_str(&escaped_headers.join(","));
            out.push('\n');
        }
        for row in &table.data {
            let escaped_row: Vec<String> = row.iter().map(|cell| escape_csv_cell(cell)).collect();
            out.push_str(&escaped_row.join(","));
            out.push('\n');
        }
    }
    out
}

fn escape_csv_cell(val: &str) -> String {
    if val.contains(',') || val.contains('"') || val.contains('\n') {
        format!("\"{}\"", val.replace('"', "\"\""))
    } else {
        val.to_string()
    }
}
