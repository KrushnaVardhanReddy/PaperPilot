import os
import sys
import time
import subprocess
import json
import urllib.request

FIXTURE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "tests", "e2e_fixtures"))
OUT_DIR = os.path.join(FIXTURE_DIR, "out", "tri_e2e")
REPORT_PATH = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "reports", "TRI_INTERFACE_E2E_BATCH3_OCR_FORMS.md"))

CLI_BIN_REL = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "release", "paperpilot-cli"))
CLI_BIN_DBG = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "debug", "paperpilot-cli"))
MCP_BIN_REL = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "release", "paperpilot-mcp"))
MCP_BIN_DBG = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "debug", "paperpilot-mcp"))

CLI_BIN = CLI_BIN_REL if os.path.exists(CLI_BIN_REL) else CLI_BIN_DBG
MCP_BIN = MCP_BIN_REL if os.path.exists(MCP_BIN_REL) else MCP_BIN_DBG

GATEWAY_URL = "http://127.0.0.1:7823"

class GatewayServer:
    def __init__(self):
        self.process = None

    def start(self):
        print("Starting paperpilot-gateway...")
        self.process = subprocess.Popen(
            [CLI_BIN, "serve", "--port", "7823"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL
        )
        time.sleep(2)

    def stop(self):
        if self.process:
            print("Stopping paperpilot-gateway...")
            self.process.terminate()
            self.process.wait()

class TestRunner:
    def __init__(self):
        self.results = []
        self.tools_run = 0
        self.interfaces = {"CLI": 0, "MCP": 0, "API": 0}
        self.interfaces_passed = {"CLI": 0, "MCP": 0, "API": 0}
        self.latencies = {"CLI": [], "MCP": [], "API": []}

    def record_result(self, tool, interface, passed, latency, command_or_request, output_path=None, error=None):
        self.results.append({
            "tool": tool,
            "interface": interface,
            "passed": passed,
            "latency": latency,
            "command": command_or_request,
            "output_path": output_path,
            "error": error
        })
        self.interfaces[interface] += 1
        if passed:
            self.interfaces_passed[interface] += 1
        self.latencies[interface].append(latency)

    def run_cli(self, tool, args_list, expected_output=None):
        cmd = [CLI_BIN] + args_list + ["--json"]
        start_time = time.time()
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            latency = (time.time() - start_time) * 1000
            passed = True
            error = None
            if expected_output and not os.path.exists(expected_output):
                passed = False
                error = f"Output file not found: {expected_output}"
            elif expected_output and os.path.getsize(expected_output) == 0:
                passed = False
                error = f"Output file is 0 bytes: {expected_output}"
        except subprocess.CalledProcessError as e:
            latency = (time.time() - start_time) * 1000
            passed = False
            error = f"Exit code {e.returncode}: {e.stderr}"

        cmd_str = " ".join(cmd)
        self.record_result(tool, "CLI", passed, latency, cmd_str, expected_output, error)
        return passed

    def run_mcp(self, tool, args_dict, expected_output=None):
        start_time = time.time()
        passed = True
        error = None
        payload = json.dumps({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {"name": tool, "arguments": args_dict}
        }) + "\n"

        try:
            result = subprocess.run(
                [MCP_BIN],
                input=payload,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                timeout=15
            )
            latency = (time.time() - start_time) * 1000

            if result.returncode != 0:
                passed = False
                error = f"Exit code {result.returncode}: {result.stderr}"
            else:
                if expected_output and not os.path.exists(expected_output):
                    passed = False
                    error = f"Output file not found: {expected_output}"
                elif expected_output and os.path.getsize(expected_output) == 0:
                    passed = False
                    error = f"Output file is 0 bytes: {expected_output}"
        except subprocess.TimeoutExpired as e:
            latency = (time.time() - start_time) * 1000
            passed = False
            error = f"MCP command timed out after 15s"
        except Exception as e:
            latency = (time.time() - start_time) * 1000
            passed = False
            error = str(e)

        self.record_result(tool, "MCP", passed, latency, payload.strip(), expected_output, error)
        return passed

    def run_api(self, tool, endpoint, payload, expected_output=None):
        url = f"{GATEWAY_URL}{endpoint}"
        req = urllib.request.Request(url, data=json.dumps(payload).encode('utf-8'), headers={'Content-Type': 'application/json'}, method='POST')
        start_time = time.time()
        passed = True
        error = None
        curl_cmd = f"curl -s -X POST {url} -H 'Content-Type: application/json' -d '{json.dumps(payload)}'"
        try:
            with urllib.request.urlopen(req) as response:
                res_data = response.read()
            latency = (time.time() - start_time) * 1000
            if expected_output and not os.path.exists(expected_output):
                passed = False
                error = f"Output file not found: {expected_output}"
            elif expected_output and os.path.getsize(expected_output) == 0:
                passed = False
                error = f"Output file is 0 bytes: {expected_output}"
        except urllib.error.URLError as e:
            latency = (time.time() - start_time) * 1000
            passed = False
            error = str(e)

        self.record_result(tool, "API", passed, latency, curl_cmd, expected_output, error)
        return passed

os.makedirs(OUT_DIR, exist_ok=True)

def main():
    gateway = GatewayServer()
    gateway.start()

    runner = TestRunner()

    # 21. pdf_render
    print("Running pdf_render...")
    in_file = os.path.join(FIXTURE_DIR, "single_page.pdf")
    out_file_cli = os.path.join(OUT_DIR, "rendered_cli.png")
    out_file_mcp = os.path.join(OUT_DIR, "rendered_mcp.png")
    out_file_api = os.path.join(OUT_DIR, "rendered_api.png")

    runner.run_cli("pdf_render", ["render", "--input", in_file, "--output", out_file_cli], expected_output=out_file_cli)
    runner.run_mcp("pdf_render", {"input": in_file, "page": 1, "output": out_file_mcp}, expected_output=out_file_mcp)
    runner.run_api("pdf_render", "/api/v1/pdf/render-page", {"input": in_file, "page": 1, "output": out_file_api}, expected_output=out_file_api)

    # 22. pdf_ocr
    print("Running pdf_ocr...")
    in_file = os.path.join(FIXTURE_DIR, "image_doc.pdf")
    out_file_cli = os.path.join(OUT_DIR, "ocr_result_cli.pdf")
    out_file_mcp = os.path.join(OUT_DIR, "ocr_result_mcp.pdf")
    out_file_api = os.path.join(OUT_DIR, "ocr_result_api.pdf")

    runner.run_cli("pdf_ocr", ["ocr", "--input", in_file, "--output", out_file_cli], expected_output=out_file_cli)
    runner.run_mcp("pdf_ocr", {"input": in_file, "output": out_file_mcp}, expected_output=out_file_mcp)
    runner.run_api("pdf_ocr", "/api/v1/pdf/tools/pdf_ocr", {"input": in_file, "output": out_file_api}, expected_output=out_file_api)

    # 23. pdf_search
    print("Running pdf_search...")
    in_file = os.path.join(FIXTURE_DIR, "search_test.pdf")

    runner.run_cli("pdf_search", ["search", "--input", in_file, "--query", "test"])
    runner.run_mcp("pdf_search", {"input": in_file, "query": "test"})
    runner.run_api("pdf_search", "/api/v1/pdf/tools/pdf_search", {"input": in_file, "query": "test"})

    # 24. pdf_bates
    print("Running pdf_bates...")
    in_file = os.path.join(FIXTURE_DIR, "multi_page.pdf")
    out_file_cli = os.path.join(OUT_DIR, "bates_stamped_cli.pdf")
    out_file_mcp = os.path.join(OUT_DIR, "bates_stamped_mcp.pdf")
    out_file_api = os.path.join(OUT_DIR, "bates_stamped_api.pdf")

    runner.run_cli("pdf_bates", ["bates", "--input", in_file, "--prefix", "CONF-", "--start", "1", "--output", out_file_cli], expected_output=out_file_cli)
    runner.run_mcp("pdf_bates", {"input": in_file, "prefix": "CONF-", "start_number": 1, "padding": 6, "output": out_file_mcp}, expected_output=out_file_mcp)
    runner.run_api("pdf_bates", "/api/v1/pdf/tools/pdf_bates", {"input": in_file, "prefix": "CONF-", "start_number": 1, "padding": 6, "output": out_file_api}, expected_output=out_file_api)

    # 25. pdf_watermark
    print("Running pdf_watermark...")
    in_file = os.path.join(FIXTURE_DIR, "single_page.pdf")
    out_file_cli = os.path.join(OUT_DIR, "watermarked_cli.pdf")
    out_file_mcp = os.path.join(OUT_DIR, "watermarked_mcp.pdf")
    out_file_api = os.path.join(OUT_DIR, "watermarked_api.pdf")

    runner.run_cli("pdf_watermark", ["watermark", "--input", in_file, "--text", "SAMPLE", "--output", out_file_cli], expected_output=out_file_cli)
    runner.run_mcp("pdf_watermark", {"input": in_file, "text": "SAMPLE", "output": out_file_mcp}, expected_output=out_file_mcp)
    runner.run_api("pdf_watermark", "/api/v1/pdf/watermark", {"input": in_file, "text": "SAMPLE", "output": out_file_api}, expected_output=out_file_api)

    # 26. pdf_header_footer
    print("Running pdf_header_footer...")
    in_file = os.path.join(FIXTURE_DIR, "multi_page.pdf")
    out_file_cli = os.path.join(OUT_DIR, "header_footer_cli.pdf")
    out_file_mcp = os.path.join(OUT_DIR, "header_footer_mcp.pdf")
    out_file_api = os.path.join(OUT_DIR, "header_footer_api.pdf")

    runner.run_cli("pdf_header_footer", ["header-footer", "--input", in_file, "--text", "Confidential - Page", "--output", out_file_cli], expected_output=out_file_cli)
    runner.run_mcp("pdf_header_footer", {"input": in_file, "header_left": "Confidential", "footer_center": "Page", "output": out_file_mcp}, expected_output=out_file_mcp)
    runner.run_api("pdf_header_footer", "/api/v1/pdf/tools/pdf_header_footer", {"input": in_file, "header_left": "Confidential", "footer_center": "Page", "output": out_file_api}, expected_output=out_file_api)

    # 27. pdf_read_form
    print("Running pdf_read_form...")
    in_file = os.path.join(FIXTURE_DIR, "form.pdf")

    runner.run_cli("pdf_read_form", ["form", "read", in_file])
    runner.run_mcp("pdf_read_form", {"input": in_file})
    runner.run_api("pdf_read_form", "/api/v1/pdf/tools/pdf_read_form", {"input": in_file})

    # 28. pdf_fill_form
    print("Running pdf_fill_form...")
    in_file = os.path.join(FIXTURE_DIR, "form.pdf")
    data_file = os.path.join(FIXTURE_DIR, "form_data.json")
    out_file_cli = os.path.join(OUT_DIR, "filled_form_cli.pdf")
    out_file_mcp = os.path.join(OUT_DIR, "filled_form_mcp.pdf")
    out_file_api = os.path.join(OUT_DIR, "filled_form_api.pdf")

    runner.run_cli("pdf_fill_form", ["form", "fill", in_file, "--data", data_file, "--output", out_file_cli], expected_output=out_file_cli)
    runner.run_mcp("pdf_fill_form", {"input": in_file, "values": {"TestText": "Alice"}, "output": out_file_mcp}, expected_output=out_file_mcp)
    runner.run_api("pdf_fill_form", "/api/v1/pdf/tools/pdf_fill_form", {"input": in_file, "values": {"TestText": "Alice"}, "output": out_file_api}, expected_output=out_file_api)

    # 29. pdf_create_form_field
    print("Running pdf_create_form_field...")
    in_file = os.path.join(FIXTURE_DIR, "single_page.pdf")
    out_file_cli = os.path.join(OUT_DIR, "field_added_cli.pdf")
    out_file_mcp = os.path.join(OUT_DIR, "field_added_mcp.pdf")
    out_file_api = os.path.join(OUT_DIR, "field_added_api.pdf")

    runner.run_cli("pdf_create_form_field", ["form", "add-field", in_file, "--name", "signature", "--type", "text", "--rect", "50,50,150,30", "--output", out_file_cli], expected_output=out_file_cli)
    runner.run_mcp("pdf_create_form_field", {"input": in_file, "field_type": "text", "field_name": "signature", "x": 50, "y": 50, "width": 150, "height": 30, "output": out_file_mcp}, expected_output=out_file_mcp)
    runner.run_api("pdf_create_form_field", "/api/v1/pdf/tools/pdf_create_form_field", {"input": in_file, "field_type": "text", "field_name": "signature", "x": 50, "y": 50, "width": 150, "height": 30, "output": out_file_api}, expected_output=out_file_api)

    # 30. pdf_metadata
    print("Running pdf_metadata...")
    in_file = os.path.join(FIXTURE_DIR, "single_page.pdf")
    out_file_cli = os.path.join(OUT_DIR, "metadata_cli.json")

    runner.run_cli("pdf_metadata", ["metadata", "--input", in_file, "--output", out_file_cli], expected_output=out_file_cli)
    runner.run_mcp("pdf_metadata", {"input": in_file})
    runner.run_api("pdf_metadata", "/api/v1/pdf/info", {"input": in_file})

    # GENERATE REPORT
    import json

    total = len(runner.results) // 3
    passed = runner.interfaces_passed

    report_content = [
        "# Phase 4.E2E-R3 — Tri-Interface Verification Report (Batch 3: OCR, Forms & Stamps)",
        "",
        "## Executive Scorecard",
        f"- **Total Tools Verified:** {total}",
        f"- **CLI Pass Rate:** {passed['CLI']} / {total}",
        f"- **MCP Pass Rate:** {passed['MCP']} / {total}",
        f"- **API Pass Rate:** {passed['API']} / {total}",
        ""
    ]

    report_content.append("## Detailed Tool-by-Tool Documentation\n")

    tools_grouped = {}
    for res in runner.results:
        tool_name = res['tool']
        if tool_name not in tools_grouped:
            tools_grouped[tool_name] = {}
        tools_grouped[tool_name][res['interface']] = res

    for tool_name, interfaces in tools_grouped.items():
        report_content.append(f"### Tool: `{tool_name}`\n")

        # CLI
        cli = interfaces['CLI']
        report_content.append("#### 💻 CLI")
        report_content.append(f"```bash\n{cli['command']}\n```")
        report_content.append(f"- **Status:** {'✅ PASS' if cli['passed'] else '❌ FAIL'}")
        report_content.append(f"- **Latency:** {cli['latency']:.2f} ms")
        if cli['error']:
            report_content.append(f"- **Error:** {cli['error']}")
        report_content.append("")

        # MCP
        mcp = interfaces['MCP']
        report_content.append("#### 🤖 MCP")
        report_content.append("```json")
        try:
            mcp_cmd = json.dumps(json.loads(mcp['command']), indent=2)
        except:
            mcp_cmd = mcp['command']
        report_content.append(mcp_cmd)
        report_content.append("```")
        report_content.append(f"- **Status:** {'✅ PASS' if mcp['passed'] else '❌ FAIL'}")
        report_content.append(f"- **Latency:** {mcp['latency']:.2f} ms")
        if mcp['error']:
            report_content.append(f"- **Error:** {mcp['error']}")
        report_content.append("")

        # API
        api = interfaces['API']
        report_content.append("#### 🌐 REST API")
        report_content.append(f"```bash\n{api['command']}\n```")
        report_content.append(f"- **Status:** {'✅ PASS' if api['passed'] else '❌ FAIL'}")
        report_content.append(f"- **Latency:** {api['latency']:.2f} ms")
        if api['error']:
            report_content.append(f"- **Error:** {api['error']}")
        report_content.append("")

        report_content.append("---")
        report_content.append("")

    with open(REPORT_PATH, 'w') as f:
        f.write('\n'.join(report_content))

    print(f"Report generated at {REPORT_PATH}")

    gateway.stop()

if __name__ == "__main__":
    main()