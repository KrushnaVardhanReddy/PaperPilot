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

## Architectural Boundaries: Traits

The core architecture uses traits to establish boundaries between the application logic and specific PDF engine implementations.

- **`PdfDocument`**: Represents a PDF document and exposes operations that can be performed on it. This interface acts as an abstraction layer, separating core logic from specific implementations (like `lopdf` or `pdfium-render`). It defines standard methods like `page_count` and `save`.

- **`PdfOperation`**: Represents an operation that can be executed against a `PdfDocument`. Operations implement the `execute` method, allowing for various document manipulations without coupling the operations to concrete document types.
