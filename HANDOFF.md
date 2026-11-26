# PaperPilot — Session Handoff Guide
 
> **Single Source of Truth** for resuming work on PaperPilot.  
> Start any new agent or human session by reading this document.  
> **Date of Last Update:** November 25, 2026
>
> **Active Target (Milestone: v0.8.2):** 
> 1. **Enterprise Security Hardening & DAST Penetration Suite**: Mitigated DoS panic vectors, implemented DNS-rebinding SSRF protection in core HTTP fetch, enforced OWASP baseline security headers (`X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy: no-referrer`, `Cross-Origin-Resource-Policy: same-origin`), added automated PDF fuzzing & penetration test suite (`security_dast_test.rs`).
> 2. **Configurable Gateway Body Limits**: Added dynamic request body limit configuration via `PAPERPILOT_MAX_BODY_LIMIT_MB` and `--max-body-mb <MB>` (supporting safe 100MB defaults, custom sizes, and `0`/`unlimited` for enterprise high-throughput nodes).
> 3. **Sub-5ms Document Ingestion Gateway for LLMs**: Validated drop-in Apache Tika 3 (`/tika/text`) and Tika 4 (`/tika/json/md`) wire compatibility with Open-WebUI & Ollama, parsing multi-page tabular reports in 3.42ms under 50MB RAM with zero cloud egress.
> 4. **Monthly Security Pentest Automation**: Wired `make security-audit`, `make audit-deps`, `make audit-sast`, `make test-security-dast`, and `make pentest` into the Makefile.
> 5. **v0.8.2 Workspace Synchronization**: All Rust crates (`paperpilot-core`, `pdf`, `cli`, `mcp`, `gateway`, `wasm`, `ai`, `nlp`, `storage`), `@usepaperpilot/mcp` npm package, and Starlight documentation versioning upgraded to `v0.8.2`.
> 6. **Active Branch**: `chore/bump-v0.8.2` (preparing for merge into `feature/dev`).

---

## 🧭 Executive Summary & Current Repository State

- **Current Working Branch**: `chore/bump-v0.8.2` (ready to merge to `feature/dev`).
- **Release Status**: `v0.8.2` target version bump applied across all crates and documentation; `@usepaperpilot/mcp@0.8.2` package synchronized.
- **System Stability & Audits**:
  - **100/100 Tools Penta-Interface Parity**: **100% PASS** across CLI, MCP, REST Gateway API, Desktop UI, and Core Rust.
  - **Security DAST & Fuzzing**: 6/6 tests passing in `paperpilot-gateway/tests/security_dast_test.rs` covering DoS panic resilience, body limit overflow, path traversal defense, and malformed PDF fuzzing.
  - **Spec 170 Complete**: Pure-Rust NeuQuant GIF encoding, animated PDF/image sequencer, projection profile baseline deskew, bounding box content crop, token-budgeted LLM vision optimizer, coordinate redact, and image transcoding.
  - **Spec 171 Complete**: Glama MCP TDQS Grade A+ schema audit and negative constraints across all tools in `paperpilot-mcp/src/server.rs`.
  - **WASM Engine**: 54 supported in-browser operations in `paperpilot-wasm`, full UI bindings in `apps/web`.
  - **Documentation Suite**: Starlight / Astro documentation updated to 100 tools with `v0.8.2` default release, version dropdown selector, dynamic sidebar filtering, and direct-visit compatibility warning banners. Clean build (116 pages indexed, 0 broken links).
  - **Cargo Workspace**: Compiles cleanly with `-j 4` concurrency limit.

---

## 🏆 Recently Merged Milestones

1. **v0.8.2 Enterprise Security Hardening & Penetration Testing Suite**:
   - Closed DoS panic vectors in `paperpilot-gateway` by replacing raw `.unwrap()` on streaming multipart and folder watcher handlers with safe `Option` handling and bounded body conversions.
   - Enforced SSRF DNS rebinding prevention in `paperpilot-core/src/url_fetch.rs` by resolving socket addresses before connection and blocking loopback, RFC-1918 private subnets, cloud metadata (169.254.169.254), and CGNAT ranges.
   - Injected OWASP baseline security headers globally across the REST gateway (`nosniff`, `DENY`, `no-referrer`, `same-origin`).
   - Made gateway payload limits dynamically configurable via `PAPERPILOT_MAX_BODY_LIMIT_MB` and `--max-body-mb` CLI arguments (default 100MB, supports `0`/`unlimited`).
   - Integrated automated monthly pentesting into `Makefile` (`make security-audit`, `make pentest`).

2. **v0.8.1 Release & CI Stabilization**:
   - Modernized CI GitHub Actions (upgraded Node 20 deprecated actions to Node 24 compliant versions: `pnpm/action-setup@v4`, `actions/checkout@v4`, `actions/setup-node@v4`, `actions/setup-java@v4`, `softprops/action-gh-release@v2`, `android-actions/setup-android@v3`).
   - Harmonized release pipeline with cross-platform python version updater and automated release packaging.
   - Tagged `v0.8.1` and published official GitHub release artifacts across Linux, macOS, Windows, Android APK, and WASM packages.

3. **Docs Portal Versioning & Compatibility Filter (`apps/docs`)**:
   - Defaulted documentation to `v0.8.2 (Current Release)`.
   - Added client-side version switching supporting `v0.8.2`, `v0.8.1`, `v0.8.0`, `v0.7.0`, `v0.6.0`, and `v0.5.0`.
   - Implemented dynamic DOM filtering to hide `v0.8.0+` tools when browsing older documentation versions, and added an interactive compatibility switch banner on direct tool visits.

3. **Spec 171: Glama MCP TDQS Grade A+ Schema Hardening**:
   - Audited all tools in `paperpilot-mcp/src/server.rs` enforcing `"additionalProperties": false`.
   - Structured 4-section documentation per tool (`When to use:`, `Difference from related tools:`, `Key parameters:`, `[Input: ... | Output: ...]`).
   - Clean canonical tool list with backwards-compatible execution alias resolution.
   - Updated `@usepaperpilot/mcp` to `0.8.1`.

4. **Complete WASM Tool Catalog & Web Studio Expansion**:
   - Implemented and verified all 54 browser-safe tools in `paperpilot-wasm`.
   - Wired tool controls into `apps/web` operations studio with instant client-side execution.

5. **Spec 170: Native Image Intelligence & Animated Showcase Engine**:
   - 8 new pure-Rust tools (`pdf_to_gif`, `showcase_gif`, `images_to_gif`, `img_deskew`, `img_crop_to_content`, `img_redact`, `img_optimize_for_llm`, `img_convert`). Zero external runtime dependencies.
   - Text baseline deskew via horizontal projection profile variance optimization.
   - Semantic REST Gateway routes (`/api/v1/image/*`, `/api/v1/office/*`) with legacy `/api/v1/pdf/*` aliases.

---

## 🎯 Prioritized Next Tasks & Execution Roadmap

```
[Tier 1: Distribution & Registries] ➔ [Tier 2: Enterprise Moats & Parsing] ➔ [Tier 3: Inline Typography & Editing]
```

### 🥇 Tier 1: Distribution & Registries (Immediate Focus)
*Goal: Expand reach, installability, and ecosystem presence following the v0.8.2 release (turn 1,200+ direct GitHub downloads into 10,000+ automated terminal installs).*

1. **Homebrew Distribution Plan (macOS / Linux)**:
   - **Phase A (Custom Tap - Launch Immediately)**:
     - Repository: `https://github.com/KrushnaVardhanReddy/homebrew-tap`
     - Formula/Cask type: **Cask** (`Casks/paperpilot.rb`) pointing to official GitHub Release macOS `.dmg` assets (`aarch64` and `x64`).
     - Target user command:
       ```bash
       brew install KrushnaVardhanReddy/tap/paperpilot
       ```
     - Add automated GitHub Action dispatch in `.github/workflows/release.yml` on release publish to compute SHA256 of `.dmg` and push updated cask to `KrushnaVardhanReddy/homebrew-tap`.
   - **Phase B (Official Upstream `homebrew/cask` Submission)**:
     - Run `brew create --cask https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v<version>/PaperPilot_<version>_aarch64.dmg`.
     - Validate via `brew audit --cask --strict --online paperpilot` and `brew style --cask paperpilot`.
     - Submit PR to `Homebrew/homebrew-cask` for universal `brew install --cask paperpilot`.

2. **WinGet Distribution Plan (Windows - Pending)**:
   - Target User Command: `winget install PaperPilot.PaperPilot` (deferred to subsequent phase; focus on Homebrew tap automation first).

3. **Flathub Strategy (Linux)**:
   - On hold for a few weeks before raising any new PR to respect submission intervals.

4. **Glama TDQS Re-scoring & Verification**:
   - Trigger Glama.ai re-inspection to confirm TDQS Grade A / A+ status on the newly published `@usepaperpilot/mcp@0.8.1`.
3. **In-Memory Parser Self-Healing**:
   - Pre-flight repair of malformed XREF tables, damaged trailers, and truncated streams across all document intake tools.

---

### 🥈 Tier 2: AI Document Parsing & High-Volume Streaming (Open-Core Moats)
*Goal: Solidify PaperPilot as the highest-performance offline document ingestion and RAG engine (< 10ms/page).*

> [!NOTE]
> **Enterprise Boundary Notice**: Asynchronous priority job queues (`crates/enterprise-async`), worker leasing, and HMAC-SHA256 signed webhook dispatchers are **strictly commercial features** maintained and already implemented in the private [`PaperPilot-Enterprise`](/home/krushna/Project/PaperPilot-Enterprise) repository. They are intentionally kept out of open-core `PaperPilot` to preserve the commercial moat.

4. **Phase 5.8: Advanced Document AST & Layout-Aware Structured Markdown (`DocAST`)**:
   - Layout-aware structured Markdown extraction preserving reading order, headers (`# H1`, `## H2`), multi-column flow, tables, and math blocks.
   - Outperform Python-heavy Marker and Docling in pure Rust with sub-10ms per page throughput and zero Python runtime dependencies.
5. **Phase 5.2.9: Memory-Mapped Streaming (`memmap2`)**:
   - Zero-copy streaming and multi-core Rayon processing for instant loading and transformation of 1,000+ page high-volume enterprise document processing without memory bloat.
6. **Phase 6.8.3: Embedded libSQL / Turso Local Storage**:
   - Embedded local-first database in `get_app_data_root()` for local audit trails, persistent receipts, and offline tool execution history.

---

### 🥉 Tier 3: Inline Typography & Content Editing (Phase 5.6)
*Goal: Close the gap with Adobe Acrobat Pro inline content authoring.*

7. **Text Block & Paragraph Detection**:
   - Segment raw PDF `BT...ET` operator streams into visual text blocks and paragraphs.
8. **In-Place Text Editing & Content Stream Rewriting**:
   - Support clicking text elements, editing typos, and rewriting streams with font metric matching.

---

## 📋 Resuming Checklist for Next Session

When resuming a new session, follow these steps:

1. **Verify Branch Alignment**:
   - Ensure local branch is `feature/dev` and up to date with `origin/feature/dev`.
   - Verify `git status` is clean.

2. **Run Resource-Safe Compilation (`-j 4` MANDATORY)**:
   - Check workspace: `cargo check --workspace --all-targets --all-features -j 4`
   - Run tests: `cargo test --workspace --no-run -j 4`
   - Run desktop Vitest component tests: `pnpm --dir apps/desktop test`
   - Check docs build: `pnpm --dir apps/docs build`

3. **Active Next Tasks**:
   - **Distribution**: Monitor Glama re-indexing for `v0.8.1`, set up Homebrew tap formula and crates.io publishing.
   - **Resilience**: Implement in-memory self-healing parser for malformed XREF tables.
   - **Enterprise**: Implement async job queue & webhook event dispatcher (Phase 6.8).

---

## ⚖️ Rules of Engagement & Production Constraints

> ⚠️ **CRITICAL**: PaperPilot is a live open-source product. All development must preserve production stability.

### 📦 Version Bumping & Release PR Protocol (How to prepare next version)

When raising a PR for the next release (e.g. `0.8.2` or `0.9.0`), you **do not** need to edit 15 files manually. Run the automated version bumper:
```bash
make bump-version VERSION=0.8.2
```
This single command automatically updates:
1. **Rust Workspace Crates**: `version = "x.y.z"` across `paperpilot-core`, `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`, `paperpilot-wasm`, `paperpilot-ai`, `paperpilot-nlp`, `paperpilot-storage`.
2. **NPM MCP Runner**: Updates `packages/paperpilot-mcp/package.json` to the target version.
3. **Cargo.lock**: Automatically re-synchronizes with `cargo check ... -j 4`.

#### Optional/Public-Facing Docs Checklist for major/minor releases:
- If new tools were introduced: Add the new tool slug to `V08_TOOLS` in `apps/docs/src/components/VersionSelectHeader.astro` and verify `apps/docs/astro.config.mjs` sidebar.
- Add the new version option to the `<select id="doc-version-select">` dropdown in `VersionSelectHeader.astro`.

#### How Release Tagging Works:
When triggering `.github/workflows/release.yml` with tag `v0.8.2`:
- CI automatically reads the input tag and injects it into Desktop Tauri metadata (`tauri.conf.json`), Flatpak, WinGet, and creates the GitHub release artifacts automatically.

#### 📦 Flathub Packaging & Submission Protocol:
For every new release (e.g. `v0.8.2`):
1. **Retrieve the release `.deb` SHA-256 hash**:
   ```bash
   curl -sL https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v0.8.2/PaperPilot_0.8.2_amd64.deb.sha256
   ```
2. **Update local packaging in PaperPilot**:
   - `packaging/flatpak/com.usepaperpilot.PaperPilot.yml`:
     - Update download URL to `.../releases/download/v0.8.2/PaperPilot_0.8.2_amd64.deb`.
     - Update `sha256` with the exact hash retrieved above.
   - `packaging/flatpak/com.usepaperpilot.PaperPilot.metainfo.xml`:
     - Ensure `<project_license>Apache-2.0</project_license>` is preserved.
     - Add the new `<release version="0.8.2" date="YYYY-MM-DD">` block with summary notes.
3. **Synchronize the Flathub fork repository**:
   - The standalone Flathub clone lives at `~/Project/flathub` on branch `add-com.usepaperpilot.PaperPilot`:
     ```bash
     cd ~/Project/flathub
     git pull origin add-com.usepaperpilot.PaperPilot
     cp ~/Project/PaperPilot/packaging/flatpak/* ~/Project/flathub/
     git add . && git commit -m "update(paperpilot): bump to v0.8.2" && git push origin add-com.usepaperpilot.PaperPilot
     ```
4. **Submitting / Updating the Flathub PR**:
   - **Initial Submission**: Open PR from `KrushnaVardhanReddy:add-com.usepaperpilot.PaperPilot` targeting `flathub/flathub` on branch **`new-pr`** ([one-click PR link](https://github.com/flathub/flathub/compare/new-pr...KrushnaVardhanReddy:flathub:add-com.usepaperpilot.PaperPilot?expand=1)).
   - **Subsequent Version Updates**: Once accepted by Flathub, updates are pushed directly to `flathub/com.usepaperpilot.PaperPilot`.

---

1. **System Resource Constraint (`-j 4` is MANDATORY on local machine):**
   - **ALWAYS** pass `-j 4` for all local `cargo test`, `cargo build`, or `cargo check` commands (e.g. `cargo test -p paperpilot-pdf -j 4`).
   - Unbounded cargo compilations will exhaust memory and freeze the host machine / log off the system.

2. **Feature Branch Protocol (No Direct Pushes to `main`):**
   - All development, bug fixes, and experiments must occur on dedicated feature branches or reviewed into `feature/dev`. Never push directly to `main`.

3. **The Single Core Runner + 5 Wrappers Architecture Pattern:**
   - Every tool operation MUST have exactly **ONE** core in-memory engine implementation in `paperpilot-pdf`:
     ```rust
     pub fn run_operation_bytes(input_bytes: &[u8], config: &Config) -> Result<Vec<u8>, OperationError>
     ```
   - 5 Thin Interface Wrappers: CLI (`paperpilot-cli`), MCP (`paperpilot-mcp`), REST Gateway (`paperpilot-gateway`), WASM Engine (`paperpilot-wasm`), and Desktop/Embed (`apps/desktop`, `apps/embed`).

4. **Zero-Mock Policy on Tests:**
   - Test fixtures must be real disk fixtures or synthesized byte buffers (`fixtures/`, `tests/e2e_fixtures/real/`). Never mock PDF bytes or operation responses.

5. **WASM Isolation Guarantee:**
   - Never add OS-specific filesystem watchers or native network sockets (`mio`, `tokio/net`, `notify`) to `paperpilot-core` or common dependencies of `paperpilot-wasm`.

6. **Knowledge Graph Maintenance:**
   - After modifying project code files, run `/home/krushna/.cargo/bin/graphify-rs build` to keep the knowledge graph synchronized.

---

## 🛠️ Quick Command Reference

```bash
# 1. WASM Build & Tests (Memory-Safe)
wasm-pack build --target web --release paperpilot-wasm -- --jobs 4
cargo test -p paperpilot-wasm -j 4

# 2. Rust Unit Tests (Always use -j 4)
cargo test -p paperpilot-pdf --lib -j 4
cargo test -p paperpilot-mcp --lib -j 4
cargo test -p paperpilot-gateway -j 4
cargo test -p paperpilot-cli -j 4

# 3. Penta-Interface E2E Audit Suites
make test-wave1
make test-wave2
make test-wave3
make test-wave4
make test-wave5
make test-wave6
make test-wave7
make test-wave8

# 4. System Benchmark Suite
python3 scripts/run_system_benchmark.py --quick

# 5. Desktop Vitest Tests
pnpm --dir apps/desktop test

# 6. Knowledge Graph Refresh
/home/krushna/.cargo/bin/graphify-rs build
```
