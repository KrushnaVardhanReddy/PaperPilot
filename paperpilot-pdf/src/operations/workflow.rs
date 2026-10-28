use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfOperation;
use paperpilot_core::workflow::{SavedWorkflow, WorkflowExecutionReport, WorkflowStep, ConditionExpr, StagePredicate, NumberComparator, OnNoMatchAction};
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
    let mut report = WorkflowExecutionReport {
        workflow_id: workflow.id.clone(),
        workflow_name: workflow.name.clone(),
        status: paperpilot_core::workflow::WorkflowExecutionStatus::Success,
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
        if let Some(cond) = &step.condition {
            let inspector = DocumentInspector::new(&current_bytes);
            let matched = inspector.evaluate_condition(cond)?;
            if !matched {
                match &step.on_no_match {
                    OnNoMatchAction::Skip => {
                        report.stages.push(paperpilot_core::workflow::StageExecutionRecord {
                            id: step.id.clone(),
                            op: step.operation_id.clone(),
                            status: paperpilot_core::workflow::StageStatus::Skipped,
                            condition_matched: Some(false),
                            reason: Some("Condition evaluated to false; skipping stage".to_string()),
                            duration_ms: 0,
                        });
                        report.steps_skipped += 1;
                        continue;
                    }
                    OnNoMatchAction::Stop => {
                        report.stages.push(paperpilot_core::workflow::StageExecutionRecord {
                            id: step.id.clone(),
                            op: step.operation_id.clone(),
                            status: paperpilot_core::workflow::StageStatus::Stopped,
                            condition_matched: Some(false),
                            reason: Some("Condition evaluated to false; pipeline stopped cleanly".to_string()),
                            duration_ms: 0,
                        });
                        report.steps_stopped += 1;

                        for unreached_step in &workflow.steps[index + 1..] {
                            report.stages.push(paperpilot_core::workflow::StageExecutionRecord {
                                id: unreached_step.id.clone(),
                                op: unreached_step.operation_id.clone(),
                                status: paperpilot_core::workflow::StageStatus::Unreached,
                                condition_matched: None,
                                reason: Some("Previous stage invoked on_no_match: stop".to_string()),
                                duration_ms: 0,
                            });
                            report.steps_unreached += 1;
                        }
                        report.status = paperpilot_core::workflow::WorkflowExecutionStatus::StoppedEarly;
                        break;
                    }
                    OnNoMatchAction::Error => {
                        report.status = paperpilot_core::workflow::WorkflowExecutionStatus::Failed;
                        return Err(PdfError::ConditionFailed(format!(
                            "Stage '{}' condition failed with on_no_match: error",
                            step.id
                        )));
                    }
                    OnNoMatchAction::CustomError { error } => {
                        report.status = paperpilot_core::workflow::WorkflowExecutionStatus::Failed;
                        return Err(PdfError::ConditionFailed(error.clone()));
                    }
                }
            }
        }

        let step_start = Instant::now();
        current_bytes = execute_step_in_memory(&current_bytes, step)?;
        report.stages.push(paperpilot_core::workflow::StageExecutionRecord {
            id: step.id.clone(),
            op: step.operation_id.clone(),
            status: paperpilot_core::workflow::StageStatus::Executed,
            condition_matched: step.condition.as_ref().map(|_| true),
            reason: None,
            duration_ms: step_start.elapsed().as_millis() as u64,
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


pub struct DocumentInspector<'a> {
    bytes: &'a [u8],
    doc: Option<lopdf::Document>,
}

impl<'a> DocumentInspector<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let doc = lopdf::Document::load_mem(bytes).ok();
        Self { bytes, doc }
    }

    pub fn evaluate_condition(&self, expr: &ConditionExpr) -> OperationResult<bool> {
        match expr {
            ConditionExpr::Flat(pred) => self.evaluate_predicate(pred),
            ConditionExpr::Nested { all, any, not } => {
                if let Some(all_exprs) = all {
                    for e in all_exprs {
                        if !self.evaluate_condition(e)? {
                            return Ok(false);
                        }
                    }
                    return Ok(true);
                }
                if let Some(any_exprs) = any {
                    for e in any_exprs {
                        if self.evaluate_condition(e)? {
                            return Ok(true);
                        }
                    }
                    return Ok(false);
                }
                if let Some(not_expr) = not {
                    return Ok(!self.evaluate_condition(not_expr)?);
                }
                Ok(true) // Should not reach here due to serde but vacuous true
            }
        }
    }

    fn evaluate_predicate(&self, pred: &StagePredicate) -> OperationResult<bool> {
        let doc = self.doc.as_ref().ok_or_else(|| {
            PdfError::InspectionFailure("Failed to parse document for inspection".to_string())
        })?;

        if let Some(comp) = &pred.page_count {
            if !self.evaluate_comparator(comp, doc.get_pages().len() as f64) {
                return Ok(false);
            }
        }

        if let Some(comp) = &pred.file_size_bytes {
            if !self.evaluate_comparator(comp, self.bytes.len() as f64) {
                return Ok(false);
            }
        }

        if let Some(is_enc) = pred.is_encrypted {
            let has_enc = doc.trailer.get(b"Encrypt").is_ok();
            if has_enc != is_enc {
                return Ok(false);
            }
        }

        if let Some(has_forms) = pred.has_form_fields {
            let forms = self.check_has_form_fields(doc)?;
            if forms != has_forms {
                return Ok(false);
            }
        }

        if let Some(is_signed) = pred.is_signed {
            let signed = self.check_is_signed(doc)?;
            if signed != is_signed {
                return Ok(false);
            }
        }

        if let Some(is_scan) = pred.is_scanned {
            let scanned = self.check_is_scanned(doc)?;
            if scanned != is_scan {
                return Ok(false);
            }
        }

        if let Some(text) = &pred.text_contains {
            let text_lower = (*text).to_lowercase();
            let contains = self.check_text_contains(doc, &text_lower)?;
            if !contains {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn check_has_form_fields(&self, doc: &lopdf::Document) -> OperationResult<bool> {
        let catalog = doc.catalog().map_err(|e| PdfError::InspectionFailure(format!("Failed to resolve catalog: {}", e)))?;

        if let Ok(acro_ref) = catalog.get(b"AcroForm") {
            let acro = self.resolve_dict(doc, acro_ref).map_err(|e| PdfError::InspectionFailure(format!("Failed to resolve AcroForm: {}", e)))?;

            if let Ok(fields) = acro.get(b"Fields") {
                let arr = self.resolve_array(doc, fields).map_err(|e| PdfError::InspectionFailure(format!("Failed to resolve Fields: {}", e)))?;
                return Ok(!arr.is_empty());
            }
        }
        Ok(false)
    }

    fn check_is_signed(&self, doc: &lopdf::Document) -> OperationResult<bool> {
        let catalog = doc.catalog().map_err(|e| PdfError::InspectionFailure(format!("Failed to resolve catalog: {}", e)))?;

        if let Ok(acro_ref) = catalog.get(b"AcroForm") {
            let acro = self.resolve_dict(doc, acro_ref).map_err(|e| PdfError::InspectionFailure(format!("Failed to resolve AcroForm: {}", e)))?;

            if let Ok(fields) = acro.get(b"Fields") {
                let arr = self.resolve_array(doc, fields).map_err(|e| PdfError::InspectionFailure(format!("Failed to resolve Fields: {}", e)))?;

                for obj in arr {
                    let field_dict = self.resolve_dict(doc, obj).map_err(|e| PdfError::InspectionFailure(format!("Failed to resolve field dict: {}", e)))?;

                    if let Ok(ft) = field_dict.get(b"FT") {
                        if ft.as_name().map_or(false, |name| name == b"Sig") {
                            return Ok(true);
                        }
                    }
                }
            }
        }
        Ok(false)
    }

    fn resolve_dict<'d>(&self, doc: &'d lopdf::Document, obj: &'d lopdf::Object) -> OperationResult<&'d lopdf::Dictionary> {
        match obj {
            lopdf::Object::Dictionary(d) => Ok(d),
            lopdf::Object::Reference(id) => {
                let resolved = doc.get_object(*id).map_err(|e| PdfError::InspectionFailure(format!("Failed to resolve object {}: {}", id.0, e)))?;
                resolved.as_dict().map_err(|e| PdfError::InspectionFailure(format!("Resolved object is not a dictionary: {}", e)))
            }
            _ => Err(PdfError::InspectionFailure("Expected dictionary or reference to dictionary".to_string()))
        }
    }

    fn resolve_array<'d>(&self, doc: &'d lopdf::Document, obj: &'d lopdf::Object) -> OperationResult<&'d Vec<lopdf::Object>> {
        match obj {
            lopdf::Object::Array(a) => Ok(a),
            lopdf::Object::Reference(id) => {
                let resolved = doc.get_object(*id).map_err(|e| PdfError::InspectionFailure(format!("Failed to resolve object {}: {}", id.0, e)))?;
                resolved.as_array().map_err(|e| PdfError::InspectionFailure(format!("Resolved object is not an array: {}", e)))
            }
            _ => Err(PdfError::InspectionFailure("Expected array or reference to array".to_string()))
        }
    }

    fn evaluate_comparator(&self, comp: &NumberComparator, val: f64) -> bool {
        match comp {
            NumberComparator::Eq(v) => (val - v).abs() < f64::EPSILON,
            NumberComparator::Gt(v) => val > *v,
            NumberComparator::Gte(v) => val >= *v,
            NumberComparator::Lt(v) => val < *v,
            NumberComparator::Lte(v) => val <= *v,
            NumberComparator::Between(min, max) => val >= *min && val <= *max,
        }
    }

    fn check_is_scanned(&self, doc: &lopdf::Document) -> OperationResult<bool> {
        let pages = doc.get_pages();
        let max_pages = if pages.len() <= 5 { pages.len() } else { 3 };

        let mut char_count = 0;
        let mut has_image = false;

        for i in 1..=max_pages as u32 {
            let content = doc.extract_text(&[i]).map_err(|e| {
                PdfError::InspectionFailure(format!("Failed to extract text: {}", e))
            })?;
            char_count += content.chars().filter(|c| !c.is_whitespace()).count();

            if let Some(page_id) = pages.get(&i) {
                let obj = doc.get_object(*page_id).map_err(|e| PdfError::InspectionFailure(format!("Failed to get page: {}", e)))?;
                let page = obj.as_dict().map_err(|e| PdfError::InspectionFailure(format!("Page is not a dict: {}", e)))?;
                if let Ok(resources_obj) = page.get(b"Resources") {
                    let resources = resources_obj.as_dict().map_err(|e| PdfError::InspectionFailure(format!("Resources is not a dict: {}", e)))?;
                    if let Ok(xobjs_obj) = resources.get(b"XObject") {
                        let xobjs = xobjs_obj.as_dict().map_err(|e| PdfError::InspectionFailure(format!("XObject is not a dict: {}", e)))?;
                        for (_, xobj) in xobjs.iter() {
                            if let Ok(ref_id) = xobj.as_reference() {
                                let stream_obj = doc.get_object(ref_id).map_err(|e| PdfError::InspectionFailure(format!("Failed to get XObject: {}", e)))?;
                                let xobj_stream = stream_obj.as_stream().map_err(|e| PdfError::InspectionFailure(format!("XObject is not a stream: {}", e)))?;
                                if let Ok(subtype) = xobj_stream.dict.get(b"Subtype") {
                                    if subtype.as_name().map_or(false, |name| name == b"Image") {
                                        has_image = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(char_count < 10 && has_image)
    }

    fn check_text_contains(&self, doc: &lopdf::Document, query_lower: &str) -> OperationResult<bool> {
        let pages = doc.get_pages();
        let max_pages = if pages.len() <= 5 { pages.len() } else { 3 };

        for i in 1..=max_pages as u32 {
            let content = doc.extract_text(&[i]).map_err(|e| {
                PdfError::InspectionFailure(format!("Failed to extract text: {}", e))
            })?;
            let normalized = content.to_lowercase();
            // Normalize whitespace: replace all whitespace chars with a single space
            let mut ws_normalized = String::with_capacity(normalized.len());
            let mut last_was_ws = false;
            for c in normalized.chars() {
                if c.is_whitespace() {
                    if !last_was_ws {
                        ws_normalized.push(' ');
                        last_was_ws = true;
                    }
                } else {
                    ws_normalized.push(c);
                    last_was_ws = false;
                }
            }
            if ws_normalized.contains(query_lower) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paperpilot_core::workflow::get_preset_by_id;

        fn create_text_pdf() -> Vec<u8> {
        let mut doc = lopdf::Document::with_version("1.4");
        let pages_id = doc.new_object_id();
        let font_id = doc.new_object_id();
        let content_id = doc.new_object_id();
        let catalog_id = doc.new_object_id();
        let page_id = doc.new_object_id();

        let mut font_dict = lopdf::Dictionary::new();
        font_dict.set("Type", lopdf::Object::Name(b"Font".to_vec()));
        font_dict.set("Subtype", lopdf::Object::Name(b"Type1".to_vec()));
        font_dict.set("BaseFont", lopdf::Object::Name(b"Helvetica".to_vec()));
        doc.objects.insert(font_id, lopdf::Object::Dictionary(font_dict));

        let content_stream = lopdf::Stream::new(
            lopdf::Dictionary::new(),
            b"BT /F1 12 Tf 100 700 Td (TAX INVOICE 2025) Tj ET".to_vec()
        );
        doc.objects.insert(content_id, lopdf::Object::Stream(content_stream));

        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", lopdf::Object::Reference(pages_id));
        page_dict.set(
            "MediaBox",
            lopdf::Object::Array(vec![0.into(), 0.into(), 612.into(), 792.into()]),
        );
        page_dict.set("Contents", lopdf::Object::Reference(content_id));
        let mut font_res = lopdf::Dictionary::new();
        font_res.set("F1", lopdf::Object::Reference(font_id));
        let mut resources = lopdf::Dictionary::new();
        resources.set("Font", lopdf::Object::Dictionary(font_res));
        page_dict.set("Resources", resources);
        doc.objects.insert(page_id, lopdf::Object::Dictionary(page_dict));

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

    fn create_scanned_pdf() -> Vec<u8> {
        let mut doc = lopdf::Document::with_version("1.4");
        let pages_id = doc.new_object_id();
        let image_id = doc.new_object_id();
        let catalog_id = doc.new_object_id();
        let page_id = doc.new_object_id();

        let mut stream_dict = lopdf::Dictionary::new();
        stream_dict.set("Type", lopdf::Object::Name(b"XObject".to_vec()));
        stream_dict.set("Subtype", lopdf::Object::Name(b"Image".to_vec()));
        stream_dict.set("Width", lopdf::Object::Integer(10));
        stream_dict.set("Height", lopdf::Object::Integer(10));
        stream_dict.set("ColorSpace", lopdf::Object::Name(b"DeviceGray".to_vec()));
        stream_dict.set("BitsPerComponent", lopdf::Object::Integer(8));
        let image_stream = lopdf::Stream::new(
            stream_dict,
            vec![0; 100]
        );
        doc.objects.insert(image_id, lopdf::Object::Stream(image_stream));

        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", lopdf::Object::Reference(pages_id));
        page_dict.set(
            "MediaBox",
            lopdf::Object::Array(vec![0.into(), 0.into(), 612.into(), 792.into()]),
        );
        let mut xobj_res = lopdf::Dictionary::new();
        xobj_res.set("Img1", lopdf::Object::Reference(image_id));
        let mut resources = lopdf::Dictionary::new();
        resources.set("XObject", lopdf::Object::Dictionary(xobj_res));
        page_dict.set("Resources", resources);
        doc.objects.insert(page_id, lopdf::Object::Dictionary(page_dict));

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
    fn test_eval_scanned_pdf_true() {
        let pdf = create_scanned_pdf();
        let inspector = DocumentInspector::new(&pdf);
        let cond = ConditionExpr::Flat(StagePredicate {
            is_scanned: Some(true),
            page_count: None,
            file_size_bytes: None,
            text_contains: None,
            has_form_fields: None,
            is_encrypted: None,
            is_signed: None,
        });
        assert_eq!(inspector.evaluate_condition(&cond).unwrap(), true);
    }

    #[test]
    fn test_eval_scanned_pdf_false_on_digital() {
        let pdf = create_text_pdf();
        let inspector = DocumentInspector::new(&pdf);
        let cond = ConditionExpr::Flat(StagePredicate {
            is_scanned: Some(true),
            page_count: None,
            file_size_bytes: None,
            text_contains: None,
            has_form_fields: None,
            is_encrypted: None,
            is_signed: None,
        });
        assert_eq!(inspector.evaluate_condition(&cond).unwrap(), false);
    }

    #[test]
    fn test_eval_text_contains_case_insensitive() {
        let pdf = create_text_pdf();
        let inspector = DocumentInspector::new(&pdf);
        let cond = ConditionExpr::Flat(StagePredicate {
            is_scanned: None,
            page_count: None,
            file_size_bytes: None,
            text_contains: Some("invoice".to_string()),
            has_form_fields: None,
            is_encrypted: None,
            is_signed: None,
        });
        assert_eq!(inspector.evaluate_condition(&cond).unwrap(), true);
    }

    #[test]
    fn test_eval_corrupt_pdf_fails_explicitly() {
        let pdf = b"%%EOF this is totally fake".to_vec();
        let inspector = DocumentInspector::new(&pdf);
        let cond = ConditionExpr::Flat(StagePredicate {
            is_scanned: Some(true),
            page_count: None,
            file_size_bytes: None,
            text_contains: None,
            has_form_fields: None,
            is_encrypted: None,
            is_signed: None,
        });
        let res = inspector.evaluate_condition(&cond);
        assert!(matches!(res, Err(PdfError::InspectionFailure(_))));
    }

    #[test]
    fn test_routing_stop_pipeline() {
        let pdf = create_text_pdf();
        let wf = SavedWorkflow {
            id: "wf-1".into(),
            name: "test stop".into(),
            description: None,
            updated_at: "now".into(),
            is_preset: false,
            team_id: None,
            steps: vec![
                WorkflowStep {
                    id: "step-1".into(),
                    operation_id: "pdf_watermark".into(),
                    label: None,
                    icon: None,
                    params: std::collections::HashMap::new(),
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                },
                WorkflowStep {
                    id: "step-2".into(),
                    operation_id: "pdf_flatten".into(),
                    label: None,
                    icon: None,
                    params: std::collections::HashMap::new(),
                    condition: Some(ConditionExpr::Flat(StagePredicate {
                        is_scanned: Some(true), // will be false
                        page_count: None,
                        file_size_bytes: None,
                        text_contains: None,
                        has_form_fields: None,
                        is_encrypted: None,
                        is_signed: None,
                    })),
                    on_no_match: OnNoMatchAction::Stop,
                },
                WorkflowStep {
                    id: "step-3".into(),
                    operation_id: "pdf_compress".into(),
                    label: None,
                    icon: None,
                    params: std::collections::HashMap::new(),
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                },
            ]
        };

        let (out, report) = run_workflow_bytes(&pdf, &wf).unwrap();
        assert_eq!(report.steps_completed, 1);
        assert_eq!(report.steps_stopped, 1);
        assert_eq!(report.steps_unreached, 1);
        assert_eq!(report.status, paperpilot_core::workflow::WorkflowExecutionStatus::StoppedEarly);
        assert!(out.len() > 0);
    }

    #[test]
    fn test_routing_custom_error() {
        let pdf = create_text_pdf();
        let wf = SavedWorkflow {
            id: "wf-2".into(),
            name: "test err".into(),
            description: None,
            updated_at: "now".into(),
            is_preset: false,
            team_id: None,
            steps: vec![
                WorkflowStep {
                    id: "step-err".into(),
                    operation_id: "pdf_watermark".into(),
                    label: None,
                    icon: None,
                    params: std::collections::HashMap::new(),
                    condition: Some(ConditionExpr::Flat(StagePredicate {
                        is_scanned: Some(true), // will be false
                        page_count: None,
                        file_size_bytes: None,
                        text_contains: None,
                        has_form_fields: None,
                        is_encrypted: None,
                        is_signed: None,
                    })),
                    on_no_match: OnNoMatchAction::CustomError { error: "I wanted scanned".into() },
                },
            ]
        };

        let res = run_workflow_bytes(&pdf, &wf);
        match res {
            Err(PdfError::ConditionFailed(msg)) => {
                assert_eq!(msg, "I wanted scanned");
            }
            _ => panic!("Expected ConditionFailed")
        }
    }
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
