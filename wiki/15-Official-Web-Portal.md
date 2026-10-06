# Official Web Portal

The `apps/web/` directory serves as the official landing page and conversion portal for `usepaperpilot.com`. It is built with Svelte 5 and Vite, focusing on a "Product IS the Landing Page" philosophy by embedding the core WASM engine directly into the user experience.

## Architecture & Components

The portal is composed of several key sections, cleanly separated into modular Svelte components within `apps/web/src/components/`:

### 1. `HeroPlayground.svelte`
The immediate entry point for the user. It features strong value-proposition copy and mounts the `OperationsView.svelte` (the actual working WASM tool interface) inside a stylized OS window frame. This allows users to immediately test the product's capabilities without downloading or signing up.

### 2. `TriSurfaceShowcase.svelte`
A visually distinct three-column layout that explains the different ways PaperPilot can be consumed:
*   **Desktop App:** For native, offline usage with maximum privacy.
*   **Embed Widget:** A drop-in solution for website owners.
*   **API & MCP:** Headless integration for developers and AI agents.

### 3. `EmbedGenerator.svelte`
An interactive tool leveraging Svelte 5's reactivity (`$state` and `$derived` runes) to let users configure a custom PDF widget. It updates an HTML snippet in real-time as users select tools, change colors, and toggle themes, providing a seamless "copy-to-clipboard" experience.

### 4. `ApiExplorer.svelte`
A developer-focused section featuring a tabbed code editor interface. It provides quick-start snippets for interacting with PaperPilot via cURL, TypeScript, Python, and Model Context Protocol (MCP), reinforcing its readiness for modern AI and backend integrations.

### 5. `BenchmarkMatrix.svelte`
A comparative data table that positions PaperPilot against industry incumbents (Adobe Acrobat, iLovePDF, Stirling-PDF) across critical metrics: privacy (client-side vs. cloud), speed, security (Rust vs. C++), licensing, and size.

## State Management

The portal utilizes Svelte 5's rune system for local state management, particularly within the interactive components like `EmbedGenerator` and `ApiExplorer`. This ensures reactive updates to UI elements and code snippets without the need for complex global stores.

## Testing

End-to-end (E2E) testing is implemented using Playwright. Tests are located in `apps/web/tests/portal_landing.spec.ts` and cover:
*   Visibility and content of key structural elements.
*   Interactive behavior of the Embed Generator (reactivity of the code snippet and clipboard functionality).
*   Tab switching logic in the API Explorer.
*   Smooth scrolling navigation from Calls to Action (CTAs).