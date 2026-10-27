# Task 84D: Compound Pipeline Recipes, Multi-Step Workflows & Chained Schema Generation

## Metadata
- **Spec**: [Spec 084: On-Device Document SLM (PaperPilot-Qwen-0.6B)](file:///home/krushna/Project/PaperPilot/specs/084-on-device-document-slm-qwen-finetuning/spec.md)
- **Phase**: Phase 5.9.73 / Task 84D
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
   - The generator can pick **ANY arbitrary combination of 2 to 5 compatible tools** from the entire 45+ tool catalog (`TOOLS_REGISTRY`).
   - Synthesizes dynamic multi-step instructions regardless of order (e.g. *"First tool_a, then tool_b, then tool_c"* or *"Do tool_x on this file, then encrypt it, and finally convert it"*).
   - Dynamically glues intermediate outputs (`/tmp/inter_1.pdf` ➔ `/tmp/inter_2.pdf` ➔ `/data/final_output.ext`).
2. **Dedicated Curated Industry Workflow Presets** (at least 15 base recipes):
   - Legal Production: `pdf_bates` ➔ `pdf_watermark` ➔ `document_pii_scan`
   - Clean & OCR Scanned Document: `pdf_remove_blank` ➔ `pdf_auto_rotate` ➔ `pdf_ocr`
   - Redact & Sanitize: `pdf_redact` ➔ `pdf_sanitize` ➔ `pdf_linearize`
   - Secure Distribution: `pdf_page_numbers` ➔ `pdf_encrypt`
   - Print Preparation: `pdf_nup` ➔ `pdf_crop` ➔ `pdf_compress`
   - Collate & Optimize: `pdf_merge` ➔ `pdf_compress`
   - Document Forensics: `pdf_validate_signatures` ➔ `pdf_extract_tables` ➔ `pdf_audit_log`
   - Digital Book Production: `pdf_split_scanned_book` ➔ `pdf_auto_rotate` ➔ `pdf_booklet`
   - Office Archival: `office_modernize_legacy` ➔ `pdf_to_markdown` ➔ `pdf_compress`
   - Dual-Document Interleaving: `pdf_merge_alternate` ➔ `pdf_page_numbers`
   - Form Processing: `pdf_flatten_annotations` ➔ `pdf_stamp` ➔ `pdf_encrypt`
   - Presentation Export: `pdf_to_pptx` ➔ `office_extract_text`
   - Batch Web Optimization: `pdf_repair` ➔ `pdf_compress` ➔ `pdf_linearize`
   - Bilingual Document Split: `pdf_split_by_outline` ➔ `pdf_extract_text`
   - Visual Comparison & Inversion: `pdf_compare_visual` ➔ `pdf_invert_colors`

For every multi-step combination, support varied phrasing:
- Explicit sequence markers: *"Step 1: ..., Step 2: ..., Step 3: ..."*
- Natural conjunctions: *"Please watermark this document, and then compress it, and finally protect it with password..."*
- Inverted/Passive user phrasing: *"Before compressing to /data/out.pdf, make sure to rotate all pages 90 degrees and add Bates stamps"*
- Developer pipelines: *"doc.pdf | rotate angle=90 | compress | encrypt"*

### 4. Parameter Chaining Engine in `generate_dataset.py`
Update `generate_compound_workflow_record()` to properly model intermediate pipelines:
- **First tool**: takes `main_input` (or `main_inputs` for merge operations).
- **Intermediate tools**: input is chained from previous step's output (e.g. `/tmp/step_1.pdf` ➔ `/tmp/step_2.pdf`).
- **Final tool**: writes to `main_output` (e.g. `/data/final_output.pdf`).
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
