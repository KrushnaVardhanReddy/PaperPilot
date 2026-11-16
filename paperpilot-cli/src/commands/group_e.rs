#![cfg(feature = "pdf")]
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::form::{
    CreateFormFieldOperation, FillFormOperation, ReadFormOperation,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn handle_form_read(input: &Path, output: Option<&Path>) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = ReadFormOperation::new();
    op.execute(&mut doc)?;

    let map = op.extracted_fields.lock().unwrap();
    let json = if let Some(ref fields) = *map {
        serde_json::to_string_pretty(fields)
            .map_err(|e| PdfError::InvalidInput(e.to_string()))?
    } else {
        "{}".to_string()
    };

    if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(PdfError::IoError)?;
            }
        }
        fs::write(out_path, &json).map_err(PdfError::IoError)?;
    } else {
        println!("{}", json);
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
    label: Option<String>,
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
    let mut op =
        CreateFormFieldOperation::new(name.to_string(), field_type.to_string(), page, rect_arr);
    if let Some(lbl) = label {
        op = op.with_label(Some(lbl));
    }
    op.execute(&mut doc)?;
    doc.save(output)?;

    Ok(())
}
