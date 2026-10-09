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

## 4. Verification Deliverables
1. `demo/legal_discovery/README.md`
2. `scripts/run_ai_legal_demo.sh` (executable via `chmod +x`)
3. `Makefile` with `demo-legal-discovery` target.
4. Report at `reports/LOCAL_AI_AGENT_LEGAL_DISCOVERY_DEMO_REPORT.md` documenting output validation and latency profile.
