---
title: MCP Agent Integration
description: Connect Codex, Claude Code, Kimi Code, Pi, Claude Desktop, and Cursor to local document tools.
---

# MCP Agent Integration

The Model Context Protocol (MCP) allows AI agents to directly interact with PaperPilot's tools.

PaperPilot uses **stdio MCP**. The client launches `paperpilot-mcp` locally and
uses the same PDF tools regardless of the selected agent.

## Generate client configuration

Build the current source with `cargo build --release -p paperpilot-mcp`, then run:

```sh
paperpilot-mcp --client-config codex
# Also: claude-code, kimi-code, pi, claude-desktop, cursor
```

This prints configuration without modifying settings. It uses the running
executable's absolute path; add `--server-command "/absolute/path/to/paperpilot-mcp"`
to target another installation. Merge the resulting server entry into your
existing settings. Older released binaries may not include this option yet.

## Codex

```sh
codex mcp add paperpilot -- "/absolute/path/to/paperpilot-mcp"
codex mcp list
```

Alternatively, merge the generated TOML under `[mcp_servers.paperpilot]` in
`~/.codex/config.toml`. In a new session, inspect `/mcp`.
See [Codex MCP documentation](https://developers.openai.com/codex/mcp/).

## Claude Code

```sh
claude mcp add --transport stdio --scope project paperpilot -- "/absolute/path/to/paperpilot-mcp"
claude mcp get paperpilot
```

This creates a project entry in `.mcp.json`; use `--scope user` for personal
configuration across projects. In a new session, inspect `/mcp` and approve the
project configuration when prompted.
See [Claude Code MCP documentation](https://code.claude.com/docs/en/mcp).

## Kimi Code

Merge the JSON from `paperpilot-mcp --client-config kimi-code` into
`.kimi-code/mcp.json` (project) or `~/.kimi-code/mcp.json` (user).
Start a new session and inspect `/mcp`. Older `kimi-cli` versions may use
`~/.kimi/`; follow the documentation for your installed version.
See [Kimi Code MCP documentation](https://www.kimi.com/code/docs/en/kimi-code-cli/customization/mcp.html).

## Pi

Current versions of Pi support native MCP:

```sh
npm install -g @earendil-works/pi-coding-agent
pi mcp add paperpilot -- "/absolute/path/to/paperpilot-mcp"
pi mcp list
```

Alternatively, merge the JSON from `paperpilot-mcp --client-config pi` into
`.pi/mcp.json` (project) or `~/.pi/agent/mcp.json` (user).
Start a new session or run `/reload`, then inspect `/mcp`.
If your Pi version lacks `pi mcp`, upgrade to a version with native MCP, or use
PaperPilot's CLI from Pi's shell tool. Installed MCP extensions may override
native behavior. The older `@mariozechner/pi-coding-agent` package is deprecated.
See [Pi's upstream MCP guide](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/mcp.md).

## Claude Desktop Configuration

To enable PaperPilot capabilities in Claude Desktop, edit your `claude_desktop_config.json` file.

**Path:**
*   **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
*   **Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

**Configuration:**

```json
{
  "mcpServers": {
    "paperpilot": {
      "command": "/path/to/paperpilot-mcp",
      "args": []
    }
  }
}
```

## Cursor IDE Integration

1.  Open Cursor Settings.
2.  Navigate to **Features** > **MCP Servers**.
3.  Click **Add New MCP Server**.
4.  Set **Name** to `paperpilot`.
5.  Set **Command** to the absolute path of your compiled `paperpilot-mcp` binary.

Once configured, you can ask Claude or Cursor to perform PDF operations (e.g., "Merge report1.pdf and report2.pdf") and the LLM will seamlessly call PaperPilot tools.

## Verify and troubleshoot

Ask your agent to merge two PDFs using absolute input and output paths. It should
discover `pdf_merge`, whose arguments are `inputs` (an array of paths) and `output`.
On Windows, include `.exe` in the executable path. The configuration generator
escapes backslashes and spaces correctly. Files and the executable must be visible
in the agent's runtime, including WSL, SSH, or container sessions.

Leave the server's `args` empty. `--direct-tool-call` is a diagnostic mode and
must not be used for MCP clients. The gateway on port 7823 exposes REST, not an
MCP HTTP endpoint.

For complete examples and developer regression checks, see the
[coding agent setup guide](https://github.com/KrushnaVardhanReddy/PaperPilot/blob/main/docs/coding-agents.md).
