use paperpilot_core::error::PdfError;
use paperpilot_pdf::operations::table::{TableExtractOptions, extract_tables_bytes};
use std::fs;
use std::path::PathBuf;

pub fn execute(
    input: PathBuf,
    output: Option<PathBuf>,
    format: String,
    pages: String,
) -> Result<(), PdfError> {
    let input_bytes = fs::read(&input).map_err(|e| PdfError::IoError(e))?;

    let options = TableExtractOptions {
        format: format.clone(),
        pages,
        ..Default::default()
    };

    let result = extract_tables_bytes(&input_bytes, &options)?;

    let out_str = if format.to_lowercase() == "csv" {
        result.csv_output.unwrap_or_default()
    } else {
        serde_json::to_string_pretty(&result).map_err(|e| PdfError::ParseError(e.to_string()))?
    };

    if let Some(out_path) = output {
        fs::write(&out_path, out_str).map_err(|e| PdfError::IoError(e))?;
        println!("Extracted {} tables to {:?}", result.total_tables, out_path);
    } else {
        println!("{}", out_str);
    }

    Ok(())
}
