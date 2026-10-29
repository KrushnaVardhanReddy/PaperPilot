use super::{OnNoMatch, SavedWorkflow};
use crate::error::{OperationResult, PdfError};
use crate::workflow::{NumericComparator, WorkflowCondition};
use std::collections::{HashMap, HashSet};

/// Parse the version-1 TOML recipe or the existing SavedWorkflow JSON format.
/// Parsing has no filesystem, environment-variable, or network side effects.
pub fn parse_workflow(content: &str) -> OperationResult<SavedWorkflow> {
    let content = content.trim_start_matches('\u{feff}').trim();
    let mut workflow: SavedWorkflow = if content.starts_with('{') {
        serde_json::from_str(content)
            .map_err(|e| PdfError::ParseError(format!("Invalid workflow JSON: {e}")))?
    } else {
        toml::from_str(content).map_err(|e: toml::de::Error| {
            PdfError::ParseError(format!("Invalid workflow TOML: {}", e.message()))
        })?
    };
    if workflow.id.is_empty() {
        workflow.id = workflow.name.clone();
    }
    workflow_order(&workflow)?;
    Ok(workflow)
}

/// Stable topological order. An omitted dependency means the preceding declared
/// stage; an explicit dependency selects that stage's PDF, including fan-out.
/// Joining multiple PDF artifacts requires a future explicit merge operation.
pub fn workflow_order(workflow: &SavedWorkflow) -> OperationResult<Vec<usize>> {
    let invalid = |message: String| PdfError::InvalidInput(message);
    if workflow.version != "1" {
        return Err(invalid("Unsupported workflow version (expected 1)".into()));
    }
    if workflow.name.trim().is_empty() || workflow.steps.is_empty() || workflow.steps.len() > 256 {
        return Err(invalid(
            "A named workflow must contain 1..=256 stages".into(),
        ));
    }
    let mut ids = HashMap::new();
    for (index, step) in workflow.steps.iter().enumerate() {
        if step.id.trim().is_empty() || ids.insert(step.id.as_str(), index).is_some() {
            return Err(invalid("Stage IDs must be nonempty and unique".into()));
        }
        if step.operation_id.trim().is_empty() {
            return Err(invalid(format!(
                "Stage '{}' requires an operation",
                step.id
            )));
        }
        if step.depends_on.len() > 1 {
            return Err(invalid(format!(
                "Stage '{}': multiple PDF dependencies require an explicit merge and are not supported",
                step.id
            )));
        }
        if let Some(c) = &step.condition {
            validate_condition(c, step.id.as_str(), &invalid)?;
        } else if step.on_no_match != OnNoMatch::Skip {
            return Err(invalid(format!(
                "Stage '{}': on_no_match requires a condition",
                step.id
            )));
        }
    }
    let mut predecessors = vec![None; workflow.steps.len()];
    for (index, step) in workflow.steps.iter().enumerate() {
        predecessors[index] = if let Some(id) = step.depends_on.first() {
            Some(*ids.get(id.as_str()).ok_or_else(|| {
                invalid(format!(
                    "Stage '{}': dependency '{}' does not exist",
                    step.id, id
                ))
            })?)
        } else {
            index.checked_sub(1)
        };
    }
    let mut order = Vec::new();
    let mut done = HashSet::new();
    while order.len() < workflow.steps.len() {
        let next = (0..workflow.steps.len())
            .find(|index| {
                !done.contains(index) && predecessors[*index].is_none_or(|p| done.contains(&p))
            })
            .ok_or_else(|| invalid("Workflow dependencies contain a cycle".into()))?;
        order.push(next);
        done.insert(next);
    }
    Ok(order)
}

fn validate_condition(
    condition: &WorkflowCondition,
    stage_id: &str,
    invalid: &impl Fn(String) -> PdfError,
) -> OperationResult<()> {
    match condition {
        WorkflowCondition::All { all } => {
            for c in all {
                validate_condition(c, stage_id, invalid)?;
            }
        }
        WorkflowCondition::Any { any } => {
            for c in any {
                validate_condition(c, stage_id, invalid)?;
            }
        }
        WorkflowCondition::Not { not } => {
            validate_condition(not, stage_id, invalid)?;
        }
        WorkflowCondition::Leaf(c) => {
            if c.if_scanned.is_none()
                && c.if_page_count_gt.is_none()
                && c.comparator.is_none()
                && c.text_contains.is_none()
                && c.has_form_fields.is_none()
                && c.is_signed.is_none()
                && c.is_encrypted.is_none()
                && c.file_size_bytes.is_none()
            {
                return Err(invalid(format!("Stage '{}': empty condition", stage_id)));
            }
            if c.comparator.is_some() != c.value.is_some() {
                return Err(invalid(format!(
                    "Stage '{}': comparator and value must be supplied together",
                    stage_id
                )));
            }
            if let Some(NumericComparator::Between { between: [min, max] }) = &c.file_size_bytes {
                if min > max {
                    return Err(invalid("between: min must be <= max".into()));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toml_aliases_and_legacy_json_are_compatible() {
        let wf = parse_workflow("name = 'Mail'\nversion = '1'\n[[stage]]\nid = 'rotate'\nop = 'rotate'\ncondition = { if_page_count_gt = 2 }\n[stage.params]\nangle = 180").unwrap();
        assert_eq!(wf.id, "Mail");
        assert_eq!(wf.steps[0].params["angle"], 180);
        let encoded = serde_json::to_string(&wf).unwrap();
        assert_eq!(parse_workflow(&encoded).unwrap(), wf);
        let legacy = r#"{"id":"old","name":"Old","steps":[{"id":"a","operationId":"rotate","params":{}}],"updatedAt":"2025-01-01"}"#;
        assert_eq!(parse_workflow(legacy).unwrap().version, "1");
    }

    #[test]
    fn rejects_unsupported_schema_and_invalid_graphs() {
        let base = "name = 'Test'\n[[stage]]\nid = 'a'\nop = 'rotate'\n";
        for suffix in [
            "retry = 2",
            "condition = {}",
            "condition = { comparator = 'Greater' }",
            "on_no_match = 'error'",
            "depends_on = ['missing']",
            "depends_on = ['a']",
            "depends_on = ['a', 'b']",
            "condition = { if_scaned = true }",
        ] {
            assert!(
                parse_workflow(&format!("{base}{suffix}")).is_err(),
                "{suffix}"
            );
        }
        assert!(parse_workflow(&format!("{base}[[stage]]\nid = 'a'\nop = 'rotate'")).is_err());
        assert!(parse_workflow(&format!("version = '2'\n{base}")).is_err());
        assert!(parse_workflow("name = 'Empty'\nstage = []").is_err());
    }

    #[test]
    fn stable_order_supports_forward_dependencies() {
        let dag = parse_workflow("name = 'DAG'\n[[stage]]\nid = 'root'\nop = 'rotate'\n[[stage]]\nid = 'a'\nop = 'rotate'\ndepends_on = ['b']\n[[stage]]\nid = 'b'\nop = 'rotate'\ndepends_on = ['root']").unwrap();
        assert_eq!(workflow_order(&dag).unwrap(), vec![0, 2, 1]);
        let wf = parse_workflow(
            "name = 'DAG'\n[[stage]]\nid = 'a'\nop = 'rotate'\ndepends_on = ['c']\n[[stage]]\nid = 'b'\nop = 'rotate'\ndepends_on = ['a']\n[[stage]]\nid = 'c'\nop = 'rotate'\ndepends_on = ['root']\n[[stage]]\nid = 'root'\nop = 'rotate'\ndepends_on = ['b']",
        );
        assert!(wf.is_err());
    }
}
