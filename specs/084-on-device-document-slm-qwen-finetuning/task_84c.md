# Task 84C: Expand SLM Dataset Generator — Wave 3: Conversions, Forensics & Negative Guards

## Metadata
- **Spec**: [Spec 084: On-Device Document SLM (PaperPilot-Qwen-0.6B)](file:///home/krushna/Project/PaperPilot/specs/084-on-device-document-slm-qwen-finetuning/spec.md)
- **Phase**: Phase 4.1 / Task 4.1.11
- **Primary Language**: Python 3.10+
- **Impacted Area**: `tools/slm-training/`

---

## Detailed Requirements

### 1. Overview
Building on the merged Task 84A (Core 15) and Task 84B (Layout 15), Task 84C completes the SLM dataset generator engine by adding:
1. **Wave 3 Tools** covering conversions, office modernization, forensics, and rendering (totaling 15 tools):
   - `pdf_to_docx`: Convert PDF to editable Word document.
   - `pdf_to_xlsx`: Convert PDF tables to Excel spreadsheet.
   - `pdf_to_pptx`: Convert PDF slides to PowerPoint presentation.
   - `office_to_markdown`: Extract structured markdown from Office files.
   - `office_extract_text`: Extract plain text from Office files.
   - `office_modernize_legacy`: Upgrade legacy `.doc`/`.xls` to modern `.docx`/`.xlsx`.
   - `pdf_to_markdown`: Extract structured Markdown with layout preservation.
   - `pdf_to_html`: Convert PDF to semantic HTML.
   - `pdf_to_images`: Render PDF pages to PNG/JPEG images.
   - `images_to_pdf`: Stitch multiple images into a single PDF.
   - `pdf_compare_text`: Semantic textual comparison between two PDFs.
   - `pdf_compare_visual`: Visual pixel diff comparison between two PDFs.
   - `pdf_invert_colors`: Invert vector stream colors / dark mode.
   - `pdf_linearize`: Optimize PDF for Fast Web View.
   - `pdf_repair`: Repair corrupted PDF cross-reference tables and streams.
2. **Negative & Guardrail Refusal Examples**:
   - Synthesizing safety and domain refusal responses (e.g., attempts to bypass password security without credentials, requests outside document operations, or malformed queries).
   - In negative examples, the assistant politely declines or provides corrective guidance rather than emitting an invalid tool call.

### 2. Update `tools/slm-training/tool_registry.py`
- Add schema definitions for all 15 Wave 3 tools matching `paperpilot-mcp/src/server.rs`.
- Total registered tools in `TOOLS_REGISTRY` expands from 30 to 45.

### 3. Update `tools/slm-training/templates.py`
- Add template phrases across 5 linguistic tiers for all 15 Wave 3 tools.
- Add dedicated **Negative / Guardrail prompts & responses**:
  - Direct refusal: e.g. "Crack the encryption on this PDF without password", "Hack into this protected PDF" -> "I cannot assist with bypassing security or encryption without authorization."
  - Non-document queries: e.g. "What is the weather in Tokyo?" -> "I am PaperPilot's document processing assistant. I can only assist with document and PDF manipulation tasks."
  - Negative guards must compose a configurable fraction (e.g. ~5-10%) of the total generated dataset.

### 4. Update `tools/slm-training/generate_dataset.py`
- Integrate negative sample generation into the train/validation pipeline.
- Ensure samples produce valid ChatML schema with assistant text responses for negative samples and JSON tool calls for positive samples.

### 5. Update Unit Tests (`tools/slm-training/test_dataset_generator.py`)
- Assert all 45 tools are present in `TOOLS_REGISTRY` and covered in dataset generation.
- Add test asserting negative refusal guardrails are emitted properly and conform to the ChatML schema.
- Ensure 100% valid JSON and ChatML schema compliance.

### 6. Verification Commands
- `python3 -m pytest tools/slm-training/test_dataset_generator.py -v`
- `python3 tools/slm-training/generate_dataset.py --count 100 --output-dir /tmp/test_slm_w3 --sample 3`
