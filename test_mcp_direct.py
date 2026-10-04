import sys
import subprocess
import json
import time

MCP_BIN = "target/debug/paperpilot-mcp"
process = subprocess.Popen([MCP_BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

# No init handshake based on memory note: "handle `tools/call` JSON-RPC requests directly from `stdin`"
req = {
    "jsonrpc": "2.0",
    "id": 2,
    "method": "tools/call",
    "params": {
        "name": "pdf_encrypt",
        "arguments": {"input": "tests/e2e_fixtures/single_page.pdf", "user_password": "secret123", "owner_password": "secret123", "output": "tests/e2e_fixtures/out/tri_e2e/encrypted_test.pdf.mcp"}
    }
}
process.stdin.write(json.dumps(req) + "\n")
process.stdin.flush()

print("Sent call. Waiting for response...")
while True:
    line = process.stdout.readline()
    print("Received:", line.strip())
    if not line:
        break
    try:
        resp = json.loads(line)
        if resp.get("id") == 2:
            print("MCP Call succeeded")
            break
    except:
        pass

process.terminate()
