"""Smoke tests for paperpilot-py native bindings."""

import pytest

paperpilot = pytest.importorskip("paperpilot")


def _make_minimal_pdf(text: str = "Hello PaperPilot") -> bytes:
    """Build a minimal valid 1-page PDF in memory without external deps."""
    stream = f"BT /F1 12 Tf 72 720 Td ({text}) Tj ET".encode("latin-1")
    objs = [
        b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n",
        b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n",
        (
            b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] "
            b"/Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n"
        ),
        (
            f"4 0 obj\n<< /Length {len(stream)} >>\nstream\n".encode("latin-1")
            + stream
            + b"\nendstream\nendobj\n"
        ),
        b"5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n",
    ]
    out = bytearray(b"%PDF-1.4\n")
    offsets = []
    for obj in objs:
        offsets.append(len(out))
        out.extend(obj)
    xref_pos = len(out)
    out.extend(f"xref\n0 {len(objs) + 1}\n0000000000 65535 f \n".encode("latin-1"))
    for off in offsets:
        out.extend(f"{off:010d} 00000 n \n".encode("latin-1"))
    out.extend(
        f"trailer\n<< /Size {len(objs) + 1} /Root 1 0 R >>\nstartxref\n{xref_pos}\n%%EOF\n".encode(
            "latin-1"
        )
    )
    return bytes(out)


def test_version_and_info():
    pdf = _make_minimal_pdf("Info test")
    meta = paperpilot.info(pdf)
    assert meta["page_count"] == 1
    assert isinstance(meta["version"], str)
    assert hasattr(paperpilot, "__version__")


def test_extract_text_and_to_markdown():
    pdf = _make_minimal_pdf("PaperPilot RAG Engine")
    text = paperpilot.extract_text(pdf)
    assert "PaperPilot" in text
    md = paperpilot.to_markdown(pdf)
    assert "PaperPilot" in md


def test_chunk_rag():
    pdf = _make_minimal_pdf("Chunking test sentence for PaperPilot.")
    chunks = paperpilot.chunk_rag(pdf, max_tokens=128, overlap=16)
    assert isinstance(chunks, list)
    assert len(chunks) >= 1
    assert "text" in chunks[0]
    assert "token_count" in chunks[0]


def test_merge_and_split():
    p1 = _make_minimal_pdf("Page One")
    p2 = _make_minimal_pdf("Page Two")
    merged = paperpilot.merge([p1, p2])
    assert paperpilot.info(merged)["page_count"] == 2

    parts = paperpilot.split(merged, "1")
    assert len(parts) == 2
    assert paperpilot.info(parts[0])["page_count"] == 1
    assert paperpilot.info(parts[1])["page_count"] == 1


def test_compress_redact_encrypt_repair():
    pdf = _make_minimal_pdf("SecretToken123")
    compressed = paperpilot.compress(pdf, level="medium")
    assert compressed.startswith(b"%PDF-")

    redacted = paperpilot.redact(pdf, ["SecretToken123"])
    assert redacted.startswith(b"%PDF-")

    encrypted = paperpilot.encrypt(pdf, user_password="pass")
    assert encrypted.startswith(b"%PDF-")

    repaired = paperpilot.repair(pdf)
    assert repaired.startswith(b"%PDF-")


def test_rotate_and_watermark():
    pdf = _make_minimal_pdf("Rotate and Watermark")
    rotated = paperpilot.rotate(pdf, degrees=90)
    assert rotated.startswith(b"%PDF-")
    assert paperpilot.info(rotated)["page_count"] == 1

    watermarked = paperpilot.watermark(pdf, "CONFIDENTIAL", angle=45.0, opacity=0.25)
    assert watermarked.startswith(b"%PDF-")
    assert paperpilot.info(watermarked)["page_count"] == 1


def test_bytes_aliases():
    p1 = _make_minimal_pdf("Bytes API One")
    p2 = _make_minimal_pdf("Bytes API Two")

    merged = paperpilot.merge_bytes([p1, p2])
    assert paperpilot.info_bytes(merged)["page_count"] == 2

    compressed = paperpilot.compress_bytes(p1, level="medium")
    assert compressed.startswith(b"%PDF-")

    assert "Bytes API One" in paperpilot.extract_text_bytes(p1)
    assert "Bytes API One" in paperpilot.to_markdown_bytes(p1)

    chunks = paperpilot.chunk_rag_bytes(p1, max_tokens=128, overlap=16)
    assert len(chunks) >= 1

    rotated = paperpilot.rotate_bytes(p1, degrees=180)
    assert rotated.startswith(b"%PDF-")

    watermarked = paperpilot.watermark_bytes(p1, "DRAFT")
    assert watermarked.startswith(b"%PDF-")


def test_file_roundtrip_with_tmp_path(tmp_path):
    """Real end-to-end integration test reading/writing actual PDF files on disk."""
    p1_path = tmp_path / "doc1.pdf"
    p2_path = tmp_path / "doc2.pdf"
    p1_path.write_bytes(_make_minimal_pdf("Disk Document One"))
    p2_path.write_bytes(_make_minimal_pdf("Disk Document Two"))

    b1 = p1_path.read_bytes()
    b2 = p2_path.read_bytes()

    merged_path = tmp_path / "merged.pdf"
    merged_path.write_bytes(paperpilot.merge([b1, b2]))
    assert paperpilot.info(merged_path.read_bytes())["page_count"] == 2

    parts = paperpilot.split(merged_path.read_bytes(), "1")
    assert len(parts) == 2

    rotated_path = tmp_path / "rotated.pdf"
    rotated_path.write_bytes(paperpilot.rotate(b1, degrees=90))
    assert paperpilot.info(rotated_path.read_bytes())["page_count"] == 1

    watermarked_path = tmp_path / "watermarked.pdf"
    watermarked_path.write_bytes(paperpilot.watermark(b1, "CONFIDENTIAL"))
    assert watermarked_path.read_bytes().startswith(b"%PDF-")

    assert "Disk Document One" in paperpilot.extract_text(b1)
    assert "Disk Document One" in paperpilot.to_markdown(b1)
    assert len(paperpilot.chunk_rag(b1, max_tokens=128, overlap=16)) >= 1


def test_list_tools_and_run_tool(tmp_path):
    tools = paperpilot.list_tools()
    assert isinstance(tools, list)
    tool_ids = {t["id"] for t in tools}
    assert {"merge", "split", "info", "extract_text", "chunk_rag"}.issubset(tool_ids)

    pdf_path = tmp_path / "run_tool.pdf"
    pdf_path.write_bytes(_make_minimal_pdf("RunTool Disk Test"))

    import json

    info_res = json.loads(
        paperpilot.run_tool("info", json.dumps({"input_path": str(pdf_path)}))
    )
    assert info_res["page_count"] == 1

    text_res = json.loads(
        paperpilot.run_tool("extract_text", json.dumps({"input_path": str(pdf_path)}))
    )
    assert "RunTool Disk Test" in text_res["text"]
