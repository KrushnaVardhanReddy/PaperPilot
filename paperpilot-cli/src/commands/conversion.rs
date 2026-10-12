use paperpilot_core::error::OperationResult;
use paperpilot_pdf::operations::conversion::UrlToPdfOperation;
use std::path::PathBuf;

pub fn handle_convert_url(url: &str, output: &PathBuf, delay_ms: Option<u64>) -> OperationResult<()> {
    let op = UrlToPdfOperation::new(
        url.to_string(),
        output.clone(),
        delay_ms,
        Some("A4".to_string()),
        true,
    );
    op.render()
}
