# WASM 15 Tools Suite

## Overview
The PaperPilot WebAssembly engine has been expanded from 12 tools to 15 tools to include advanced client-side processing features utilizing pure-Rust image manipulation and cryptographic hashing.

## New Tools

### 1. `images_to_pdf`
Converts an array of image buffers (PNG or JPEG) into a single, multi-page PDF document.
* Uses the `image` crate to load from memory and detect dimensions.
* Compresses PNG images via `flate2` and Zlib encoding for `FlateDecode`.
* Embeds raw JPEG bytes via `DCTDecode`.

### 2. `extract_images`
Extracts all embedded images from a given PDF into raw image buffers (PNG/JPEG).
* Parses `/XObject` definitions in the PDF dictionary looking for `/Subtype /Image`.
* Identifies JPEGs via `/DCTDecode` and exports directly.
* Decodes Zlib compressed streams (`/FlateDecode`), converts to `ImageBuffer`, and re-encodes to PNG bytes.

### 3. `pdf_hash`
Computes the SHA-256 cryptographic integrity hash of a PDF entirely in memory.
* Uses the `sha2` crate.
* Returns a standard hexadecimal formatted string representation of the digest.

## Architecture
These tools execute inside a 100% client-side Web Worker bridging Svelte 5 and the `wasm32-unknown-unknown` compiled `paperpilot-wasm` engine, ensuring zero data transmission to cloud providers.
