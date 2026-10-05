# Zero-Install Web App

PaperPilot's Zero-Install Web App provides a client-side only version of the core PDF processing tools, completely built on WebAssembly (WASM).

## Architecture

The web app is located in `apps/web/` and relies on:
- **Svelte 5 & Vite**: For rendering the user interface and bundling.
- **paperpilot-wasm**: The core engine compiled from Rust to WASM (`wasm32-unknown-unknown`). It contains operations for Merging, Splitting, Rotating, Compressing, Encrypting, and Watermarking PDFs.
- **Web Workers**: Used to offload the WASM computation to a background thread to prevent UI blocking during heavy PDF processing tasks.
- **Client-Side Only (No Server)**: File manipulation relies strictly on browser-native File API and memory buffering. No files are uploaded or sent over HTTP.

## Usage

When visiting the web version:
1. **Drop Files**: Users drop their PDFs directly into the browser.
2. **Select Tool**: Users choose one of the 6 core tools from the left sidebar.
3. **Execute**: The tool converts the file into a `Uint8Array`, sends it to the Web Worker bridging the WASM engine, and retrieves the modified byte array.
4. **Download**: A standard Blob anchor trick initiates the download of the processed PDF instantly.

## Viral Funnel Hook

The web application acts as a funnel for the PaperPilot desktop application. By instantly proving the product works with zero latency, it prompts users to download the full Desktop client for advanced functionality like OCR, Semantic Search, and Offline AI.
