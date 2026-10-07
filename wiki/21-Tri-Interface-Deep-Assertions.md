# Tri-Interface Deep Assertions

## Overview

The Tri-Interface Test Suite in PaperPilot executes tests across the three core interfaces:
1. **💻 CLI** (Command Line Interface via `paperpilot-cli`)
2. **🤖 MCP** (Model Context Protocol via `paperpilot-mcp`)
3. **🌐 REST API** (HTTP endpoints via `paperpilot-gateway`)

To ensure robust behavioral validation of these interfaces, the test suite now performs **Deep Artifact Assertions**. This goes beyond merely checking if a tool exited with code `0`. It rigorously verifies the final output, producing an expected versus actual comparison for every operation.

## Deep Verification Methodology

When an output file or response is generated, the `TestRunner.validate_output` method intercepts the artifact and evaluates it against expected criteria defined in `tools_definitions`.

### Validation per Data Type

#### 1. PDFs (`expected_type: "pdf"`)
- **File Existence & Size**: Ensures the target output path exists and its file size is strictly `> 0 bytes`.
- **Magic Byte Validation**: Reads the first few bytes to guarantee it starts with `%PDF-`.
- **Page Counting (Deep Inspection)**: Scans the raw binary for the number of `/Type /Page` dictionary objects, asserting that the actual page count strictly matches the `expected_pages`.
- **Summary**: Produces an artifact report like: `Valid PDF, 2 pages, 1,711 bytes`.

#### 2. Other Formats (`expected_type: "docx" | "xlsx" | "pptx" | "image" | "text"`)
- **File Existence & Size**: Asserts that the target output exists and size `> 0`.
- **Summary**: Produces a format-aware report like: `Valid XLSX file, 15,200 bytes` or `Valid PNG file, 24,000 bytes`.

#### 3. Directories (`expected_type: "dir"`)
- **Directory Existence**: Asserts the target directory was created.
- **Content Check**: Counts the number of files generated (e.g. for operations like `pdf_burst`).

#### 4. Console & JSON Outputs (`expected_type: "json"`)
- **Parsing**: Attempts to parse the output (`stdout` or JSON-RPC result payload) into a Python dictionary or list.
- **Summary**: Reports the size of the structure (e.g. `Success: 12 keys` or `Success: 5 items`) to verify that meaningful data was extracted from operations like metadata reading, search, and form reading.

## Unified Comparison Matrix

The output report (`reports/TRI_INTERFACE_E2E_100_VERIFIED.md` and `reports/TRI_INTERFACE_E2E_AND_DOCS.md`) leverages these deep assertions to generate a unified comparison matrix for every tool.

This visual matrix compares latency, expected outcome (e.g. `Valid 2-page PDF (%PDF-)`), and the actual validated result (e.g. `Valid PDF, 2 pages, 1,803 bytes`), standardizing parity and tracking differences between local environments and simulated Browser WASM / Cloudflare Edge contexts.
