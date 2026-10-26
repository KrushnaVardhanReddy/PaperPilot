# Spec 126: Open-WebUI & Apache Tika Replacement Ingestion Adapter

## 1. Overview & Problem Statement
Users running Open-WebUI and local AI harnesses (e.g. user `Tepix` on Hacker News) want a drop-in replacement for Apache Tika and heavy Python parsing pipelines (`marker`, `docling`) that consumes minimal RAM and runs sub-15ms extraction.

## 2. Requirements & Objectives
1. **Tika-Compatible REST Endpoint**:
   - In `paperpilot-gateway/src/handlers.rs`, add:
     - `PUT /tika` or `POST /api/v1/tika/rmeta/text`: Accepts binary PDF streams and returns Apache Tika-compatible JSON or text responses.
2. **Streaming Text & Table Extraction**:
   - Extract raw text and markdown tables directly without spawning external JVM or Python processes.
3. **Open-WebUI Tool Manifest**:
   - Provide ready-to-paste tool specification in Open-WebUI's custom tool schema format (`docs/integrations/open_webui.md`).
4. **Integration Test**:
   - Add integration test in `paperpilot-gateway/tests/server_test.rs` simulating an Open-WebUI document upload request.
