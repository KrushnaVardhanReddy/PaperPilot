import sys
with open("scripts/test_tri_interface_e2e.py", "r") as f:
    content = f.read()

# Fix the MCP error since self.start is missing the print and probably something else
content = content.replace('    def start(self):\n        self.process = subprocess.Popen([MCP_BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)',
                          '    def start(self):\n        self.process = subprocess.Popen([MCP_BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)')

# Remove multiple MCP server stop since they cause connection closed on next run!
content = content.replace("                try:\n                    resp = json.loads(line)\n                    if resp.get(\"id\") == req_id:\n                        self.stop()\n                        return {\"response\": resp, \"latency_ms\": (time.time() - start) * 1000}\n                except json.JSONDecodeError:\n                    pass\n        except Exception as e:\n            self.stop()\n            return {\"error\": str(e), \"latency_ms\": (time.time() - start) * 1000}",
                          "                try:\n                    resp = json.loads(line)\n                    if resp.get(\"id\") == req_id:\n                        return {\"response\": resp, \"latency_ms\": (time.time() - start) * 1000}\n                except json.JSONDecodeError:\n                    pass\n        except Exception as e:\n            return {\"error\": str(e), \"latency_ms\": (time.time() - start) * 1000}")

with open("scripts/test_tri_interface_e2e.py", "w") as f:
    f.write(content)
