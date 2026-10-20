# Spec 079: Universal Pipeline Parity & Comprehensive Parameter Configuration

## 1. Context & Goal
PaperPilot provides a multi-step automation pipeline on both Desktop (`apps/desktop`) and Web Embed (`apps/embed`).
While basic tools (rotate, watermark, compress) execute, user testing identified gaps:
1. **Interactive Config Panel (Desktop)**:
   - When users add tools like `pdf_crop`, `pdf_bates`, `pdf_header_footer`, `pdf_redact`, `pdf_encrypt`, or `pdf_extract_pages`, they need full, intuitive input fields with validation and live state updates.
2. **Intermediate Data Hand-off (Desktop)**:
   - Intermediate tools that generate multiple files (like `pdf_split`) or conversions must reliably pass output files to downstream tools without file/directory type mismatches.
3. **Embed & WASM Pipeline Parity (`apps/embed`)**:
   - `apps/embed/src/api.ts` `runPipeline` must support the complete list of client-side operations:
     `rotate`, `watermark`, `extract_pages`/`extract`/`split`, `delete_pages`, `reorder_pages`, `compress`, `flatten`, `crop`, `encrypt` (metadata dictionary password setup), and `metadata`.
4. **End-to-End Test Suite**:
   - Automated tests across Desktop and Embed verifying multi-step pipelines with parameter inputs.
