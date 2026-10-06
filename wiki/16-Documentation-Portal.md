# Architecture: Documentation Portal (Astro Starlight)

## Module
`apps/docs/`

## Technology Stack
- **Framework:** Astro 4.x
- **Theme:** `@astrojs/starlight`
- **Search:** Pagefind (Client-side WASM search)
- **Deployment Target:** Cloudflare Pages (Static)

## Architectural Design
The documentation portal is designed as a fully static site to ensure zero cloud compute costs, fast load times, and complete independence from backend services.

### Core Features
1. **Zero-Cloud Local Search:** Using Pagefind, the search index is built statically at compile time. Queries are executed locally via WebAssembly in the user's browser, matching PaperPilot's overall philosophy of local execution and privacy (~50-100ms WASM Execution).
2. **Automated Documentation Sourcing:** The Master 44-Tool Reference is automatically populated using the verified ground truth generated during our E2E Tri-Interface testing phase (`reports/TRI_INTERFACE_E2E_100_VERIFIED.md`). This ensures the CLI commands, MCP JSON-RPC payloads, and REST API cURL requests are 100% accurate and functional, precisely listing all 44 tools matching the Master test execution suite.
3. **Pillar Structure:**
   - **Getting Started:** For standard users (Desktop, CLI, Web).
   - **Embed Widget:** For webmasters deploying the drop-in component.
   - **Reference:** Comprehensive domain-mapped guide to every atomic tool.
   - **Developer Gateway:** For integrating AI Agents (Claude Desktop, Cursor IDE) via MCP, and querying the local REST API.

## Build and Deployment
- Locally, `npm --prefix apps/docs run dev` starts the Astro dev server.
- The build process (`npm --prefix apps/docs run build`) outputs a highly optimized static bundle into `apps/docs/dist/`, which is directly deployable to Edge networks like Cloudflare Pages.
