# PDF Overlay and Underlay Engine

The `pdf_overlay_underlay` engine is responsible for seamlessly combining two PDF documents, allowing one (the template) to serve as either a foreground overlay or a background underlay for the other (the target).

## Architecture

This tool adheres to the PaperPilot 'Single Core Runner + 5 Thin Wrappers' pattern. The core engine is implemented entirely in pure Rust using `lopdf`, providing memory-safe and efficient document manipulation.

### Core Logic (`paperpilot-pdf/src/operations/pdf_overlay_underlay.rs`)

1.  **Template Loading:** The template PDF is loaded either from a file path (`template_path`) or from base64-encoded inline data (`template_data`).
2.  **XObject Importation:** The first page of the template document is extracted and converted into a Form XObject. This is a critical step for efficiency, as it ensures the template data is embedded only once within the target document, regardless of how many pages it is applied to.
3.  **Page Iteration:** The engine iterates through the target document's pages, filtering them according to the `pages` option (e.g., "all", "first", "last", or specific indices).
4.  **Content Modification:** For each matching page:
    *   The Form XObject is registered within the page's `/Resources` dictionary.
    *   The page's content stream (`/Contents`) is decoded.
    *   A drawing operation (`Do`) referencing the XObject is inserted into the content stream.
    *   For `mode: "underlay"`, the operation is *prepended* so it renders first (behind existing content).
    *   For `mode: "overlay"`, the operation is *appended* so it renders last (on top of existing content).
    *   State saving (`q`) and restoring (`Q`) operators are utilized to prevent the template's graphics state from affecting the original page content.

## Integration

The engine is exposed across all five PaperPilot architectural surfaces:
-   **CLI:** `pdf-overlay-underlay` subcommand.
-   **MCP:** `pdf_overlay_underlay` JSON-RPC tool.
-   **Gateway:** REST API endpoint `/api/v1/pdf/overlay-underlay`.
-   **WASM:** Client-side execution in the browser.
-   **Desktop/Mobile:** Tauri integration via IPC.

## Performance Considerations

By utilizing Form XObjects, the engine minimizes the output file size bloat when a template is applied to multiple pages. Instead of duplicating the template's content stream and resources for every page, the document contains a single copy of the template that is merely referenced by each page's content stream.