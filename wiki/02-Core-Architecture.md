# Core Architecture

## Architectural Boundaries: Traits

The core architecture uses traits to establish boundaries between the application logic and specific PDF engine implementations.

- **`PdfDocument`**: Represents a PDF document and exposes operations that can be performed on it. This interface acts as an abstraction layer, separating core logic from specific implementations (like `lopdf` or `pdfium-render`). It defines standard methods like `page_count` and `save`.

- **`PdfOperation`**: Represents an operation that can be executed against a `PdfDocument`. Operations implement the `execute` method, allowing for various document manipulations without coupling the operations to concrete document types.

## Error Strategy

Error handling relies on a `PdfError` enum and an `OperationResult` type alias for typed errors. This ensures errors can be programmatically matched without relying on string panics, maintaining a robust boundary across operations.
