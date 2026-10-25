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

### 3. Tool Registry Definition (`tool_registry.py`)
- Define all PaperPilot tools (60+ tools from `paperpilot-mcp` and desktop toolList) with:
  - `tool_name` (e.g., `pdf_merge`, `pdf_compress`, `pdf_bates_numbering`, `pdf_watermark`, `pdf_ocr`, `pdf_invert_colors`, `pdf_stitch_canvas`, `pdf_attachments`, `document_pii_scan`, etc.)
  - `category` (`pages`, `security`, `conversion`, `organize`, `enhance`, etc.)
  - `parameters`: typed definitions with defaults and valid ranges.

### 4. Template Engine (`templates.py`)
Generate training pairs across 5 styles:
1. **Direct Commands**: e.g., "Rotate page 3 by 90 degrees clockwise", "Compress this PDF to high compression".
2. **Informal / Chat Phrasing**: e.g., "hey can u make this pdf smaller it's too big to email", "turn this sideways scan upright".
3. **Domain Legal & Finance Jargon**: e.g., "Apply Bates prefix SEC-DISCOVERY- starting at 1001 to all production documents", "Redact all SSN and PAN numbers and deep sanitize metadata".
4. **Noisy / Typo Robustness**: e.g., "watermrak 'CONFIDENTIAL' on pgs 1-5", "extarct tables to exel".
5. **Multi-Step Workflows**: Compound chained recipes:
   - "Clean this scan, deskew it, and run OCR" $\to$ `[pdf_auto_rotate, sanitize, pdf_ocr]`
   - "Prepare for legal production: Bates stamp, add confidential watermark, and linearize" $\to$ `[pdf_bates_numbering, pdf_watermark, pdf_linearize]`

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

### 6. Automated Unit Tests (`test_dataset_generator.py`)
- Runs with `pytest tools/slm-training/test_dataset_generator.py`.
- Asserts that generated lines parse as valid JSON.
- Asserts ChatML message structure (`system`, `user`, `assistant`).
- Asserts that all registered tools have at least $N$ diverse training examples.
- Asserts multi-step workflows parse correctly into arrays of steps.

---

## Verification & Acceptance Criteria
1. `python3 -m venv .venv && pip install -r tools/slm-training/requirements.txt`
2. `pytest tools/slm-training/test_dataset_generator.py` passes 100%.
3. `python3 tools/slm-training/generate_dataset.py --count 1000 --output-dir /tmp/paperpilot_test_dataset` runs cleanly and generates valid `train.jsonl` and `val.jsonl`.
