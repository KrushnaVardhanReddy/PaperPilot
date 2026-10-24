# Task 103B: libSQL Cross-Document Vector Store & Semantic Search Engine

## Status
- Spec: [Spec 103](file:///home/krushna/Project/PaperPilot/specs/103-embedded-libsql-vector-store/spec.md)
- Subsystem: `paperpilot-storage`
- Target: Wave 7 / Enterprise Foundation
- Status: 📋 Scoped

---

## Objectives & Work Items

### 1. Vector Embeddings Persistence (`paperpilot-storage/src/vector.rs`)
- [ ] Implement `store_chunks(&self, chunks: &[NewChunk]) -> Result<(), StorageError>` in `StorageEngine`.
- [ ] Batch insert document chunks (document_hash, document_name, page_number, chunk_index, content, embedding_json).
- [ ] Store embeddings as serialized JSON float vectors (`Vec<f32>`).

### 2. Semantic Search & Cosine Similarity
- [ ] Implement `search_chunks(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<SearchResult>, StorageError>`.
- [ ] Calculate pure-Rust dot product and cosine similarity over indexed chunks.
- [ ] Return top-K scored chunks with document name, page number, similarity score, and excerpt.

### 3. Unit Tests & Verification
- [ ] Write integration test `test_vector_similarity_search`:
  - Insert 3 synthetic chunks with distinct vectors (e.g. `[1.0, 0.0]`, `[0.0, 1.0]`, `[0.707, 0.707]`).
  - Search with query `[1.0, 0.0]`, verify first result is exact match with cosine similarity ~1.0.
- [ ] Run `cargo test -p paperpilot-storage -j 2`
- [ ] Run `cargo check --workspace -j 2`
