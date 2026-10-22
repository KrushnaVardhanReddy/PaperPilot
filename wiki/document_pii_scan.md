# `document_pii_scan`

## Overview
The `document_pii_scan` component (Spec 086) implements an automated PII and sensitive information scanner across the PaperPilot multi-surface architecture. It strictly adheres to the "Single Core Engine + 5 Thin Wrappers" design pattern.

## Core Engine
Located in `paperpilot-pdf/src/operations/pii.rs`, the core engine is pure Rust and executes natively in memory using `lopdf`. It is capable of extracting text layout strings and pattern matching various structured elements utilizing the `regex` crate and custom validations:
- **SSN:** Detects valid formatted Social Security Numbers.
- **Credit Cards:** Extracts digit sequences and validates them using the standard Luhn algorithm.
- **Email Addresses:** Detects and captures valid internet e-mail addresses.
- **Phone Numbers:** Extracts typical formatted North American phone patterns.
- **IBANs:** Uses standard country/digit formatting logic to extract International Bank Account Numbers.

The results are packaged in a structured, serialized JSON report object (`PiiScanReport`) detailing category frequencies and context snippets, ensuring the output is perfectly formed for downstream LLM integrations.

## 5 Thin Wrappers
1. **CLI (`paperpilot-cli`):** Provided via `paperpilot pii-scan --input <in.pdf>`.
2. **MCP (`paperpilot-mcp`):** Registered as `document_pii_scan` with input parameters matching the standard options boolean flag interface.
3. **REST Gateway (`paperpilot-gateway`):** Exposed on POST `/api/v1/pdf/pii-scan`.
4. **WASM Browser:** The identical logic runs client-side strictly avoiding server processing.
5. **Cloudflare Edge:** Runs via the V8 isolation context for scalable REST API invocation via Worker.
