# Technical Implementation Plan: Core Traits and Abstractions

**Feature**: `specs/001-core-traits-abstractions/spec.md`

## 1. System Architecture

The core abstractions will be implemented in the `paperpilot-core` crate. They form the foundational API boundary between the core logic and the specific PDF backend implementations (like `lopdf` or `pdfium-render`).

- **Core Module (`paperpilot-core/src/traits.rs`)**: Contains the `PdfDocument` and `PdfOperation` definitions.
- **Error Module (`paperpilot-core/src/error.rs`)**: Contains the strongly typed `PdfError` enum using `thiserror`.
- **Event Module (`paperpilot-core/src/events.rs`)**: Contains `JobProgress`.

## 2. Component Design

### 2.1 Error Handling (`error.rs`)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("PDF parsing error: {0}")]
    ParseError(String),
    #[error("Corrupt file: {0}")]
    CorruptFile(String),
    #[error("Operation not supported by backend")]
    UnsupportedOperation,
    #[error("Other error: {0}")]
    Other(String),
}

pub type OperationResult<T> = Result<T, PdfError>;
```

### 2.2 Core Traits (`traits.rs`)

```rust
use crate::error::OperationResult;
use std::path::Path;

pub trait PdfDocument {
    fn page_count(&self) -> OperationResult<u32>;
    fn save(&self, path: &Path) -> OperationResult<()>;
    // Future metadata access methods can be added here
}

pub trait PdfOperation {
    /// Executes the operation on a given document, modifying it in place or returning a new document state
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()>;
}
```

### 2.3 Progress Events (`events.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobProgress {
    pub job_id: String,
    pub percentage: u8,
    pub current_step: u32,
    pub total_steps: u32,
    pub status_message: String,
}
```

## 3. Data Flow

1. An operation (e.g., `MergeOperation`) implements the `PdfOperation` trait.
2. The executing agent instantiates the operation with its parameters.
3. The engine calls `operation.execute(&mut document)`.
4. The execution returns an `OperationResult<()>`.
5. Progress can optionally be communicated back via Tokio MPSC channels emitting `JobProgress` events.

## 4. Testing Strategy

- **Mock Implementation**: Create a `MockPdfDocument` in tests that implements `PdfDocument`.
- **Error Propagation**: Write unit tests to trigger mock IO/Parse errors and assert that the `OperationResult` safely bubbles up `PdfError` rather than panicking.
- **Operation Contract**: Write a test for a dummy `PdfOperation` (e.g., `DummyRotate`) ensuring the `execute` method signature correctly binds to `&mut dyn PdfDocument`.
