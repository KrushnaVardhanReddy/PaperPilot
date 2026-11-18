# Week 22: CLI to Desktop Sync: One-Command Install (Terminal Hacker to Native GUI App)

**Theme:** Seamless Developer Onboarding & Multi-Platform Runtime Synchronization  
**Tools Featured:** Flash Installer (`install.sh`, `install.ps1`), Desktop AppImage/Windows installer, CLI & MCP instant launcher  
**Target Audience:** Software Engineers, DevOps Professionals, Claude Desktop & Cursor Users, macOS/Linux/Windows Developers  
**Asset:** ![Week 22 Demo](../assets/showcase/week22-flash-installation-sync.gif)

---

## 📝 LinkedIn Post Copy

### Hook
Developer onboarding shouldn't take 45 minutes of compiling C++ dependencies, configuring CMake, or hunting down missing shared libraries. ⚡💻

Whether you prefer living inside a headless terminal or clicking through a native dark-mode GUI app, getting started with PaperPilot takes exactly **one command and 3.2 seconds**:

```bash
curl -fsSL https://usepaperpilot.com/install.sh | bash -s -- --all
```

No Python venvs. No Node version mismatches. No sudo required.

Just an instantaneous, production-ready local document intelligence engine across Linux, macOS, and Windows. 🚀

---

### The Problem: Fragmented Developer Experiences Across CLI and GUI
Most enterprise tooling forces engineers into an artificial split:

1. **Terminal Tools Lack Visual Validation**:
   Command-line utilities are unmatched for shell pipelines, headless CI servers, and batch scripts. But when auditing complex Bates-stamped exhibits, verifying visual pixel diffs, or previewing multi-page redactions, developers are forced to context-switch to clumsy desktop viewers.

2. **Desktop GUI Apps Are AI Silos**:
   Commercial PDF desktop applications cannot be summoned by an autonomous coding agent. Claude Desktop, Cursor, and your terminal automation scripts have zero programmatic access to their internal engines.

3. **Install Fatigue & Platform Disparity**:
   Tools that claim to be "cross-platform" frequently break across Windows PowerShell, macOS Apple Silicon, and Linux arm64/x86_64, requiring complex prerequisite packages and administrator privileges.

---

### The PaperPilot Solution: Unified Engine, Instant Cross-Platform Delivery
PaperPilot unifies the hacker terminal, the native desktop application, and the agentic AI bridge under a single runtime:

1. **The Flash Installer (`install.sh` & `install.ps1`)**:
   - Auto-detects operating system architecture (Linux x86_64/arm64, macOS Darwin ARM/Intel, Windows 64-bit).
   - Installs pre-compiled SIMD-accelerated binaries into `~/.local/bin` without requiring root/admin elevation.
   - Idempotent and version-pinnable: `VERSION=0.7.0 bash install.sh --cli`.

2. **One Engine, Identical Results**:
   - Running `paperpilot pdf_redact` in your terminal executes the exact same verified cryptographic C++/Rust logic as clicking "Apply Redaction" inside the PaperPilot Desktop AppImage or Windows installer.
   - Changes in your files instantly reflect across CLI watchers and the GUI workspace.

3. **Zero-Configuration MCP Agent Bridge**:
   - The `--all` flag automatically detects local installations of **Claude Desktop** and **Cursor**.
   - Registers `@usepaperpilot/mcp` directly into your client config files, turning your favorite AI chat into an autonomous legal and document clerk before your terminal prompt even returns.

---

### CLI Command Example
```bash
# 1. Install CLI, Desktop GUI, and Claude MCP server in one step
curl -fsSL https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.sh | bash -s -- --all

# 2. On Windows (PowerShell):
# irm https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/scripts/install.ps1 | iex

# 3. Verify health across core binaries and MCP connectivity
paperpilot doctor --check-mcp

# 4. Launch the native desktop interface directly from terminal
paperpilot-desktop &
```

---

### Automated MCP Registration Behind the Scenes
When you run the flash installer with `--all`, PaperPilot writes the zero-latency stdio contract into your AI client configurations:

```json
{
  "mcpServers": {
    "paperpilot": {
      "command": "paperpilot",
      "args": ["mcp", "serve"],
      "env": {
        "PKEY_STORAGE": "keyring",
        "LOG_LEVEL": "info"
      }
    }
  }
}
```

Instantly, 92 local-first document tools are accessible to Claude and Cursor with zero egress or cloud latency.

---

### Why It Matters for Developers & Platform Teams
- **Sub-4s Time-to-Hello-World**: From an empty shell prompt to Bates stamping a 100-page document or running an MCP tool call in under 4 seconds.
- **Offline & Air-Gapped Ready**: Standalone binaries bundled with zero dynamic runtime dependencies (`glibc` compatible, static assets bundled).
- **Homogeneous CI/CD & Local Parity**: Run the exact same binary in GitHub Actions, GitLab CI, dev containers, and macOS workstations.

---

### 💬 Discussion
How does your team bridge the gap between headless CI CLI tools and developer GUI apps? Are you standardizing on MCP for local development tool dispatch? Drop your thoughts below! 👇

---

### 🏷️ Hashtags
#DevOps #CLI #DeveloperExperience #OpenSource #Rust #LocalFirst #ClaudeDesktop #MCP #CrossPlatform #Linux #macOS #Windows #SoftwareEngineering
