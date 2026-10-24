# 103 Vector Store Engine

## Overview
The Vector Store Engine in `paperpilot-storage` provides a semantic search solution over document chunks. It leverages pure-Rust math to compute cosine similarity against a Turso/libSQL embedded SQLite database.

## Architecture
- **Data Models**: `NewDocumentChunk` (insertion), `DocumentChunkRecord` (retrieval), `SemanticSearchResult` (search output).
- **Vector Math**: Fast, pure-Rust implementation of cosine similarity (`paperpilot-storage/src/vector.rs`).
- **StorageEngine API**: Extended `StorageEngine` to handle insertion, deletion, fetching, counting, and searching semantic document chunks. Embeddings are serialized as JSON strings in the database.

## Testing
Unit tests in `paperpilot-storage/src/lib.rs` and `paperpilot-storage/src/vector.rs` verify similarity math, CRUD, and semantic search ranking behavior in-memory without mocks.
