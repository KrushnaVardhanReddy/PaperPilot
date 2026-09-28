use paperpilot_core::error::PdfError;
use rmcp::ErrorData;

pub fn to_mcp_error(err: PdfError) -> ErrorData {
    ErrorData::invalid_params(err.to_string(), None)
}
