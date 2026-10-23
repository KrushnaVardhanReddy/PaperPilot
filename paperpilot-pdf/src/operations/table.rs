#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]

use paperpilot_core::error::PdfError;
use pdf_oxide::api::Pdf;
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
    // Extract words using pdf_oxide for spatial positioning
    let mut pdf = Pdf::from_bytes(input_bytes.to_vec())
        .map_err(|e| PdfError::ParseError(format!("Failed to load PDF with pdf_oxide: {:?}", e)))?;

    let total_pages = pdf.page_count().unwrap_or(0);
    let mut tables = Vec::new();
    let mut overall_table_index = 0;

    for page_idx in 0..total_pages {
        let page_num = (page_idx + 1) as u32;

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

        if let Ok(mut words) = pdf.extract_words(page_idx) {
            if words.is_empty() {
                continue;
            }

            // Y-Clustering (Rows)
            // Sort by Y descending (PDF coordinates usually have 0,0 at bottom-left)
            words.sort_by(|a, b| {
                b.bbox
                    .y
                    .partial_cmp(&a.bbox.y)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let mut rows: Vec<Vec<&pdf_oxide::layout::text_block::Word>> = Vec::new();
            let mut current_row: Vec<&pdf_oxide::layout::text_block::Word> = Vec::new();
            let mut current_y = words[0].bbox.y;
            let y_threshold = 8.0; // ~8pt tolerance

            for word in &words {
                if (word.bbox.y - current_y).abs() <= y_threshold {
                    current_row.push(word);
                } else {
                    rows.push(current_row);
                    current_row = vec![word];
                    current_y = word.bbox.y;
                }
            }
            if !current_row.is_empty() {
                rows.push(current_row);
            }

            // If we only have 1 or 0 rows, it's not a table
            if rows.len() < 2 {
                continue;
            }

            // Find columns based on X intervals
            // First, find all X coordinates
            let mut x_intervals: Vec<(f32, f32)> = Vec::new();
            for row in &rows {
                for word in row {
                    let start_x = word.bbox.x;
                    let end_x = word.bbox.x + word.bbox.width;

                    // Try to merge into existing intervals
                    let mut merged = false;
                    for interval in &mut x_intervals {
                        // If overlaps or is very close (e.g. 5pt)
                        if (start_x <= interval.1 + 5.0) && (end_x + 5.0 >= interval.0) {
                            interval.0 = interval.0.min(start_x);
                            interval.1 = interval.1.max(end_x);
                            merged = true;
                            break;
                        }
                    }
                    if !merged {
                        x_intervals.push((start_x, end_x));
                    }
                }
            }

            // Clean up intervals (merge any overlapping ones after the first pass)
            x_intervals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let mut merged_intervals = Vec::new();
            if !x_intervals.is_empty() {
                let mut current = x_intervals[0];
                for interval in x_intervals.iter().skip(1) {
                    if interval.0 <= current.1 + 5.0 {
                        current.1 = current.1.max(interval.1);
                    } else {
                        merged_intervals.push(current);
                        current = *interval;
                    }
                }
                merged_intervals.push(current);
            }

            let num_cols = merged_intervals.len();
            if num_cols < 2 {
                continue; // Not a table if less than 2 columns
            }

            // Map rows into grid cells
            let mut grid: Vec<Vec<String>> = Vec::new();
            for row in &rows {
                let mut grid_row = vec![String::new(); num_cols];
                for word in row {
                    let mid_x = word.bbox.x + word.bbox.width / 2.0;
                    // Find which column this belongs to
                    for (col_idx, interval) in merged_intervals.iter().enumerate() {
                        if mid_x >= interval.0 - 5.0 && mid_x <= interval.1 + 5.0 {
                            if !grid_row[col_idx].is_empty() {
                                grid_row[col_idx].push(' ');
                            }
                            grid_row[col_idx].push_str(&word.text);
                            break;
                        }
                    }
                }
                grid.push(grid_row);
            }

            // Header detection: treat the first row as headers
            let headers = grid[0].clone();
            let data = grid[1..].to_vec();

            overall_table_index += 1;

            let table = ExtractedTable {
                page: page_num,
                table_index: overall_table_index,
                bbox: [
                    merged_intervals.first().map(|i| i.0).unwrap_or(0.0),
                    rows.last().map(|r| r[0].bbox.y).unwrap_or(0.0),
                    merged_intervals.last().map(|i| i.1).unwrap_or(0.0)
                        - merged_intervals.first().map(|i| i.0).unwrap_or(0.0),
                    rows.first().map(|r| r[0].bbox.y).unwrap_or(0.0)
                        - rows.last().map(|r| r[0].bbox.y).unwrap_or(0.0),
                ],
                rows: data.len(),
                cols: num_cols,
                headers,
                data,
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
}
