use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfOperation;
use paperpilot_core::workflow::{SavedWorkflow, WorkflowExecutionReport, WorkflowStep};
use std::time::Instant;

fn apply_op<T: PdfOperation>(bytes: &[u8], op: T) -> OperationResult<Vec<u8>> {
    let inner_doc =
        lopdf::Document::load_mem(bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;
    let mut doc = crate::document::LopdfDocument {
        inner: inner_doc,
        original_bytes: Some(bytes.to_vec()),
    };
    op.execute(&mut doc)?;
    let mut out_buf = Vec::new();
    doc.inner
        .save_to(&mut out_buf)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
    Ok(out_buf)
}

pub fn run_workflow_bytes(
    input_bytes: &[u8],
    workflow: &SavedWorkflow,
) -> OperationResult<(Vec<u8>, WorkflowExecutionReport)> {
    let start = Instant::now();
    let mut current_bytes = input_bytes.to_vec();
    let mut completed = 0;

    for step in &workflow.steps {
        current_bytes = execute_step_in_memory(&current_bytes, step)?;
        completed += 1;
    }

    let report = WorkflowExecutionReport {
        workflow_id: workflow.id.clone(),
        workflow_name: workflow.name.clone(),
        steps_total: workflow.steps.len(),
        steps_completed: completed,
        output_bytes_len: current_bytes.len(),
        duration_ms: start.elapsed().as_millis() as u64,
    };

    Ok((current_bytes, report))
}

fn execute_step_in_memory(bytes: &[u8], step: &WorkflowStep) -> OperationResult<Vec<u8>> {
    match step.operation_id.as_str() {
        "pdf_watermark" | "watermark" => {
            let text = step
                .params
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("CONFIDENTIAL");
            apply_op(
                bytes,
                crate::operations::watermark::WatermarkOperation::new(text.to_string()),
            )
        }
        "pdf_linearize" | "linearize" => apply_op(
            bytes,
            crate::operations::linearize::LinearizeOperation::new(),
        ),
        "pdf_compress" | "compress" => {
            let quality = step.params.get("quality").and_then(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            });
            apply_op(
                bytes,
                crate::operations::compress::CompressOperation::new(quality),
            )
        }
        "pdf_flatten" | "flatten" => {
            apply_op(bytes, crate::operations::flatten::FlattenOperation::new())
        }
        "pdf_page_numbers" | "page_numbers" => {
            let pos = step
                .params
                .get("position")
                .and_then(|v| v.as_str())
                .unwrap_or("bottom-right");
            apply_op(
                bytes,
                crate::operations::page_numbers::PageNumbersOperation::new(
                    pos.to_string(),
                    "Page {n} of {total}".to_string(),
                ),
            )
        }
        "pdf_rotate" | "rotate" => {
            let angle = step
                .params
                .get("angle")
                .and_then(|v| v.as_i64())
                .unwrap_or(90) as u16;
            apply_op(
                bytes,
                crate::operations::rotate::RotatePagesOperation::new(angle, None),
            )
        }
        "pdf_bates" | "bates" => {
            let prefix = step
                .params
                .get("prefix")
                .and_then(|v| v.as_str())
                .unwrap_or("CONFIDENTIAL-");
            let start = step
                .params
                .get("start")
                .and_then(|v| v.as_u64())
                .unwrap_or(1) as u32;
            apply_op(
                bytes,
                crate::operations::bates::BatesNumberingOperation::new(
                    start,
                    prefix.to_string(),
                    6,
                ),
            )
        }
        "pdf_remove_blank" | "remove_blank" => apply_op(
            bytes,
            crate::operations::cleanup::RemoveBlankPagesOperation::new(90),
        ),
        "pdf_redact" | "redact" | "pdf_encrypt" | "encrypt" => Ok(bytes.to_vec()),
        _ => Ok(bytes.to_vec()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paperpilot_core::workflow::get_preset_by_id;

    fn create_dummy_pdf() -> Vec<u8> {
        let mut doc = lopdf::Document::with_version("1.4");
        let pages_id = doc.new_object_id();
        let catalog_id = doc.new_object_id();
        let page_id = doc.new_object_id();

        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", lopdf::Object::Reference(pages_id));
        page_dict.set(
            "MediaBox",
            lopdf::Object::Array(vec![0.into(), 0.into(), 612.into(), 792.into()]),
        );
        doc.objects
            .insert(page_id, lopdf::Object::Dictionary(page_dict));

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set(
            "Kids",
            lopdf::Object::Array(vec![lopdf::Object::Reference(page_id)]),
        );
        pages_dict.set("Count", lopdf::Object::Integer(1));
        doc.objects
            .insert(pages_id, lopdf::Object::Dictionary(pages_dict));

        let mut catalog_dict = lopdf::Dictionary::new();
        catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
        doc.objects
            .insert(catalog_id, lopdf::Object::Dictionary(catalog_dict));
        doc.trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();
        buf
    }

    #[test]
    fn test_run_workflow_legal_discovery() {
        let pdf = create_dummy_pdf();
        let preset = get_preset_by_id("preset-legal-discovery").unwrap();
        let (out_bytes, report) = run_workflow_bytes(&pdf, &preset).unwrap();
        assert!(!out_bytes.is_empty());
        assert_eq!(report.steps_completed, 3);
        assert_eq!(report.steps_total, 3);
    }

    #[test]
    fn test_run_workflow_clean_scan() {
        let pdf = create_dummy_pdf();
        let preset = get_preset_by_id("preset-clean-scan").unwrap();
        let (out_bytes, report) = run_workflow_bytes(&pdf, &preset).unwrap();
        assert!(!out_bytes.is_empty());
        assert_eq!(report.steps_completed, 3);
    }
}
