# Spec 058: Independent Multi-Interface Packaging & Modular Distribution (CLI, MCP, Gateway, All-in-One)

## 1. Overview & Motivation
PaperPilot provides five distinct interfaces into its core document processing engine:
1. **Command Line Interface (`paperpilot-cli` / `paperpilot`)**: Fast, scriptable local CLI for developers, CI/CD pipelines, and shell utilities.
2. **AI Agent / Model Context Protocol (`paperpilot-mcp`)**: Lightweight JSON-RPC standard-I/O server for Claude Desktop, Cursor, and autonomous AI agents.
3. **HTTP/REST Gateway (`paperpilot-gateway`)**: Microservice HTTP server with Swagger/OpenAPI docs for web backends and microservice topologies.
4. **Desktop GUI (`apps/desktop`)**: Tauri-based visual document workstation.
5. **WebAssembly / Edge (`paperpilot-wasm`)**: In-browser client-side engine and Cloudflare Edge worker.

### The Problem
Currently, our release workflow bundles all compiled binaries into a single monolithic archive (`paperpilot-linux-x86_64.tar.gz`). If an AI engineer wants only the lightweight MCP server for Claude Desktop, they shouldn't be forced to download the entire CLI suite and web gateway. Similarly, a DevOps engineer embedding PaperPilot into a minimal Docker container for headless batch conversions shouldn't be forced to pull in unnecessary server runtimes or MCP dependencies.

### The Solution
Implement **Independent Multi-Interface Packaging**:
1. **Independent Standalone Packages**:
   - `paperpilot-cli` (`paperpilot`): Pure CLI binary + shell completions (bash, zsh, fish).
   - `paperpilot-mcp`: Pure AI MCP server executable.
   - `paperpilot-gateway`: Pure REST API daemon.
2. **All-in-One Bundle (`paperpilot-all`)**:
   - A consolidated bundle containing all binaries for power users and offline developer workstations.
3. **Cargo Feature Flags in `paperpilot-cli`**:
   - Modular compile-time features (`pdf`, `office`, `ocr`, `search`, `all`) so resource-constrained environments can compile ultra-lean binaries.
4. **Automated Multi-Asset Release Pipeline**:
   - Update `.github/workflows/release.yml` to package and release each binary independently alongside the all-in-one bundle, with standalone SHA-256 checksums.

---

## 2. Requirements & Deliverables

### Requirement 1: Cargo Feature Flags in `paperpilot-cli`
- Define modular features in `paperpilot-cli/Cargo.toml`:
  - `default = ["all"]`
  - `all = ["pdf", "office", "ocr", "search", "daemon"]`
  - `pdf = ["paperpilot-pdf"]`
  - `office = ["paperpilot-pdf/office"]`
  - `ocr = ["paperpilot-pdf/ocr"]`
  - `daemon = ["paperpilot-gateway"]`
- Ensure `cargo build -p paperpilot-cli --no-default-features --features pdf` compiles cleanly with zero unused dependency bloat.

### Requirement 2: Shell Completion Generation
- In `paperpilot-cli`, add a `--generate-completions <SHELL>` command (or a build script) using `clap_complete` to produce official completions for `bash`, `zsh`, `fish`, and `powershell`.
- Include completions in the standalone `paperpilot-cli` package.

### Requirement 3: Automated Multi-Artifact Release Workflow (`release.yml`)
- Update `.github/workflows/release.yml` packaging step to output separate compressed archives per platform:
  - `paperpilot-cli-<os>-<arch>.tar.gz` (and `.zip` on Windows)
  - `paperpilot-mcp-<os>-<arch>.tar.gz` (and `.zip` on Windows)
  - `paperpilot-gateway-<os>-<arch>.tar.gz` (and `.zip` on Windows)
  - `paperpilot-all-<os>-<arch>.tar.gz` (and `.zip` on Windows)
- Generate SHA-256 checksum files for each individual package (`.sha256`).

### Requirement 4: Verification & Integration Test
- Create an automated verification script or test in `tests/packaging_test.rs` or `tools/` that builds each standalone binary independently, verifies that binary size meets lean thresholds, and confirms that `--help` or `--version` executes correctly for each standalone package.

---

## 3. Success Criteria
- [ ] Users can install/download `paperpilot-cli` independently without MCP or Gateway runtimes.
- [ ] AI developers can install/download `paperpilot-mcp` independently (~8MB) for instant Claude Desktop / Cursor hookup.
- [ ] Backend teams can install/download `paperpilot-gateway` independently for Docker microservice deployment.
- [ ] Power users can download `paperpilot-all` to get all tools together.
- [ ] All standalone packages have their own cryptographic SHA-256 checksums in GitHub Releases.
