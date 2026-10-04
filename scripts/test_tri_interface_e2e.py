import os
import sys
import json
import time
import subprocess
import urllib.request
import urllib.error

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FIXTURES_DIR = os.path.join(REPO_ROOT, "tests", "e2e_fixtures")
OUT_DIR = os.path.join(FIXTURES_DIR, "out", "tri_e2e")
REPORT_PATH = os.path.join(REPO_ROOT, "reports", "TRI_INTERFACE_E2E_AND_DOCS.md")
GATEWAY_URL = "http://127.0.0.1:7823"

def get_pdf_info(filepath):
    if not os.path.exists(filepath):
        return {"exists": False, "size": 0, "pages": 0, "valid_pdf": False}
    size = os.path.getsize(filepath)
    pages = 0
    valid_pdf = False
    try:
        with open(filepath, "rb") as f:
            content = f.read()
            if content.startswith(b"%PDF"):
                valid_pdf = True
            # Fast count /Type /Page
            pages = max(1, content.count(b"/Type /Page") - content.count(b"/Type /Pages"))
    except Exception:
        pages = 1
    return {"exists": True, "size": size, "pages": pages, "valid_pdf": valid_pdf}

def run_cli(args):
    start = time.time()
    cli_bin = os.path.join(REPO_ROOT, "target", "debug", "paperpilot-cli")
    cmd = [cli_bin] + args + ["--json"]
    proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    latency = int((time.time() - start) * 1000)
    return {"exit_code": proc.returncode, "stdout": proc.stdout, "stderr": proc.stderr, "latency_ms": latency}

def run_mcp(tool_name, arguments):
    start = time.time()
    mcp_bin = os.path.join(REPO_ROOT, "target", "debug", "paperpilot-mcp")
    payload = json.dumps({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {"name": tool_name, "arguments": arguments}
    })
    proc = subprocess.run([mcp_bin], input=payload, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    latency = int((time.time() - start) * 1000)
    return {"exit_code": proc.returncode, "stdout": proc.stdout, "stderr": proc.stderr, "latency_ms": latency}

def run_api(endpoint, payload):
    start = time.time()
    url = f"{GATEWAY_URL}{endpoint}"
    req = urllib.request.Request(url, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"}, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=10) as resp:
            data = resp.read().decode("utf-8")
            latency = int((time.time() - start) * 1000)
            return {"status": resp.status, "body": data, "latency_ms": latency, "ok": True}
    except urllib.error.HTTPError as e:
        latency = int((time.time() - start) * 1000)
        return {"status": e.code, "body": e.read().decode("utf-8"), "latency_ms": latency, "ok": False}
    except Exception as e:
        latency = int((time.time() - start) * 1000)
        return {"status": 500, "body": str(e), "latency_ms": latency, "ok": False}

def prepare_fixtures():
    os.makedirs(OUT_DIR, exist_ok=True)
    # create page_1.pdf and page_2.pdf from multi_page.pdf
    multi = os.path.join(FIXTURES_DIR, "multi_page.pdf")
    page_1 = os.path.join(FIXTURES_DIR, "page_1.pdf")
    page_2 = os.path.join(FIXTURES_DIR, "page_2.pdf")
    if not os.path.exists(page_1) or not os.path.exists(page_2):
        cli_bin = os.path.join(REPO_ROOT, "target", "debug", "paperpilot-cli")
        subprocess.run([cli_bin, "extract", "--input", multi, "--pages", "1", "--output", page_1])
        subprocess.run([cli_bin, "extract", "--input", multi, "--pages", "2", "--output", page_2])
    return {
        "multi": multi,
        "single": os.path.join(FIXTURES_DIR, "single_page.pdf"),
        "page_1": page_1,
        "page_2": page_2,
    }

TOOLS = [
    {
        "id": "pdf_merge",
        "name": "Merge",
        "synopsis": "Merges multiple PDF files into a single document.",
        "input": lambda f: [f["page_1"], f["page_2"]],
        "output": "merged.pdf",
        "cli": lambda in_f, out_f: ["merge", "--input", in_f[0], in_f[1], "--output", out_f],
        "mcp": lambda in_f, out_f: {"inputs": in_f, "output": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_merge", {"inputs": in_f, "output": out_f})
    },
    {
        "id": "pdf_split",
        "name": "Split",
        "synopsis": "Splits a multi-page PDF document into smaller documents based on specified pages.",
        "input": lambda f: f["multi"],
        "output": "split.pdf",
        "cli": lambda in_f, out_f: ["split", "--input", in_f, "--pages", "1-2", "--output", out_f],
        "mcp": lambda in_f, out_f: {"input": in_f, "pages": "1-2", "output": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_split", {"input": in_f, "pages": "1-2", "output": out_f})
    },
    {
        "id": "pdf_rotate",
        "name": "Rotate",
        "synopsis": "Rotates all pages of a PDF document.",
        "input": lambda f: f["single"],
        "output": "rotated.pdf",
        "cli": lambda in_f, out_f: ["rotate", "--input", in_f, "--angle", "90", "--output", out_f],
        "mcp": lambda in_f, out_f: {"input": in_f, "angle": 90, "output": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_rotate", {"input": in_f, "angle": 90, "output": out_f})
    },
    {
        "id": "pdf_extract_pages",
        "name": "Extract Pages",
        "synopsis": "Extracts specific pages from a PDF document to create a new PDF.",
        "input": lambda f: f["multi"],
        "output": "extracted.pdf",
        "cli": lambda in_f, out_f: ["extract", "--input", in_f, "--pages", "1,3", "--output", out_f],
        "mcp": lambda in_f, out_f: {"input": in_f, "pages": "1,3", "output": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_extract_pages", {"input": in_f, "pages": "1,3", "output": out_f})
    },
    {
        "id": "pdf_delete_pages",
        "name": "Delete Pages",
        "synopsis": "Removes specific pages from a PDF document.",
        "input": lambda f: f["multi"],
        "output": "deleted.pdf",
        "cli": lambda in_f, out_f: ["delete", "--input", in_f, "--pages", "2", "--output", out_f],
        "mcp": lambda in_f, out_f: {"input": in_f, "pages": "2", "output": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_delete_pages", {"input": in_f, "pages": "2", "output": out_f})
    },
    {
        "id": "pdf_reorder_pages",
        "name": "Reorder Pages",
        "synopsis": "Reorders the pages of a PDF document according to a specified sequence.",
        "input": lambda f: f["multi"],
        "output": "reordered.pdf",
        "cli": lambda in_f, out_f: ["reorder", "--input", in_f, "--order", "3,2,1", "--output", out_f],
        "mcp": lambda in_f, out_f: {"input": in_f, "order": "3,2,1", "output": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_reorder_pages", {"input": in_f, "order": "3,2,1", "output": out_f})
    },
    {
        "id": "pdf_burst",
        "name": "Burst",
        "synopsis": "Bursts a PDF document into individual 1-page PDF files.",
        "input": lambda f: f["multi"],
        "output": "burst", # This will be a directory
        "cli": lambda in_f, out_f: ["burst", "--input", in_f, "--output-dir", out_f],
        "mcp": lambda in_f, out_f: {"input": in_f, "output_dir": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_burst", {"input": in_f, "output_dir": out_f})
    },
    {
        "id": "pdf_crop",
        "name": "Crop",
        "synopsis": "Crops all pages of a PDF document to the specified dimensions.",
        "input": lambda f: f["single"],
        "output": "cropped.pdf",
        "cli": lambda in_f, out_f: ["crop", "--input", in_f, "--x", "10", "--y", "10", "--width", "500", "--height", "700", "--output", out_f],
        "mcp": lambda in_f, out_f: {"input": in_f, "x": 10.0, "y": 10.0, "width": 500.0, "height": 700.0, "output": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_crop", {"input": in_f, "x": 10.0, "y": 10.0, "width": 500.0, "height": 700.0, "output": out_f})
    },
    {
        "id": "pdf_remove_blank",
        "name": "Remove Blank Pages",
        "synopsis": "Removes empty or blank pages from a PDF document.",
        "input": lambda f: f["multi"],
        "output": "nonblank.pdf",
        "cli": lambda in_f, out_f: ["remove-blank", "--input", in_f, "--output", out_f],
        "mcp": lambda in_f, out_f: {"input": in_f, "output": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_remove_blank", {"input": in_f, "output": out_f})
    },
    {
        "id": "pdf_page_numbers",
        "name": "Page Numbers",
        "synopsis": "Adds page numbers to a PDF document.",
        "input": lambda f: f["multi"],
        "output": "numbered.pdf",
        "cli": lambda in_f, out_f: ["page-numbers", "--input", in_f, "--position", "bottom-center", "--output", out_f],
        "mcp": lambda in_f, out_f: {"input": in_f, "position": "bottom-center", "output": out_f},
        "api": lambda in_f, out_f: ("/api/v1/pdf/tools/pdf_page_numbers", {"input": in_f, "position": "bottom-center", "output": out_f})
    }
]

def run_all_tests():
    print("Building binaries...")
    subprocess.run(["cargo", "build", "--workspace"], check=True)

    print("Starting gateway...")
    gateway_bin = os.path.join(REPO_ROOT, "target", "debug", "paperpilot-cli")
    gateway_proc = subprocess.Popen([gateway_bin, "serve", "--port", "7823"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    time.sleep(2) # wait for gateway to start

    fixtures = prepare_fixtures()
    results = []

    for tool in TOOLS:
        print(f"Running tests for {tool['name']}...")
        in_f = tool["input"](fixtures)
        out_f = os.path.join(OUT_DIR, tool["output"])

        # Helper to get info for single or multiple input files
        if isinstance(in_f, list):
            before_info = get_pdf_info(in_f[0]) # just use the first for simplistic comparison
        else:
            before_info = get_pdf_info(in_f)

        cli_res = run_cli(tool["cli"](in_f, out_f))

        # Check MCP
        out_f_mcp = os.path.join(OUT_DIR, f"mcp_{tool['output']}")
        mcp_res = run_mcp(tool["id"], tool["mcp"](in_f, out_f_mcp))

        # Check API
        out_f_api = os.path.join(OUT_DIR, f"api_{tool['output']}")
        api_path, api_payload = tool["api"](in_f, out_f_api)
        api_res = run_api(api_path, api_payload)

        # For burst, the output is a directory. Handling size of dir might be complicated, we use 0 or check dir.
        if tool["id"] == "pdf_burst":
            after_info_cli = {"exists": os.path.isdir(out_f), "size": 0, "pages": 0}
            after_info_mcp = {"exists": os.path.isdir(out_f_mcp), "size": 0, "pages": 0}
            after_info_api = {"exists": os.path.isdir(out_f_api), "size": 0, "pages": 0}
        else:
            after_info_cli = get_pdf_info(out_f)
            after_info_mcp = get_pdf_info(out_f_mcp)
            after_info_api = get_pdf_info(out_f_api)

        results.append({
            "tool": tool,
            "before": before_info,
            "cli": {"res": cli_res, "after": after_info_cli, "cmd": tool["cli"](in_f, out_f)},
            "mcp": {"res": mcp_res, "after": after_info_mcp, "payload": {"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": tool["id"], "arguments": tool["mcp"](in_f, out_f_mcp)}}},
            "api": {"res": api_res, "after": after_info_api, "endpoint": api_path, "payload": api_payload}
        })

    gateway_proc.terminate()
    gateway_proc.wait()
    return results

def generate_report(results):
    with open(REPORT_PATH, "w", encoding="utf-8") as f:
        f.write("# Phase 4.E2E-R3 — Batch 1: Structural & Page Operations Tri-Interface Test & Living Documentation\n\n")
        f.write("This document provides verified copy-pasteable working examples for CLI, MCP, and REST API across PaperPilot operations.\n\n")

        f.write("## Master Parity Matrix\n\n")
        f.write("| Tool | Synopsis | CLI Status | MCP Status | API Status |\n")
        f.write("|---|---|---|---|---|\n")

        for r in results:
            t = r["tool"]
            cli_pass = "✅ PASS" if r["cli"]["res"]["exit_code"] == 0 and r["cli"]["after"]["exists"] else f"❌ FAIL ({r['cli']['res']['exit_code']})"
            mcp_pass = "✅ PASS" if r["mcp"]["res"]["exit_code"] == 0 and r["mcp"]["after"]["exists"] else f"❌ FAIL ({r['mcp']['res']['exit_code']})"
            api_pass = "✅ PASS" if r["api"]["res"]["ok"] and r["api"]["after"]["exists"] else f"❌ FAIL ({r['api']['res'].get('status', 'Error')})"
            f.write(f"| `{t['id']}` | {t['synopsis']} | {cli_pass} | {mcp_pass} | {api_pass} |\n")

        f.write("\n## Operations Detail\n\n")

        for r in results:
            t = r["tool"]
            f.write(f"### {t['name']} (`{t['id']}`)\n\n")
            f.write(f"**Synopsis:** {t['synopsis']}\n\n")

            f.write("#### Metrics Table\n\n")
            f.write("| Interface | Before Size | After Size | Size Delta | After Pages | Valid %PDF | Latency (ms) | Result |\n")
            f.write("|---|---|---|---|---|---|---|---|\n")

            b_size = r["before"]["size"]

            c_after = r["cli"]["after"]
            c_delta = c_after["size"] - b_size
            c_pass = "✅ PASS" if r["cli"]["res"]["exit_code"] == 0 and c_after["exists"] else "❌ FAIL"
            c_valid = "Yes" if c_after.get("valid_pdf") else "No"
            f.write(f"| CLI | {b_size} | {c_after['size']} | {c_delta} | {c_after['pages']} | {c_valid} | {r['cli']['res']['latency_ms']} | {c_pass} |\n")

            m_after = r["mcp"]["after"]
            m_delta = m_after["size"] - b_size
            m_pass = "✅ PASS" if r["mcp"]["res"]["exit_code"] == 0 and m_after["exists"] else "❌ FAIL"
            m_valid = "Yes" if m_after.get("valid_pdf") else "No"
            f.write(f"| MCP | {b_size} | {m_after['size']} | {m_delta} | {m_after['pages']} | {m_valid} | {r['mcp']['res']['latency_ms']} | {m_pass} |\n")

            a_after = r["api"]["after"]
            a_delta = a_after["size"] - b_size
            a_pass = "✅ PASS" if r["api"]["res"]["ok"] and a_after["exists"] else "❌ FAIL"
            a_valid = "Yes" if a_after.get("valid_pdf") else "No"
            f.write(f"| API | {b_size} | {a_after['size']} | {a_delta} | {a_after['pages']} | {a_valid} | {r['api']['res']['latency_ms']} | {a_pass} |\n\n")

            if "FAIL" in c_pass:
                f.write(f"**CLI Error:**\n```\n{r['cli']['res']['stderr']}\n```\n\n")
            if "FAIL" in m_pass:
                f.write(f"**MCP Error:**\n```\n{r['mcp']['res']['stderr']}\n```\n\n")
            if "FAIL" in a_pass:
                f.write(f"**API Response:**\n```\n{r['api']['res']['body']}\n```\n\n")

            f.write("#### Examples\n\n")

            f.write("**CLI:**\n")
            cmd_str = " ".join([f'"{x}"' if " " in x else x for x in r["cli"]["cmd"]])
            f.write(f"```bash\npaperpilot-cli {cmd_str}\n```\n\n")

            f.write("**MCP:**\n")
            f.write(f"```json\n{json.dumps(r['mcp']['payload'], indent=2)}\n```\n\n")

            f.write("**API:**\n")
            f.write(f"```bash\ncurl -s -X POST {GATEWAY_URL}{r['api']['endpoint']} \\\n  -H \"Content-Type: application/json\" \\\n  -d '{json.dumps(r['api']['payload'])}'\n```\n\n")

            f.write("---\n\n")
if __name__ == "__main__":
    results = run_all_tests()
    generate_report(results)
    print(f"Report generated at {REPORT_PATH}")
