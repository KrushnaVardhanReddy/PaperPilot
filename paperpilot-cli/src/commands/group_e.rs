#![cfg(feature = "pdf")]
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::flatten::FlattenOperation;
use paperpilot_pdf::operations::form::{
    CreateFormFieldOperation, FillFormOperation, ReadFormOperation,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn resolve_pattern(pattern: &str, row: &HashMap<String, String>, index: usize) -> String {
    let now = chrono::Local::now();
    let date = now.format("%Y%m%d").to_string();
    let time = now.format("%H%M%S").to_string();
    let mut result = pattern.to_string();
    result = result.replace("{_index}", &format!("{:04}", index));
    result = result.replace("{_row}", &(index + 1).to_string());
    result = result.replace("{_date}", &date);
    result = result.replace("{_time}", &time);
    for (k, v) in row {
        result = result.replace(&format!("{{{}}}", k), v);
    }
    result
}

pub fn handle_form_fill_batch(
    template: &Path,
    data: &Path,
    output_dir: &Path,
    name_pattern: &str,
    flatten: bool,
    fail_fast: bool,
    dry_run: bool,
) -> OperationResult<()> {
    let ext = data
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    let records: Vec<HashMap<String, String>> = if ext == "csv" {
        let mut rdr = csv::Reader::from_path(data)
            .map_err(|e| PdfError::InvalidInput(format!("CSV error: {}", e)))?;
        let headers = rdr
            .headers()
            .map_err(|e| PdfError::InvalidInput(format!("CSV error: {}", e)))?
            .clone();
        let mut rows = Vec::new();
        for record in rdr.records() {
            let record = record.map_err(|e| PdfError::InvalidInput(format!("CSV error: {}", e)))?;
            let mut row = HashMap::new();
            for (i, value) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    row.insert(header.to_string(), value.to_string());
                }
            }
            rows.push(row);
        }
        rows
    } else if ext == "json" {
        let data_str = fs::read_to_string(data).map_err(PdfError::IoError)?;
        serde_json::from_str(&data_str).map_err(|e| PdfError::InvalidInput(e.to_string()))?
    } else {
        return Err(PdfError::InvalidInput(
            "Unsupported data format. Please use .csv or .json".to_string(),
        ));
    };

    if !dry_run {
        fs::create_dir_all(output_dir).map_err(PdfError::IoError)?;
    }

    let mut written = 0usize;
    for (index, row) in records.iter().enumerate() {
        let filename = resolve_pattern(name_pattern, row, index);
        let output_path = output_dir.join(&filename);

        if dry_run {
            println!("[dry-run] would write: {}", output_path.display());
            continue;
        }

        let result = (|| -> OperationResult<()> {
            let mut doc = LopdfDocument::load(template)?;
            let op = FillFormOperation::new(row.clone());
            op.execute(&mut doc)?;
            if flatten {
                FlattenOperation::new().execute(&mut doc)?;
            }
            doc.save(&output_path)?;
            Ok(())
        })();

        match result {
            Ok(_) => written += 1,
            Err(e) => {
                if fail_fast {
                    return Err(e);
                }
                eprintln!("[warn] Skipping row {}: {}", index, e);
            }
        }
    }

    if !dry_run {
        println!("Wrote {} file(s) to {}", written, output_dir.display());
    }
    Ok(())
}

pub fn handle_form_read(input: &Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = ReadFormOperation::new();
    op.execute(&mut doc)?;

    let map = op.extracted_fields.lock().unwrap();
    if let Some(ref fields) = *map {
        let json = serde_json::to_string_pretty(fields)
            .map_err(|e| PdfError::InvalidInput(e.to_string()))?;
        println!("{}", json);
    } else {
        println!("{{}}");
    }

    Ok(())
}

pub fn handle_form_fill(input: &Path, data: &Path, output: &Path) -> OperationResult<()> {
    let data_str = fs::read_to_string(data).map_err(PdfError::IoError)?;

    let values: HashMap<String, String> =
        serde_json::from_str(&data_str).map_err(|e| PdfError::InvalidInput(e.to_string()))?;

    let mut doc = LopdfDocument::load(input)?;
    let op = FillFormOperation::new(values);
    op.execute(&mut doc)?;
    doc.save(output)?;

    Ok(())
}

pub fn handle_form_add_field(
    input: &Path,
    name: &str,
    field_type: &str,
    page: i32,
    rect: &str,
    output: &Path,
) -> OperationResult<()> {
    let rect_parts: Vec<&str> = rect.split(',').collect();
    if rect_parts.len() != 4 {
        return Err(PdfError::InvalidInput(
            "Rect must be in format 'llx,lly,urx,ury'".to_string(),
        ));
    }

    let parse_f32 = |s: &str| -> OperationResult<f32> {
        s.trim()
            .parse::<f32>()
            .map_err(|_| PdfError::InvalidInput(format!("Invalid float: {}", s)))
    };

    let rect_arr = [
        parse_f32(rect_parts[0])?,
        parse_f32(rect_parts[1])?,
        parse_f32(rect_parts[2])?,
        parse_f32(rect_parts[3])?,
    ];

    let mut doc = LopdfDocument::load(input)?;
    let op =
        CreateFormFieldOperation::new(name.to_string(), field_type.to_string(), page, rect_arr);
    op.execute(&mut doc)?;
    doc.save(output)?;

    Ok(())
}
