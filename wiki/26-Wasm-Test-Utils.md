# WASM Test Utils (@paperpilot/test-utils)

The `@paperpilot/test-utils` module brings high-performance, in-memory PDF assertions to JavaScript testing environments (Jest, Vitest, Bun, Mocha, etc.). It enables CI/CD pipelines to assert PDF content, properties, and metadata without installing external dependencies like Docker, Java (`pdfbox`), or Python (`pdfplumber`).

## Architecture
The module bridges `wasm-bindgen` and native TypeScript environments. We compile the core assertions in `paperpilot-wasm/src/assertions.rs` into NodeJS-compatible WASM, which is then dynamically evaluated by our Test Utils library.

### Key Components
1. **`PdfInspector` (Rust -> WASM):** A fast `lopdf`-backed evaluator exposing properties like:
   - `page_count`
   - `get_text` / `contains_text`
   - `page_dimensions`
   - `is_encrypted`
   - `form_fields`
   - `sha256_hex`

2. **TypeScript Wrapper (`packages/test-utils`):**
   Exposes Jest/Vitest compatible matchers for fluid API usage.
   - `toHavePageCount()`
   - `toContainPdfText()`
   - `toHaveFormField()`

## Usage
Simply install `@paperpilot/test-utils` into your NodeJS test suite and extend your assertion engine:

```typescript
import { expect } from 'vitest';
import { pdfMatchers } from '@paperpilot/test-utils';
import * as fs from 'fs';

expect.extend(pdfMatchers);

test('validate generated PDF', () => {
    const pdfBytes = fs.readFileSync('output.pdf');
    expect(pdfBytes).toHavePageCount(5);
    expect(pdfBytes).toContainPdfText('Confidential');
    expect(pdfBytes).toHaveFormField('SignatureField');
});
```

## Performance
Built completely in Rust using `lopdf`, execution operates fully in-memory, avoiding disk I/O and process-spawning latency. Standard assertions validate small to medium documents well under `< 10ms`.
