use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::operations::form::{CreateFormFieldOperation, FillFormOperation, ReadFormOperation};
use std::path::Path;
use std::collections::HashMap;
use std::fs;

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
    let data_str = fs::read_to_string(data)
        .map_err(PdfError::IoError)?;

    let values: HashMap<String, String> = serde_json::from_str(&data_str)
        .map_err(|e| PdfError::InvalidInput(e.to_string()))?;

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
        return Err(PdfError::InvalidInput("Rect must be in format 'llx,lly,urx,ury'".to_string()));
    }

    let parse_f32 = |s: &str| -> OperationResult<f32> {
        s.trim().parse::<f32>().map_err(|_| PdfError::InvalidInput(format!("Invalid float: {}", s)))
    };

    let rect_arr = [
        parse_f32(rect_parts[0])?,
        parse_f32(rect_parts[1])?,
        parse_f32(rect_parts[2])?,
        parse_f32(rect_parts[3])?,
    ];

    let mut doc = LopdfDocument::load(input)?;
    let op = CreateFormFieldOperation::new(name.to_string(), field_type.to_string(), page, rect_arr);
    op.execute(&mut doc)?;
    doc.save(output)?;

    Ok(())
}
