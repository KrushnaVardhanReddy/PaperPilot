# @usepaperpilot/mcp

> **Zero-Install Model Context Protocol (MCP) Server for PaperPilot**  
> Run PaperPilot's complete, 92+ tool PDF manipulation & document intelligence suite directly from Claude Desktop, Cursor, Gemini CLI, or any MCP client via `npx`.

[![npm version](https://img.shields.io/npm/v/@usepaperpilot/mcp.svg)](https://www.npmjs.com/package/@usepaperpilot/mcp)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

---

## ⚡ Quick Start

Run instantly without installing anything:

```bash
npx -y @usepaperpilot/mcp
```

On first run, the runner automatically identifies your OS and architecture, downloads the matching precompiled high-performance Rust binary to `~/.paperpilot/bin/`, and starts the stdio JSON-RPC MCP server. Subsequent runs start immediately with zero delay.

---

## 💻 Configuration Examples

### Cursor IDE

Add the following to your Cursor MCP settings (`~/.cursor/mcp.json` or Cursor Settings → MCP):

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

### Claude Desktop

Add PaperPilot to your Claude Desktop configuration:

- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux**: `~/.config/Claude/claude_desktop_config.json`

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

### Gemini CLI / Other MCP Hosts

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

## 🛠️ Supported Architectures & Platforms

| Platform | Architecture | Status |
|:---|:---|:---:|
| **macOS** | Apple Silicon (`arm64` / M1/M2/M3/M4) | ✅ Supported |
| **macOS** | Intel 64-bit (`x64`) | ✅ Supported |
| **Linux** | x86_64 (`x64`, glibc >= 2.17) | ✅ Supported |
| **Linux** | ARM64 (`aarch64`) | ✅ Supported |
| **Windows** | x86_64 (`x64`) | ✅ Supported |

---

## 🔧 Advanced Configuration

You can override binary selection by specifying the `PAPERPILOT_MCP_PATH` environment variable:

```bash
export PAPERPILOT_MCP_PATH="/path/to/custom/paperpilot-mcp"
npx -y @usepaperpilot/mcp
```

Or configure local builds directly in your MCP host config:

```json
{
  "mcpServers": {
    "paperpilot": {
      "command": "npx",
      "args": ["-y", "@usepaperpilot/mcp"],
      "env": {
        "PAPERPILOT_MCP_PATH": "/usr/local/bin/paperpilot-mcp"
      }
    }
  }
}
```

---

## 📄 License

Apache License 2.0. See [LICENSE](LICENSE) for details.
Part of the [PaperPilot](https://github.com/KrushnaVardhanReddy/PaperPilot) open-source document platform.
