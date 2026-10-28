# PaperPilot SLM training and workflow validation

This local tool generates schema-checked ChatML data for the PaperPilot Qwen
assistant. Single requests produce one tool-call object; compound requests
produce an ordered array of 2–5 calls. Defaults remain 50% single operations,
40% compound workflows, and 10% guardrail refusals.

## Supported catalog and artifact contracts

The training catalog contains **42 real MCP tool names**. Its parameter schemas
come from a checked-in `mcp_tool_schemas.json` snapshot of the server's
`tools/list` response. `workflow.py` additionally describes actual artifact types:
PDF, encrypted PDF, Office formats, Markdown, HTML, text, JSON, and directories.
A filename ending in `.pdf` cannot disguise a JSON report as a PDF input.

The 15 curated recipes and random compatible sequences use these rules:

- Reports (PII, signatures, comparisons, text extraction) keep the source document
  available for subsequent operations. They are written to separate typed paths.
- Office exports can feed Office tools: PDF → PPTX → extracted text, or legacy
  Office → modern Office → Markdown.
- Split/image-extraction directories are terminal. Directory fan-out requires an
  explicit binding design before it can participate in scalar document chains.
- Encryption feeds decryption with the same password, or ends the document chain.
- Merge always receives at least two inputs. Required parameters and settings
  such as passwords, watermark text and Bates prefixes are present in the prompt.
- Refusal labels are generated from actual guardrail templates; benign prompts
  are not randomly relabeled as refusals.

Obsolete training aliases (`pdf_reorder`, `pdf_flatten_annotations`,
`pdf_to_images`, `images_to_pdf`, `pdf_compare_text`, `pdf_compare_visual`) have
been replaced with registered tool names. `pdf_reverse` and the unadvertised
`pdf_invert_colors` are excluded. Three advertised tools are also excluded:
`pdf_extract_tables` has no dispatch arm; `pdf_stamp` and
`pdf_overlay_underlay` depend on repository test-fixture defaults instead of
accepting the required image/template through their advertised MCP schemas.
Fix and verify those handlers before restoring them to training candidates.

## Generate and check data

From the repository root, with Python 3.10+:

```bash
python3 -m pip install -r tools/slm-training/requirements.txt
python3 tools/slm-training/generate_dataset.py --count 10000 --seed 84 --output-dir tools/slm-training/dataset
python3 -m pytest tools/slm-training/test_dataset_generator.py tools/slm-training/test_infer_local.py -q
```

Generation validates each label against the MCP schema and artifact contracts.
`--seed` makes train/validation splits and labels reproducible. Ratios and counts
are checked before any output is written. Schema snapshots and test entrypoints
are resolved relative to the script, so tests work from other working directories too.

## Verify against the real MCP server

```bash
cargo build -p paperpilot-mcp -j 2
python3 tools/slm-training/sync_tool_schemas.py --check --mcp-bin target/debug/paperpilot-mcp
PAPERPILOT_MCP_BIN="$PWD/target/debug/paperpilot-mcp" python3 -m pytest tools/slm-training/test_workflow_mcp.py -q
```

On Windows, set `$env:PAPERPILOT_MCP_BIN` to the absolute `.exe` path before
running pytest. If the variable is unset, integration tests explicitly skip;
a supplied invalid binary or schema mismatch fails. CI always supplies the
binary and runs real document workflows. Refresh schemas by running
`sync_tool_schemas.py --mcp-bin ...` without `--check` after a reviewed server change.

The integration suite synthesizes genuine PDFs, invokes the stdio MCP server,
and reads actual outputs. Assertions cover preserved text/page counts,
watermarks, encryption/passwords, separate JSON reports, PPTX → text export,
blank-page removal, chapter extraction, and malformed/missing-file failures.
It uses no mocked PDF bytes or tool responses.

## Evaluate local model inference separately

Use the existing ML training environment (PyTorch, Transformers and optional
PEFT/Accelerate) and a local merged model directory:

```bash
python3 tools/slm-training/infer_local.py --model-dir /path/to/local/merged-model --report reports/slm-inference.json
```

The benchmark has 6 single-tool cases, all 15 curated workflows, and 1 refusal.
It checks JSON shape, registered names, required parameters, file-type
compatibility, and expected tool order, then writes a machine-readable report.
A failing case returns a nonzero exit status. `--cases` accepts a local JSON list
of `{"prompt": "...", "tools": ["pdf_merge", "pdf_compress"]}` or
`{"prompt": "...", "refusal": true}` cases. The default 1,536-token budget
accommodates longer plans; override it with `--max-new-tokens`.

Weights and tokenizers are loaded with `local_files_only=True`. If the default
merged model is absent, an existing local LoRA adapter may be used with its
cached base model. Missing fine-tuned weights cause an error. Evaluation never
silently falls back to an untrained base model or downloads weights.

Dataset checks and MCP execution do **not** establish fine-tuned model accuracy.
This change does not retrain or replace the published weights. Regenerate data,
retrain, and run the local inference benchmark before publishing new weights.
