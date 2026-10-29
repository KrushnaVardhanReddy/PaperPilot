//! Local execution history. Pure workflow processing remains in `workflow`.
use paperpilot_core::{
    error::{OperationResult, PdfError},
    workflow::{SavedWorkflow, WorkflowExecutionReport},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    time::Instant,
};

fn invalid(message: impl Into<String>) -> PdfError {
    PdfError::InvalidInput(message.into())
}
fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub schema_version: u32,
    pub job_id: String,
    pub operation: String,
    pub started_at: String,
    pub finished_at: String,
    pub duration_ms: u64,
    pub success: bool,
    pub input: PathBuf,
    pub input_sha256: Option<String>,
    pub requested_output: PathBuf,
    pub output: Option<PathBuf>,
    pub output_sha256: Option<String>,
    pub error: Option<String>,
    pub report: Option<WorkflowExecutionReport>,
}

#[derive(Debug, Serialize)]
pub struct RecordedRun {
    pub receipt_path: PathBuf,
    #[serde(flatten)]
    pub receipt: ExecutionReceipt,
}
impl RecordedRun {
    pub fn outcome(&self) -> OperationResult<()> {
        if self.receipt.success {
            Ok(())
        } else {
            Err(invalid(
                self.receipt
                    .error
                    .clone()
                    .unwrap_or_else(|| "Workflow failed".into()),
            ))
        }
    }
    /// Existing callers retain their stage-report shape, with additive provenance.
    pub fn report_json(&self) -> Value {
        let mut value = self
            .receipt
            .report
            .as_ref()
            .map(|r| json!(r))
            .unwrap_or_else(|| json!({}));
        value["job_id"] = json!(self.receipt.job_id);
        value["receipt_path"] = json!(self.receipt_path);
        value
    }
}

/// Persist terminal results next to the requested output. A receipt location is
/// reserved before processing; unavailable history storage cannot silently drop a run.
pub fn run_to_path(
    input: &Path,
    output: &Path,
    workflow: &SavedWorkflow,
) -> OperationResult<RecordedRun> {
    let parent = output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent)?;
    let directory = parent.join(".paperpilot-receipts");
    fs::create_dir_all(&directory)?;
    let mut pending = tempfile::Builder::new()
        .prefix(".run-")
        .tempfile_in(&directory)?;
    let job_id = pending
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .trim_start_matches('.')
        .to_string();
    let receipt_path = directory.join(format!("{job_id}.json"));
    let started = chrono::Utc::now();
    let clock = Instant::now();
    let mut record = ExecutionReceipt {
        schema_version: 1,
        job_id,
        operation: "workflow".into(),
        started_at: started.to_rfc3339(),
        finished_at: String::new(),
        duration_ms: 0,
        success: false,
        input: input.into(),
        input_sha256: None,
        requested_output: output.into(),
        output: None,
        output_sha256: None,
        error: None,
        report: None,
    };
    let result = (|| -> OperationResult<()> {
        let bytes = fs::read(input)?;
        record.input_sha256 = Some(hash(&bytes));
        if output.exists() && input.canonicalize()? == output.canonicalize()? {
            return Err(invalid("Input and output must be different files"));
        }
        let (bytes, report) = super::workflow::run_workflow_bytes_detailed(&bytes, workflow)
            .map_err(|f| {
                record.report = Some(f.report);
                f.cause
            })?;
        record.report = Some(report);
        let mut temp = tempfile::NamedTempFile::new_in(parent)?;
        temp.write_all(&bytes)?;
        temp.as_file().sync_all()?;
        temp.persist(output)
            .map_err(|e| PdfError::IoError(e.error))?;
        record.output = Some(output.into());
        record.output_sha256 = Some(hash(&bytes));
        Ok(())
    })();
    record.success = result.is_ok();
    record.error = result.err().map(|e| e.to_string());
    if !record.success {
        if let Some(report) = &mut record.report {
            report.error = record.error.clone();
            report.output_bytes_len = 0;
        }
    }
    record.finished_at = chrono::Utc::now().to_rfc3339();
    record.duration_ms = clock.elapsed().as_millis().try_into().unwrap_or(u64::MAX);
    serde_json::to_writer_pretty(&mut pending, &record).map_err(|e| invalid(e.to_string()))?;
    pending.write_all(b"\n")?;
    pending.as_file().sync_all()?;
    pending
        .persist_noclobber(&receipt_path)
        .map_err(|e| PdfError::IoError(e.error))?;
    Ok(RecordedRun {
        receipt_path,
        receipt: record,
    })
}

#[derive(Debug, Serialize)]
pub struct HistoryEntry {
    pub receipt_id: String,
    pub job_id: Option<String>,
    pub source_schema: String,
    pub success: bool,
    pub started_at: Option<String>,
    pub duration_ms: Option<u64>,
    pub input_sha256: Option<String>,
    pub output_sha256: Option<String>,
    pub failed_stage: Option<String>,
    #[serde(skip_serializing_if = "Value::is_null")]
    pub details: Value,
}
fn decode(path: &Path, id: String) -> OperationResult<HistoryEntry> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 16 * 1024 * 1024
    {
        return Err(invalid("Receipt must be a regular JSON file up to 16 MiB"));
    }
    let bytes = fs::read(path)?;
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|e| invalid(format!("Invalid receipt {id}: {e}")))?;
    let schema = if value["schema_version"] == 1 {
        "workflow/v1"
    } else if value.get("results").is_some() && value.get("template_sha256").is_some() {
        "batch/legacy"
    } else if value.get("file").is_some() && value.get("sha256").is_some() {
        "watcher/legacy"
    } else {
        return Err(invalid(format!("Unsupported receipt schema: {id}")));
    };
    let success = value["success"]
        .as_bool()
        .ok_or_else(|| invalid(format!("Missing receipt outcome: {id}")))?;
    let string = |key: &str| {
        value[key]
            .as_str()
            .filter(|s| !s.is_empty())
            .map(str::to_string)
    };
    let failed_stage = value["report"]["stages"]
        .as_array()
        .and_then(|s| s.iter().find(|s| s["status"] == "failed"))
        .and_then(|s| s["id"].as_str())
        .map(str::to_string);
    Ok(HistoryEntry {
        receipt_id: id,
        job_id: string("job_id"),
        source_schema: schema.into(),
        success,
        started_at: string("started_at").or_else(|| string("timestamp")),
        duration_ms: value["duration_ms"]
            .as_u64()
            .or_else(|| value["execution_time_ms"].as_u64())
            .or_else(|| value["report"]["duration_ms"].as_u64()),
        input_sha256: string("input_sha256")
            .or_else(|| string("template_sha256"))
            .or_else(|| string("sha256")),
        output_sha256: string("output_sha256"),
        failed_stage,
        details: value,
    })
}
pub fn get(directory: &Path, receipt_id: &str) -> OperationResult<HistoryEntry> {
    let path = Path::new(receipt_id);
    let mut parts = path.components();
    if !matches!(parts.next(), Some(Component::Normal(_)))
        || parts.next().is_some()
        || receipt_id.contains(['/', '\\', ':'])
        || !receipt_id.ends_with(".json")
    {
        return Err(invalid("Receipt ID must be a single .json filename"));
    }
    let path = directory.join(path);
    if fs::symlink_metadata(&path)?.file_type().is_symlink() {
        return Err(invalid("Receipt symlinks are not supported"));
    }
    decode(&path, receipt_id.into())
}
pub fn list(directory: &Path, limit: usize) -> OperationResult<Vec<HistoryEntry>> {
    if !(1..=1000).contains(&limit) {
        return Err(invalid("History limit must be between 1 and 1000"));
    }
    let mut entries = Vec::new();
    for file in fs::read_dir(directory)? {
        let file = file?;
        let id = file.file_name().to_string_lossy().to_string();
        if !file.file_type()?.is_file()
            || !(id.starts_with("run-") && id.ends_with(".json")
                || id.ends_with(".receipt.json")
                || id == "execution_receipt.json")
        {
            continue;
        }
        entries.push((file.metadata()?.modified()?, id, file.path()));
    }
    entries.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));
    entries.truncate(limit);
    entries
        .into_iter()
        .map(|(_, id, path)| {
            let mut entry = decode(&path, id)?;
            entry.details = Value::Null;
            Ok(entry)
        })
        .collect()
}
