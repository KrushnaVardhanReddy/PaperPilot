# Penta-Interface Declarative Workflow Engine

The `paperpilot-core` crate exposes the `workflow::SavedWorkflow` schema which enables fully declarative multi-step execution.
This RAG/Extraction schema is deeply executed natively in Rust through `paperpilot-pdf/src/operations/workflow.rs`, bypassing any need for Node or CLI external invocations.
Since everything parses back to the single `run_workflow_bytes` Rust interface, it ensures Penta-Interface Parity.

## Architecture

1. **Schema (`paperpilot-core::workflow`)**: Contains `SavedWorkflow`, `WorkflowStep`, and `WorkflowExecutionReport`.
2. **Execution (`paperpilot-pdf::operations::workflow`)**: The single source-of-truth runner.
3. **Five Thin Interfaces**:
   - Native CLI (`paperpilot workflow run`)
   - MCP Server (`document_workflow` tool)
   - REST Gateway (`/api/v1/workflow/run` and `/api/v1/workflow/run-json`)
   - Browser WASM Engine (`WasmPdfEngine.workflow()`)
   - Cloudflare Edge (`POST /api/v1/workflow`)

## Presets

Built-in presets automatically handle common, multi-step actions. Examples:
- `preset-legal-discovery`: Bates numbering -> Watermark -> Linearize
- `preset-clean-scan`: Remove Blank Pages -> Page Numbers -> Compress
- `preset-redact-protect`: Redact -> Flatten -> Encrypt
