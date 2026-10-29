//! Local mail merge orchestration; PDF work is delegated to the shared core.

use super::form_fill::FormFillTemplate;
use chrono::{NaiveDate, Utc};
use paperpilot_core::error::{OperationResult, PdfError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::time::Instant;

/// Local batch settings shared by native adapters. Outputs never overwrite files.
#[derive(Clone, Debug)]
pub struct BatchFillConfig {
    pub template: PathBuf,
    pub data: PathBuf,
    pub output_dir: PathBuf,
    pub name_template: String,
    pub field_map: Option<PathBuf>,
    pub keep_editable: bool,
    pub date: Option<String>,
}

fn default_name() -> String {
    "{index:04}.pdf".into()
}

/// File-based or inline JSON request for MCP and local REST adapters.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchFillRequest {
    pub template: PathBuf,
    pub output_dir: PathBuf,
    pub data: Option<PathBuf>,
    pub records: Option<Value>,
    #[serde(default = "default_name")]
    pub name_template: String,
    pub field_map: Option<PathBuf>,
    #[serde(default)]
    pub keep_editable: bool,
    pub date: Option<String>,
}

impl BatchFillRequest {
    pub fn execute(self) -> OperationResult<BatchReceipt> {
        let (data, inline) = match (self.data, self.records) {
            (Some(path), None) => (path, None),
            (None, Some(value)) if value.is_array() => (
                PathBuf::from("inline.json"),
                Some(serde_json::to_vec(&value).map_err(|e| invalid(e.to_string()))?),
            ),
            _ => {
                return Err(invalid(
                    "Specify exactly one of data (CSV/JSON path) or records (JSON array)",
                ));
            }
        };
        run_impl(
            &BatchFillConfig {
                template: self.template,
                data,
                output_dir: self.output_dir,
                name_template: self.name_template,
                field_map: self.field_map,
                keep_editable: self.keep_editable,
                date: self.date,
            },
            inline,
        )
    }
}

type Record = HashMap<String, String>;

fn invalid(message: impl Into<String>) -> PdfError {
    PdfError::InvalidInput(message.into())
}
fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[derive(Serialize)]
pub struct RowResult {
    index: usize,
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
pub struct BatchReceipt {
    pub success: bool,
    job_id: String,
    started_at: String,
    date: String,
    template: PathBuf,
    data: Option<PathBuf>,
    data_source: &'static str,
    template_sha256: String,
    data_sha256: String,
    receipt_path: PathBuf,
    operations_applied: Vec<&'static str>,
    total: usize,
    succeeded: usize,
    failed: usize,
    execution_time_ms: u128,
    results: Vec<RowResult>,
}

impl BatchReceipt {
    pub fn outcome(&self) -> OperationResult<()> {
        if self.failed == 0 {
            Ok(())
        } else {
            Err(invalid(format!(
                "{} of {} records failed; see {}",
                self.failed,
                self.total,
                self.receipt_path.display()
            )))
        }
    }
}

fn scalar(value: &Value) -> OperationResult<String> {
    match value {
        Value::Null => Ok(String::new()),
        Value::String(value) => Ok(value.clone()),
        Value::Bool(_) | Value::Number(_) => Ok(value.to_string()),
        _ => Err(invalid(
            "Record values must be strings, numbers, booleans or null",
        )),
    }
}

fn records(data: &[u8], path: &Path) -> OperationResult<Vec<OperationResult<Record>>> {
    let data = data.strip_prefix(b"\xef\xbb\xbf").unwrap_or(data);
    match path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "json" => {
            let value: Value = serde_json::from_slice(data)
                .map_err(|e| invalid(format!("Invalid JSON data: {e}")))?;
            let rows = value
                .as_array()
                .ok_or_else(|| invalid("JSON data must be an array of record objects"))?;
            Ok(rows
                .iter()
                .map(|row| {
                    row.as_object()
                        .ok_or_else(|| invalid("Each JSON record must be an object"))?
                        .iter()
                        .map(|(key, value)| Ok((key.clone(), scalar(value)?)))
                        .collect()
                })
                .collect())
        }
        "csv" => {
            let mut reader = csv::ReaderBuilder::new().from_reader(data);
            let headers = reader
                .headers()
                .map_err(|e| invalid(format!("Invalid CSV header: {e}")))?
                .clone();
            let mut seen = HashSet::new();
            if headers.is_empty() || headers.iter().any(|h| h.is_empty() || !seen.insert(h)) {
                return Err(invalid("CSV headers must be nonempty and unique"));
            }
            Ok(reader
                .records()
                .map(|row| {
                    row.map(|row| {
                        headers
                            .iter()
                            .zip(row.iter())
                            .map(|(key, value)| (key.to_string(), value.to_string()))
                            .collect()
                    })
                    .map_err(|e| invalid(format!("Invalid CSV record: {e}")))
                })
                .collect())
        }
        _ => Err(invalid("Data file extension must be .csv or .json")),
    }
}

enum Token {
    Literal(String),
    Row(String),
    Date,
    Index(usize),
}

fn tokens(pattern: &str) -> OperationResult<Vec<Token>> {
    if pattern.is_empty() {
        return Err(invalid("Filename template must not be empty"));
    }
    let mut result = Vec::new();
    let mut rest = pattern;
    while !rest.is_empty() {
        let Some(start) = rest.find('{') else {
            if rest.contains('}') {
                return Err(invalid("Unmatched filename token brace"));
            }
            result.push(Token::Literal(rest.to_string()));
            break;
        };
        let literal = &rest[..start];
        if literal.contains('}') {
            return Err(invalid("Unmatched filename token brace"));
        }
        result.push(Token::Literal(literal.to_string()));
        let end = rest[start + 1..]
            .find('}')
            .ok_or_else(|| invalid("Unclosed filename token"))?
            + start
            + 1;
        let token = &rest[start + 1..end];
        result.push(if token == "date" {
            Token::Date
        } else if token == "index" {
            Token::Index(0)
        } else if let Some(width) = token.strip_prefix("index:") {
            let width = width
                .parse::<usize>()
                .map_err(|_| invalid("Invalid index width"))?;
            if width > 12 {
                return Err(invalid("Index padding must not exceed 12"));
            }
            Token::Index(width)
        } else if let Some(key) = token
            .strip_prefix("row.")
            .filter(|key| !key.is_empty() && !key.contains('{'))
        {
            Token::Row(key.to_string())
        } else {
            return Err(invalid(format!("Unsupported filename token: {{{token}}}")));
        });
        rest = &rest[end + 1..];
    }
    for token in &result {
        if let Token::Literal(value) = token {
            if value.chars().any(forbidden) {
                return Err(invalid("Filename template must produce one local filename"));
            }
        }
    }
    Ok(result)
}

fn forbidden(c: char) -> bool {
    c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
}

fn filename(tokens: &[Token], row: &Record, index: usize, date: &str) -> OperationResult<String> {
    let mut name = String::new();
    for token in tokens {
        match token {
            Token::Literal(value) => name += value,
            Token::Date => name += date,
            Token::Index(width) => name += &format!("{index:0width$}"),
            Token::Row(key) => {
                let value = row
                    .get(key)
                    .ok_or_else(|| invalid(format!("Missing filename column: {key}")))?;
                let value: String = value
                    .chars()
                    .map(|c| if forbidden(c) { '_' } else { c })
                    .collect();
                let value = value.trim().trim_end_matches('.');
                if value.is_empty() {
                    return Err(invalid(format!("Empty filename column: {key}")));
                }
                name += value;
            }
        }
    }
    let stem = name
        .split('.')
        .next()
        .unwrap_or("")
        .trim_end_matches([' ', '.'])
        .to_ascii_uppercase();
    let reserved = matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (stem.len() == 4
            && (stem.starts_with("COM") || stem.starts_with("LPT"))
            && matches!(stem.as_bytes()[3], b'1'..=b'9'));
    let mut components = Path::new(&name).components();
    if name.len() > 240
        || name.chars().any(forbidden)
        || reserved
        || !matches!(components.next(), Some(Component::Normal(_)))
        || components.next().is_some()
        || !name.to_ascii_lowercase().ends_with(".pdf")
        || name.ends_with(' ')
        || name.ends_with('.')
    {
        return Err(invalid(
            "Output name must be a portable single .pdf filename (at most 240 UTF-8 bytes)",
        ));
    }
    Ok(name)
}

pub fn run(args: &BatchFillConfig) -> OperationResult<BatchReceipt> {
    run_impl(args, None)
}

fn run_impl(args: &BatchFillConfig, inline: Option<Vec<u8>>) -> OperationResult<BatchReceipt> {
    let start = Instant::now();
    let started = Utc::now();
    let date = match &args.date {
        Some(date) => NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| invalid("Date must be YYYY-MM-DD"))?,
        None => started.date_naive(),
    }
    .format("%Y-%m-%d")
    .to_string();
    let pattern = tokens(&args.name_template)?;
    let template_bytes = fs::read(&args.template)?;
    let is_inline = inline.is_some();
    let data_bytes = match inline {
        Some(bytes) => bytes,
        None => fs::read(&args.data)?,
    };
    let template = FormFillTemplate::from_bytes(&template_bytes)?;
    let rows = records(&data_bytes, &args.data)?;
    if rows.is_empty() {
        return Err(invalid("Data contains no records"));
    }
    let mapping: HashMap<String, String> = match &args.field_map {
        Some(path) => {
            let data = fs::read_to_string(path)?;
            serde_json::from_str(data.trim_start_matches('\u{feff}'))
                .map_err(|e| invalid(format!("Invalid field map: {e}")))?
        }
        None => HashMap::new(),
    };
    let field_names = template.field_names();
    if mapping
        .iter()
        .any(|(key, value)| !field_names.contains(&key.as_str()) || value.is_empty())
    {
        return Err(invalid(
            "Field map must map existing PDF field names to nonempty input columns",
        ));
    }
    fs::create_dir_all(&args.output_dir)?;
    let output_dir = fs::canonicalize(&args.output_dir)?;
    let receipt_path = output_dir.join("execution_receipt.json");
    // Reserve a new batch receipt before producing anything. Never overwrite an
    // existing batch, PDF, symlink target or concurrent writer's output.
    let mut receipt_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&receipt_path)?;
    let mut results = Vec::new();
    for (offset, row) in rows.into_iter().enumerate() {
        let index = offset + 1;
        let outcome = (|| -> OperationResult<(PathBuf, String)> {
            let row = row?;
            let name = filename(&pattern, &row, index, &date)?;
            let values: Record = field_names
                .iter()
                .map(|field| {
                    let column = mapping.get(*field).map(String::as_str).unwrap_or(field);
                    let value = row.get(column).ok_or_else(|| {
                        invalid(format!("Missing column {column} for field {field}"))
                    })?;
                    Ok((field.to_string(), value.clone()))
                })
                .collect::<OperationResult<_>>()?;
            let output = template.fill(&values, !args.keep_editable)?;
            let target = output_dir.join(name);
            let mut temporary = tempfile::NamedTempFile::new_in(&output_dir)?;
            temporary.write_all(&output)?;
            temporary.as_file().sync_all()?;
            temporary
                .persist_noclobber(&target)
                .map_err(|error| PdfError::IoError(error.error))?;
            Ok((target, hash(&output)))
        })();
        results.push(match outcome {
            Ok((path, sha256)) => RowResult {
                index,
                success: true,
                output_path: Some(path),
                output_sha256: Some(sha256),
                error: None,
            },
            Err(error) => RowResult {
                index,
                success: false,
                output_path: None,
                output_sha256: None,
                error: Some(error.to_string()),
            },
        });
    }
    let succeeded = results.iter().filter(|row| row.success).count();
    let failed = results.len() - succeeded;
    let mut operations = vec!["pdf_fill_form"];
    if !args.keep_editable {
        operations.push("pdf_flatten");
    }
    let receipt = BatchReceipt {
        success: failed == 0,
        job_id: format!(
            "fill-batch-{}-{}",
            started.timestamp_micros(),
            std::process::id()
        ),
        started_at: started.to_rfc3339(),
        date,
        template: args.template.clone(),
        data: (!is_inline).then(|| args.data.clone()),
        data_source: if is_inline { "inline_json" } else { "file" },
        template_sha256: hash(&template_bytes),
        data_sha256: hash(&data_bytes),
        receipt_path,
        operations_applied: operations,
        total: results.len(),
        succeeded,
        failed,
        execution_time_ms: start.elapsed().as_millis(),
        results,
    };
    serde_json::to_writer_pretty(&mut receipt_file, &receipt)
        .map_err(|e| invalid(format!("Cannot write receipt: {e}")))?;
    receipt_file.write_all(b"\n")?;
    receipt_file.sync_all()?;
    Ok(receipt)
}
