use crate::intent::Intent;
use crate::entities::ExtractedEntities;

/// Represents a sequence of PDF operations to execute.
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub struct OperationPlan {
    pub intent: Intent,
    pub input_files: Vec<String>,
    pub output_file: Option<String>,
    pub page_ranges: Vec<String>,
    pub angles: Vec<i32>,
    pub passwords: Vec<String>,
    pub raw_query: String,
}

impl OperationPlan {
    pub fn new(intent: Intent, entities: ExtractedEntities, raw_query: String) -> Self {
        // Separate the last file as output if multiple files and intent is Merge
        let (inputs, output) = Self::split_io(intent, entities.files);
        OperationPlan {
            intent,
            input_files: inputs,
            output_file: output,
            page_ranges: entities.page_ranges,
            angles: entities.angles,
            passwords: entities.passwords,
            raw_query,
        }
    }

    fn split_io(intent: Intent, files: Vec<String>) -> (Vec<String>, Option<String>) {
        // Heuristic: if 2+ files and intent is Merge, last file is output
        if matches!(intent, Intent::Merge) && files.len() >= 2 {
            let mut inputs = files;
            let output = inputs.pop();
            (inputs, output)
        } else {
            (files, None)
        }
    }
}

/// Error type for NLP resolution failures
#[derive(Debug, thiserror::Error)]
pub enum NlpError {
    #[error("Could not determine intent from the query: {0}")]
    AmbiguousIntent(String),
    #[error("Failed to extract required parameters for intent: {0}")]
    MissingParameters(String),
    #[error("Internal NLP engine error: {0}")]
    InternalError(String),
}

/// The core abstraction for natural language command processing.
/// This trait is implemented by both the embedded Offline NLP engine
/// and the cloud/local LLM providers (via `paperpilot-ai`).
pub trait NlpResolver: Send + Sync {
    /// Takes a natural language string and attempts to resolve it into a structured
    /// `OperationPlan` that the PaperPilot engine can execute.
    fn resolve(&self, query: &str) -> Result<OperationPlan, NlpError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::Intent;
    use crate::entities::ExtractedEntities;

    #[test]
    fn test_operation_plan_split_io_merge() {
        let intent = Intent::Merge;
        let entities = ExtractedEntities {
            files: vec!["a.pdf".to_string(), "b.pdf".to_string(), "out.pdf".to_string()],
            ..Default::default()
        };
        let plan = OperationPlan::new(intent, entities, "merge a b to out".to_string());
        assert_eq!(plan.input_files, vec!["a.pdf", "b.pdf"]);
        assert_eq!(plan.output_file, Some("out.pdf".to_string()));
    }

    #[test]
    fn test_operation_plan_split_io_not_merge() {
        let intent = Intent::Rotate;
        let entities = ExtractedEntities {
            files: vec!["a.pdf".to_string(), "b.pdf".to_string()],
            ..Default::default()
        };
        let plan = OperationPlan::new(intent, entities, "rotate a b".to_string());
        assert_eq!(plan.input_files, vec!["a.pdf", "b.pdf"]);
        assert_eq!(plan.output_file, None);
    }
}
