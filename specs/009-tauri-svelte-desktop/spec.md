# Spec: Phase 3 — Tauri 2.0 Desktop Application Architecture

## Objective
Build a native, cross-platform desktop application using Tauri 2.0 and Svelte 5 (runes). The app must run completely offline, execute fast PDF mutations via the shared Rust backend, and protect host memory through lazy loading.

## Technology Stack (Locked)
- **Shell**: Tauri 2.0 (`tauri = "2"`, `features = ["protocol-asset"]`)
- **Frontend**: SvelteKit / Svelte 5 (runes only: `$state`, `$derived`, `$effect`, `$props`)
- **Styling**: Vanilla CSS with design system custom properties (`var(--bg-primary)`, `var(--bg-surface)`, etc.). TailwindCSS is prohibited.
- **Plugins**: `@tauri-apps/plugin-dialog`, `@tauri-apps/plugin-opener`.

## IPC Bridge & File Management

### 1. The Linux WebKitGTK File Path Constraint
In WebKitGTK (Linux Tauri runtime), the standard HTML5 `File.prototype.path` getter is non-writable and returns an empty string or throws errors.
- **Solution**: Native file browsing via `@tauri-apps/plugin-dialog` returns absolute OS filesystem paths.
- Paths are stored in `(file as any)._localPath` and mirrored in `appState.documentPaths[]`.

### 2. High-Volume Lazy Ingestion (Zero-Byte Placeholders)
When loading large documents or batch importing (e.g. 50–100 files):
- Rust exposes `get_file_metadata(path: String) -> Result<FileMetadata, String>` querying `std::fs::metadata`.
- Frontend creates lightweight 0-byte `File` placeholders with Real `size` and `_isLoaded: false`.
- Binary payloads are only loaded into memory via `invoke('read_file_bytes', { path })` on-demand when a document is actively viewed in the canvas.

### 3. Open Tab Ceiling
- Tab bar enforces `MAX_OPEN_TABS = 8`.
- Multi-file batch uploads add all files to the **Documents Library list** but do NOT create 100 individual tabs.
- Single file uploads beyond the limit automatically evict the oldest unselected tab to prevent tab bar overflow.
