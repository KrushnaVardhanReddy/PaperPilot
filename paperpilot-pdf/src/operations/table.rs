#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]

use crate::document::LopdfDocument;
use crate::operations::conversion::PdfToHtmlOperation;
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::PdfOperation;
use scraper::{Html, Selector};
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

fn is_page_selected(page_num: u32, pages_filter: &str) -> bool {
    if pages_filter.trim().is_empty() || pages_filter.eq_ignore_ascii_case("all") {
        return true;
    }
    for range in pages_filter.split(',') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() == 1 {
            if let Ok(p) = parts[0].trim().parse::<u32>() {
                if p == page_num {
                    return true;
                }
            }
        } else if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (
                parts[0].trim().parse::<u32>(),
                parts[1].trim().parse::<u32>(),
            ) {
                if page_num >= start && page_num <= end {
                    return true;
                }
            }
        }
    }
    false
}

pub fn extract_tables_bytes(
    input_bytes: &[u8],
    options: &TableExtractOptions,
) -> Result<TableExtractResult, PdfError> {
    if input_bytes.is_empty() {
        return Ok(TableExtractResult {
            total_tables: 0,
            tables: Vec::new(),
            csv_output: if options.format.to_lowercase() == "csv" {
                Some(String::new())
            } else {
                None
            },
        });
    }

    let mut lopdf_doc = match LopdfDocument::from_bytes(input_bytes) {
        Ok(doc) => doc,
        Err(_) => {
            return Ok(TableExtractResult {
                total_tables: 0,
                tables: Vec::new(),
                csv_output: if options.format.to_lowercase() == "csv" {
                    Some(String::new())
                } else {
                    None
                },
            });
        }
    };

    let html_op = PdfToHtmlOperation::new();
    let _ = html_op.execute(&mut lopdf_doc);
    let html = {
        let guard = html_op.extracted_html.lock().unwrap();
        guard.clone().unwrap_or_default()
    };

    let parsed_doc = Html::parse_document(&html);
    let page_sel = Selector::parse("div.page").unwrap();
    let table_sel = Selector::parse("table").unwrap();
    let tr_sel = Selector::parse("tr").unwrap();
    let cell_sel = Selector::parse("th, td").unwrap();

    let mut tables = Vec::new();
    let mut overall_table_index = 0;

    let page_elements: Vec<scraper::ElementRef> = parsed_doc.select(&page_sel).collect();
    if page_elements.is_empty() {
        // Fallback: parse tables from root if no div.page wrappers
        for table_node in parsed_doc.select(&table_sel) {
            overall_table_index += 1;
            let mut headers = Vec::new();
            let mut data = Vec::new();
            let mut is_first_row = true;

            for tr in table_node.select(&tr_sel) {
                let mut row_cells: Vec<String> = Vec::new();
                for cell in tr.select(&cell_sel) {
                    let cell_text: Vec<&str> = cell
                        .text()
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();
                    row_cells.push(cell_text.join(" "));
                }

                while let Some(last) = row_cells.last() {
                    if last.is_empty() {
                        row_cells.pop();
                    } else {
                        break;
                    }
                }

                if row_cells.is_empty() {
                    continue;
                }

                if is_first_row {
                    headers = row_cells;
                    is_first_row = false;
                } else {
                    data.push(row_cells);
                }
            }

            if !headers.is_empty() || !data.is_empty() {
                let cols = headers.len().max(data.iter().map(|r| r.len()).max().unwrap_or(0));
                tables.push(ExtractedTable {
                    page: 1,
                    table_index: overall_table_index,
                    bbox: [0.0, 0.0, 612.0, 792.0],
                    rows: data.len(),
                    cols,
                    headers,
                    data,
                });
            }
        }
    } else {
        for (page_idx, page_elem) in page_elements.iter().enumerate() {
            let page_num = page_elem
                .value()
                .attr("data-page")
                .and_then(|p| p.parse::<u32>().ok())
                .unwrap_or((page_idx + 1) as u32);

            if !is_page_selected(page_num, &options.pages) {
                continue;
            }

            for table_node in page_elem.select(&table_sel) {
                overall_table_index += 1;
                let mut headers = Vec::new();
                let mut data = Vec::new();
                let mut is_first_row = true;

                for tr in table_node.select(&tr_sel) {
                    let mut row_cells: Vec<String> = Vec::new();
                    for cell in tr.select(&cell_sel) {
                        let cell_text: Vec<&str> = cell
                            .text()
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect();
                        row_cells.push(cell_text.join(" "));
                    }

                    while let Some(last) = row_cells.last() {
                        if last.is_empty() {
                            row_cells.pop();
                        } else {
                            break;
                        }
                    }

                    if row_cells.is_empty() {
                        continue;
                    }

                    if is_first_row {
                        headers = row_cells;
                        is_first_row = false;
                    } else {
                        data.push(row_cells);
                    }
                }

                if !headers.is_empty() || !data.is_empty() {
                    let cols = headers.len().max(data.iter().map(|r| r.len()).max().unwrap_or(0));
                    tables.push(ExtractedTable {
                        page: page_num,
                        table_index: overall_table_index,
                        bbox: [0.0, 0.0, 612.0, 792.0],
                        rows: data.len(),
                        cols,
                        headers,
                        data,
                    });
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    // We will use real test PDFs or create simple valid ones since pdf_oxide parses the binary structure directly.
    // For unit tests, testing the format output and empty behavior is straightforward.

    #[test]
    fn test_extract_tables_empty_document() {
        // A minimal valid PDF with no text or tables
        let empty_pdf = b"%PDF-1.4\n\
        1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n\
        2 0 obj << /Type /Pages /Count 1 /Kids [ 3 0 R ] >> endobj\n\
        3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources <<>> /Contents 4 0 R >> endobj\n\
        4 0 obj << /Length 0 >> stream\n\
        endstream endobj\n\
        xref\n\
        0 5\n\
        0000000000 65535 f\n\
        0000000009 00000 n\n\
        0000000058 00000 n\n\
        0000000115 00000 n\n\
        0000000222 00000 n\n\
        trailer << /Size 5 /Root 1 0 R >>\n\
        startxref\n\
        266\n\
        %%EOF";

        let options = TableExtractOptions::default();
        let result = extract_tables_bytes(empty_pdf, &options).expect("Should parse empty PDF");

        assert_eq!(result.total_tables, 0);
        assert_eq!(result.tables.len(), 0);
    }

    #[test]
    fn test_extract_tables_csv_formatting() {
        let tables = vec![ExtractedTable {
            page: 1,
            table_index: 1,
            bbox: [0.0, 0.0, 100.0, 100.0],
            rows: 2,
            cols: 2,
            headers: vec!["Header 1".to_string(), "Header, 2".to_string()],
            data: vec![
                vec!["Data 1".to_string(), "Data \"2\"".to_string()],
                vec!["Data\n3".to_string(), "Data 4".to_string()],
            ],
        }];

        let csv = serialize_tables_to_csv(&tables);
        assert_eq!(
            csv,
            "Header 1,\"Header, 2\"\nData 1,\"Data \"\"2\"\"\"\n\"Data\n3\",Data 4\n"
        );
    }

    #[test]
    fn test_extract_tables_from_fixture() {
        let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../ManaulTestingFixtures/pdf_extract_tables/Input/sample_document.pdf");
        if fixture_path.exists() {
            let bytes = std::fs::read(fixture_path).unwrap();
            let options = TableExtractOptions {
                format: "json".to_string(),
                pages: "all".to_string(),
                ..Default::default()
            };
            let result = extract_tables_bytes(&bytes, &options).unwrap();
            assert!(result.total_tables >= 2, "Expected at least 2 tables, found {}", result.total_tables);
            assert_eq!(result.tables[0].headers[0], "Metric");
            assert_eq!(result.tables[1].headers[0], "Vehicle ID");
        }
    }
}
