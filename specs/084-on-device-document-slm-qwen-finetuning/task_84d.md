# Task 84D: Compound Pipeline Recipes, Multi-Step Workflows & Chained Schema Generation

## Metadata
- **Spec**: [Spec 084: On-Device Document SLM (PaperPilot-Qwen-0.6B)](file:///home/krushna/Project/PaperPilot/specs/084-on-device-document-slm-qwen-finetuning/spec.md)
- **Phase**: Phase 5.9.73 / Task 84D
- **Status**: ✅ **Completed & Verified** (PR #277)
- **Primary Language**: Python 3.10+
- **Impacted Area**: `tools/slm-training/`

---

## Detailed Requirements

### 1. Context & Motivation
Testing the fine-tuned PaperPilot SLM model revealed that while single-tool operations achieve 100% exact accuracy, complex multi-step user prompts (e.g., *"Prepare document for court: Bates stamp, confidential watermark, and scan for PII"*) currently collapse into a single dominant tool dictionary because only ~5% of training samples were multi-step compound recipes.

To make the on-device assistant reliably reason about complex multi-step pipelines, this task expands compound workflow generation, parameter chaining, and structured multi-tool JSON array outputs.

### 2. Update System Prompt Specification
In `generate_dataset.py`, `templates.py`, and `infer_local.py`, ensure the system prompt explicitly establishes the output structure:
```
You are PaperPilot's on-device neural assistant. Convert user instructions into strict JSON tool calls. For single operations, emit a single JSON object. For multi-step workflows, emit a JSON array of tool calls chained in execution order.
```

### 3. Expand Compound Workflow Recipes & Dynamic Sequencing in `templates.py` & `generate_dataset.py`
The generator must NOT only rely on static fixed templates. It must synthesize:
1. **Dynamic Arbitrary Tool Chaining Engine**:
   - The generator picks **2 to 5 compatible tools** from the 42-tool verified training subset (`TOOLS_REGISTRY`). The snapshot is checked against live MCP `tools/list`; unsupported aliases, the undispatched table extractor, and fixture-dependent stamp/overlay handlers are excluded (see `tools/slm-training/README.md`).
   - Synthesizes dynamic multi-step instructions regardless of order (e.g. *"First tool_a, then tool_b, then tool_c"* or *"Do tool_x on this file, then encrypt it, and finally convert it"*).
   - Dynamically glues intermediate outputs (`/tmp/inter_1.pdf` ➔ `/tmp/inter_2.pdf` ➔ `/data/final_output.ext`).
2. **Dedicated Curated Industry Workflow Presets** (at least 15 base recipes):
   - Legal Production: `pdf_bates` ➔ `pdf_watermark` ➔ `document_pii_scan`
   - Clean & OCR Scanned Document: `pdf_remove_blank` ➔ `pdf_auto_rotate` ➔ `pdf_ocr`
   - Redact & Sanitize: `pdf_redact` ➔ `pdf_sanitize` ➔ `pdf_linearize`
   - Secure Distribution: `pdf_page_numbers` ➔ `pdf_encrypt`
   - Print Preparation: `pdf_nup` ➔ `pdf_crop` ➔ `pdf_compress`
   - Collate & Optimize: `pdf_merge` ➔ `pdf_compress`
   - Document Forensics: `pdf_validate_signatures` ➔ `document_pii_scan` ➔ `pdf_compare` (separate reports reading the original PDF)
   - Digital Book Production: `pdf_extract_pages` ➔ `pdf_auto_rotate` ➔ `pdf_booklet`
   - Office Archival: `office_modernize_legacy` ➔ `office_to_markdown`
   - Page Ordering: `pdf_reorder_pages` ➔ `pdf_header_footer`
   - Form Processing: `pdf_flatten` ➔ `pdf_watermark` ➔ `pdf_encrypt`
   - Presentation Export: `pdf_to_pptx` ➔ `office_extract_text`
   - Batch Web Optimization: `pdf_repair` ➔ `pdf_compress` ➔ `pdf_linearize`
   - Chapter Text Extraction: `pdf_extract_pages` ➔ `pdf_extract_text`
   - Comparison & Watermark: `pdf_compare` ➔ `pdf_watermark` (watermark reads the source PDF, not the JSON report)

For every multi-step combination, support varied phrasing:
- Explicit sequence markers: *"Step 1: ..., Step 2: ..., Step 3: ..."*
- Natural conjunctions: *"Please watermark this document, and then compress it, and finally protect it with password..."*
- Inverted/Passive user phrasing: *"Before compressing to /data/out.pdf, make sure to rotate all pages 90 degrees and add Bates stamps"*
- Developer pipelines: *"doc.pdf | rotate angle=90 | compress | encrypt"*

### 4. Parameter Chaining Engine in `generate_dataset.py`
Update `generate_compound_workflow_record()` to properly model intermediate pipelines:
- **First tool**: takes `main_input` (or `main_inputs` for merge operations).
- **Intermediate tools**: document-transforming outputs feed the next compatible document input, retaining their actual extensions. Report steps write separate artifacts and preserve the latest source document. Directory-producing operations are terminal until explicit fan-out bindings exist.
- **Final tool**: writes to `main_output` with the actual output type (`.pdf`, `.json`, `.txt`, `.md`, Office format, or directory).
- **Tool-specific parameters**: preserved from user query (e.g. password `"Secret1234"`, watermark text `"CONFIDENTIAL"`, Bates prefix `"CASE-2026-"`).

### 5. Configurable Dataset Distribution
In `generate_dataset.py`:
- Set default distribution:
  - **50%** Single-tool operations
  - **40%** Multi-step compound workflows (JSON array output)
  - **10%** Negative guardrails & safety refusals
- Add CLI flag `--compound-ratio` (default `0.4`).

### 6. Unit Testing (`test_dataset_generator.py`)
Add unit tests verifying:
1. `test_compound_workflow_array_schema`: All compound workflows emit valid JSON arrays `[...]` with length $\ge 2$.
2. `test_intermediate_parameter_chaining`: Assert intermediate files chain sequentially (`/tmp/step_1.pdf` $\to$ `/tmp/step_2.pdf` $\to$ final output).
3. `test_dataset_distribution_ratios`: Generate 1,000 test samples and verify single, compound, and negative ratios match configured thresholds within $\pm 5\%$.

### 7. Verification Commands
- `python3 -m pytest tools/slm-training/test_dataset_generator.py -v`
- `python3 tools/slm-training/generate_dataset.py --count 100 --output-dir /tmp/test_slm_compound --sample 5`

### October 28 stabilization verification

- `python3 -m pytest tools/slm-training -q` (set `PAPERPILOT_MCP_BIN` to include real execution tests).
- `python3 tools/slm-training/sync_tool_schemas.py --check --mcp-bin target/debug/paperpilot-mcp`.
- Local fine-tuned model generation is evaluated separately with `infer_local.py --model-dir ...`; dataset and executor checks do not establish model accuracy.
- Evidence and limitations: [`reports/SLM_COMPOUND_WORKFLOW_VALIDATION_REPORT.md`](../../reports/SLM_COMPOUND_WORKFLOW_VALIDATION_REPORT.md).
