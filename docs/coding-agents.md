# Coding agent setup

PaperPilot gives coding agents local PDF and document tools through **stdio MCP**.
Codex, Claude Code, Kimi Code, and current Pi releases use the same
`paperpilot-mcp` executable and tool schemas. No model-provider API key is needed
by PaperPilot; each agent keeps its own model authentication.

## Install the server

Download `paperpilot-mcp` from [Releases](https://github.com/KrushnaVardhanReddy/PaperPilot/releases),
or build the version in this checkout:

```sh
cargo build --release -p paperpilot-mcp
```

The source changes below require a build containing `--client-config`; older
release binaries may not have that option. Use `paperpilot-mcp --help` to check.

Use the absolute path to `target/release/paperpilot-mcp` (Windows:
`target\release\paperpilot-mcp.exe`). Keep quotes around paths containing spaces.
The agent starts the server; you do not need to run it in a separate terminal.

## Generate configuration

```sh
paperpilot-mcp --client-config codex
paperpilot-mcp --client-config claude-code
paperpilot-mcp --client-config kimi-code
paperpilot-mcp --client-config pi
```

These commands print configuration and exit. They do not change client settings.
The executable defaults to the running binary's absolute path. To configure a
different installation, add `--server-command "/absolute/path/to/paperpilot-mcp"`.
`claude-desktop` and `cursor` are also supported.

| Client | Format | Configuration location |
|---|---|---|
| Codex | TOML, `mcp_servers` | `~/.codex/config.toml` |
| Claude Code | JSON, `mcpServers` | Project `.mcp.json` |
| Kimi Code | JSON, `mcpServers` | Project `.kimi-code/mcp.json` or `~/.kimi-code/mcp.json` |
| Pi | JSON, `mcpServers` | Project `.pi/mcp.json` or `~/.pi/agent/mcp.json` |
| Cursor | JSON, `mcpServers` | Project `.cursor/mcp.json` |
| Claude Desktop | JSON, `mcpServers` | `claude_desktop_config.json` |

Merge the `paperpilot` entry into an existing configuration, preserving other
servers and settings. Do not redirect output over an existing settings file.
Project configurations may require the client's workspace trust or approval.

## Codex

Register with the CLI:

```sh
codex mcp add paperpilot -- "/absolute/path/to/paperpilot-mcp"
codex mcp list
```

Or merge the generated TOML into `~/.codex/config.toml`:

```toml
[mcp_servers.paperpilot]
command = "/absolute/path/to/paperpilot-mcp"
args = []
```

Start a new Codex session and inspect `/mcp`.
See the [official Codex MCP documentation](https://developers.openai.com/codex/mcp/).

## Claude Code

```sh
claude mcp add --transport stdio --scope project paperpilot -- "/absolute/path/to/paperpilot-mcp"
claude mcp get paperpilot
```

This writes the project's `.mcp.json`. For personal configuration across projects,
use `--scope user` instead. In a new session, use `/mcp` to inspect the connection
and approve the project server when prompted.
See the [official Claude Code MCP documentation](https://code.claude.com/docs/en/mcp).

## Kimi Code

Merge the following into `.kimi-code/mcp.json` (project) or
`~/.kimi-code/mcp.json` (user), then start a **new session**:

```json
{
  "mcpServers": {
    "paperpilot": {
      "command": "/absolute/path/to/paperpilot-mcp",
      "args": []
    }
  }
}
```

Use `/mcp` to check the connection. Existing sessions do not automatically load
newly configured servers. Older `kimi-cli` installations may use `~/.kimi/`;
check your installed version's documentation before migrating settings.
See the [official Kimi Code MCP documentation](https://www.kimi.com/code/docs/en/kimi-code-cli/customization/mcp.html).

## Pi

Current Pi versions support MCP directly:

```sh
npm install -g @earendil-works/pi-coding-agent
pi mcp add paperpilot -- "/absolute/path/to/paperpilot-mcp"
pi mcp list
```

Or merge the generated JSON into `.pi/mcp.json` or `~/.pi/agent/mcp.json`.
Start a new session or use `/reload`, then inspect `/mcp`.

The older `@mariozechner/pi-coding-agent` package is deprecated. Some older Pi
versions do not include built-in MCP. If `pi mcp --help` is
unavailable, upgrade to a version with native MCP support, or use the PaperPilot
CLI from Pi's shell tool. Third-party MCP extensions can override native MCP
behavior; follow the extension's configuration instructions in that case.
See [Pi's upstream MCP guide](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/mcp.md).

## Try a document task

Use absolute input and output paths in your prompt, for example:

> Use PaperPilot to merge `/work/report-a.pdf` and `/work/report-b.pdf`,
> saving the result to `/work/combined.pdf`.

The agent should discover `pdf_merge` with `inputs` (an array of paths) and
`output` (the destination). The MCP process must have access to those files.
If your agent runs in a container, WSL, SSH session, or sandbox, install the
server there and use paths visible in that environment.

On Windows, MSVC builds need Microsoft's C++ build tools. GNU builds need a
compatible MinGW C/C++ compiler; the workspace uses Rust's bundled linker runtime
and GNU stack-size flags for that target. The two Windows targets have separate
linker settings.

## Troubleshooting

- **Executable not found:** use an absolute path; on Windows include `.exe`.
  JSON backslashes must be escaped, for example `C:\\Tools\\paperpilot-mcp.exe`.
  The generator handles escaping automatically.
- **No tools or initialization timeout:** build this version of the server.
  Default mode reserves stdin/stdout exclusively for MCP; diagnostics go to stderr.
  `--direct-tool-call` is a separate one-request diagnostic mode and must not be
  included in client configuration.
- **REST URL rejected:** the gateway at `localhost:7823` is a REST API, not an MCP
  HTTP endpoint. Configure the stdio executable for these integrations.
- **Tool failure:** check local file permissions and the operation's input format.
  Native rendering operations may also need the project's PDFium setup.
- **Codex headless approval error:** approve the operation in an interactive
  session, or explicitly preapprove only the intended tool for an authorized
  automated task. A headless run with approval policy `never` cannot answer a
  pending tool approval. See the official MCP documentation's per-tool
  `approval_mode` settings.
- **Older Kimi on a Windows GBK console:** if `kimi mcp test` fails while printing
  a checkmark, use a UTF-8 Python console (`PYTHONUTF8=1` and
  `PYTHONIOENCODING=utf-8`). This is separate from server connectivity.

## Developer checks

```sh
cargo fmt --all -- --check
cargo test -p paperpilot-mcp
```

The process tests cover immediate, delayed, and fragmented initialization,
tool capability advertisement, tool discovery, JSON-RPC tool errors, configuration
parsing for all supported clients, and invalid flag combinations. They do not
require agent accounts. Interactive client sessions still need verification on
the installed agent versions.
