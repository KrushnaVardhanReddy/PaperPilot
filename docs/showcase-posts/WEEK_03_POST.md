# Week 03: Structure-Aware RAG vs. Naive Text Chunking

**Theme:** Advanced Retrieval-Augmented Generation (RAG) & Vector Ingestion  
**Tools Featured:** `pdf_to_markdown`, `pdf_extract_text`, `pdf_chunk`  
**Target Audience:** AI Engineers, Vector DB Specialists (Qdrant, Pinecone, Weaviate), RAG Practitioners  
**Asset:** ![Week 03 Demo](../assets/showcase/week03-structure-aware-rag.gif)

---

## 📝 LinkedIn Post Copy

### Hook
Why does your enterprise RAG pipeline hallucinate on financial tables and pricing agreements? 📉

It's not your embedding model. It's your chunker. 🧩

---

### The Problem: Naive Character Splitting
Most RAG ingestion pipelines do something like this:
```python
# The hallucination generator:
RecursiveCharacterTextSplitter(chunk_size=500, chunk_overlap=50)
```

When this naive window hits a 5-column financial statement or service-level agreement:
- Headers get severed from their data rows.
- Currency figures lose their column context (`$12,000` floats without knowing if it's "Annual Cost" or "Setup Fee").
- H1/H2 document hierarchy is obliterated into an amorphous blob of words.

When the user asks: *"What is the enterprise renewal discount in Year 3?"*, your vector search retrieves an orphan table row with no header, and the LLM confidently fabricates an answer.

---

### The PaperPilot Solution: Structure-Aware Extraction
PaperPilot (`pdf_to_markdown` / `pdf_chunk`) respects the visual and semantic hierarchy of the document tree:

1. **Table Integrity**: Tables are identified and converted into clean GFM (GitHub Flavored Markdown) pipes (`| Tier | Monthly | SLA |`). Rows remain locked to their column headers.
2. **Semantic Hierarchy**: Headers (`# Header`, `## Section`) stay attached to child content blocks, preserving contextual relevance.
3. **Semantic Boundary Chunking**: Never split across a table boundary or mid-sentence.

---

### The Difference in Vector Retrieval
| Metric | Naive Token Chunker | PaperPilot Structure-Aware |
|---|---|---|
| **Table Layout** | Scrambled strings | Pristine Markdown tables |
| **Context Retention** | Lost across cut-offs | Preserved H1/H2 parent tags |
| **LLM Grounding** | Frequent hallucinations | 100% deterministic lookup |
| **Extraction Latency** | Slow Python OCR wrapper | Sub-15ms native Rust engine |

---

### See it in action:
In the comparison graphic below:
- **Left**: Naive 500-token character chunking splits a multi-tier SaaS pricing matrix into fragmented, unreadable gibberish.
- **Right**: PaperPilot's `pdf_to_markdown` preserves clean markdown syntax, headers, and bulleted SLAs—ready for vector embeddings that actually work.

Garbage in, garbage out. High-performance RAG starts with structure-aware document parsing.

---

⭐ Explore PaperPilot on GitHub: https://github.com/usepaperpilot/paperpilot  
Try it: `paperpilot pdf-to-markdown --input spec.pdf --output spec.md`

#RAG #GenerativeAI #VectorDatabase #AI #MachineLearning #LlamaIndex #LangChain #DataEngineering #Rust
