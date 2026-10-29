# Spec 142: Desktop Real Watched Folders Daemon & Document Smart Filter Tabs

## Metadata
- **Spec Number**: 142
- **Phase**: Phase 4.7.8 (Desktop Automation & Document Intelligence)
- **Status**: 📋 Ready for Implementation
- **Target Branch**: `feat/desktop-watched-folders-and-smart-filters-task-4-7-8`
- **Impacted Crates & Packages**:
  - `apps/desktop/src-tauri` (`apps/desktop/src-tauri/src/watch_commands.rs`, `apps/desktop/src-tauri/src/lib.rs`)
  - `apps/desktop/src` (`apps/desktop/src/lib/components/layout/*`, `apps/desktop/src/lib/components/ui/DocumentList.svelte`, `apps/desktop/src/lib/state/app.svelte.ts`)
- **Parent Specs**:
  - [Spec 139: Headless Hot Folder Watcher Daemon](../139-hot-folder-watcher-daemon/spec.md)
  - [Spec 136: Conditional Workflow Engine](../136-conditional-pipeline-execution-engine/spec.md)

---

## 1. Overview & Objective

In Spec 139, the hot folder watcher daemon (`paperpilot watch`) was implemented in `paperpilot_pdf::operations::watch` and exposed to the CLI.
However, in the desktop app:
1. **Watched Folders** were missing entirely from the GUI (only mocked in a discontinued V2 PR without Tauri IPC wiring).
2. **"Smart Categories"** were previously hardcoded mock badges instead of real dynamic document filters.

**Task 4.7.8 / Spec 142 delivers genuine, connected implementations of both features:**
1. **Real Watched Folders Daemon in Desktop**:
   - Expose Tauri commands to register, start, stop, and inspect filesystem watch folders running against active `.ppflow` workflows.
   - Provide an **"Automations / Watched Folders"** view in the desktop GUI (accessible from the sidebar or settings drawer) allowing users to pick a local folder, select a pipeline preset, and toggle background watching.
   - Emits Tauri events (`watch:file_processed`, `watch:error`) so the GUI updates in real time when incoming PDFs are ingested.
2. **Real Document Smart Filter Pills (replacing mock categories)**:
   - Provide a dynamic filter strip directly above `DocumentList.svelte`:
     - `All Documents` (count of all loaded files)
     - `Scanned` (filtered by `isScanned === true`)
     - `Digitally Signed` (filtered by `isSigned === true`)
     - `Form Fields` (filtered by `hasFormFields === true`)
     - `Multi-Page` (filtered by `pageCount > 1`)
   - Filtering is reactive against real loaded documents in `appState.documents`.

---

## 2. Technical Architecture & Tauri IPC

### 2.1 Tauri Commands (`apps/desktop/src-tauri/src/watch_commands.rs`)

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::sync::oneshot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedFolderConfig {
    pub id: String,
    pub path: String,
    pub recipe_id_or_path: String,
    pub output_dir: Option<String>,
    pub is_active: bool,
}

lazy_static::lazy_static! {
    static ref WATCHER_HANDLES: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref WATCHED_FOLDERS: Arc<Mutex<Vec<WatchedFolderConfig>>> =
        Arc::new(Mutex::new(Vec::new()));
}

#[tauri::command]
pub async fn start_watch_folder(
    app_handle: tauri::AppHandle,
    config: WatchedFolderConfig,
) -> Result<(), String> {
    // Spawns paperpilot_pdf::operations::watch::handle_watch with cancellation channel
    // Emits "watcher-event" to app_handle when files are processed
    Ok(())
}

#[tauri::command]
pub fn stop_watch_folder(folder_id: String) -> Result<(), String> {
    // Sends shutdown signal to oneshot channel
    Ok(())
}

#[tauri::command]
pub fn list_watched_folders() -> Result<Vec<WatchedFolderConfig>, String> {
    let folders = WATCHED_FOLDERS.lock().unwrap();
    Ok(folders.clone())
}
```

### 2.2 Svelte State Model (`apps/desktop/src/lib/state/watchers.svelte.ts`)

```typescript
export interface WatchedFolder {
  id: string;
  path: string;
  recipeName: string;
  outputDir?: string;
  isActive: boolean;
  lastProcessedFile?: string;
  lastProcessedAt?: string;
}

export class WatchersState {
  folders = $state<WatchedFolder[]>([]);
  
  async loadFolders() { ... }
  async addFolder(path: string, recipeName: string, outputDir?: string) { ... }
  async toggleFolder(id: string) { ... }
  async removeFolder(id: string) { ... }
}

export const watchersState = new WatchersState();
```

---

## 3. UI Component Specifications

### 3.1 Watched Folders Management Panel (`apps/desktop/src/lib/components/layout/WatchedFoldersPanel.svelte`)
- Placed in Sidebar navigation under `Automations` (`🤖`) or within `SettingsPanel.svelte`.
- Renders:
  - Header: "Watched Folders (Hot Folders)" with status pill (e.g. `🟢 2 Running`).
  - List of watched folders showing:
    - Folder path (`~/Downloads/Scans`).
    - Assigned workflow (`Auto-OCR & Compress`).
    - Active toggle switch (Start / Stop daemon).
    - Delete button (`✕`).
  - "+ Add Folder Watcher" button opening folder selector and recipe dropdown.

### 3.2 Dynamic Smart Filters Strip (`DocumentList.svelte`)
- Above the document items:
  ```svelte
  <div class="filter-pills">
    {#each filters as f}
      <button
        class="filter-pill"
        class:active={activeFilter === f.id}
        onclick={() => activeFilter = f.id}
      >
        {f.label} ({f.count})
      </button>
    {/each}
  </div>
  ```
- Dynamically counts and filters:
  - `All` (`appState.documents.length`)
  - `Scanned` (count of documents where `doc.isScanned === true`)
  - `Signed` (count of documents where `doc.isSigned === true`)
  - `Forms` (count of documents where `doc.hasFormFields === true`)
  - `Multi-Page` (count of documents where `doc.pageCount > 1`)

---

## 4. Acceptance Criteria Checklist

- [ ] Backend & Tauri IPC:
  - `start_watch_folder`, `stop_watch_folder`, `list_watched_folders` registered in `desktop_lib::run()`.
  - Watcher gracefully routes processed documents to output directory.
- [ ] Desktop State & UI:
  - `WatchedFoldersPanel.svelte` lets users pick folder, assign pipeline flow, and toggle active state.
  - `watchersState` persists watched folder configurations in `localStorage`.
  - `DocumentList.svelte` displays dynamic Smart Filter pills that update reactively as documents are added or removed.
- [ ] Tests & Dark Mode:
  - Unit tests verifying filter pill counts and active filter filtering.
  - Tests for watcher state serialization.
  - All 117+ tests pass cleanly via `pnpm --dir apps/desktop test`.
