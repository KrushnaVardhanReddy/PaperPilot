use calamine::{Data, Reader, open_workbook_auto};
use office_oxide::Document;
use paperpilot_core::error::{OperationResult, PdfError};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

pub struct ExcelToJsonOperation;

impl ExcelToJsonOperation {
    pub fn execute(
        input_path: &Path,
        sheet_name: Option<&str>,
        has_headers: bool,
    ) -> OperationResult<String> {
        let mut workbook = open_workbook_auto(input_path).map_err(|e| {
            PdfError::IoError(std::io::Error::other(format!(
                "Failed to open workbook: {}",
                e
            )))
        })?;

        let sheet_names = workbook.sheet_names().to_vec();
        let target_sheets: Vec<String> = match sheet_name {
            Some(name) => {
                if !sheet_names.contains(&name.to_string()) {
                    return Err(PdfError::IoError(std::io::Error::other(format!(
                        "Sheet not found: {}",
                        name
                    ))));
                }
                vec![name.to_string()]
            }
            None => sheet_names,
        };

        let mut sheets_data = Vec::new();

        for name in target_sheets {
            if let Ok(range) = workbook.worksheet_range(&name) {
                let mut rows_iter = range.rows();
                let mut columns: Vec<String> = Vec::new();
                let mut data: Vec<Value> = Vec::new();

                if has_headers {
                    if let Some(header_row) = rows_iter.next() {
                        for (i, cell) in header_row.iter().enumerate() {
                            let header = match cell {
                                Data::String(s) => s.to_string(),
                                Data::Float(f) => f.to_string(),
                                Data::Int(i) => i.to_string(),
                                Data::Bool(b) => b.to_string(),
                                Data::DateTime(d) => d.as_f64().to_string(),
                                Data::Error(e) => format!("Error({})", e),
                                Data::Empty => format!("Column{}", i),
                                _ => format!("Column{}", i),
                            };
                            columns.push(header);
                        }
                    }
                    for row in rows_iter {
                        let mut row_obj = serde_json::Map::new();
                        for (i, cell) in row.iter().enumerate() {
                            let key = if i < columns.len() {
                                columns[i].clone()
                            } else {
                                format!("Column{}", i)
                            };
                            let val = match cell {
                                Data::String(s) => Value::String(s.to_string()),
                                Data::Float(f) => Value::Number(
                                    serde_json::Number::from_f64(*f)
                                        .unwrap_or_else(|| serde_json::Number::from(0)),
                                ),
                                Data::Int(i) => Value::Number(serde_json::Number::from(*i)),
                                Data::Bool(b) => Value::Bool(*b),
                                Data::DateTime(d) => Value::Number(
                                    serde_json::Number::from_f64(d.as_f64())
                                        .unwrap_or_else(|| serde_json::Number::from(0)),
                                ),
                                Data::Error(e) => Value::String(format!("Error({})", e)),
                                Data::Empty => Value::Null,
                                _ => Value::Null,
                            };
                            row_obj.insert(key, val);
                        }
                        data.push(Value::Object(row_obj));
                    }
                } else {
                    for row in rows_iter {
                        let mut row_arr = Vec::new();
                        for cell in row.iter() {
                            let val = match cell {
                                Data::String(s) => Value::String(s.to_string()),
                                Data::Float(f) => Value::Number(
                                    serde_json::Number::from_f64(*f)
                                        .unwrap_or_else(|| serde_json::Number::from(0)),
                                ),
                                Data::Int(i) => Value::Number(serde_json::Number::from(*i)),
                                Data::Bool(b) => Value::Bool(*b),
                                Data::DateTime(d) => Value::Number(
                                    serde_json::Number::from_f64(d.as_f64())
                                        .unwrap_or_else(|| serde_json::Number::from(0)),
                                ),
                                Data::Error(e) => Value::String(format!("Error({})", e)),
                                Data::Empty => Value::Null,
                                _ => Value::Null,
                            };
                            row_arr.push(val);
                        }
                        data.push(Value::Array(row_arr));
                    }
                }

                sheets_data.push(json!({
                    "name": name,
                    "rows_count": range.get_size().0,
                    "columns": columns,
                    "data": data,
                }));
            }
        }

        let result = json!({ "sheets": sheets_data });
        Ok(result.to_string())
    }
}

pub struct ExcelToCsvOperation;

impl ExcelToCsvOperation {
    pub fn execute(
        input_path: &Path,
        output_path: &Path,
        sheet_name: Option<&str>,
    ) -> OperationResult<()> {
        let mut workbook = open_workbook_auto(input_path).map_err(|e| {
            PdfError::IoError(std::io::Error::other(format!(
                "Failed to open workbook: {}",
                e
            )))
        })?;

        let sheet_names = workbook.sheet_names().to_vec();
        if sheet_names.is_empty() {
            return Err(PdfError::IoError(std::io::Error::other(
                "No sheets found in workbook",
            )));
        }

        // Helper to export a single sheet to a destination path
        let export_sheet = |wb: &mut calamine::Sheets<std::io::BufReader<std::fs::File>>,
                            sheet: &str,
                            dest: &Path|
         -> OperationResult<()> {
            if let Ok(range) = wb.worksheet_range(sheet) {
                let mut wtr = csv::Writer::from_path(dest).map_err(|e| {
                    PdfError::IoError(std::io::Error::other(format!(
                        "Failed to create csv writer: {}",
                        e
                    )))
                })?;

                for row in range.rows() {
                    let mut record = Vec::new();
                    for cell in row.iter() {
                        let s = match cell {
                            Data::String(s) => s.to_string(),
                            Data::Float(f) => f.to_string(),
                            Data::Int(i) => i.to_string(),
                            Data::Bool(b) => b.to_string(),
                            Data::DateTime(d) => d.as_f64().to_string(),
                            Data::Error(e) => format!("Error({})", e),
                            Data::Empty => "".to_string(),
                            _ => "".to_string(),
                        };
                        record.push(s);
                    }
                    wtr.write_record(&record).map_err(|e| {
                        PdfError::IoError(std::io::Error::other(format!(
                            "Failed to write csv record: {}",
                            e
                        )))
                    })?;
                }
                wtr.flush().map_err(|e| {
                    PdfError::IoError(std::io::Error::other(format!("Failed to flush csv: {}", e)))
                })?;
                Ok(())
            } else {
                Err(PdfError::IoError(std::io::Error::other(format!(
                    "Failed to read sheet: {}",
                    sheet
                ))))
            }
        };

        if let Some(target_sheet) = sheet_name {
            // Specific sheet requested
            export_sheet(&mut workbook, target_sheet, output_path)?;
        } else if sheet_names.len() == 1 {
            // Single sheet: write directly to output_path
            export_sheet(&mut workbook, &sheet_names[0], output_path)?;
        } else {
            // Multiple sheets: write output_path for first sheet, plus filename_<sheet>.csv for each sheet
            let parent_dir = output_path.parent().unwrap_or_else(|| Path::new("."));
            let stem = output_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let ext = output_path
                .extension()
                .map(|e| e.to_string_lossy())
                .unwrap_or_else(|| "csv".into());

            for (idx, s_name) in sheet_names.iter().enumerate() {
                // Sanitize sheet name for filenames (replace spaces and special chars with underscores)
                let safe_sheet_name: String = s_name
                    .chars()
                    .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
                    .collect();

                let sheet_dest = parent_dir.join(format!("{}_{}.{}", stem, safe_sheet_name, ext));
                export_sheet(&mut workbook, s_name, &sheet_dest)?;

                // Also populate primary output_path with the first sheet for caller compatibility
                if idx == 0 && sheet_dest != output_path {
                    let _ = std::fs::copy(&sheet_dest, output_path);
                }
            }
        }

        Ok(())
    }
}

pub struct OfficeToMarkdownOperation;

impl OfficeToMarkdownOperation {
    pub fn execute(input_path: &Path, output_path: &Path) -> OperationResult<()> {
        let md = office_oxide::to_markdown(input_path)
            .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
        fs::write(output_path, md).map_err(PdfError::IoError)?;
        Ok(())
    }
}

pub struct OfficeExtractTextOperation;

impl OfficeExtractTextOperation {
    pub fn execute(input_path: &Path, output_path: &Path) -> OperationResult<()> {
        let text = office_oxide::extract_text(input_path)
            .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
        fs::write(output_path, text).map_err(PdfError::IoError)?;
        Ok(())
    }
}

pub struct OfficeModernizeOperation;

impl OfficeModernizeOperation {
    pub fn execute(input_path: &Path, output_path: &Path) -> OperationResult<()> {
        let doc = Document::open(input_path)
            .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
        doc.save_as(output_path)
            .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use rust_xlsxwriter::Workbook;
    use tempfile::NamedTempFile;

    fn create_test_excel() -> NamedTempFile {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        worksheet.write_string(0, 0, "Header1").unwrap();
        worksheet.write_string(0, 1, "Header2").unwrap();
        worksheet.write_string(1, 0, "Value1").unwrap();
        worksheet.write_number(1, 1, 42.0).unwrap();
        let worksheet2 = workbook.add_worksheet().set_name("Sheet2").unwrap();
        worksheet2.write_string(0, 0, "OnlyOne").unwrap();

        let file = NamedTempFile::new().unwrap();
        workbook.save(file.path()).unwrap();
        file
    }

    #[test]
    fn test_excel_to_json_success() {
        let file = create_test_excel();
        let res = ExcelToJsonOperation::execute(file.path(), None, true);
        assert!(res.is_ok());
        let json_str = res.unwrap();
        let v: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(v["sheets"].as_array().unwrap().len(), 2);
        assert_eq!(v["sheets"][0]["name"], "Sheet1");
        assert_eq!(v["sheets"][0]["data"][0]["Header1"], "Value1");
        assert_eq!(v["sheets"][0]["data"][0]["Header2"], 42.0);
    }

    #[test]
    fn test_excel_to_json_invalid_file() {
        let res = ExcelToJsonOperation::execute(Path::new("does_not_exist.xlsx"), None, true);
        assert!(res.is_err());
        match res {
            Err(PdfError::IoError(_)) => {}
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_excel_to_csv_success() {
        let file = create_test_excel();
        let out_file = NamedTempFile::new().unwrap();
        let res = ExcelToCsvOperation::execute(file.path(), out_file.path(), Some("Sheet1"));
        assert!(res.is_ok());
        let content = std::fs::read_to_string(out_file.path()).unwrap();
        assert!(content.contains("Header1,Header2"));
        assert!(content.contains("Value1,42"));
    }

    #[test]
    fn test_excel_to_csv_multi_sheet() {
        let file = create_test_excel(); // has Sheet1 and Sheet2
        let temp_dir = tempfile::tempdir().unwrap();
        let out_path = temp_dir.path().join("report.csv");
        let res = ExcelToCsvOperation::execute(file.path(), &out_path, None);
        assert!(res.is_ok());

        // Verify primary report.csv has Sheet1
        let content_primary = std::fs::read_to_string(&out_path).unwrap();
        assert!(content_primary.contains("Header1,Header2"));

        // Verify report_Sheet1.csv exists
        let sheet1_file = temp_dir.path().join("report_Sheet1.csv");
        assert!(sheet1_file.exists());
        let content_sheet1 = std::fs::read_to_string(sheet1_file).unwrap();
        assert!(content_sheet1.contains("Header1,Header2"));

        // Verify report_Sheet2.csv exists with Sheet2 data
        let sheet2_file = temp_dir.path().join("report_Sheet2.csv");
        assert!(sheet2_file.exists());
        let content_sheet2 = std::fs::read_to_string(sheet2_file).unwrap();
        assert!(content_sheet2.contains("OnlyOne"));
    }

    #[test]
    fn test_excel_to_csv_invalid_file() {
        let out_file = NamedTempFile::new().unwrap();
        let res =
            ExcelToCsvOperation::execute(Path::new("does_not_exist.xlsx"), out_file.path(), None);
        assert!(res.is_err());
        match res {
            Err(PdfError::IoError(_)) => {}
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_office_to_markdown_invalid_file() {
        let res = OfficeToMarkdownOperation::execute(
            &PathBuf::from("does_not_exist.docx"),
            &PathBuf::from("out.md"),
        );
        assert!(res.is_err());
        match res {
            Err(PdfError::IoError(_)) => {}
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_office_extract_text_invalid_file() {
        let res = OfficeExtractTextOperation::execute(
            &PathBuf::from("does_not_exist.docx"),
            &PathBuf::from("out.txt"),
        );
        assert!(res.is_err());
        match res {
            Err(PdfError::IoError(_)) => {}
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_office_modernize_invalid_file() {
        let res = OfficeModernizeOperation::execute(
            &PathBuf::from("does_not_exist.doc"),
            &PathBuf::from("out.docx"),
        );
        assert!(res.is_err());
        match res {
            Err(PdfError::IoError(_)) => {}
            _ => panic!("Expected IoError"),
        }
    }
}
