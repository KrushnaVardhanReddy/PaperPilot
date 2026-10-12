# Spec 040: Universal `pdf_oxide` Core Migration Report

## Execution Summary
The migration of PaperPilot's extraction toolchain to `pdf_oxide` has been completed successfully across all five interfaces.

## Feature Upgrades Delivered
1. **Markdown Fidelity**: `pdf_convert_markdown` now preserves semantic table boundaries and text hierarchy.
2. **HTML Synthesis**: Established `pdf_to_html` as a first-class citizen in the Gateway API, MCP, and Desktop UI, generating structural HTML5 instead of raw `<p>` dumps.
3. **Bounding Box Search**: `pdf_search` now emits bounding box coordinates, allowing exact spatial rendering over frontend views.
4. **RAG Intelligence**: Implemented `pdf_rag_chunks`, splitting docs into vector-ready segments using layout headings. 

## Code Quality
* **Concurrency**: Rust workspaces verified against `-j 6` constraint.
* **Compatibility**: Integration runs successfully as native execution within WASM structures and avoids raw Python shell commands entirely. 

## Next Steps
Frontend systems will map the resulting bbox objects to highlighting layers on `pdf.js` canvas instances. 
