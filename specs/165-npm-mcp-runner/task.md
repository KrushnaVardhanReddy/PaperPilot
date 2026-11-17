# Task 165: Package and Distribute `@usepaperpilot/mcp` on NPM

## Checklist
- [x] Scaffold `packages/paperpilot-mcp` directory structure
- [x] Create `packages/paperpilot-mcp/package.json` with scoped name `@usepaperpilot/mcp`, version `0.7.0`, and bin entry point
- [x] Implement `bin/paperpilot-mcp.js` cross-platform binary resolver and release asset downloader
- [x] Guarantee stdio isolation (all logs to stderr, clean stdin/stdout piping for MCP JSON-RPC protocol)
- [x] Implement local binary resolution (target/release, target/debug, system PATH)
- [x] Align release matrix names with `.github/workflows/release.yml` and tag versioning (`v${pkgVersion}`)
- [x] Add `packages/paperpilot-mcp/README.md` with Claude Desktop, Cursor, and Zed client configurations
- [x] Verify local execution via diagnostic flags (`--help`) and JSON-RPC initialize/tools list
- [x] Create `@usepaperpilot` organization on npmjs.com
- [x] Publish `@usepaperpilot/mcp@0.7.0` to npm registry with public access
- [x] Update specs, tasks, and refresh knowledge graph via `graphify-rs build`
