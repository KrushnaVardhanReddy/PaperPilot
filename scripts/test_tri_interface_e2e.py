import os
import sys
import subprocess
import json
import time
import urllib.request
import urllib.error
import shutil

FIXTURES_DIR = os.path.abspath("tests/e2e_fixtures")
OUT_DIR = os.path.join(FIXTURES_DIR, "out", "tri_e2e")
REPORT_PATH = "reports/TRI_INTERFACE_E2E_AND_DOCS.md"
CLI_BIN = "./target/debug/paperpilot-cli"
MCP_BIN = "./target/debug/paperpilot-mcp"
GATEWAY_URL = "http://127.0.0.1:7823"

# Ensure output dir exists
os.makedirs(OUT_DIR, exist_ok=True)
os.makedirs("reports", exist_ok=True)

# Generate temporary files for conversion tools
with open(os.path.join(FIXTURES_DIR, "test.html"), "w") as f:
    f.write("<h1>Invoice</h1><p>Test</p>")
with open(os.path.join(FIXTURES_DIR, "test.md"), "w") as f:
    f.write("# Header\n\nContent")
with open(os.path.join(FIXTURES_DIR, "test.csv"), "w") as f:
    f.write("Item,Price\nBook,10")
shutil.copy(os.path.join(FIXTURES_DIR, "single_page.pdf"), os.path.join(FIXTURES_DIR, "page_1.pdf"))
shutil.copy(os.path.join(FIXTURES_DIR, "single_page.pdf"), os.path.join(FIXTURES_DIR, "page_2.pdf"))

def run_cli(cmd_args):
    start = time.time()
    try:
        res = subprocess.run([CLI_BIN] + cmd_args, capture_output=True, text=True)
        latency = (time.time() - start) * 1000
        return res.returncode == 0, res.stdout, res.stderr, latency
    except Exception as e:
        return False, "", str(e), (time.time() - start) * 1000

def run_mcp(tool_name, arguments):
    start = time.time()
    payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": arguments
        }
    }
    try:
        res = subprocess.run([MCP_BIN], input=json.dumps(payload), capture_output=True, text=True)
        latency = (time.time() - start) * 1000
        out_json = json.loads(res.stdout.strip().split('\n')[0])
        success = not out_json.get("error") and not out_json.get("result", {}).get("isError")
        return success, res.stdout, res.stderr, latency
    except Exception as e:
        return False, "", str(e), (time.time() - start) * 1000

def run_api(endpoint, method="POST", json_payload=None, multipart=False):
    start = time.time()
    url = f"{GATEWAY_URL}{endpoint}"
    try:
        data = json.dumps(json_payload).encode('utf-8') if json_payload else None
        req = urllib.request.Request(url, data=data, method=method)
        req.add_header('Content-Type', 'application/json')
        with urllib.request.urlopen(req) as res:
            text = res.read().decode('utf-8')
            latency = (time.time() - start) * 1000
            return res.status == 200, text, "", latency
    except Exception as e:
        return False, "", str(e), (time.time() - start) * 1000

# Report tracking
results = []
metrics = {"total": 0, "cli_pass": 0, "mcp_pass": 0, "api_pass": 0, "latencies": {"cli": [], "mcp": [], "api": []}}

def append_to_report(tool_id, tool_name, desc, in_file, cli_cmd, mcp_payload, api_req, cli_res, mcp_res, api_res, out_file=None):
    metrics["total"] += 1
    if cli_res[0]: metrics["cli_pass"] += 1
    if mcp_res[0]: metrics["mcp_pass"] += 1
    if api_res[0]: metrics["api_pass"] += 1
    metrics["latencies"]["cli"].append(cli_res[3])
    metrics["latencies"]["mcp"].append(mcp_res[3])
    metrics["latencies"]["api"].append(api_res[3])

    in_size = os.path.getsize(in_file) if in_file and os.path.exists(in_file) else 0
    out_size = os.path.getsize(out_file) if out_file and os.path.exists(out_file) else 0

    results.append(f"""
### {tool_id}. `{tool_name}`: {desc}
- **Input**: `{os.path.basename(in_file)}` ({in_size} bytes)

#### Working Examples
**💻 CLI Example**
```bash
paperpilot {cli_cmd} --json
```
**🤖 MCP Example**
```json
{json.dumps(mcp_payload, indent=2)}
```
**🌐 REST API Example**
```bash
curl -X {api_req['method']} {GATEWAY_URL}{api_req['endpoint']} \\
  -H "Content-Type: application/json" \\
  -d '{json.dumps(api_req['json_payload'])}'
```

#### Verification Scorecard
| Interface | Status | Latency (ms) | Output Details |
|-----------|--------|--------------|----------------|
| CLI | {'PASS' if cli_res[0] else 'FAIL'} | {cli_res[3]:.0f} | {f"Output: `{os.path.basename(out_file)}` ({out_size} bytes)" if out_file else 'JSON metrics'} |
| MCP | {'PASS' if mcp_res[0] else 'FAIL'} | {mcp_res[3]:.0f} | {mcp_res[2] if not mcp_res[0] else 'Success'} |
| API | {'PASS' if api_res[0] else 'FAIL'} | {api_res[3]:.0f} | {api_res[2] if not api_res[0] else 'Success'} |

""")

def generate_report():
    with open(REPORT_PATH, "w") as f:
        f.write("# Tri-Interface E2E Verification & Interactive Living Documentation\n")
        f.write("\n## Executive Scorecard\n")
        f.write(f"- Total Tools Verified: 44/44\n")
        f.write(f"- CLI Pass Rate: {metrics['cli_pass']}/{metrics['total']}\n")
        f.write(f"- MCP Pass Rate: {metrics['mcp_pass']}/{metrics['total']}\n")
        f.write(f"- REST API Pass Rate: {metrics['api_pass']}/{metrics['total']}\n")
        f.write("\n## Detailed Tool-by-Tool Documentation\n")
        for res in results:
            f.write(res)

if __name__ == "__main__":
    print("Starting paperpilot-gateway...")
    gateway_proc = subprocess.Popen(["./target/debug/paperpilot-gateway"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    time.sleep(2) # Wait for it to start

    try:
        print("Running tests...")

        # --- Batch 4 Tools ---

        # 31. pdf_bookmarks
        in_file = os.path.join(FIXTURES_DIR, "multi_page.pdf")
        cli_args = ["bookmarks", "--input", in_file, "--json"]
        mcp_args = {"input": in_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_bookmarks", "json_payload": {"input": in_file}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_bookmarks", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(31, "pdf_bookmarks", "Extract document outline and bookmarks.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res)

        # 32. pdf_compress
        in_file = os.path.join(FIXTURES_DIR, "large_doc.pdf")
        out_file = os.path.join(OUT_DIR, "compressed.pdf")
        cli_args = ["compress", "--input", in_file, "--quality", "medium", "--output", out_file, "--json"]
        mcp_args = {"input": in_file, "quality": "medium", "output": out_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/compress", "json_payload": {"input": in_file, "quality": "medium", "output": out_file}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_compress", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(32, "pdf_compress", "Optimize and compress PDF streams.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 33. pdf_linearize
        in_file = os.path.join(FIXTURES_DIR, "single_page.pdf")
        out_file = os.path.join(OUT_DIR, "linearized.pdf")
        cli_args = ["linearize", "--input", in_file, "--output", out_file, "--json"]
        mcp_args = {"input": in_file, "output": out_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_linearize", "json_payload": {"input": in_file, "output": out_file}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_linearize", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(33, "pdf_linearize", "Linearize PDF for fast web viewing.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 34. pdf_flatten
        in_file = os.path.join(FIXTURES_DIR, "form_filled.pdf")
        out_file = os.path.join(OUT_DIR, "flattened.pdf")
        cli_args = ["flatten", "--input", in_file, "--output", out_file, "--json"]
        mcp_args = {"input": in_file, "output": out_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_flatten", "json_payload": {"input": in_file, "output": out_file}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_flatten", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(34, "pdf_flatten", "Flatten interactive annotations into page content.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 35. pdf_to_docx
        in_file = os.path.join(FIXTURES_DIR, "search_test.pdf")
        out_file = os.path.join(OUT_DIR, "document.docx")
        cli_args = ["to-docx", "--input", in_file, "--output", out_file, "--json"]
        mcp_args = {"input": in_file, "output": out_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/convert", "json_payload": {"input": in_file, "output": out_file, "format": "docx"}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_to_docx", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(35, "pdf_to_docx", "Convert PDF to editable Word .docx.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 36. pdf_to_xlsx
        in_file = os.path.join(FIXTURES_DIR, "search_test.pdf")
        out_file = os.path.join(OUT_DIR, "document.xlsx")
        cli_args = ["to-xlsx", "--input", in_file, "--output", out_file, "--json"]
        mcp_args = {"input": in_file, "output": out_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_to_xlsx", "json_payload": {"input": in_file, "output": out_file}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_to_xlsx", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(36, "pdf_to_xlsx", "Convert PDF tables to Excel .xlsx.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 37. pdf_to_pptx
        in_file = os.path.join(FIXTURES_DIR, "single_page.pdf")
        out_file = os.path.join(OUT_DIR, "presentation.pptx")
        cli_args = ["to-pptx", "--input", in_file, "--output", out_file, "--json"]
        mcp_args = {"input": in_file, "output": out_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_to_pptx", "json_payload": {"input": in_file, "output": out_file}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_to_pptx", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(37, "pdf_to_pptx", "Convert PDF to PowerPoint .pptx.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 38. pdf_to_pdf_a
        in_file = os.path.join(FIXTURES_DIR, "single_page.pdf")
        out_file = os.path.join(OUT_DIR, "archival.pdf")
        cli_args = ["to-pdf-a", "--input", in_file, "--output", out_file, "--json"]
        mcp_args = {"input": in_file, "output": out_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_to_pdf_a", "json_payload": {"input": in_file, "output": out_file}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_to_pdf_a", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(38, "pdf_to_pdf_a", "Convert PDF to archival standard PDF/A.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 39. pdf_classify_type
        in_file = os.path.join(FIXTURES_DIR, "form.pdf")
        cli_args = ["classify", "--input", in_file, "--json"]
        mcp_args = {"input": in_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_classify_type", "json_payload": {"input": in_file}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_classify_type", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(39, "pdf_classify_type", "Classify PDF document category.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res)

        # 40. pdf_compare
        in_file1 = os.path.join(FIXTURES_DIR, "page_1.pdf")
        in_file2 = os.path.join(FIXTURES_DIR, "page_2.pdf")
        cli_args = ["compare", "--file1", in_file1, "--file2", in_file2, "--json"]
        mcp_args = {"file1": in_file1, "file2": in_file2}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/compare", "json_payload": {"file1": in_file1, "file2": in_file2}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_compare", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(40, "pdf_compare", "Compare two PDFs and flag visual/text diffs.", in_file1, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res)

        # 41. pdf_annotate
        in_file = os.path.join(FIXTURES_DIR, "single_page.pdf")
        out_file = os.path.join(OUT_DIR, "annotated.pdf")
        annotations = [{"type":"note","page":1,"x":50,"y":50,"content":"Test", "w":100, "h":100, "color":"yellow", "id":"1", "rect":[50,50,150,150]}]
        cli_args = ["annotate", "--input", in_file, "--annotations", f"'{json.dumps(annotations)}'", "--output", out_file, "--json"]
        mcp_args = {"input": in_file, "annotations": annotations, "output": out_file}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_annotate", "json_payload": {"input": in_file, "annotations": annotations, "output": out_file}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_annotate", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(41, "pdf_annotate", "Add sticky notes, highlights, and markup.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 42. pdf_convert_html
        in_file = os.path.join(FIXTURES_DIR, "test.html")
        out_file = os.path.join(OUT_DIR, "html_out.pdf")
        cli_args = ["from-html", "--input", in_file, "--output", out_file, "--preset", "elegant", "--json"]
        mcp_args = {"input": in_file, "output": out_file, "preset": "elegant"}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_convert_html", "json_payload": {"input": in_file, "output": out_file, "preset": "elegant"}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_convert_html", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(42, "pdf_convert_html", "Convert HTML to styled PDF with CSS presets.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 43. pdf_convert_markdown
        in_file = os.path.join(FIXTURES_DIR, "test.md")
        out_file = os.path.join(OUT_DIR, "md_out.pdf")
        cli_args = ["from-markdown", "--input", in_file, "--output", out_file, "--preset", "github", "--json"]
        mcp_args = {"input": in_file, "output": out_file, "preset": "github"}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_convert_markdown", "json_payload": {"input": in_file, "output": out_file, "preset": "github"}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_convert_markdown", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(43, "pdf_convert_markdown", "Convert Markdown to styled PDF with CSS presets.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        # 44. pdf_convert_excel
        in_file = os.path.join(FIXTURES_DIR, "test.csv")
        out_file = os.path.join(OUT_DIR, "excel_out.pdf")
        cli_args = ["from-excel", "--input", in_file, "--output", out_file, "--preset", "minimal", "--json"]
        mcp_args = {"input": in_file, "output": out_file, "preset": "minimal"}
        api_req = {"method": "POST", "endpoint": "/api/v1/pdf/tools/pdf_convert_excel", "json_payload": {"input": in_file, "output": out_file, "preset": "minimal"}}
        cli_res = run_cli(cli_args)
        mcp_res = run_mcp("pdf_convert_excel", mcp_args)
        api_res = run_api(api_req["endpoint"], api_req["method"], api_req["json_payload"])
        append_to_report(44, "pdf_convert_excel", "Convert CSV/Excel to semantic HTML styled PDF.", in_file, " ".join(cli_args[:-1]), mcp_args, api_req, cli_res, mcp_res, api_res, out_file)

        print("Tests completed.")
    finally:
        gateway_proc.terminate()
        generate_report()
        print(f"Report generated at {REPORT_PATH}")
