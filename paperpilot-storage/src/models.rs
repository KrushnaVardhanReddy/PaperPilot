use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JobRecord {
    pub id: String,
    pub operation: String,
    pub status: JobStatus,
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub parameters_json: String,
    pub error_message: Option<String>,
    pub progress_percentage: u8,
    pub callback_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewJob {
    pub operation: String,
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub parameters_json: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuditEvent {
    pub id: String,
    pub event_type: String,
    pub tool_id: String,
    pub actor: String,
    pub payload_hash: String,
    pub details_json: String,
    pub timestamp: String,
}
