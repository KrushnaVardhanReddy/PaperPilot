import asyncio
from mcp.client.stdio import stdio_client, StdioServerParameters
from mcp.client.session import ClientSession
from mcp.shared.exceptions import MCPError
import os
import sys
import time
from datetime import datetime

FIXTURE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "tests", "e2e_fixtures"))
REPORT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "reports"))
BIN_PATH = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "debug", "paperpilot-mcp"))

EXPECTED_TOOLS = [
    "pdf_merge", "pdf_split", "pdf_rotate", "pdf_extract_pages",
    "pdf_delete_pages", "pdf_reorder", "pdf_burst", "pdf_crop",
    "pdf_encrypt", "pdf_decrypt", "pdf_redact", "pdf_watermark",
    "pdf_header_footer", "pdf_metadata", "pdf_compress", "pdf_linearize",
    "pdf_flatten", "pdf_repair", "pdf_pdf_a", "pdf_sign",
    "pdf_extract_text", "pdf_extract_images", "pdf_images_to_pdf",
    "pdf_bates", "pdf_render", "pdf_ocr", "pdf_compare",
    "pdf_search", "pdf_bookmarks", "pdf_form", "pdf_hash",
    "pdf_validate", "pdf_convert", "pdf_classify"
]

report_data = {
    "total_expected": len(EXPECTED_TOOLS),
    "found_tools": [],
    "missing_tools": [],
    "results": [],
    "schema_issues": [],
    "error_handling": [],
    "failed": False
}

def resolve_fixture(name):
    return os.path.join(FIXTURE_DIR, name)

def check_file(path, expected_min_size=1):
    if not os.path.exists(path):
        return False, f"File not found: {path}"
    if os.path.getsize(path) < expected_min_size:
        return False, f"File too small: {path}"
    return True, ""

async def test_all_tools(session):
    async def run_test(tool_name, args, check_fn):
        start = time.time()
        try:
            res = await session.call_tool(tool_name, arguments=args)
            latency_ms = int((time.time() - start) * 1000)

            passed, err_msg = check_fn(res)
            report_data["results"].append({
                "tool": tool_name,
                "passed": passed,
                "latency_ms": latency_ms,
                "error": err_msg,
                "args": args
            })
            if not passed:
                report_data["failed"] = True
        except MCPError as e:
            latency_ms = int((time.time() - start) * 1000)
            msg = str(e)
            if "Not implemented" in msg or "Unsupported" in msg:
                report_data["results"].append({
                    "tool": tool_name,
                    "passed": "Not Implemented",
                    "latency_ms": latency_ms,
                    "error": msg,
                    "args": args
                })
            else:
                report_data["results"].append({
                    "tool": tool_name,
                    "passed": False,
                    "latency_ms": latency_ms,
                    "error": f"MCPError: {msg}",
                    "args": args
                })
                report_data["failed"] = True
        except Exception as e:
            report_data["results"].append({
                "tool": tool_name,
                "passed": False,
                "latency_ms": 0,
                "error": f"Exception: {str(e)}",
                "args": args
            })
            report_data["failed"] = True

    print("Running Group A - Core Operations...")

    # pdf_merge
    await run_test("pdf_merge",
        {"input_files": [resolve_fixture("multi_page.pdf"), resolve_fixture("single_page.pdf")], "output_file": resolve_fixture("merged.pdf")},
        lambda r: check_file(resolve_fixture("merged.pdf"))
    )

    # pdf_split
    await run_test("pdf_split",
        {"input": resolve_fixture("multi_page.pdf"), "output_dir": FIXTURE_DIR},
        lambda r: check_file(resolve_fixture("multi_page_part_1.pdf")) # approximate check
    )

    # pdf_rotate
    await run_test("pdf_rotate",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("rotated.pdf"), "angle": 90, "pages": [1]},
        lambda r: check_file(resolve_fixture("rotated.pdf"))
    )

    # pdf_extract_pages
    await run_test("pdf_extract_pages",
        {"input": resolve_fixture("multi_page.pdf"), "output": resolve_fixture("extracted.pdf"), "pages": [1, 2, 3]},
        lambda r: check_file(resolve_fixture("extracted.pdf"))
    )

    # pdf_delete_pages
    await run_test("pdf_delete_pages",
        {"input": resolve_fixture("multi_page.pdf"), "output": resolve_fixture("deleted.pdf"), "pages": [2, 4]},
        lambda r: check_file(resolve_fixture("deleted.pdf"))
    )

    # pdf_reorder
    await run_test("pdf_reorder",
        {"input": resolve_fixture("multi_page.pdf"), "output": resolve_fixture("reordered.pdf"), "order": [5, 4, 3, 2, 1]},
        lambda r: check_file(resolve_fixture("reordered.pdf"))
    )

    # pdf_burst
    await run_test("pdf_burst",
        {"input": resolve_fixture("multi_page.pdf"), "output_dir": FIXTURE_DIR},
        lambda r: check_file(resolve_fixture("multi_page_page_1.pdf")) # approximate check
    )

    # pdf_crop
    await run_test("pdf_crop",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("cropped.pdf"), "pages": [1], "x": 10.0, "y": 10.0, "width": 100.0, "height": 100.0},
        lambda r: check_file(resolve_fixture("cropped.pdf"))
    )

    print("Running Group B - Security & Polish...")

    await run_test("pdf_encrypt",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("encrypted2.pdf"), "password": "testpass123"},
        lambda r: check_file(resolve_fixture("encrypted2.pdf"))
    )

    await run_test("pdf_decrypt",
        {"input": resolve_fixture("encrypted.pdf"), "output": resolve_fixture("decrypted.pdf"), "password": "testpass123"},
        lambda r: check_file(resolve_fixture("decrypted.pdf"))
    )

    await run_test("pdf_redact",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("redacted.pdf"), "pages": [1], "x": 10.0, "y": 10.0, "width": 10.0, "height": 10.0},
        lambda r: check_file(resolve_fixture("redacted.pdf"))
    )

    await run_test("pdf_watermark",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("watermarked.pdf"), "text": "DRAFT", "pages": [1]},
        lambda r: check_file(resolve_fixture("watermarked.pdf"))
    )

    await run_test("pdf_header_footer",
        {"input": resolve_fixture("multi_page.pdf"), "output": resolve_fixture("headerfooter.pdf"), "text": "Page {n}", "pages": [1,2]},
        lambda r: check_file(resolve_fixture("headerfooter.pdf"))
    )

    await run_test("pdf_metadata",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("metadata.pdf"), "title": "Test"},
        lambda r: check_file(resolve_fixture("metadata.pdf"))
    )

    await run_test("pdf_compress",
        {"input": resolve_fixture("large_doc.pdf"), "output": resolve_fixture("compressed.pdf")},
        lambda r: check_file(resolve_fixture("compressed.pdf"))
    )

    await run_test("pdf_linearize",
        {"input": resolve_fixture("large_doc.pdf"), "output": resolve_fixture("linearized.pdf")},
        lambda r: check_file(resolve_fixture("linearized.pdf"))
    )

    await run_test("pdf_flatten",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("flattened.pdf")},
        lambda r: check_file(resolve_fixture("flattened.pdf"))
    )

    await run_test("pdf_repair",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("repaired.pdf")},
        lambda r: check_file(resolve_fixture("repaired.pdf"))
    )

    await run_test("pdf_pdf_a",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("pdf_a.pdf")},
        lambda r: check_file(resolve_fixture("pdf_a.pdf"))
    )

    await run_test("pdf_sign",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("signed.pdf"), "cert_path": resolve_fixture("cert.p12"), "password": "pass", "reason": "test"},
        lambda r: check_file(resolve_fixture("signed.pdf"))
    )

    print("Running Group C - Extraction & Advanced...")

    await run_test("pdf_extract_text",
        {"input": resolve_fixture("multi_page.pdf"), "pages": [1]},
        lambda r: (True, "") if len(r.content) > 0 and r.content[0].text else (False, "Empty text")
    )

    await run_test("pdf_extract_images",
        {"input": resolve_fixture("image_doc.pdf"), "output_dir": FIXTURE_DIR},
        lambda r: check_file(resolve_fixture("image_doc_img_1.png")) # Just best effort check since we don't know the generated name
    )

    await run_test("pdf_images_to_pdf",
        {"inputs": [resolve_fixture("img1.png"), resolve_fixture("img2.png")], "output": resolve_fixture("images.pdf")},
        lambda r: check_file(resolve_fixture("images.pdf"))
    )

    await run_test("pdf_bates",
        {"input": resolve_fixture("multi_page.pdf"), "output": resolve_fixture("bates.pdf"), "prefix": "DOC-", "start_number": 1, "padding": 4},
        lambda r: check_file(resolve_fixture("bates.pdf"))
    )

    await run_test("pdf_render",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("rendered.png"), "pages": [1]},
        lambda r: check_file(resolve_fixture("rendered.png"))
    )

    await run_test("pdf_ocr",
        {"input": resolve_fixture("image_doc.pdf"), "output": resolve_fixture("ocr.pdf")},
        lambda r: check_file(resolve_fixture("ocr.pdf"))
    )

    await run_test("pdf_compare",
        {"input_a": resolve_fixture("single_page.pdf"), "input_b": resolve_fixture("single_page.pdf"), "output": resolve_fixture("diff.pdf")},
        lambda r: check_file(resolve_fixture("diff.pdf"))
    )

    await run_test("pdf_search",
        {"input": resolve_fixture("multi_page.pdf"), "query": "Page 1"},
        lambda r: (True, "") if len(r.content) > 0 and r.content[0].text else (False, "No results returned")
    )

    await run_test("pdf_bookmarks",
        {"input": resolve_fixture("multi_page.pdf"), "output": resolve_fixture("bookmarks.json")},
        lambda r: check_file(resolve_fixture("bookmarks.json"))
    )

    await run_test("pdf_form",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("form.json")},
        lambda r: check_file(resolve_fixture("form.json"))
    )

    await run_test("pdf_hash",
        {"input": resolve_fixture("single_page.pdf")},
        lambda r: (True, "") if len(r.content) > 0 and r.content[0].text else (False, "No hash returned")
    )

    await run_test("pdf_validate",
        {"input": resolve_fixture("single_page.pdf")},
        lambda r: (True, "") if len(r.content) > 0 and "true" in r.content[0].text.lower() else (False, "Validation failed")
    )

    await run_test("pdf_convert",
        {"input": resolve_fixture("single_page.pdf"), "output": resolve_fixture("converted.md"), "format": "markdown"},
        lambda r: check_file(resolve_fixture("converted.md"))
    )

    await run_test("pdf_classify",
        {"input": resolve_fixture("single_page.pdf")},
        lambda r: (True, "") if len(r.content) > 0 and r.content[0].text else (False, "Classification failed")
    )

async def test_error_handling(session):
    print("Testing error handling...")
    try:
        await session.call_tool("pdf_rotate", arguments={"input": resolve_fixture("single_page.pdf")}) # Missing required args
        report_data["error_handling"].append("pdf_rotate missing arguments DID NOT FAIL")
        report_data["failed"] = True
    except MCPError as e:
        report_data["error_handling"].append("pdf_rotate missing arguments correctly threw MCPError")

    try:
        await session.call_tool("pdf_decrypt", arguments={"input": resolve_fixture("encrypted.pdf"), "output": resolve_fixture("dec.pdf"), "password": "wrong"})
        report_data["error_handling"].append("pdf_decrypt wrong password DID NOT FAIL")
        report_data["failed"] = True
    except MCPError as e:
        report_data["error_handling"].append("pdf_decrypt wrong password correctly threw MCPError")

    try:
        await session.call_tool("unknown_tool_name", arguments={})
        report_data["error_handling"].append("unknown_tool DID NOT FAIL")
        report_data["failed"] = True
    except MCPError as e:
        report_data["error_handling"].append("unknown_tool correctly threw MCPError")

async def test_tool_discovery_and_schema(session):
    tools_res = await session.list_tools()
    tools = {t.name: t for t in tools_res.tools}

    report_data["found_tools"] = list(tools.keys())
    missing = [t for t in EXPECTED_TOOLS if t not in tools]
    report_data["missing_tools"] = missing
    if missing:
        report_data["failed"] = True

    for name, tool in tools.items():
        if not tool.inputSchema:
            report_data["schema_issues"].append(f"{name}: empty inputSchema")
            continue

        properties = tool.inputSchema.get("properties", {})
        required = tool.inputSchema.get("required", [])

        for prop, details in properties.items():
            if "description" not in details and prop in required:
                report_data["schema_issues"].append(f"{name}: missing description for required '{prop}'")
            if "type" not in details:
                report_data["schema_issues"].append(f"{name}: missing type for '{prop}'")

async def main():
    server_params = StdioServerParameters(
        command=BIN_PATH,
        args=[],
        env=None
    )

    os.makedirs(REPORT_DIR, exist_ok=True)

    async with stdio_client(server_params) as (read_stream, write_stream):
        async with ClientSession(read_stream, write_stream) as session:
            await session.initialize()
            print("Running tool discovery and schema validation...")
            tools_res = await session.list_tools()
            tools = {t.name: t for t in tools_res.tools}

            report_data["found_tools"] = list(tools.keys())
            missing = [t for t in EXPECTED_TOOLS if t not in tools]
            report_data["missing_tools"] = missing
            if missing:
                report_data["failed"] = True

            for name, tool in tools.items():
                if not tool.input_schema:
                    report_data["schema_issues"].append(f"{name}: empty inputSchema")
                    continue

                properties = tool.input_schema.get("properties", {})
                required = tool.input_schema.get("required", [])

                for prop, details in properties.items():
                    if "description" not in details and prop in required:
                        report_data["schema_issues"].append(f"{name}: missing description for required '{prop}'")
                    if "type" not in details:
                        report_data["schema_issues"].append(f"{name}: missing type for '{prop}'")

            print("Done! Tools length:", len(report_data["found_tools"]))
            print("Missing tools:", report_data["missing_tools"])
            print("Schema issues:", report_data["schema_issues"])

            await test_all_tools(session)
            await test_error_handling(session)

            # GENERATE REPORT
            report_path = os.path.join(REPORT_DIR, "MCP_TEST_REPORT.md")
            with open(report_path, "w") as f:
                f.write(f"# PaperPilot MCP Server — E2E Test Report\n")
                f.write(f"Generated: {datetime.now().isoformat()}\n\n")

                f.write("## Summary\n")
                f.write(f"- Total Tools Tested: {report_data['total_expected']}\n")

                passed = [r for r in report_data['results'] if r['passed'] == True]
                failed = [r for r in report_data['results'] if r['passed'] == False]
                not_impl = [r for r in report_data['results'] if r['passed'] == "Not Implemented"]

                f.write(f"- Passed: {len(passed)} ✅\n")
                f.write(f"- Failed: {len(failed)} ❌\n")
                f.write(f"- Not Implemented: {len(not_impl)} ⚠️\n\n")

                f.write("## Tool Discovery\n")
                f.write(f"- Expected tools: {report_data['total_expected']}\n")
                f.write(f"- Found tools: {len(report_data['found_tools'])}\n")
                f.write(f"- Missing: {', '.join(report_data['missing_tools']) if report_data['missing_tools'] else 'None'}\n\n")

                f.write("## Detailed Results\n\n")

                f.write("### ✅ Working Tools\n")
                f.write("| Tool | Latency (ms) | Notes |\n")
                f.write("|---|---|---|\n")
                for r in passed:
                    f.write(f"| {r['tool']} | {r['latency_ms']} | OK |\n")
                f.write("\n")

                f.write("### ❌ Failed Tools\n")
                f.write("| Tool | Latency (ms) | Error | Root Cause Guess |\n")
                f.write("|---|---|---|---|\n")
                for r in failed:
                    f.write(f"| {r['tool']} | {r['latency_ms']} | {r['error']} | |\n")
                f.write("\n")

                f.write("### ⚠️ Not Implemented\n")
                f.write("| Tool | Error |\n")
                f.write("|---|---|\n")
                for r in not_impl:
                    f.write(f"| {r['tool']} | {r['error']} |\n")
                f.write("\n")

                f.write("### Schema Issues\n")
                for issue in report_data["schema_issues"]:
                    f.write(f"- {issue}\n")
                f.write("\n")

                f.write("### Error Handling\n")
                for issue in report_data["error_handling"]:
                    f.write(f"- {issue}\n")
                f.write("\n")

            print(f"Report generated at {report_path}")
            if report_data["failed"]:
                sys.exit(1)
            else:
                sys.exit(0)

if __name__ == "__main__":
    asyncio.run(main())
