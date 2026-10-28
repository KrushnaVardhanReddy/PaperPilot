"""Real stdio MCP execution against synthesized PDF fixtures; never mock responses."""

import json
import os
from pathlib import Path, PurePosixPath
import random
from zipfile import ZipFile

import pytest
from pypdf import PdfReader, PdfWriter
from pypdf.generic import DecodedStreamObject, DictionaryObject, NameObject

from generate_dataset import generate_compound_workflow_record
from mcp_client import McpClient
from sync_tool_schemas import read_live_schemas
from tool_registry import TOOL_SCHEMAS
from workflow import CONTRACTS, validate_workflow


@pytest.fixture(scope="module")
def client():
    executable = os.environ.get("PAPERPILOT_MCP_BIN")
    if not executable:
        pytest.skip("Set PAPERPILOT_MCP_BIN to run real MCP integration tests (CI requires it)")
    assert Path(executable).is_file(), executable
    with McpClient(Path(executable).resolve()) as connection:
        assert read_live_schemas(connection) == TOOL_SCHEMAS
        yield connection


def write_pdf(path, pages, *, blank_last=False):
    writer = PdfWriter()
    font = DictionaryObject({NameObject("/Type"): NameObject("/Font"),
                             NameObject("/Subtype"): NameObject("/Type1"),
                             NameObject("/BaseFont"): NameObject("/Helvetica")})
    font_id = writer._add_object(font)
    for index in range(pages):
        page = writer.add_blank_page(width=612, height=792)
        if blank_last and index == pages - 1:
            continue
        page[NameObject("/Resources")] = DictionaryObject({NameObject("/Font"): DictionaryObject({NameObject("/F1"): font_id})})
        stream = DecodedStreamObject()
        stream.set_data(f"BT /F1 12 Tf 50 700 Td (PaperPilot contract page {index + 1} analyst@example.org) Tj ET".encode())
        page[NameObject("/Contents")] = writer._add_object(stream)
    with path.open("wb") as handle:
        writer.write(handle)


@pytest.fixture
def sources(tmp_path):
    first, second = tmp_path / "contract.pdf", tmp_path / "appendix.pdf"
    write_pdf(first, 4)
    write_pdf(second, 2)
    return first, second


def bind_plan(names, directory, sources):
    state = random.getstate()
    try:
        random.seed(84)
        record = generate_compound_workflow_record(tools=names)
    finally:
        random.setstate(state)
    steps = json.loads(record["messages"][-1]["content"])
    bindings = {}
    for step in steps:
        contract, params = CONTRACTS[step["tool"]], step["parameters"]
        for key in contract.input_parameters:
            values = params[key] if isinstance(params[key], list) else [params[key]]
            paths = []
            for index, value in enumerate(values):
                if value not in bindings:
                    bindings[value] = str(sources[1 if key == "input_b" or index else 0])
                paths.append(bindings[value])
            params[key] = paths if isinstance(params[key], list) else paths[0]
        value = params[contract.output_parameter]
        bindings[value] = str(directory / PurePosixPath(value).name)
        params[contract.output_parameter] = bindings[value]
    validate_workflow(steps)
    return steps


def execute(client, steps):
    validate_workflow(steps)
    for step in steps:
        params, contract = step["parameters"], CONTRACTS[step["tool"]]
        result = client.call_tool(step["tool"], params)
        output = Path(params[contract.output_parameter])
        assert result["output_path"] == str(output)
        assert output.is_file() and output.stat().st_size > 0
    return Path(steps[-1]["parameters"][CONTRACTS[steps[-1]["tool"]].output_parameter])


def test_merge_compress_preserves_pages_and_text(client, tmp_path, sources):
    steps = bind_plan(["pdf_merge", "pdf_compress"], tmp_path, sources)
    reader = PdfReader(execute(client, steps))
    assert len(reader.pages) == 6
    assert all("PaperPilot contract" in page.extract_text() for page in reader.pages)


def test_watermark_encrypt_preserves_content_and_requires_password(client, tmp_path, sources):
    steps = bind_plan(["pdf_watermark", "pdf_encrypt"], tmp_path, sources)
    output = execute(client, steps)
    reader = PdfReader(output)
    assert reader.is_encrypted
    assert reader.decrypt("incorrect-password") == 0
    assert reader.decrypt(steps[-1]["parameters"]["password"]) != 0
    assert len(reader.pages) == 4
    for page in reader.pages:
        text = page.extract_text()
        assert "PaperPilot contract" in text
        assert "CONFIDENTIAL" in text


def test_analysis_reports_do_not_replace_pdf_input(client, tmp_path, sources):
    steps = bind_plan(["pdf_validate_signatures", "document_pii_scan", "pdf_watermark"], tmp_path, sources)
    output = execute(client, steps)
    assert {step["parameters"]["input"] for step in steps} == {str(sources[0])}
    signatures = json.loads(Path(steps[0]["parameters"]["output"]).read_text())
    assert signatures["total_signatures"] == 0
    pii = json.loads(Path(steps[1]["parameters"]["output"]).read_text())
    assert "analyst@example.org" in json.dumps(pii)
    assert len(PdfReader(output).pages) == 4


def test_pdf_pptx_text_uses_real_office_intermediate(client, tmp_path, sources):
    steps = bind_plan(["pdf_to_pptx", "office_extract_text"], tmp_path, sources)
    output = execute(client, steps)
    with ZipFile(steps[0]["parameters"]["output"]) as presentation:
        assert "ppt/presentation.xml" in presentation.namelist()
    assert "PaperPilot contract" in output.read_text(encoding="utf-8")


def test_extract_pages_then_text_preserves_selected_chapter(client, tmp_path, sources):
    steps = bind_plan(["pdf_extract_pages", "pdf_extract_text"], tmp_path, sources)
    output = execute(client, steps)
    assert len(PdfReader(steps[0]["parameters"]["output"]).pages) == 2
    text = output.read_text(encoding="utf-8")
    assert "page 1" in text and "page 2" in text
    assert "page 3" not in text


def test_remove_blank_then_compress_uses_correct_schema(client, tmp_path, sources):
    write_pdf(sources[0], 4, blank_last=True)
    steps = bind_plan(["pdf_remove_blank", "pdf_compress"], tmp_path, sources)
    reader = PdfReader(execute(client, steps))
    assert len(reader.pages) == 3
    assert all("PaperPilot contract" in page.extract_text() for page in reader.pages)


def test_comparison_report_can_precede_pdf_modification(client, tmp_path, sources):
    steps = bind_plan(["pdf_compare", "pdf_watermark"], tmp_path, sources)
    output = execute(client, steps)
    report = json.loads(Path(steps[0]["parameters"]["output"]).read_text())
    assert report["diff_detected"] is True
    assert len(PdfReader(output).pages) == 4


@pytest.mark.parametrize("password", [None, ""], ids=["password-protected", "empty-user-password"])
def test_encrypt_decrypt_compress_round_trip(client, tmp_path, sources, password):
    steps = bind_plan(["pdf_encrypt", "pdf_decrypt", "pdf_compress"], tmp_path, sources)
    if password is not None:
        steps[0]["parameters"]["password"] = password
        steps[1]["parameters"]["password"] = password
    reader = PdfReader(execute(client, steps))
    assert not reader.is_encrypted
    assert len(reader.pages) == 4
    assert "PaperPilot contract" in reader.pages[0].extract_text()


@pytest.mark.parametrize("source", ["missing", "malformed"])
def test_real_tool_failures_are_not_reported_as_success(client, tmp_path, source):
    path = tmp_path / f"{source}.pdf"
    if source == "malformed":
        path.write_bytes(b"This intentionally malformed file is not a PDF.")
    output = tmp_path / "result.pdf"
    with pytest.raises(RuntimeError):
        client.call_tool("pdf_compress", {"input": str(path), "output": str(output)})
    assert not output.exists()
