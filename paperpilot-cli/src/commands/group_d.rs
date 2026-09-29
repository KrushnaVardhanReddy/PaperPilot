use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;
use std::path::Path;

pub fn handle_convert(format: &str, input: &Path, output: Option<&Path>) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;

    match format {
        "html" => {
            let out_path = output.unwrap_or(Path::new("output.html"));
            let op = paperpilot_pdf::operations::conversion::PdfToHtmlOperation::new();
            op.execute(&mut doc)?;
            let html = op
                .extracted_html
                .lock()
                .unwrap()
                .clone()
                .unwrap_or_default();
            std::fs::write(out_path, html)
                .map_err(|e| paperpilot_core::error::PdfError::Other(e.to_string()))?;
        }
        "md" => {
            let out_path = output.unwrap_or(Path::new("output.md"));
            let op = paperpilot_pdf::operations::conversion::PdfToMarkdownOperation::new();
            op.execute(&mut doc)?;
            let md = op
                .extracted_markdown
                .lock()
                .unwrap()
                .clone()
                .unwrap_or_default();
            std::fs::write(out_path, md)
                .map_err(|e| paperpilot_core::error::PdfError::Other(e.to_string()))?;
        }
        "llm" => {
            let base_path = output.unwrap_or(Path::new("output"));
            let mut md_path = base_path.to_path_buf();
            md_path.set_extension("md");
            let mut json_path = base_path.to_path_buf();
            json_path.set_extension("json");

            let op = paperpilot_pdf::operations::conversion::PdfToLlmExportOperation::new();
            op.execute(&mut doc)?;

            let md = op
                .extracted_markdown
                .lock()
                .unwrap()
                .clone()
                .unwrap_or_default();
            std::fs::write(md_path, md)
                .map_err(|e| paperpilot_core::error::PdfError::Other(e.to_string()))?;

            if let Some(meta) = op.metadata.lock().unwrap().clone() {
                let json = serde_json::to_string_pretty(&meta).unwrap_or_default();
                std::fs::write(json_path, json)
                    .map_err(|e| paperpilot_core::error::PdfError::Other(e.to_string()))?;
            }
        }
        "json" => {
            let out_path = output.unwrap_or(Path::new("output.json"));
            let doc_name = input
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let op = paperpilot_pdf::operations::conversion::PdfToJsonOperation::new(doc_name);
            op.execute(&mut doc)?;
            let json = op
                .extracted_json
                .lock()
                .unwrap()
                .clone()
                .unwrap_or_default();
            std::fs::write(out_path, json)
                .map_err(|e| paperpilot_core::error::PdfError::Other(e.to_string()))?;
        }
        "docx" => {
            let out_path = output.unwrap_or(Path::new("output.docx"));
            let op = paperpilot_pdf::operations::conversion::PdfToDocxOperation::new(out_path.to_path_buf());
            op.execute(&mut doc)?;
        }
        "xlsx" => {
            let out_path = output.unwrap_or(Path::new("output.xlsx"));
            let op = paperpilot_pdf::operations::conversion::PdfToXlsxOperation::new(out_path.to_path_buf());
            op.execute(&mut doc)?;
        }
        "pptx" => {
            let out_path = output.unwrap_or(Path::new("output.pptx"));
            let op = paperpilot_pdf::operations::conversion::PdfToPptxOperation::new(out_path.to_path_buf());
            op.execute(&mut doc)?;
        }
        _ => {
            return Err(paperpilot_core::error::PdfError::Other(
                "Invalid format specified".to_string(),
            ));
        }
    }

    Ok(())
}



pub fn handle_classify(input: &Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::classify::PdfClassifyOperation::new();
    op.execute(&mut doc)?;

    let res = op.classification.lock().unwrap().clone();
    if let Some(c) = res {
        let json = serde_json::to_string_pretty(&c).unwrap_or_default();
        println!("{}", json);
    }

    Ok(())
}
