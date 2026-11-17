# Spec 165: `@usepaperpilot/mcp` NPM Package & NPX Instant Runner

## 1. Executive Summary
Provide zero-configuration, 5-second onboarding for AI agents and LLM clients (Claude Desktop, Cursor, Zed, Windsurf, Antigravity, VS Code Copilot) using:
```bash
npx -y @usepaperpilot/mcp
```
Without requiring users to manually install Rust, compile the workspace, or configure custom binary paths.

---

## 2. Technical Architecture & Packaging Layout

Location: `packages/paperpilot-mcp`

```
packages/paperpilot-mcp/
├── package.json             # Scoped package definition (@usepaperpilot/mcp), bin entry, keywords
├── bin/
│   └── paperpilot-mcp.js    # Node.js stdio runner and cross-platform native binary orchestrator
├── README.md                # Fast onboarding & client configuration guides (Claude, Cursor, etc.)
└── LICENSE                  # MIT License
```

### 2.1 Binary Resolution & Fallback Hierarchy
When executed via `npx @usepaperpilot/mcp` or `node bin/paperpilot-mcp.js`:
1. **Explicit Override**: Checks environment variable `PAPERPILOT_MCP_PATH`.
2. **Local Release Target**: Checks `../../target/release/paperpilot-mcp` (monorepo local dev workflow).
3. **Local Debug Target**: Checks `../../target/debug/paperpilot-mcp`.
4. **Global System PATH**: Checks `which paperpilot-mcp` / `where paperpilot-mcp`.
5. **OS Cache Directory**: Checks `~/.cache/paperpilot/bin/paperpilot-mcp` (Linux/macOS) or `%LOCALAPPDATA%\paperpilot\bin\paperpilot-mcp.exe` (Windows).
6. **Automated GitHub Release Asset Download**:
   - Detects architecture (`x64`, `arm64`) and OS (`linux`, `darwin`, `win32`).
   - Downloads matching precompiled release archive from GitHub Release corresponding to the package version (e.g., `v0.7.0`):
     - `paperpilot-mcp-ubuntu-22.04-x86_64-unknown-linux-gnu.tar.gz`
     - `paperpilot-mcp-windows-latest-x86_64-pc-windows-msvc.zip`
     - `paperpilot-mcp-macos-latest-aarch64-apple-darwin.tar.gz`
   - Unpacks binary into user cache, sets execution permissions (`chmod 0755`), and invokes it.

### 2.2 Stdio Isolation Guarantee
- Adheres strictly to **Spec 146: MCP Protocol Stdio Isolation**.
- Standard input (`process.stdin`) and standard output (`process.stdout`) are piped directly to the child process without buffering, logging, or decoration.
- All runner diagnostic messages, download progress indicators, and health messages are strictly emitted to `process.stderr`.

---

## 3. Client Configuration Matrix

### Claude Desktop (`claude_desktop_config.json`)
```json
{
  "mcpServers": {
    "paperpilot": {
      "command": "npx",
      "args": ["-y", "@usepaperpilot/mcp"]
    }
  }
}
```

### Cursor (`cursor_mcp_config.json`)
```json
{
  "mcpServers": {
    "paperpilot": {
      "command": "npx",
      "args": ["-y", "@usepaperpilot/mcp"]
    }
  }
}
```

---

## 4. Verification & Testing Standards
1. CLI flag test: `node bin/paperpilot-mcp.js --help` outputs PaperPilot MCP help.
2. JSON-RPC handshake: Echoing `{"jsonrpc":"2.0","id":1,"method":"initialize",...}` produces a valid response.
3. Tool listing: `tools/list` returns the full catalog of 92 document intelligence tools.
4. Clean npm publishing under `@usepaperpilot/mcp` with public access.
