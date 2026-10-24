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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewDocumentChunk {
    pub id: Option<String>,
    pub document_hash: String,
    pub document_name: String,
    pub page_number: u32,
    pub chunk_index: u32,
    pub content: String,
    pub embedding: Vec<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DocumentChunkRecord {
    pub id: String,
    pub document_hash: String,
    pub document_name: String,
    pub page_number: u32,
    pub chunk_index: u32,
    pub content: String,
    pub embedding: Vec<f32>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SemanticSearchResult {
    pub chunk: DocumentChunkRecord,
    pub score: f32,
}
