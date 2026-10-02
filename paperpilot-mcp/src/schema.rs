use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
    pub output_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_detected: Option<bool>,
    pub data: Option<serde_json::Value>,
}
