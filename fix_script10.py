import sys
with open("scripts/test_tri_interface_e2e.py", "r") as f:
    content = f.read()

# MCP tool failing with connection closed because MCP server crashes on start if no initialization? No, we used it before without it.
# Wait! In previous iterations we had a successful MCP Call!
# Let's revert back to how MCPClient was set up in `test_mcp_direct.py`

content = content.replace('    def start(self):\n        if self.process:\n            try:\n                self.process.terminate()\n            except:\n                pass\n        self.process = subprocess.Popen([MCP_BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)',
                          '    def start(self):\n        if self.process:\n            try:\n                self.process.terminate()\n            except:\n                pass\n        self.process = subprocess.Popen([MCP_BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)')

content = content.replace("                try:\n                    resp = json.loads(line)\n                    if resp.get(\"id\") == req_id:\n                        return {\"response\": resp, \"latency_ms\": (time.time() - start) * 1000}\n                except json.JSONDecodeError:\n                    pass\n        except Exception as e:\n            return {\"error\": str(e), \"latency_ms\": (time.time() - start) * 1000}",
                          "                try:\n                    resp = json.loads(line)\n                    if resp.get(\"id\") == req_id:\n                        return {\"response\": resp, \"latency_ms\": (time.time() - start) * 1000}\n                except json.JSONDecodeError:\n                    pass\n        except Exception as e:\n            return {\"error\": str(e), \"latency_ms\": (time.time() - start) * 1000}")


with open("scripts/test_tri_interface_e2e.py", "w") as f:
    f.write(content)
