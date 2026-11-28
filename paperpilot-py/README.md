# PaperPilot Python SDK

> **Let the LLM be the brain. Let PaperPilot handle the documents.**  
> *Save your tokens for reasoning — parse, chunk, redact, and transform PDFs locally in pure Rust at zero token cost.*

Native Python bindings for PaperPilot's pure-Rust PDF engine. Zero cloud calls, zero C++ runtime dependencies.

## Installation

```bash
pip install paperpilot
```

## Quick Examples

```python
import paperpilot

with open("document.pdf", "rb") as f:
    pdf = f.read()

# Convert PDF to clean Markdown for LLM context
markdown = paperpilot.to_markdown(pdf)

# Chunk into token-bounded RAG segments
chunks = paperpilot.chunk_rag(pdf, max_tokens=512, overlap=50)
for chunk in chunks:
    print(chunk["index"], chunk["token_count"], chunk["headings"])

# Permanently redact sensitive terms
redacted = paperpilot.redact(pdf, terms=["123-45-6789", "CONFIDENTIAL"])
with open("redacted.pdf", "wb") as f:
    f.write(redacted)

# Merge, compress, or watermark PDFs
merged = paperpilot.merge([pdf, redacted])
compressed = paperpilot.compress(merged, level="medium")
watermarked = paperpilot.watermark(compressed, text="INTERNAL", opacity=0.2)
```

## Available Functions

- **Extract & Chunk:** `to_markdown`, `extract_text`, `chunk_rag`, `info`
- **Redact & Secure:** `redact`, `encrypt`
- **Transform:** `merge`, `split`, `compress`, `rotate`, `watermark`, `repair`
- **Tool Registry:** `list_tools`, `run_tool`
