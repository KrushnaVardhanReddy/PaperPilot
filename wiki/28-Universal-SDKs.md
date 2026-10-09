# Universal SDKs

PaperPilot provides lightweight, high-performance client libraries for Python and TypeScript/JavaScript. These SDKs abstract away the complexity of invoking the Rust binary locally or communicating with the Gateway REST API remotely.

## Architecture

The SDKs follow a dual-mode execution strategy:

1. **Local Mode:** When instantiated without an `endpointUrl`, the SDK acts as a wrapper around the `paperpilot-cli` binary. It shells out to the binary using `subprocess` (Python) or `child_process` (Node.js). This ensures zero overhead parsing and pure Rust performance, without linking complex C-extensions.
2. **Remote Mode:** When instantiated with an `endpointUrl` (and optional `apiKey`), the SDK acts as a REST client to a running instance of `paperpilot-gateway`. Payload models map directly to the Gateway OpenAPI definitions.

## Key Components

### TypeScript (`packages/paperpilot-js`)
- `PaperPilot` Class: Exposes asynchronous methods like `merge`, `compress`, `split`, and `batesStamp`.
- Execution switching occurs transparently within the methods based on client configuration.

### Python (`packages/paperpilot-py`)
- Pydantic Models: Validation schemas for requests ensuring type safety and hinting.
- Sync & Async Support: Native support for both synchronous (`subprocess` / `httpx.Client`) and asynchronous (`httpx.AsyncClient`) workflows, satisfying the Python ecosystem's demand for high-concurrency event loops.
