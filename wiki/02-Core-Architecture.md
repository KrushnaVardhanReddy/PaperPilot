# Core Architecture

## Error Handling

PaperPilot core uses a robust error handling strategy based on the `thiserror` crate. We define a comprehensive `PdfError` enum in `paperpilot-core/src/error.rs` to abstract away specific library errors and provide a unified error interface.

Variants of `PdfError` include:
- `IoError`: Wraps standard `std::io::Error`.
- `ParseError`: Represents errors during parsing or processing PDF data.
- `CorruptFile`: Indicates that a PDF file is corrupt or malformed.
- `UnsupportedOperation`: Thrown when a requested operation is not supported by the underlying implementation.
- `Other`: A catch-all for any other errors.

We also expose a generic type alias for convenience: `pub type OperationResult<T> = Result<T, PdfError>;`

## Event Models

To track the progress of long-running operations or jobs, we define a standardized event model in `paperpilot-core/src/events.rs`.

The primary struct is `JobProgress`, which contains:
- `job_id`: A unique string identifying the job.
- `percentage`: An 8-bit unsigned integer representing the completion percentage (0-100).
- `current_step`: A 32-bit unsigned integer indicating the current step number.
- `total_steps`: A 32-bit unsigned integer denoting the total number of steps.
- `status_message`: A descriptive string for the current status.

The `JobProgress` struct implements `Serialize`, `Deserialize`, `Debug`, and `Clone` to facilitate easy passing across boundaries (e.g., to the Tauri frontend or through MCP).
