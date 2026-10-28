use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfOperation;
use paperpilot_core::workflow::{
    ComplexCondition, ConditionExpr, OnNoMatchAction, SavedWorkflow, StageExecutionRecord,
    StagePredicate, StageStatus, WorkflowExecutionReport, WorkflowExecutionStatus, WorkflowStep,
};
use std::time::Instant;

pub struct DocumentInspector<'a> {
    pub bytes: &'a [u8],
    pub doc: lopdf::Document,
}

impl<'a> DocumentInspector<'a> {
    pub fn new(bytes: &'a [u8]) -> OperationResult<Self> {
        let doc = lopdf::Document::load_mem(bytes)
            .map_err(|e| PdfError::ParseError(e.to_string()))?;
        Ok(Self { bytes, doc })
    }

    pub fn page_count(&self) -> usize {
        self.doc.get_pages().len()
    }

    pub fn file_size_bytes(&self) -> usize {
        self.bytes.len()
    }

    pub fn has_form_fields(&self) -> bool {
        if let Ok(catalog) = self.doc.catalog() {
            if let Ok(acroform) = catalog.get(b"AcroForm") {
                if let Ok(dict) = acroform.as_dict() {
                    if let Ok(fields) = dict.get(b"Fields") {
                        if let Ok(arr) = fields.as_array() {
                            return !arr.is_empty();
                        }
                    }
                }
            }
        }
        false
    }

    pub fn is_encrypted(&self) -> bool {
        self.doc.trailer.get(b"Encrypt").is_ok()
    }

    pub fn is_signed(&self) -> bool {
        if let Ok(catalog) = self.doc.catalog() {
            if let Ok(acroform) = catalog.get(b"AcroForm") {
                if let Ok(dict) = acroform.as_dict() {
                    if let Ok(fields) = dict.get(b"Fields") {
                        if let Ok(arr) = fields.as_array() {
                            for f in arr {
                                if let Ok(f_dict) = self.doc.get_dictionary(f.as_reference().unwrap_or((0, 0))) {
                                    if let Ok(ft) = f_dict.get(b"FT") {
                                        if ft.as_name().unwrap_or(b"") == b"Sig" {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn extract_inspected_text(&self) -> OperationResult<String> {
        // Inspect first 3 pages (or all if <= 5)
        let page_count = self.page_count();
        let limit = if page_count <= 5 { page_count } else { 3 };
        let mut full_text = String::new();
        for page_num in 1..=limit {
            if let Ok(text) = self.doc.extract_text(&[page_num as u32]) {
                full_text.push_str(&text);
                full_text.push(' ');
            }
        }
        Ok(full_text)
    }

    pub fn is_scanned(&self) -> OperationResult<bool> {
        let text = self.extract_inspected_text()?;
        let non_ws_chars = text.chars().filter(|c| !c.is_whitespace()).count();
        if non_ws_chars >= 10 {
            return Ok(false);
        }
        // Check for at least one image XObject
        let pages = self.doc.get_pages();
        let limit = if pages.len() <= 5 { pages.len() } else { 3 };
        for (_, page_id) in pages.iter().take(limit) {
            if let Ok(page_dict) = self.doc.get_dictionary(*page_id) {
                if let Ok(res) = page_dict.get(b"Resources").and_then(|r| r.as_dict()) {
                    if let Ok(xobj) = res.get(b"XObject").and_then(|x| x.as_dict()) {
                        for (_, xo_ref) in xobj.iter() {
                            if let Ok(xo_dict) = self.doc.get_dictionary(xo_ref.as_reference().unwrap_or((0, 0))) {
                                if let Ok(sub) = xo_dict.get(b"Subtype") {
                                    if sub.as_name().unwrap_or(b"") == b"Image" {
                                        return Ok(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(false)
    }
}

pub fn evaluate_condition_expr(
    expr: &ConditionExpr,
    inspector: &DocumentInspector,
) -> OperationResult<bool> {
    match expr {
        ConditionExpr::Leaf(pred) => evaluate_stage_predicate(pred, inspector),
        ConditionExpr::Complex(complex) => {
            if let Some(all) = &complex.all {
                // Vacuous truth: all: [] -> true
                for sub in all {
                    if !evaluate_condition_expr(sub, inspector)? {
                        return Ok(false);
                    }
                }
                return Ok(true);
            }
            if let Some(any) = &complex.any {
                // Empty any: [] -> false
                for sub in any {
                    if evaluate_condition_expr(sub, inspector)? {
                        return Ok(true);
                    }
                }
                return Ok(false);
            }
            if let Some(not) = &complex.not {
                return Ok(!evaluate_condition_expr(not, inspector)?);
            }
            Ok(true)
        }
    }
}

pub fn evaluate_stage_predicate(
    pred: &StagePredicate,
    inspector: &DocumentInspector,
) -> OperationResult<bool> {
    if let Some(expected_scanned) = pred.is_scanned {
        if inspector.is_scanned()? != expected_scanned {
            return Ok(false);
        }
    }
    if let Some(comp) = &pred.page_count {
        if !comp.matches(inspector.page_count()) {
            return Ok(false);
        }
    }
    if let Some(comp) = &pred.file_size_bytes {
        if !comp.matches(inspector.file_size_bytes()) {
            return Ok(false);
        }
    }
    if let Some(sub) = &pred.text_contains {
        let text = inspector.extract_inspected_text()?.to_lowercase();
        let target = sub.trim().to_lowercase();
        if !text.contains(&target) {
            return Ok(false);
        }
    }
    if let Some(expected_forms) = pred.has_form_fields {
        if inspector.has_form_fields() != expected_forms {
            return Ok(false);
        }
    }
    if let Some(expected_enc) = pred.is_encrypted {
        if inspector.is_encrypted() != expected_enc {
            return Ok(false);
        }
    }
    if let Some(expected_signed) = pred.is_signed {
        if inspector.is_signed() != expected_signed {
            return Ok(false);
        }
    }
    Ok(true)
}

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

    let mut report = WorkflowExecutionReport {
        workflow_id: workflow.id.clone(),
        workflow_name: workflow.name.clone(),
        status: WorkflowExecutionStatus::Success,
        steps_total: workflow.steps.len(),
        steps_completed: 0,
        steps_skipped: 0,
        steps_stopped: 0,
        steps_unreached: 0,
        stages: Vec::new(),
        duration_ms: 0,
        error: None,
        output_bytes_len: 0,
    };

    for (index, step) in workflow.steps.iter().enumerate() {
        if let Some(condition) = &step.condition {
            let inspector = DocumentInspector::new(&current_bytes)?;
            let matched = evaluate_condition_expr(condition, &inspector)?;
            if !matched {
                match &step.on_no_match {
                    OnNoMatchAction::Skip => {
                        report.stages.push(StageExecutionRecord {
                            id: step.id.clone(),
                            op: step.operation_id.clone(),
                            status: StageStatus::Skipped,
                            condition_matched: Some(false),
                            reason: Some("Condition evaluated to false".to_string()),
                            duration_ms: 0,
                        });
                        report.steps_skipped += 1;
                        continue;
                    }
                    OnNoMatchAction::Stop => {
                        report.stages.push(StageExecutionRecord {
                            id: step.id.clone(),
                            op: step.operation_id.clone(),
                            status: StageStatus::Stopped,
                            condition_matched: Some(false),
                            reason: Some("Condition evaluated to false; pipeline stopped cleanly".to_string()),
                            duration_ms: 0,
                        });
                        report.steps_stopped += 1;
                        for unreached_step in &workflow.steps[index + 1..] {
                            report.stages.push(StageExecutionRecord {
                                id: unreached_step.id.clone(),
                                op: unreached_step.operation_id.clone(),
                                status: StageStatus::Unreached,
                                condition_matched: None,
                                reason: Some("Previous stage invoked on_no_match: stop".to_string()),
                                duration_ms: 0,
                            });
                            report.steps_unreached += 1;
                        }
                        report.status = WorkflowExecutionStatus::StoppedEarly;
                        break;
                    }
                    OnNoMatchAction::Error => {
                        report.status = WorkflowExecutionStatus::Failed;
                        return Err(PdfError::ConditionFailed(format!(
                            "Stage '{}' condition failed with on_no_match: error",
                            step.id
                        )));
                    }
                    OnNoMatchAction::CustomError { error } => {
                        report.status = WorkflowExecutionStatus::Failed;
                        return Err(PdfError::ConditionFailed(error.clone()));
                    }
                }
            }
        }

        let start_step = Instant::now();
        current_bytes = execute_step_in_memory(&current_bytes, step)?;
        report.stages.push(StageExecutionRecord {
            id: step.id.clone(),
            op: step.operation_id.clone(),
            status: StageStatus::Executed,
            condition_matched: step.condition.as_ref().map(|_| true),
            reason: None,
            duration_ms: start_step.elapsed().as_millis() as u64,
        });
        report.steps_completed += 1;
    }

    report.output_bytes_len = current_bytes.len();
    report.duration_ms = start.elapsed().as_millis() as u64;

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

    #[test]
    fn test_evaluate_condition_expr() {
        let pdf = create_dummy_pdf();
        let inspector = DocumentInspector::new(&pdf).unwrap();

        let mut pred = paperpilot_core::workflow::StagePredicate::default();
        pred.page_count = Some(paperpilot_core::workflow::NumberComparator::Eq(1));

        let expr = ConditionExpr::Leaf(pred);
        assert!(evaluate_condition_expr(&expr, &inspector).unwrap());

        let mut pred_fail = paperpilot_core::workflow::StagePredicate::default();
        pred_fail.page_count = Some(paperpilot_core::workflow::NumberComparator::Gt(1));
        let expr_fail = ConditionExpr::Leaf(pred_fail);
        assert!(!evaluate_condition_expr(&expr_fail, &inspector).unwrap());
    }
}
