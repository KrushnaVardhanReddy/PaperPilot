use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfOperation;
use paperpilot_core::workflow::{
    OnNoMatch, SavedWorkflow, WorkflowExecutionReport, WorkflowStageReport, WorkflowStageStatus,
    workflow_order,
};
use web_time::Instant;

mod conditions;
mod dispatch;

#[derive(Debug)]
pub struct WorkflowFailure {
    pub report: WorkflowExecutionReport,
    pub cause: PdfError,
}

impl std::fmt::Display for WorkflowFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cause.fmt(f)
    }
}

impl std::error::Error for WorkflowFailure {}

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
    run_workflow_bytes_detailed(input_bytes, workflow).map_err(|failure| failure.cause)
}

/// Shared engine for CLI, MCP, REST and WASM. Failure reports preserve completed,
/// skipped, failed and not-run stages without publishing a partial output PDF.
pub fn run_workflow_bytes_detailed(
    input_bytes: &[u8],
    workflow: &SavedWorkflow,
) -> Result<(Vec<u8>, WorkflowExecutionReport), WorkflowFailure> {
    let start = Instant::now();
    let mut report = WorkflowExecutionReport {
        workflow_id: workflow.id.clone(),
        workflow_name: workflow.name.clone(),
        steps_total: workflow.steps.len(),
        steps_completed: 0,
        output_bytes_len: 0,
        duration_ms: 0,
        steps_skipped: 0,
        archived_original: false,
        error: None,
        stages: workflow
            .steps
            .iter()
            .map(|step| WorkflowStageReport {
                id: step.id.clone(),
                operation_id: step.operation_id.clone(),
                status: WorkflowStageStatus::NotRun,
                reason: None,
                duration_ms: 0,
            })
            .collect(),
    };
    let mut active = None;
    let mut active_started = Instant::now();
    let result = (|| -> OperationResult<Vec<u8>> {
        let order = workflow_order(workflow)?;
        // Validate every operation before condition evaluation, so typos or
        // unsupported operations cannot hide behind an unmatched condition.
        for (index, step) in workflow.steps.iter().enumerate() {
            active = Some(index);
            active_started = Instant::now();
            dispatch::validate_step(step)?;
            if matches!(step.operation_id.as_str(), "encrypt" | "pdf_encrypt")
                && (index + 1 != workflow.steps.len()
                    || workflow
                        .steps
                        .iter()
                        .any(|other| other.depends_on.contains(&step.id)))
            {
                return Err(PdfError::InvalidInput(
                    "Encryption must be the final declared workflow stage with no dependent stages"
                        .into(),
                ));
            }
        }
        active = None;
        let doc = conditions::load_pdf(input_bytes)?;
        if doc.get_pages().is_empty() {
            return Err(PdfError::InvalidInput(
                "Workflow input has no PDF pages".into(),
            ));
        }
        let mut outputs: Vec<Option<Vec<u8>>> = vec![None; workflow.steps.len()];
        let predecessors: Vec<_> = workflow
            .steps
            .iter()
            .enumerate()
            .map(|(index, step)| {
                step.depends_on
                    .first()
                    .map(|id| {
                        workflow
                            .steps
                            .iter()
                            .position(|s| &s.id == id)
                            .expect("validated dependency")
                    })
                    .or_else(|| index.checked_sub(1))
            })
            .collect();
        let mut consumers = vec![0usize; workflow.steps.len()];
        for predecessor in predecessors.iter().flatten() {
            consumers[*predecessor] += 1;
        }
        for index in order {
            active = Some(index);
            let step = &workflow.steps[index];
            let stage_start = Instant::now();
            active_started = stage_start;
            let predecessor = predecessors[index];
            let bytes = match predecessor {
                Some(p) => outputs[p].as_deref().ok_or_else(|| {
                    PdfError::Other("Workflow dependency artifact is unavailable".into())
                })?,
                None => input_bytes,
            };
            let evaluation = step
                .condition
                .as_ref()
                .map(|condition| conditions::evaluate(bytes, condition))
                .transpose()?;
            if let Some((false, reason)) = evaluation {
                match step.on_no_match {
                    OnNoMatch::Skip => {
                        outputs[index] = Some(bytes.to_vec());
                        report.steps_skipped += 1;
                        report.stages[index].status = WorkflowStageStatus::Skipped;
                        report.stages[index].reason = Some(reason);
                    }
                    OnNoMatch::ArchiveOriginal => {
                        report.steps_skipped += 1;
                        report.stages[index].status = WorkflowStageStatus::Skipped;
                        report.stages[index].reason =
                            Some(format!("{reason}; archive_original selected"));
                        report.stages[index].duration_ms = stage_start.elapsed().as_millis() as u64;
                        report.archived_original = true;
                        for stage in &mut report.stages {
                            if stage.status == WorkflowStageStatus::NotRun {
                                stage.reason = Some("Workflow stopped by archive_original".into());
                            }
                        }
                        return Ok(input_bytes.to_vec());
                    }
                    OnNoMatch::Error => {
                        return Err(PdfError::InvalidInput(format!(
                            "Condition did not match: {reason}"
                        )));
                    }
                }
            } else {
                outputs[index] = Some(dispatch::execute_step(bytes, step)?);
                report.steps_completed += 1;
                report.stages[index].status = WorkflowStageStatus::Completed;
                report.stages[index].reason = evaluation.map(|(_, reason)| reason);
            }
            report.stages[index].duration_ms = stage_start.elapsed().as_millis() as u64;
            // Retain only artifacts still needed by a dependency or final output.
            if let Some(p) = predecessor {
                consumers[p] -= 1;
                if consumers[p] == 0 && p + 1 != workflow.steps.len() {
                    outputs[p] = None;
                }
            }
            if consumers[index] == 0 && index + 1 != workflow.steps.len() {
                outputs[index] = None;
            }
        }
        // The final declared stage selects the output, regardless of DAG order.
        Ok(outputs
            .last_mut()
            .and_then(Option::take)
            .expect("validated nonempty workflow"))
    })();
    report.duration_ms = start.elapsed().as_millis() as u64;
    match result {
        Ok(bytes) => {
            report.output_bytes_len = bytes.len();
            Ok((bytes, report))
        }
        Err(error) => {
            let context = active
                .map(|index| {
                    format!(
                        "Stage '{}' ({}): ",
                        workflow.steps[index].id, workflow.steps[index].operation_id
                    )
                })
                .unwrap_or_default();
            let message = format!("{context}{error}");
            if let Some(index) = active {
                report.stages[index].status = WorkflowStageStatus::Failed;
                report.stages[index].reason = Some(message.clone());
                report.stages[index].duration_ms = active_started.elapsed().as_millis() as u64;
            }
            report.error = Some(message.clone());
            Err(WorkflowFailure {
                report,
                cause: PdfError::Other(message),
            })
        }
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
