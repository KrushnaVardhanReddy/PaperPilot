# SLM compound workflow stabilization

- Date: October 28, 2026 (Asia/Hong_Kong)
- Branch: `fix/slm-compound-workflow-validation`
- Base: `origin/feature/dev` at `51c59fd`
- Status: implemented and locally verified; proposed for review on this branch, not merged.

## Result

All 15 curated recipes now describe compatible artifact flows. Synthetic single
and compound labels are validated against 42 schemas captured from the actual
MCP server. Reports no longer replace source PDFs; directories cannot be used as
scalar PDF inputs; Office exports use their actual extensions. Encryption and
decryption share the specified password. Merge receives at least two inputs.
Settings appearing in labels are included in requests, and refusal labels come
from guardrail templates rather than randomly selected benign requests.

The training subset deliberately excludes obsolete/unregistered aliases,
`pdf_extract_tables` (registered but absent from dispatch), and
`pdf_stamp` / `pdf_overlay_underlay` (their current MCP handlers rely on test
fixture paths rather than advertised image/template arguments). This does not
remove runtime tools. Those handlers need separate fixes before training resumes
for them. The catalog and refresh procedure are documented in
[`tools/slm-training/README.md`](../tools/slm-training/README.md).

## Reproduced runtime defects and fixes

1. `pdf_remove_blank` required nonexistent `input_a`/`input_b` fields in its MCP
   schema. It now requires the handler's `input` and `output` arguments.
2. `pdf_compare` required an undeclared `input` argument. It now requires
   `input_a` and `input_b`; its description correctly identifies the JSON output.
3. PDF → PPTX → Office text extraction failed with “officeDocument relationship
   not found”. PPTX export now writes package and presentation relationships,
   the presentation manifest, correct content types, and text-box structure.
   The real Office reader can extract the original PDF text from this artifact.
4. Encrypt → decrypt → compress returned success while producing a PDF with no
   readable catalog/pages. lopdf 0.45 only partially parses password-protected
   input when loaded without a password. Decryption now reloads encrypted source
   bytes with the password before export, and clears incremental encryption
   state. Empty-user-password files are also handled. Real output readers verify
   page counts and text; wrong passwords still fail.

Core fixes live in the existing operation implementations, so all wrappers use
the same corrected PPTX and decryption behavior.

## Verification performed

| Check | Observed result |
| --- | --- |
| `python3 -m pytest tools/slm-training -q` with `PAPERPILOT_MCP_BIN` set | **223 passed, 0 skipped**: 207 data/contract checks, 5 inference-evaluator checks, 11 real MCP cases |
| `sync_tool_schemas.py --check --mcp-bin ...` | **42/42** schemas match the actual stdio server |
| `generate_dataset.py --count 10000 --seed 84` | **10,000** labels validated; 9,000 train / 1,000 validation; 5,042 single / 3,954 compound / 1,004 refusals; all 42 tools covered |
| `cargo build -p paperpilot-mcp -j 2` | Passed |
| `cargo test -p paperpilot-pdf --lib operations::decrypt::tests -j 2` | **3 passed**, including reloaded encrypted files and empty-user-password files |
| `cargo test -p paperpilot-pdf --lib operations::conversion::tests::test_pdf_to_pptx -j 2` | **1 passed**, including package parts and actual Office text extraction |
| Independent `python-pptx` reader on the generated presentation | **4 slides** opened; all four pages' text preserved |
| `cargo test -p paperpilot-mcp --lib workflow_tool_schemas_require_declared_handler_inputs -j 2` | **1 passed** |
| `rustfmt --edition 2024 --check` on changed Rust files | Passed |
| Regenerate handbook; `npm --prefix apps/docs run build` | Passed: **88 pages**, **87 pages indexed** by Pagefind |
| `git diff --check` | Passed |

The 11 real MCP cases cover merge/compress, watermark/encrypt, signature and PII
reports followed by PDF modification, PPTX/text export, chapter extraction,
blank-page removal, comparison followed by watermark, password-protected and
empty-password decrypt/compress round trips, and two expected failures for
missing/malformed files. Fixtures are genuine PDFs synthesized with pypdf.
Neither document bytes nor tool responses are mocked. Outputs are independently
read with pypdf, JSON readers, ZIP readers and the actual Office tool.

On this Windows sandbox, pytest used a dedicated workspace `--basetemp` directory.
The first sandboxed documentation build generated pages but Pagefind could not
write its index; rerunning with normal permissions completed indexing successfully.
Rust emitted incremental-cache hard-link warnings and used its normal copy
fallback. These warnings did not affect the verified outputs.

Machine-readable evidence: [`slm-workflow-validation/verification.json`](slm-workflow-validation/verification.json).

## Model evaluation and remaining scope

No fine-tuned weights, PyTorch, or Transformers are installed in this workspace.
**The actual neural model was not run, retrained, or republished.** The 223
checks are not a claim about model accuracy. The new local inference benchmark
contains 6 single requests, 15 compound workflows and 1 refusal. It validates JSON
shape, registered names, required parameters, artifact compatibility, and tool
order, writes a report, and returns failure on invalid outputs. It loads local
weights only and fails if fine-tuned weights are missing instead of silently
testing an untrained base model.

Before publishing new weights, regenerate the corrected training data, retrain,
and run `infer_local.py --model-dir /local/merged-model --report ...`. The 15
recipes are all schema/type-checked; the execution suite covers the cases above,
not every OCR/legacy Office dependency or every tool in the catalog. Directory
fan-out remains unsupported. Full workspace/desktop suites and remote GitHub CI
were not run for this scoped change.

The new `.github/workflows/slm-training.yml` runs data checks, builds the real MCP
server, checks schema drift, executes document workflows and runs the targeted
Rust regressions on relevant PRs. Its remote execution remains pending submission.

Tool reference pages, the generated handbook, Task 84D, the Phase 4 tracker and
`HANDOFF.md` are synchronized. `graphify-rs build .` was attempted, but the CLI is
not installed and no graph exists in this checkout. No graph artifacts changed.
