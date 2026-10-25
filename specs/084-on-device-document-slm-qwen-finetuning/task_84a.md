# Task 84A: Implement Synthetic Dataset Generator Engine (`tools/slm-training`)

## Metadata
- **Spec**: [Spec 084: On-Device Document SLM (PaperPilot-Qwen-0.6B)](file:///home/krushna/Project/PaperPilot/specs/084-on-device-document-slm-qwen-finetuning/spec.md)
- **Phase**: Phase 4.1 / Spec 084
- **Primary Language**: Python 3.10+
- **Impacted Area**: `tools/slm-training/`

---

## Detailed Requirements

### 1. Directory Structure
Create `tools/slm-training/`:
```text
tools/slm-training/
├── generate_dataset.py       # Main synthetic generation CLI
├── tool_registry.py          # Dynamic tool schema & parameter extractor
├── templates.py              # Template generators across 5 linguistic styles
├── test_dataset_generator.py # Unit test suite verifying ChatML JSON output
└── requirements.txt          # Python dependencies (zero heavy ML libraries needed)
```

### 2. Requirements & Dependencies (`requirements.txt`)
Keep it ultra-lightweight:
```text
pydantic>=2.0.0
pytest>=7.0.0
```

### 3. Tool Registry Definition (`tool_registry.py`) — Wave 1 Core Scope
For Task 84A, establish the architecture and implement the **top 15 core operations**:
1. `pdf_merge`: Combines multiple input PDFs into a single document.
2. `pdf_split`: Splits document by pages or ranges.
3. `pdf_compress`: Reduces document size (`screen`, `ebook`, `prepress`, etc.).
4. `pdf_rotate`: Rotates specific pages or all pages by 90, 180, 270 degrees.
5. `pdf_watermark`: Adds text or image watermark with opacity, color, rotation.
6. `pdf_encrypt`: Sets password protection and permissions.
7. `pdf_decrypt`: Removes encryption with password.
8. `pdf_ocr`: Runs OCR text recognition on scanned pages.
9. `pdf_bates_numbering`: Applies legal Bates stamping (prefix, digits, position).
10. `pdf_extract_text`: Extracts raw text streams from pages.
11. `pdf_extract_tables`: Extracts structured tables to JSON/CSV.
12. `pdf_redact`: Redacts bounding boxes or text occurrences.
13. `sanitize`: Strips metadata, hidden layers, JavaScript, and embedded files.
14. `pdf_auto_rotate`: Automatically detects page text orientation and uprights sideways pages.
15. `document_pii_scan`: Scans document for SSN, credit cards, emails, and sensitive entities.

*(Note: Subsequent waves Task 84B and 84C will add the remaining tools to `tool_registry.py`).*

### 4. Template Engine (`templates.py`)
Generate training pairs across 5 styles for the 15 Wave 1 tools:
1. **Direct Commands**: e.g., "Rotate page 3 by 90 degrees clockwise", "Compress this PDF to high compression".
2. **Informal / Casual**: e.g., "hey can u make this pdf smaller it's too big to email", "turn this sideways scan upright".
3. **Domain Legal & Finance Jargon**: e.g., "Apply Bates prefix SEC-DISCOVERY- starting at 1001 to all production documents", "Redact all SSN and PAN numbers and deep sanitize metadata".
4. **Noisy / Typo Robustness**: e.g., "watermrak 'CONFIDENTIAL' on pgs 1-5", "extarct tables to exel", "encrpyt with password pass123".
5. **Multi-Step Workflows**: Compound chained recipes:
   - "Clean this scan, upright sideways pages, and run OCR" $\to$ `[pdf_auto_rotate, sanitize, pdf_ocr]`
   - "Prepare for legal production: Bates stamp, add confidential watermark, and scan for PII" $\to$ `[pdf_bates_numbering, pdf_watermark, document_pii_scan]`

### 5. Dataset Generation CLI (`generate_dataset.py`)
- Accepts arguments:
  - `--count <int>` (default: `25000`)
  - `--output-dir <path>` (default: `dataset/`)
  - `--val-ratio <float>` (default: `0.1` for 10% validation)
  - `--sample <int>` (optional preview mode to stdout)
- Formats every record into standard **ChatML format**:
  ```json
  {
    "messages": [
      {
        "role": "system",
        "content": "You are PaperPilot's on-device neural assistant. Convert user instructions into strict JSON tool calls."
      },
      {
        "role": "user",
        "content": "Stamp 'DRAFT' diagonally in red across all pages with 30% opacity"
      },
      {
        "role": "assistant",
        "content": "{\"tool\": \"pdf_watermark\", \"parameters\": {\"text\": \"DRAFT\", \"color\": \"#FF0000\", \"opacity\": 0.3, \"rotation\": 45, \"pages\": \"all\"}}"
      }
    ]
  }
  ```
- Automatically splits into `train.jsonl` and `val.jsonl`.
- Ensures 100% valid JSON on every single generated line.

### 6. Documentation Cross-Reference & Tool Schema Accuracy
- Cross-reference all tool declarations and schemas against:
  - `paperpilot-mcp/src/server.rs` (all 75+ registered MCP tools)
  - `apps/desktop/src/lib/types/toolList.ts` (desktop tool definitions)
  - `docs/TRI_INTERFACE_E2E_AND_DOCS.md` (living CLI/REST/MCP tool usage and flags)
- Ensure all tool identifiers, parameters, and defaults match actual codebase implementations.
- Provide a `tools/slm-training/README.md` with instructions on running tests and dataset generation.

### 7. Automated Unit Tests (`test_dataset_generator.py`)
- Runs with `pytest tools/slm-training/test_dataset_generator.py -v`.
- Asserts that generated lines parse as valid JSON.
- Asserts ChatML message structure (`system`, `user`, `assistant`).
- Asserts that all registered tools have at least $N$ diverse training examples.
- Asserts multi-step workflows parse correctly into arrays of steps.
- Asserts representation across all 5 linguistic styles (direct, casual, domain jargon, typo-tolerant, compound).

---

## Verification & Acceptance Criteria
1. `python3 -m venv .venv && pip install -r tools/slm-training/requirements.txt`
2. `pytest tools/slm-training/test_dataset_generator.py -v` passes 100%.
3. `python3 tools/slm-training/generate_dataset.py --count 1000 --output-dir /tmp/paperpilot_test_dataset --sample 3` runs cleanly and generates valid `train.jsonl` and `val.jsonl`.
4. Validated zero external GPU or heavyweight CUDA dependencies are required.
