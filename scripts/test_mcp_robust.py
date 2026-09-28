import subprocess
import json
import time
import os

def run_test():
    # Create a dummy PDF
    os.makedirs("test_files", exist_ok=True)
    pdf_path = "test_files/test1.pdf"
    with open(pdf_path, "wb") as f:
        # A minimal PDF 1.4 file
        f.write(b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\nxref\n0 4\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \ntrailer\n<< /Size 4 /Root 1 0 R >>\nstartxref\n188\n%%EOF\n")

    process = subprocess.Popen(
        ["cargo", "run", "-p", "paperpilot-mcp", "--quiet"],
        stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )

    def send(msg):
        print(f"-> {json.dumps(msg)}")
        process.stdin.write((json.dumps(msg) + "\n").encode())
        process.stdin.flush()

    def recv():
        time.sleep(0.5)
        while True:
            line = process.stdout.readline()
            if not line: return None
            try:
                return json.loads(line.decode().strip())
            except:
                print("Raw output:", line)

    send({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}
    })
    print("<-", recv())
    
    send({"jsonrpc": "2.0", "method": "notifications/initialized"})
    
    send({
        "jsonrpc": "2.0", "id": 2, "method": "tools/call",
        "params": {
            "name": "pdf_metadata",
            "arguments": {"input": os.path.abspath(pdf_path)}
        }
    })
    print("<-", recv())

    process.terminate()

run_test()
