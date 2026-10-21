use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionReport {
    pub workflow_id: String,
    pub workflow_name: String,
    pub steps_total: usize,
    pub steps_completed: usize,
    pub output_bytes_len: usize,
    pub duration_ms: u64,
}

pub fn get_default_presets() -> Vec<SavedWorkflow> {
    vec![
        SavedWorkflow {
            id: "preset-legal-discovery".to_string(),
            name: "Legal Discovery".to_string(),
            description: Some("Applies Bates numbering, a watermark, and linearizes the PDF for fast web view.".to_string()),
            steps: vec![
                WorkflowStep {
                    id: "step-1".to_string(),
                    operation_id: "pdf_bates".to_string(),
                    label: Some("Bates Numbering".to_string()),
                    icon: None,
                    params: {
                        let mut m = HashMap::new();
                        m.insert("start_number".to_string(), serde_json::json!(1));
                        m.insert("prefix".to_string(), serde_json::json!("EX-"));
                        m.insert("padding".to_string(), serde_json::json!(6));
                        m
                    },
                },
                WorkflowStep {
                    id: "step-2".to_string(),
                    operation_id: "pdf_watermark".to_string(),
                    label: Some("Watermark".to_string()),
                    icon: None,
                    params: {
                        let mut m = HashMap::new();
                        m.insert("text".to_string(), serde_json::json!("CONFIDENTIAL"));
                        m
                    },
                },
                WorkflowStep {
                    id: "step-3".to_string(),
                    operation_id: "pdf_linearize".to_string(),
                    label: Some("Linearize".to_string()),
                    icon: None,
                    params: HashMap::new(),
                },
            ],
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            is_preset: true,
            team_id: None,
        },
        SavedWorkflow {
            id: "preset-clean-scan".to_string(),
            name: "Clean Scan".to_string(),
            description: Some("Removes blank pages, adds page numbers, and compresses the PDF.".to_string()),
            steps: vec![
                WorkflowStep {
                    id: "step-1".to_string(),
                    operation_id: "pdf_remove_blank".to_string(),
                    label: Some("Remove Blank Pages".to_string()),
                    icon: None,
                    params: {
                        let mut m = HashMap::new();
                        m.insert("sensitivity".to_string(), serde_json::json!(90));
                        m
                    },
                },
                WorkflowStep {
                    id: "step-2".to_string(),
                    operation_id: "pdf_page_numbers".to_string(),
                    label: Some("Page Numbers".to_string()),
                    icon: None,
                    params: {
                        let mut m = HashMap::new();
                        m.insert("position".to_string(), serde_json::json!("bottom-right"));
                        m.insert("format".to_string(), serde_json::json!("{page}"));
                        m
                    },
                },
                WorkflowStep {
                    id: "step-3".to_string(),
                    operation_id: "pdf_compress".to_string(),
                    label: Some("Compress".to_string()),
                    icon: None,
                    params: {
                        let mut m = HashMap::new();
                        m.insert("quality".to_string(), serde_json::json!("medium"));
                        m
                    },
                },
            ],
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            is_preset: true,
            team_id: None,
        },
        SavedWorkflow {
            id: "preset-redact-protect".to_string(),
            name: "Redact & Protect".to_string(),
            description: Some("Redacts a specific area, flattens annotations, and encrypts the document.".to_string()),
            steps: vec![
                WorkflowStep {
                    id: "step-1".to_string(),
                    operation_id: "pdf_redact".to_string(),
                    label: Some("Redact".to_string()),
                    icon: None,
                    params: {
                        let mut m = HashMap::new();
                        m.insert("page_number".to_string(), serde_json::json!(1));
                        m.insert("x".to_string(), serde_json::json!(10.0));
                        m.insert("y".to_string(), serde_json::json!(10.0));
                        m.insert("width".to_string(), serde_json::json!(100.0));
                        m.insert("height".to_string(), serde_json::json!(20.0));
                        m
                    },
                },
                WorkflowStep {
                    id: "step-2".to_string(),
                    operation_id: "pdf_flatten".to_string(),
                    label: Some("Flatten".to_string()),
                    icon: None,
                    params: HashMap::new(),
                },
                WorkflowStep {
                    id: "step-3".to_string(),
                    operation_id: "pdf_encrypt".to_string(),
                    label: Some("Encrypt".to_string()),
                    icon: None,
                    params: HashMap::new(),
                },
            ],
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            is_preset: true,
            team_id: None,
        },
    ]
}

pub fn get_preset_by_id(id: &str) -> Option<SavedWorkflow> {
    let presets = get_default_presets();
    presets.into_iter().find(|p| p.id == id || p.id.replace("preset-", "") == id.replace("preset-", ""))
}
