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

## 4. Multi-Tier AI Agent Execution Models (Zero-Cloud Local LLM, Universal Endpoints & BYOK)

PaperPilot acts as the deterministic execution engine for AI agents. The demo harness and core client support four flexible operational models for connecting the AI "Brain" to PaperPilot "Hands" with strict data residency:

### Tier 1: Zero-Install Standalone Mozilla `llamafile` (Recommended 1-Click Offline)
- **Concept**: Mozilla `llamafile` bundles full model weights (`Llama-3.2-3B-Instruct` or `Qwen2.5-Coder-7B`) and runtime into a single, cross-platform executable file via Cosmopolitan Libc.
- **Workflow**:
  1. **In-App Download or Pre-Packaged**: Download via PaperPilot's in-app model manager directly to local storage (`~/.local/share/paperpilot/models/`), or drop in any local `.llamafile` via file picker.
  2. **Process Supervision**: PaperPilot automatically supervises the `.llamafile` subprocess (`--server --jinja --port 8080`) in the background.
  3. PaperPilot agent connects to `http://127.0.0.1:8080/v1/chat/completions`, supplies the 45 tool schemas, and executes tool calls locally.
  4. **Value**: Zero-configuration, double-click local LLM runner for non-technical users and air-gapped enterprise environments.

### Tier 2: Universal OpenAI-Compatible Endpoint (`any_url` + optional `api_key` + `model`)
- **Concept**: Connect to **ANY** local or network-hosted LLM endpoint speaking the OpenAI `/v1/chat/completions` protocol.
- **Supported Targets**:
  - **Local engines**: Ollama (`http://localhost:11434/v1`), LM Studio (`http://localhost:1234/v1`), local `llama.cpp` server.
  - **LAN / Enterprise GPU Clusters**: Internal vLLM, TGI, SGLang, or private Ollama instances (`http://gpu-cluster.internal:8000/v1`).
  - **Alternative Cloud Endpoints**: Groq (`https://api.groq.com/openai/v1`), DeepSeek, Together AI, OpenRouter.
- **Workflow**: User inputs `Endpoint URL`, optional `API Key`, and `Model Name`. PaperPilot streams responses and executes tool calls locally.

### Tier 3: Commercial BYOK (Bring Your Own Key) & Managed Cloud
- **Concept**: Users who want frontier cloud intelligence (OpenAI GPT-4o, Anthropic Claude 3.5 Sonnet, Google Gemini 1.5 Pro) supply their own API keys.
- **Security**: Keys are stored securely in the native OS Keychain / Secret Store (`tauri-plugin-stronghold`), never transmitted to PaperPilot servers.
- **Data Privacy Guarantee**: Only textual prompts and bounding-box coordinates are sent to the cloud model; **raw PDF binaries never leave the local machine**.

### Tier 4: External MCP Agents (Claude Desktop, Cursor, OpenCode)
- **Concept**: Users running Claude Desktop, Cursor, or OpenCode connect `paperpilot-mcp` directly via standard `stdio` in their local config (`claude_desktop_config.json`).
- **Workflow**: The frontier model runs in the host agent; PaperPilot executes all file operations locally.

---

## 5. Open Source Licensing & Commercial Monetization Boundary

To maximize developer adoption and network effects while building high-margin enterprise revenue:
1. **100% Free & Open Source (Apache 2.0 / MIT)**:
   - All core PDF execution tools (50+ operations in `paperpilot-pdf`, `paperpilot-wasm`, `paperpilot-cli`, `paperpilot-mcp`).
   - The Local AI Agent execution loop (`paperpilot-ai` / `paperpilot agent`).
   - The Mozilla `llamafile` 1-click supervisor and in-app model downloader.
   - The Universal OpenAI-compatible endpoint client and BYOK OS Keyring manager.
   - The JSON-RPC MCP server for Claude Desktop, Cursor, and OpenCode.
2. **Commercial Value Levers (Pro / Teams / Enterprise)**:
   - **Desktop Pro ($5/mo)**: Background Watch Folder daemons, recurring Cron jobs, inbound webhooks, and unlimited pipeline step chains.
   - **Teams ($12/user/mo)**: Centralized license portal, shared team recipe/pipeline registry, concurrent execution, team audit trails.
   - **Enterprise ($22/user/mo or $2.5k–$10k/yr)**: SCIM 2.0 provisioning, SAML 2.0 federation, Air-gap Group Policy enforcement (GPO/MDM), Splunk/SIEM audit log streaming, and HIPAA BAA compliance.
   - **Managed White-Label Edge ($199–$399/mo)**: Cloudflare Worker custom domain with client branding for SMBs and firms without DevOps staff.

---

## 6. Verification Deliverables
1. `demo/legal_discovery/README.md` (Updated with llamafile, universal endpoint, and MCP instructions)
2. `tools/demo-legal-fixtures` (Pure-Rust mock PDF corpus generator)
3. `scripts/run_ai_legal_demo.sh` (Executable via `chmod +x` with sub-100ms latency scorecard)
4. `Makefile` with `demo-legal-discovery` target.
5. Report at `reports/LOCAL_AI_AGENT_LEGAL_DISCOVERY_DEMO_REPORT.md` documenting output validation, SHA-256 audit hash, and latency profile.
