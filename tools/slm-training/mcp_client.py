"""Small local stdio MCP client for schema synchronization and real regression tests."""

from collections import deque
import json
from pathlib import Path
from queue import Empty, Queue
import subprocess
from threading import Thread
import time


class McpClient:
    def __init__(self, executable: str | Path, timeout: float = 60):
        self.executable = str(executable)
        self.timeout = timeout
        self.process = None
        self.messages = Queue()
        self.diagnostics = deque(maxlen=20)
        self.request_id = 0

    def __enter__(self):
        # Training tools require Python 3.10+; explicit UTF-8 keeps stdio portable.
        self.process = subprocess.Popen(  # nosemgrep: python.lang.compatibility.python36.python36-compatibility-Popen2
            [self.executable], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, text=True, encoding="utf-8", bufsize=1,
        )
        self.readers = [
            Thread(target=self._read_stdout, daemon=True),
            Thread(target=self._read_stderr, daemon=True),
        ]
        for reader in self.readers:
            reader.start()
        try:
            self.request("initialize", {
                "protocolVersion": "2024-11-05", "capabilities": {},
                "clientInfo": {"name": "paperpilot-slm-validation", "version": "1.0"},
            })
            self._send({"jsonrpc": "2.0", "method": "notifications/initialized"})
        except Exception:
            self.close()
            raise
        return self

    def _read_stdout(self):
        try:
            for line in self.process.stdout:
                try:
                    self.messages.put(json.loads(line))
                except json.JSONDecodeError:
                    self.messages.put(RuntimeError(f"Invalid MCP JSON: {line[:200]}"))
        finally:
            self.messages.put(EOFError("MCP server closed stdout"))

    def _read_stderr(self):
        for line in self.process.stderr:
            self.diagnostics.append(line.rstrip())

    def _send(self, message):
        self.process.stdin.write(json.dumps(message) + "\n")
        self.process.stdin.flush()

    def request(self, method, params=None):
        self.request_id += 1
        request_id = self.request_id
        self._send({"jsonrpc": "2.0", "id": request_id,
                    "method": method, "params": params or {}})
        deadline = time.monotonic() + self.timeout
        while True:
            try:
                message = self.messages.get(timeout=max(0, deadline - time.monotonic()))
            except Empty as exc:
                raise TimeoutError(f"MCP {method} timed out after {self.timeout}s") from exc
            if isinstance(message, Exception):
                raise RuntimeError(f"{message}; stderr: {'; '.join(self.diagnostics)}") from message
            if message.get("id") != request_id:
                continue  # Server notifications are not request responses.
            if "error" in message:
                raise RuntimeError(f"MCP {method}: {message['error']}")
            return message["result"]

    def list_tools(self):
        result = self.request("tools/list")
        tools = result["tools"]
        while result.get("nextCursor"):
            result = self.request("tools/list", {"cursor": result["nextCursor"]})
            tools.extend(result["tools"])
        return {tool["name"]: tool for tool in tools}

    def call_tool(self, name, parameters):
        result = self.request("tools/call", {"name": name, "arguments": parameters})
        if result.get("isError"):
            raise RuntimeError(f"MCP {name}: {result.get('content')}")
        for item in result.get("content", []):
            if item.get("type") == "text":
                payload = json.loads(item["text"])
                if payload.get("success") is not True:
                    raise RuntimeError(f"MCP {name} failed: {payload}")
                return payload
        raise RuntimeError(f"MCP {name} returned no operation result")

    def close(self):
        if self.process is None:
            return
        if self.process.poll() is None:
            self.process.terminate()
            try:
                self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=5)
        for reader in self.readers:
            reader.join(timeout=2)
        for stream in (self.process.stdin, self.process.stdout, self.process.stderr):
            stream.close()

    def __exit__(self, *_):
        self.close()
