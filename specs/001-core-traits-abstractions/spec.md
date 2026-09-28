# Feature Specification: Core Traits and Abstractions

**Feature Branch**: `[001-core-traits-abstractions]`

**Created**: 2026-09-28

**Status**: Draft

**Input**: User description: "Phase 1.2: Core Traits and Abstractions. Design internal interfaces (PdfDocument, PdfOperation, OperationResult, error types, JobProgress) to prevent tight coupling to a specific PDF library."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Abstract PDF Document Interface (Priority: P1)

Developers interacting with PaperPilot need an abstract way to represent an open PDF document, regardless of whether the underlying engine is `lopdf` or `pdfium-render`.

**Why this priority**: It is the foundation for all document interactions.
**Independent Test**: Can be tested by implementing a mock document and ensuring the interface allows accessing page count and basic metadata.

**Acceptance Scenarios**:
1. **Given** a PDF loaded via an engine, **When** the developer queries the page count, **Then** the interface provides a uniform method to return the count.
2. **Given** a PDF loaded via an engine, **When** the developer saves it, **Then** the document is written to disk consistently.

### User Story 2 - Standardized PDF Operations (Priority: P1)

Developers need a uniform `PdfOperation` interface that every specific operation (merge, split, rotate) will implement.

**Why this priority**: It establishes the contract that the AI planner and the Rust execution engine will rely on to execute tasks safely.
**Independent Test**: Can be tested by verifying that multiple disparate operation implementations conform to the same execution signature.

**Acceptance Scenarios**:
1. **Given** an operation like `Merge`, **When** executed against a `PdfDocument`, **Then** it accepts standard parameters and returns an `OperationResult`.

### User Story 3 - Strongly Typed Error Handling (Priority: P2)

The system must handle errors gracefully without string panics.

**Why this priority**: Rust's safety guarantees require typed errors to ensure the GUI and AI layers can interpret and recover from failures.
**Independent Test**: Can be tested by triggering an error (e.g., missing file) and verifying a typed `OperationResult` is returned.

**Acceptance Scenarios**:
1. **Given** a corrupt PDF file, **When** an operation is attempted, **Then** the system returns a strongly typed error (e.g., `PdfError::CorruptFile`) rather than crashing.

### User Story 4 - Asynchronous Job Progress (Priority: P3)

Long-running operations need to report progress events to the UI/CLI.

**Why this priority**: It provides crucial UX feedback, though the core engine can technically function synchronously without it.
**Independent Test**: Can be tested by observing progress events emitted during a mock long-running operation.

**Acceptance Scenarios**:
1. **Given** a large document being processed, **When** the engine makes progress, **Then** it emits a `JobProgress` event with the current percentage.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST define a `PdfDocument` trait that abstracts opening, saving, page counting, and metadata extraction.
- **FR-002**: System MUST define a `PdfOperation` trait with a uniform `execute` interface for all transformations.
- **FR-003**: System MUST define an `OperationResult` enum and custom error types for typed error handling (no string panics).
- **FR-004**: System MUST define a `JobProgress` event type to be used by the async job runner in later phases.

### Key Entities

- **PdfDocument**: Abstract representation of an open PDF file.
- **PdfOperation**: Abstract action to perform on a document.
- **OperationResult**: Result type containing either a success payload or a typed `PdfError`.
- **JobProgress**: Struct representing progress (e.g., percentage, current page, total pages).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All future PDF operations (Merge, Split, etc.) can be implemented exclusively using the `PdfOperation` trait without bypassing it.
- **SC-002**: No instances of `panic!` or `unwrap()` exist in the error path of the core engine.
- **SC-003**: The architecture allows swapping the underlying PDF library (e.g., `lopdf` to `pdfium-render`) without modifying the `PdfOperation` trait.

## Assumptions

- The `PdfDocument` trait will initially wrap the selected `lopdf` and `pdfium-render` libraries.
- The `JobProgress` event will be compatible with Tokio async channels (to be implemented in Phase 3).
