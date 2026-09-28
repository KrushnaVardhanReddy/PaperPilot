import subprocess
import json

def send_request(process, req):
    line = json.dumps(req) + "\n"
    process.stdin.write(line.encode('utf-8'))
    process.stdin.flush()

def read_response(process):
    line = process.stdout.readline()
    if not line: return None
    return json.loads(line.decode('utf-8').strip())

process = subprocess.Popen(
    ["cargo", "run", "-p", "paperpilot-mcp", "--quiet"],
    stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE
)

send_request(process, {
    "jsonrpc": "2.0", "id": 1, "method": "initialize",
    "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}
})
print("Init:", read_response(process))

send_request(process, {
    "jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}
})
print("Tools:", read_response(process))
print("Exit code:", process.poll())
print("Stderr:", process.stderr.read().decode('utf-8'))

process.terminate()
