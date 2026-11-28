"""
LLM + PaperPilot: Real-World Document Pre-Processing Pipeline
=============================================================

"Let the LLM be the brain. Let PaperPilot handle the documents."

This standalone script demonstrates four production patterns for LLM apps:
1. Agent Pre-Flight & Cost Control (`info_bytes`, `compress_bytes`) — inspect
   page count & compress oversized PDFs before an agent touches them.
2. Zero-Token RAG Ingestion (`to_markdown_bytes`, `chunk_rag_bytes`) — parse
   & chunk locally in Rust at 0 token cost.
3. Privacy-Safe LLM Intake (`redact`) — strip PII/SSNs/emails locally before
   sending content to external LLMs (OpenAI / Claude / etc.).
4. LLM Client Integration — pass clean, redacted chunks to OpenAI-compatible,
   Ollama, or Anthropic clients.
"""

import paperpilot


def build_sample_pdf() -> bytes:
    """Create a tiny valid in-memory PDF containing sample text and PII."""
    content = (
        "BT /F1 12 Tf 50 740 Td "
        "(PaperPilot Quarterly Report) Tj "
        "0 -24 Td (Contact: Alice Smith, alice@example.com, SSN 123-45-6789.) Tj "
        "0 -24 Td (Revenue grew 42 percent year over year in Q4.) Tj "
        "ET"
    ).encode("latin-1")

    objects = [
        b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n",
        b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n",
        (
            b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] "
            b"/Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n"
        ),
        (
            f"4 0 obj\n<< /Length {len(content)} >>\nstream\n".encode("latin-1")
            + content
            + b"\nendstream\nendobj\n"
        ),
        b"5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n",
    ]

    pdf = bytearray(b"%PDF-1.4\n")
    offsets = []
    for obj in objects:
        offsets.append(len(pdf))
        pdf.extend(obj)

    xref_offset = len(pdf)
    pdf.extend(f"xref\n0 {len(objects) + 1}\n".encode("latin-1"))
    pdf.extend(b"0000000000 65535 f \n")
    for off in offsets:
        pdf.extend(f"{off:010d} 00000 n \n".encode("latin-1"))

    pdf.extend(
        f"trailer\n<< /Size {len(objects) + 1} /Root 1 0 R >>\n"
        f"startxref\n{xref_offset}\n%%EOF\n".encode("latin-1")
    )
    return bytes(pdf)


def main() -> None:
    pdf_bytes = build_sample_pdf()

    # 1. Agent Pre-Flight & Cost Control: inspect metadata & compress before an agent touches it
    info = paperpilot.info_bytes(pdf_bytes)
    compressed_bytes = paperpilot.compress_bytes(pdf_bytes)
    print(f"[Pre-Flight] PDF info: {info}")
    print(f"[Pre-Flight] Size: {len(pdf_bytes)} bytes -> {len(compressed_bytes)} bytes compressed\n")

    # 2. Zero-Token RAG Ingestion: parse & chunk locally in Rust at 0 token cost
    markdown = paperpilot.to_markdown_bytes(compressed_bytes)
    chunks = paperpilot.chunk_rag_bytes(compressed_bytes, max_tokens=256, overlap=32)
    print(f"[Zero-Token RAG] Markdown:\n{markdown.strip()}\n")
    print(f"[Zero-Token RAG] Generated {len(chunks)} semantic chunk(s):")
    for idx, chunk in enumerate(chunks, start=1):
        print(f"  Chunk {idx}: {chunk}")
    print()

    # 3. Privacy-Safe LLM Intake: strip PII locally before sending to OpenAI/Claude
    pii_terms = ["Alice Smith", "alice@example.com", "123-45-6789"]
    redacted_bytes = paperpilot.redact(compressed_bytes, pii_terms, overlay_text="[REDACTED]")
    redacted_text = paperpilot.extract_text_bytes(redacted_bytes)
    print(f"[Privacy-Safe Intake] Redacted text: {redacted_text.strip()!r}\n")

    # 4. Pass clean, redacted chunks to any LLM client
    safe_chunks = paperpilot.chunk_rag_bytes(redacted_bytes, max_tokens=256, overlap=32)
    context = "\n\n---\n\n".join(c["text"] for c in safe_chunks)
    prompt = (
        "Answer the question using only the context below.\n\n"
        f"Context:\n{context}\n\n"
        "Question: What was the Q4 revenue growth?"
    )
    print("[LLM Prompt] Ready-to-send prompt:")
    print(prompt)

    # --- OpenAI / OpenAI-compatible (e.g., vLLM, LiteLLM, OpenRouter) ---
    # from openai import OpenAI
    # client = OpenAI()
    # resp = client.chat.completions.create(
    #     model="gpt-4o-mini",
    #     messages=[{"role": "user", "content": prompt}],
    # )
    # print(resp.choices[0].message.content)

    # --- Ollama (local LLM) ---
    # import ollama
    # resp = ollama.chat(
    #     model="llama3.2",
    #     messages=[{"role": "user", "content": prompt}],
    # )
    # print(resp["message"]["content"])

    # --- Anthropic ---
    # import anthropic
    # client = anthropic.Anthropic()
    # resp = client.messages.create(
    #     model="claude-3-5-sonnet-latest",
    #     max_tokens=512,
    #     messages=[{"role": "user", "content": prompt}],
    # )
    # print(resp.content[0].text)


if __name__ == "__main__":
    main()
