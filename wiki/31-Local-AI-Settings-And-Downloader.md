# Local AI Settings & Model Downloader

PaperPilot's Local AI architecture allows users to choose how natural language queries and document intelligence tasks are executed, prioritizing privacy and offline capabilities.

## Architecture

The system is composed of a Tauri Rust backend and a Svelte 5 frontend.

### Frontend State (`aiConfig.svelte.ts`)
The configuration is managed using Svelte 5 runes (`$state`). It tracks the active mode and any required endpoint or provider details. The API keys themselves are *never* stored persistently in the frontend state.

### Backend Handlers (`ai_commands.rs`)
*   **Keyring Security:** When the user configures a BYOK (Bring Your Own Key) provider (like OpenAI or Anthropic), the API key is passed securely over the IPC bridge and saved directly to the OS Keyring (using the `keyring` crate). The key is not stored in plain-text configuration files or localStorage.
*   **Model Downloader:** To support the `Llamafile` local execution mode, the app allows users to download curated `.gguf` models.
    *   Models are downloaded directly to the OS's local data directory (e.g., `~/.local/share/paperpilot/models/`).
    *   The downloader implements chunk streaming and resume logic via `reqwest` and `Range` headers to prevent data loss on network drops for multi-gigabyte models.
    *   Tauri emits events (`model-download-progress`) to the Svelte frontend, enabling real-time progress bars with granular byte counts.
    *   The downloader supports cancellation.

## Available AI Modes

1.  **⚡ Offline NLP Mode (Default):** Uses a 100% on-device embedded ONNX model. Extremely lightweight (<1.1MB), zero setup, and requires no internet.
2.  **🚀 Offline Llamafile:** A standalone local LLM (like Qwen 2.5 Coder or Llama 3.2 3B). Requires downloading a model via the built-in downloader.
3.  **🌐 Universal Endpoint:** Connects to local or LAN-hosted AI endpoints (like Ollama on `:11434`, LM Studio, or vLLM).
4.  **🔑 Commercial BYOK Cloud:** Connects to commercial cloud providers (OpenAI, Anthropic, Gemini). Keys are securely locked in the OS Keyring.

## Usage
Users can access these settings through the AI Settings Modal drawer in the desktop application. Visual feedback is provided when testing endpoint connections or actively downloading models.
