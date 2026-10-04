import sys
with open("scripts/test_tri_interface_e2e.py", "r") as f:
    content = f.read()

# We need to print stderr if connection closed
content = content.replace('                if not line:\n                    return {"error": "Connection closed", "latency_ms": (time.time() - start) * 1000}',
                          '                if not line:\n                    err = self.process.stderr.read()\n                    return {"error": f"Connection closed. stderr: {err}", "latency_ms": (time.time() - start) * 1000}')


with open("scripts/test_tri_interface_e2e.py", "w") as f:
    f.write(content)
