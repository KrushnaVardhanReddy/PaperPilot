# PaperPilot Tri-Interface E2E Verification & Interactive Living Documentation

## Executive Scorecard
| Interface | Pass Rate | Avg Latency |
|---|---|---|
| CLI | 5/10 (50.0%) | 16.65ms |
| MCP | 7/10 (70.0%) | 10.17ms |
| REST API | 7/10 (70.0%) | 4.03ms |

### Tool 11: `pdf_encrypt`
**Description**: Encrypt PDF with user/owner password.
**Input Snapshot**: /app/tests/e2e_fixtures/single_page.pdf (539 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli encrypt --input /app/tests/e2e_fixtures/single_page.pdf --user-password secret123 --owner-password secret123 --output /app/tests/e2e_fixtures/out/tri_e2e/encrypted_test.pdf.cli --json
```
**Result**: PASS ✅ (20.46ms)
**Details**: Exit code 0. Output file created (750 bytes)



#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "pdf_encrypt", "arguments": {"input": "/app/tests/e2e_fixtures/single_page.pdf", "password": "secret123", "owner_password": "secret123", "output": "/app/tests/e2e_fixtures/out/tri_e2e/encrypted_test.pdf.mcp"}}}
```
**Result**: PASS ✅ (13.35ms)
**Details**: Output file created (750 bytes)


#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_encrypt -H 'Content-Type: application/json' -d '{"input":"/app/tests/e2e_fixtures/single_page.pdf", "password":"secret123", "owner_password":"secret123", "output":"/app/tests/e2e_fixtures/out/tri_e2e/encrypted_test.pdf.api"}'
```
**Result**: PASS ✅ (6.77ms)
**Details**: Output file created (751 bytes)


### Tool 12: `pdf_decrypt`
**Description**: Decrypt password-protected PDF.
**Input Snapshot**: /app/tests/e2e_fixtures/encrypted.pdf (867 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli decrypt --input /app/tests/e2e_fixtures/encrypted.pdf --password testpass123 --output /app/tests/e2e_fixtures/out/tri_e2e/decrypted_test.pdf.cli --json
```
**Result**: FAIL ❌ (14.58ms)
**Details**: Exit code 1.
**Output**:
```
{"success":false,"operation":"Decrypt","error":"Unsupported operation: Decryption failed or invalid password: decryption error"}

```


#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "pdf_decrypt", "arguments": {"input": "/app/tests/e2e_fixtures/encrypted.pdf", "password": "testpass123", "output": "/app/tests/e2e_fixtures/out/tri_e2e/decrypted_test.pdf.mcp"}}}
```
**Result**: FAIL ❌ (10.00ms)
**Details**: Failed to get response: {'error': 'Connection closed. stderr: Error executing MCP tool: ErrorData { code: ErrorCode(-32602), message: "Unsupported operation: Decryption failed or invalid password: decryption error", data: None }\n', 'latency_ms': 9.998083114624023}
**Response**:
```json
{}
```

#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_decrypt -H 'Content-Type: application/json' -d '{"input":"/app/tests/e2e_fixtures/encrypted.pdf", "password":"testpass123", "output":"/app/tests/e2e_fixtures/out/tri_e2e/decrypted_test.pdf.api"}'
```
**Result**: FAIL ❌ (3.36ms)
**Details**: HTTP None: None
**Response**:
```json

```

### Tool 13: `pdf_redact`
**Description**: Black out and redact text/coordinates.
**Input Snapshot**: /app/tests/e2e_fixtures/search_test.pdf (3499 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli redact --input /app/tests/e2e_fixtures/search_test.pdf --pages 1 --rect 50,50,200,50 --output /app/tests/e2e_fixtures/out/tri_e2e/redacted_test.pdf.cli --json
```
**Result**: FAIL ❌ (17.64ms)
**Details**: Exit code 1.
**Output**:
```
{"success":false,"operation":"Redact","error":"Other error: Page 1 not found"}

```
**Error**:
```
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 61: IndirectObject { offset: 61 }
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 1224: IndirectObject { offset: 1224 }
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 92: IndirectObject { offset: 92 }
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 1909: IndirectObject { offset: 1909 }
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 1
```

#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "pdf_redact", "arguments": {"input": "/app/tests/e2e_fixtures/search_test.pdf", "page": 1, "x": 50, "y": 50, "width": 200, "height": 50, "output": "/app/tests/e2e_fixtures/out/tri_e2e/redacted_test.pdf.mcp"}}}
```
**Result**: FAIL ❌ (10.69ms)
**Details**: Failed to get response: {'error': 'Connection closed. stderr: Error executing MCP tool: ErrorData { code: ErrorCode(-32602), message: "Other error: Page 1 not found", data: None }\n', 'latency_ms': 10.68735122680664}
**Response**:
```json
{}
```

#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_redact -H 'Content-Type: application/json' -d '{"input":"/app/tests/e2e_fixtures/search_test.pdf", "page":1, "x":50, "y":50, "width":200, "height":50, "output":"/app/tests/e2e_fixtures/out/tri_e2e/redacted_test.pdf.api"}'
```
**Result**: FAIL ❌ (2.70ms)
**Details**: HTTP None: None
**Response**:
```json

```

### Tool 14: `pdf_sign`
**Description**: Digital signature / stamp certification.
**Input Snapshot**: /app/tests/e2e_fixtures/single_page.pdf (539 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli signature --input /app/tests/e2e_fixtures/single_page.pdf --cert /app/tests/e2e_fixtures/out/tri_e2e/dummy.p12 --output /app/tests/e2e_fixtures/out/tri_e2e/signed_test.pdf.cli --json
```
**Result**: PASS ✅ (18.30ms)
**Details**: Exit code 0. Output file created (596 bytes)



#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "pdf_sign", "arguments": {"input": "/app/tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/signed_test.pdf.mcp", "cert_path": "/app/tests/e2e_fixtures/out/tri_e2e/dummy.p12", "password": "pass", "reason": "Verified"}}}
```
**Result**: PASS ✅ (9.63ms)
**Details**: Output file created (596 bytes)


#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_sign -H 'Content-Type: application/json' -d '{"input":"/app/tests/e2e_fixtures/single_page.pdf", "output":"/app/tests/e2e_fixtures/out/tri_e2e/signed_test.pdf.api", "cert_path":"/app/tests/e2e_fixtures/out/tri_e2e/dummy.p12", "password":"pass", "reason":"Verified"}'
```
**Result**: PASS ✅ (4.03ms)
**Details**: Output file created (596 bytes)


### Tool 15: `pdf_validate`
**Description**: Structural validation & corruption check.
**Input Snapshot**: /app/tests/e2e_fixtures/single_page.pdf (539 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli validate --input /app/tests/e2e_fixtures/single_page.pdf --json
```
**Result**: PASS ✅ (15.93ms)
**Details**: Exit code 0. No output file expected.



#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": {"name": "pdf_validate", "arguments": {"input": "/app/tests/e2e_fixtures/single_page.pdf"}}}
```
**Result**: PASS ✅ (9.55ms)
**Details**: No output file expected.


#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_validate -H 'Content-Type: application/json' -d '{"input":"/app/tests/e2e_fixtures/single_page.pdf"}'
```
**Result**: PASS ✅ (3.97ms)
**Details**: No output file expected.


### Tool 16: `pdf_hash`
**Description**: Compute cryptographic integrity hash.
**Input Snapshot**: /app/tests/e2e_fixtures/single_page.pdf (539 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli hash --input /app/tests/e2e_fixtures/single_page.pdf --json
```
**Result**: PASS ✅ (14.26ms)
**Details**: Exit code 0. No output file expected.



#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": {"name": "pdf_hash", "arguments": {"input": "/app/tests/e2e_fixtures/single_page.pdf"}}}
```
**Result**: PASS ✅ (8.22ms)
**Details**: No output file expected.


#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_hash -H 'Content-Type: application/json' -d '{"input":"/app/tests/e2e_fixtures/single_page.pdf"}'
```
**Result**: PASS ✅ (2.58ms)
**Details**: No output file expected.


### Tool 17: `pdf_repair`
**Description**: Repair corrupted xref tables and trailers.
**Input Snapshot**: /app/tests/e2e_fixtures/single_page.pdf (539 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli repair --input /app/tests/e2e_fixtures/single_page.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/repaired_test.pdf.cli --json
```
**Result**: PASS ✅ (17.32ms)
**Details**: Exit code 0. Output file created (543 bytes)



#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 8, "method": "tools/call", "params": {"name": "pdf_repair", "arguments": {"input": "/app/tests/e2e_fixtures/single_page.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/repaired_test.pdf.mcp"}}}
```
**Result**: PASS ✅ (10.17ms)
**Details**: Output file created (543 bytes)


#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_repair -H 'Content-Type: application/json' -d '{"input":"/app/tests/e2e_fixtures/single_page.pdf", "output":"/app/tests/e2e_fixtures/out/tri_e2e/repaired_test.pdf.api"}'
```
**Result**: PASS ✅ (4.60ms)
**Details**: Output file created (543 bytes)


### Tool 18: `pdf_extract_text`
**Description**: Extract plain text from PDF pages.
**Input Snapshot**: /app/tests/e2e_fixtures/search_test.pdf (3499 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli extract-text --input /app/tests/e2e_fixtures/search_test.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_text.txt.cli --json
```
**Result**: FAIL ❌ (18.18ms)
**Details**: Exit code 0. Output file missing or empty: /app/tests/e2e_fixtures/out/tri_e2e/extracted_text.txt.cli
**Output**:
```
{"success":true,"operation":"ExtractText","error":null}

```
**Error**:
```
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 61: IndirectObject { offset: 61 }
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 814: IndirectObject { offset: 814 }
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 404: IndirectObject { offset: 404 }
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 1224: IndirectObject { offset: 1224 }
[2026-10-04T01:58:28Z ERROR lopdf::reader] Object load error at offset 9
```

#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 9, "method": "tools/call", "params": {"name": "pdf_extract_text", "arguments": {"input": "/app/tests/e2e_fixtures/search_test.pdf", "output": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_text.txt.mcp"}}}
```
**Result**: FAIL ❌ (8.88ms)
**Details**: Output file missing or empty: /app/tests/e2e_fixtures/out/tri_e2e/extracted_text.txt.mcp
**Response**:
```json
{"id": 9, "jsonrpc": "2.0", "result": {"data": null, "message": "Text extracted successfully.", "output_path": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_text.txt.mcp", "success": true}}
```

#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_text -H 'Content-Type: application/json' -d '{"input":"/app/tests/e2e_fixtures/search_test.pdf", "output":"/app/tests/e2e_fixtures/out/tri_e2e/extracted_text.txt.api"}'
```
**Result**: FAIL ❌ (2.42ms)
**Details**: HTTP None: None
**Response**:
```json

```

### Tool 19: `pdf_extract_images`
**Description**: Extract embedded raster images.
**Input Snapshot**: /app/tests/e2e_fixtures/image_doc.pdf (1330 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli extract-images --input /app/tests/e2e_fixtures/image_doc.pdf --output /app/tests/e2e_fixtures/out/tri_e2e/extracted_images.cli --json
```
**Result**: FAIL ❌ (16.56ms)
**Details**: Exit code 1.
**Output**:
```
{"success":false,"operation":"ExtractImages","error":"I/O error: Failed to create output file: No such file or directory (os error 2)"}

```


#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 10, "method": "tools/call", "params": {"name": "pdf_extract_images", "arguments": {"input": "/app/tests/e2e_fixtures/image_doc.pdf", "output_dir": "/app/tests/e2e_fixtures/out/tri_e2e/extracted_images.mcp"}}}
```
**Result**: PASS ✅ (9.93ms)
**Details**: Directory exists


#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_extract_images -H 'Content-Type: application/json' -d '{"input":"/app/tests/e2e_fixtures/image_doc.pdf", "output_dir":"/app/tests/e2e_fixtures/out/tri_e2e/extracted_images.api"}'
```
**Result**: PASS ✅ (4.75ms)
**Details**: Directory exists


### Tool 20: `pdf_images_to_pdf`
**Description**: Convert image files into a single PDF.
**Input Snapshot**: /app/tests/e2e_fixtures/img1.png (287 bytes)

#### 💻 CLI Example
```bash
/app/target/debug/paperpilot-cli images-to-pdf --images /app/tests/e2e_fixtures/img1.png /app/tests/e2e_fixtures/img2.png --output /app/tests/e2e_fixtures/out/tri_e2e/images_converted.pdf.cli --json
```
**Result**: FAIL ❌ (13.30ms)
**Details**: Exit code 2.
**Output**:
```

```
**Error**:
```
error: unexpected argument '/app/tests/e2e_fixtures/img2.png' found

Usage: paperpilot-cli images-to-pdf [OPTIONS] --images <IMAGES> --output <OUTPUT>

For more information, try '--help'.

```

#### 🤖 MCP Example
```json
{"jsonrpc": "2.0", "id": 11, "method": "tools/call", "params": {"name": "pdf_images_to_pdf", "arguments": {"inputs": ["/app/tests/e2e_fixtures/img1.png", "/app/tests/e2e_fixtures/img2.png"], "output": "/app/tests/e2e_fixtures/out/tri_e2e/images_converted.pdf.mcp"}}}
```
**Result**: PASS ✅ (11.34ms)
**Details**: Output file created (1574 bytes)


#### 🌐 REST API Example
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_images_to_pdf -H 'Content-Type: application/json' -d '{"inputs":["/app/tests/e2e_fixtures/img1.png", "/app/tests/e2e_fixtures/img2.png"], "output":"/app/tests/e2e_fixtures/out/tri_e2e/images_converted.pdf.api"}'
```
**Result**: PASS ✅ (5.09ms)
**Details**: Output file created (1574 bytes)
