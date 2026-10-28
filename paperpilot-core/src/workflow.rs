use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NumberComparator {
    Eq(f64),
    Gt(f64),
    Gte(f64),
    Lt(f64),
    Lte(f64),
    Between(f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct StagePredicate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_scanned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<NumberComparator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_bytes: Option<NumberComparator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_contains: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_form_fields: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_signed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", untagged)]
pub enum ConditionExpr {
    Flat(StagePredicate),
    Nested {
        #[serde(skip_serializing_if = "Option::is_none")]
        all: Option<Vec<ConditionExpr>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        any: Option<Vec<ConditionExpr>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        not: Option<Box<ConditionExpr>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum OnNoMatchAction {
    #[default]
    Skip,
    Stop,
    Error,
    CustomError { error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStep {
    pub id: String,
    #[serde(rename = "operationId", alias = "operation_id")]
    pub operation_id: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ConditionExpr>,
    #[serde(default)]
    pub on_no_match: OnNoMatchAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SavedWorkflow {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub steps: Vec<WorkflowStep>,
    #[serde(rename = "updatedAt", alias = "updated_at")]
    pub updated_at: String,
    #[serde(default, rename = "isPreset", alias = "is_preset")]
    pub is_preset: bool,
    #[serde(default, rename = "teamId", alias = "team_id")]
    pub team_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    Executed,
    Skipped,
    Stopped,
    Failed,
    Unreached,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowExecutionStatus {
    Success,
    StoppedEarly,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageExecutionRecord {
    pub id: String,
    pub op: String,
    pub status: StageStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_matched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowExecutionReport {
    pub workflow_id: String,
    pub workflow_name: String,
    pub status: WorkflowExecutionStatus,
    pub steps_total: usize,
    pub steps_completed: usize,
    pub steps_skipped: usize,
    pub steps_stopped: usize,
    pub steps_unreached: usize,
    pub stages: Vec<StageExecutionRecord>,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub output_bytes_len: usize,
}

pub fn get_default_presets() -> Vec<SavedWorkflow> {
    vec![
        SavedWorkflow {
            id: "preset-legal-discovery".to_string(),
            name: "Legal Discovery Prep".to_string(),
            description: Some(
                "Applies Bates numbering, confidential watermark, and web linearization."
                    .to_string(),
            ),
            steps: vec![
                WorkflowStep {
                    id: "step-1".to_string(),
                    operation_id: "pdf_bates".to_string(),
                    label: Some("Bates Numbering".to_string()),
                    icon: None,
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                    params: HashMap::from([
                        ("prefix".to_string(), serde_json::json!("CONFIDENTIAL-")),
                        ("start".to_string(), serde_json::json!(1)),
                    ]),
                },
                WorkflowStep {
                    id: "step-2".to_string(),
                    operation_id: "pdf_watermark".to_string(),
                    label: Some("Watermark".to_string()),
                    icon: None,
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                    params: HashMap::from([(
                        "text".to_string(),
                        serde_json::json!("CONFIDENTIAL"),
                    )]),
                },
                WorkflowStep {
                    id: "step-3".to_string(),
                    operation_id: "pdf_linearize".to_string(),
                    label: Some("Linearize".to_string()),
                    icon: None,
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                    params: HashMap::new(),
                },
            ],
            updated_at: "2025-01-01T00:00:00Z".to_string(),
            is_preset: true,
            team_id: None,
        },
        SavedWorkflow {
            id: "preset-clean-scan".to_string(),
            name: "Clean & Optimize Scan".to_string(),
            description: Some(
                "Removes blank pages, adds page numbers, and compresses file size.".to_string(),
            ),
            steps: vec![
                WorkflowStep {
                    id: "step-1".to_string(),
                    operation_id: "pdf_remove_blank".to_string(),
                    label: Some("Remove Blank Pages".to_string()),
                    icon: None,
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                    params: HashMap::new(),
                },
                WorkflowStep {
                    id: "step-2".to_string(),
                    operation_id: "pdf_page_numbers".to_string(),
                    label: Some("Page Numbers".to_string()),
                    icon: None,
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                    params: HashMap::from([(
                        "position".to_string(),
                        serde_json::json!("bottom-right"),
                    )]),
                },
                WorkflowStep {
                    id: "step-3".to_string(),
                    operation_id: "pdf_compress".to_string(),
                    label: Some("Compress".to_string()),
                    icon: None,
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                    params: HashMap::from([("quality".to_string(), serde_json::json!(75))]),
                },
            ],
            updated_at: "2025-01-01T00:00:00Z".to_string(),
            is_preset: true,
            team_id: None,
        },
        SavedWorkflow {
            id: "preset-redact-protect".to_string(),
            name: "Redact & Secure Document".to_string(),
            description: Some(
                "Redacts sensitive content, flattens annotations, and encrypts PDF.".to_string(),
            ),
            steps: vec![
                WorkflowStep {
                    id: "step-1".to_string(),
                    operation_id: "pdf_redact".to_string(),
                    label: Some("Redact Content".to_string()),
                    icon: None,
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                    params: HashMap::new(),
                },
                WorkflowStep {
                    id: "step-2".to_string(),
                    operation_id: "pdf_flatten".to_string(),
                    label: Some("Flatten Annotations".to_string()),
                    icon: None,
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                    params: HashMap::new(),
                },
                WorkflowStep {
                    id: "step-3".to_string(),
                    operation_id: "pdf_encrypt".to_string(),
                    label: Some("Encrypt".to_string()),
                    icon: None,
                    condition: None,
                    on_no_match: OnNoMatchAction::Skip,
                    params: HashMap::new(),
                },
            ],
            updated_at: "2025-01-01T00:00:00Z".to_string(),
            is_preset: true,
            team_id: None,
        },
    ]
}

pub fn get_preset_by_id(id: &str) -> Option<SavedWorkflow> {
    let presets = get_default_presets();
    presets.into_iter().find(|p| {
        p.id == id
            || p.id.strip_prefix("preset-").unwrap_or(&p.id) == id
            || format!("preset-{}", id) == p.id
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_presets() {
        let presets = get_default_presets();
        assert_eq!(presets.len(), 3);
        assert_eq!(presets[0].id, "preset-legal-discovery");
        assert_eq!(presets[1].id, "preset-clean-scan");
        assert_eq!(presets[2].id, "preset-redact-protect");
    }

    #[test]
    fn test_get_preset_by_id() {
        assert!(get_preset_by_id("preset-legal-discovery").is_some());
        assert!(get_preset_by_id("legal-discovery").is_some());
        assert!(get_preset_by_id("preset-clean-scan").is_some());
        assert!(get_preset_by_id("clean-scan").is_some());
        assert!(get_preset_by_id("preset-redact-protect").is_some());
        assert!(get_preset_by_id("redact-protect").is_some());
        assert!(get_preset_by_id("non-existent").is_none());
    }

    #[test]
    fn test_serde_saved_workflow() {
        let preset = get_preset_by_id("legal-discovery").unwrap();
        let json_str = serde_json::to_string(&preset).unwrap();
        let deserialized: SavedWorkflow = serde_json::from_str(&json_str).unwrap();
        assert_eq!(preset, deserialized);
    }
}

#[cfg(test)]
mod condition_tests {
    use super::*;

    #[test]
    fn test_comparator_eq_match() {
        let json = r#"{"page_count": {"eq": 5.0}}"#;
        let pred: StagePredicate = serde_json::from_str(json).unwrap();
        assert_eq!(pred.page_count, Some(NumberComparator::Eq(5.0)));
    }

    #[test]
    fn test_comparator_between_invalid_range() {
        let json = r#"{"page_count": {"between": [20.0, 5.0]}}"#;
        let pred: StagePredicate = serde_json::from_str(json).unwrap();
        assert_eq!(pred.page_count, Some(NumberComparator::Between(20.0, 5.0)));
    }

    #[test]
    fn test_boolean_empty_all() {
        let json = r#"{"all": []}"#;
        let expr: ConditionExpr = serde_json::from_str(json).unwrap();
        match expr {
            ConditionExpr::Nested { all, any, not } => {
                assert_eq!(all.unwrap().len(), 0);
                assert!(any.is_none());
                assert!(not.is_none());
            }
            _ => panic!("Expected Nested"),
        }
    }

    #[test]
    fn test_boolean_empty_any() {
        let json = r#"{"any": []}"#;
        let expr: ConditionExpr = serde_json::from_str(json).unwrap();
        match expr {
            ConditionExpr::Nested { all, any, not } => {
                assert!(all.is_none());
                assert_eq!(any.unwrap().len(), 0);
                assert!(not.is_none());
            }
            _ => panic!("Expected Nested"),
        }
    }

    #[test]
    fn test_serde_deny_unknown_fields() {
        let json = r#"{"text_contain": "foo"}"#;
        let result: Result<StagePredicate, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_legacy_workflow_backward_compat() {
        let json = r#"{
            "id": "step1",
            "operation_id": "pdf_bates",
            "params": {}
        }"#;
        let step: WorkflowStep = serde_json::from_str(json).unwrap();
        assert_eq!(step.condition, None);
        assert_eq!(step.on_no_match, OnNoMatchAction::Skip);
    }
}
