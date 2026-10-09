# PaperPilot SDK Libraries Report

This report documents the newly implemented pure-Rust universal SDK client libraries for PaperPilot in TypeScript/JavaScript and Python. These libraries are designed for embedding PaperPilot into external applications and automation scripts, fulfilling Phase 8.4A of the roadmap.

## 1. TypeScript / JavaScript SDK (`packages/paperpilot-js`)

The TS/JS SDK provides a fluent, strongly-typed API for executing PaperPilot operations either locally (via the CLI binary) or remotely (via the Gateway REST API).

### Usage Example

```typescript
import { PaperPilot } from "@paperpilot/sdk";

// Local Execution (auto-discovers local binary)
const localClient = new PaperPilot();
await localClient.merge(["doc1.pdf", "doc2.pdf"], { output: "merged.pdf" });
await localClient.batesStamp("in.pdf", { prefix: "DOC-", start: 1, output: "stamped.pdf" });

// Remote Execution (Gateway)
const remoteClient = new PaperPilot({
    endpointUrl: "http://localhost:7823",
    apiKey: "secret_key"
});
await remoteClient.compress("huge.pdf", { quality: "medium", output: "compressed.pdf" });
await remoteClient.split("doc.pdf", { pages: "1-3", output: "split_dir" });
```

## 2. Python SDK (`packages/paperpilot-py`)

The Python SDK provides an ergonomic, type-annotated client for executing PaperPilot operations, supporting both synchronous and asynchronous modes.

### Usage Example

```python
from paperpilot import PaperPilot

# Synchronous Local Execution
client = PaperPilot()
client.bates_stamp("input.pdf", prefix="DISCOVERY-", start=1, output="bates.pdf")
client.merge(["doc1.pdf", "doc2.pdf"], output="merged.pdf")

# Asynchronous Remote Execution
import asyncio

async def run_remote():
    remote_client = PaperPilot(endpoint_url="http://localhost:7823", api_key="secret")
    await remote_client.compress_async("huge.pdf", quality="high", output="compressed.pdf")

asyncio.run(run_remote())
```

## 3. Performance & Architecture

- **Zero Heavy External Dependencies:** The Python SDK only relies on `httpx` and `pydantic`. The JS SDK relies only on native modules (`child_process`, `fetch`).
- **Subprocess Efficiency:** Local execution leverages the highly optimized Rust `paperpilot-cli` binary.
- **REST Fidelity:** Remote execution sends precisely structured JSON payloads mapping directly to the Gateway OpenAPI schemas.
