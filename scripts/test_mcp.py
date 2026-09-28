import subprocess
import json
import time
import sys

def send_request(process, req):
    line = json.dumps(req) + "\n"
    process.stdin.write(line.encode('utf-8'))
    process.stdin.flush()

def read_response(process):
    line = process.stdout.readline()
    if not line:
        return None
    return json.loads(line.decode('utf-8').strip())

process = subprocess.Popen(
    ["cargo", "run", "-p", "paperpilot-mcp", "--quiet"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE
)

try:
    print("\n--- 1. Initialization ---")
    send_request(process, {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "test-client", "version": "1.0.0"}
        }
    })
    print("Response:", json.dumps(read_response(process), indent=2))
    
    send_request(process, {
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    })

    print("\n--- 2. List Tools ---")
    send_request(process, {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    })
    print("Response:", json.dumps(read_response(process), indent=2))

    print("\n--- 3. Call Tool (hello_world) ---")
    send_request(process, {
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "hello_world",
            "arguments": {
                "input": "PaperPilot"
            }
        }
    })
    print("Response:", json.dumps(read_response(process), indent=2))

finally:
    process.terminate()
