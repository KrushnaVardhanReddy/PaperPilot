# Task 84B: Expand SLM Dataset Generator — Wave 2: Organization & Layout Tools

## Metadata
- **Spec**: [Spec 084: On-Device Document SLM (PaperPilot-Qwen-0.6B)](file:///home/krushna/Project/PaperPilot/specs/084-on-device-document-slm-qwen-finetuning/spec.md)
- **Phase**: Phase 4.1 / Task 4.1.10
- **Primary Language**: Python 3.10+
- **Impacted Area**: `tools/slm-training/`

---

## Detailed Requirements

### 1. Overview
Building on the successfully merged Task 84A (`tools/slm-training/`), Task 84B expands `tool_registry.py` and `templates.py` to cover **15 layout, page organization, and structure tools**:
1. `pdf_nup`: N-up page grid tiling (2-up, 4-up, 9-up).
2. `pdf_booklet`: Saddle-stitch print booklet re-ordering.
3. `pdf_scale_pages`: Resize and scale page dimensions (e.g. A4 to US-Letter).
4. `pdf_crop`: Crop page bounding boxes with margins.
5. `pdf_reorder`: Re-order page sequences (e.g., "1, 3, 2, 4").
6. `pdf_delete_pages`: Remove specified pages or ranges.
7. `pdf_reverse`: Invert entire document page order.
8. `pdf_stamp`: Apply image or dynamic stamps onto pages.
9. `pdf_overlay_underlay`: Background underlay or foreground overlay.
10. `pdf_stitch_canvas`: Continuous horizontal or vertical canvas stitching.
11. `pdf_attachments`: Embed or extract PDF portfolio attachments.
12. `pdf_validate_signatures`: Cryptographic and visual signature audit.
13. `pdf_remove_signatures`: Strip digital signatures and locks.
14. `pdf_flatten_annotations`: Flatten form fields and comments into static text.
15. `pdf_header_footer`: Add dynamic running headers and footers.

### 2. Update `tool_registry.py`
- Register all 15 tools with parameter types, descriptions, and defaults matching `paperpilot-mcp/src/server.rs`.

### 3. Update `templates.py`
- Add template phrases across 5 linguistic tiers:
  1. Direct commands: "Create a 2-up booklet for printing", "Stitch all pages vertically into a continuous canvas".
  2. Casual phrasing: "make this fit on a booklet", "delete page 2 and 4 please".
  3. Domain jargon: "Prepare saddle-stitch print booklet with creep compensation", "Extract portfolio embedded filespec attachments".
  4. Typo resilience: "boklet print 4up", "stich canvas veritcal".
  5. Multi-step workflows: "Crop margins, scale to Letter, and stitch vertically".

### 4. Update Unit Tests (`test_dataset_generator.py`)
- Assert all 30 tools (Wave 1 + Wave 2) are present in generated dataset.
- Ensure 100% valid JSON and ChatML schema compliance.

### 5. Verification
- `pytest tools/slm-training/test_dataset_generator.py -v`
- `python3 tools/slm-training/generate_dataset.py --count 100 --output-dir /tmp/test_dataset --sample 3`
