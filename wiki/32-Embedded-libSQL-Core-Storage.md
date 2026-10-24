# Embedded libSQL Core Storage

## Overview

The `paperpilot-storage` module introduces an embedded **Turso / libSQL** local storage engine. This provides air-gapped, isolated persistence for:

1. Asynchronous job queues
2. Outbound webhooks tracking
3. Audit compliance logging
4. Document chunk metadata

This architecture is entirely self-contained and does NOT interact with PDF processing logic.

## Motivation

PaperPilot requires a robust and self-contained database solution that maintains the "Air-gapped by Default" requirement while offering local, resilient state management across application restarts. `libSQL` was chosen due to its SQLite compatibility, async capabilities via `tokio`, and lightweight footprint.

## Models

### Job Lifecycle
A job goes through several states defined by `JobStatus`:
- `Pending`
- `Running`
- `Completed`
- `Failed`

Jobs are tracked using the `JobRecord` struct, which contains paths, parameters, error messages, and progress details.

### Audit Events
Audit logs are structured using the `AuditEvent` struct to ensure every action within the system can be tracked with details on the actor, tool, payload hash, and timestamp.

## Initialization

The storage engine can be initialized locally or in-memory (useful for testing):

```rust
use paperpilot_storage::StorageEngine;
use std::path::Path;

// Local persistence
let engine = StorageEngine::open_local(Path::new("./data.db")).await.unwrap();

// In-memory
let test_engine = StorageEngine::open_in_memory().await.unwrap();
```

Migrations are automatically run upon initialization.
