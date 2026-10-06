---
title: MCP Agent Integration
description: Setup instructions for Claude Desktop and Cursor IDE.
---

# MCP Agent Integration

The Model Context Protocol (MCP) allows AI agents to directly interact with PaperPilot's tools.

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
