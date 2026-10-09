# Spec 035 — Local AI Agent Legal Due Diligence & Discovery Demo Harness

## 1. Overview & Objectives
PaperPilot is built as the foundational PDF Operating System for AI Agents. To clearly demonstrate this capability to customers, investors, and developers without requiring external paid cloud APIs, we provide a complete, self-contained **Local AI Agent Legal Discovery & Due Diligence Demo**.

The demo showcases an autonomous document processing pipeline running 100% locally on-device:
1. **Realistic Input Corpus**: Automatically generate three realistic legal documents in `demo/legal_discovery/input/`:
   - `01_asset_purchase_agreement.pdf` (Multi-page contract containing PII: SSN, founder bank details).
   - `02_disclosure_schedules_tilted.pdf` (Tilted/rotated scanned schedule).
   - `03_unredacted_financial_exhibit.pdf` (Confidential financial exhibit).
2. **Autonomous Agent Tool Pipeline**: Emulate and execute the sequential MCP / CLI toolchain:
   - `pdf_rotate`: Correct page orientation (90° clockwise to upright).
   - `pdf_redact`: Permanently excise PII text and underlying streams (SSN, routing numbers).
   - `pdf_bates`: Apply courtroom discovery identifiers (`CASE-2026-DISCOVERY-000001` through `000012`).
   - `pdf_merge`: Assemble sanitized exhibits into `sanitized_master_bundle.pdf`.
   - `pdf_watermark`: Stamp legal privilege banner (`CONFIDENTIAL & PRIVILEGED - ATTORNEYS EYES ONLY`).
   - `pdf_hash`: Compute verifiable SHA-256 tamper-evident integrity hash.
   - `pdf_linearize`: Web-optimize for fast streaming preview.
3. **Interactive & Automated Demo Runner**:
   - `scripts/run_ai_legal_demo.sh` (terminal-friendly with colorized stages, progress spinners, execution timers, and file size diffs).
   - `make demo-legal-discovery` integration in root `Makefile`.
   - Comprehensive demo documentation and run instructions in `demo/legal_discovery/README.md`.

---

## 2. Directory Structure & Deliverables

```
PaperPilot/
├── demo/
│   └── legal_discovery/
│       ├── README.md               # Overview, architecture, how AI agents interact with MCP
│       ├── generate_fixtures.rs    # Pure-Rust or script fixture generator for realistic mock legal PDFs
│       ├── input/                  # Realistic sample PDFs
│       └── output/                 # Sanitized, Bates-stamped, watermarked courtroom bundle
├── scripts/
│   └── run_ai_legal_demo.sh        # Colorized 1-click execution script with timings
└── Makefile                        # Added `make demo-legal-discovery`
```

---

## 3. Detailed Execution Pipeline

### Stage 1: Fixture Generation (`demo/legal_discovery/input/`)
If inputs do not exist, generate them programmatically using `lopdf`:
- **Document 1**: 4-page Asset Purchase Agreement with mock clause containing `SSN: 987-65-4321` and `Routing: 021000021`.
- **Document 2**: 3-page Disclosure Schedule with page 2 rotated 90 degrees (`/Rotate 90`).
- **Document 3**: 2-page Financial Exhibit with confidential salary data.

### Stage 2: The Agent Execution Loop
The script or agent invokes PaperPilot CLI / MCP commands sequentially:
```bash
# 1. Rotate Page 2 of Disclosure Schedule to 0°
paperpilot rotate demo/legal_discovery/input/02_disclosure_schedules_tilted.pdf --page 2 --angle 270 --output demo/legal_discovery/tmp/02_rotated.pdf

# 2. Redact sensitive PII (SSN & Routing)
paperpilot redact demo/legal_discovery/input/01_asset_purchase_agreement.pdf --page 1 --rect "100,500,300,520" --output demo/legal_discovery/tmp/01_redacted.pdf

# 3. Apply Bates Stamping across documents
paperpilot bates demo/legal_discovery/tmp/01_redacted.pdf --prefix "DISCOVERY-2026-" --start 1 --output demo/legal_discovery/tmp/01_bates.pdf
paperpilot bates demo/legal_discovery/tmp/02_rotated.pdf --prefix "DISCOVERY-2026-" --start 5 --output demo/legal_discovery/tmp/02_bates.pdf
paperpilot bates demo/legal_discovery/input/03_unredacted_financial_exhibit.pdf --prefix "DISCOVERY-2026-" --start 8 --output demo/legal_discovery/tmp/03_bates.pdf

# 4. Merge into final Courtroom Master Dossier
paperpilot merge demo/legal_discovery/tmp/01_bates.pdf demo/legal_discovery/tmp/02_bates.pdf demo/legal_discovery/tmp/03_bates.pdf --output demo/legal_discovery/tmp/master_merged.pdf

# 5. Apply Privilege Watermark
paperpilot watermark demo/legal_discovery/tmp/master_merged.pdf --text "CONFIDENTIAL - ATTORNEYS EYES ONLY" --output demo/legal_discovery/tmp/master_watermarked.pdf

# 6. Linearize & Hash for Audit Trail
paperpilot linearize demo/legal_discovery/tmp/master_watermarked.pdf --output demo/legal_discovery/output/final_court_bundle.pdf
paperpilot hash demo/legal_discovery/output/final_court_bundle.pdf
```

### Stage 3: Verification & Output Scorecard
Display a terminal scorecard showing:
- Initial input page count & sizes vs Final court bundle.
- Execution latency per operation (sub-millisecond to sub-second).
- Audit trail SHA-256 hash.
- Clear instructions on how an LLM agent uses PaperPilot's JSON-RPC MCP server (`paperpilot-mcp`) to execute the exact same flow autonomously.

---

## 4. Multi-Tier AI Agent Execution Models (Zero-Cloud Local LLM & BYOK)

PaperPilot acts as the deterministic execution engine for AI agents. The demo harness supports three operational models for connecting the AI "Brain" to PaperPilot "Hands" with 100% data residency:

### Tier 1: Zero-Install Standalone Mozilla `llamafile` (Recommended 1-Click Offline)
- **Concept**: Mozilla `llamafile` bundles full model weights (`Llama-3.2-3B-Instruct` or `Qwen2.5-Coder-7B`) and runtime into a single, cross-platform executable file via Cosmopolitan Libc.
- **Workflow**:
  1. User downloads `llamafile` executable (zero install, zero dependencies, runs on macOS/Linux/Windows).
  2. Double-click or run `./model.llamafile --server --jinja` which spins up an OpenAI-compatible local server at `http://127.0.0.1:8080/v1`.
  3. PaperPilot agent connects to `http://127.0.0.1:8080/v1/chat/completions`, supplies the 45 tool schemas, and executes tool calls locally.
  4. **Value**: Zero-configuration, double-click local LLM runner for non-technical users and air-gapped enterprise environments.

### Tier 2: Local Developer Runtime (`ollama` / `vLLM` / `LM Studio`)
- **Concept**: Developers with existing local model managers run models locally.
- **Workflow**:
  1. User runs `ollama run llama3.2:3b` or `ollama serve` (defaulting to `http://127.0.0.1:11434/v1`).
  2. PaperPilot agent connects via standard `/v1/chat/completions` with local tool-calling loop.

### Tier 3: External MCP Agents & BYOK (Claude Desktop, Cursor, OpenAI)
- **Concept**: Users who want frontier cloud intelligence connect their existing agent tools directly to PaperPilot via MCP (`paperpilot-mcp`) or provide a BYOK key.
- **Workflow**:
  1. In Claude Desktop / Cursor: Configure `paperpilot-mcp` in `claude_desktop_config.json`.
  2. The prompt *"Prepare discovery files in ./input"* is sent to Claude / GPT-4o.
  3. Claude emits MCP tool calls (`pdf_info`, `pdf_rotate`, `pdf_redact`, `pdf_bates`, `pdf_merge`, `pdf_hash`).
  4. PaperPilot executes the operations 100% locally on the host machine. **No raw PDF binary leaves the device.**

---

## 5. Verification Deliverables
1. `demo/legal_discovery/README.md` (Updated with llamafile, Ollama, and MCP instructions)
2. `tools/demo-legal-fixtures` (Pure-Rust mock PDF corpus generator)
3. `scripts/run_ai_legal_demo.sh` (Executable via `chmod +x` with sub-100ms latency scorecard)
4. `Makefile` with `demo-legal-discovery` target.
5. Report at `reports/LOCAL_AI_AGENT_LEGAL_DISCOVERY_DEMO_REPORT.md` documenting output validation, SHA-256 audit hash, and latency profile.
