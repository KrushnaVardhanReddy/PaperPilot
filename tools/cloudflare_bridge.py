#!/usr/bin/env python3
"""
Cloudflare Workers AI -> OpenAI Adapter for Open-WebUI
Provides GET /models and proxies POST /chat/completions to Cloudflare.
Normalizes messages (flattens array content), strips unsupported tools,
and supports streaming (SSE).
Runs locally on port 7890.
"""

from http.server import HTTPServer, BaseHTTPRequestHandler
import os
import urllib.request
import json
import sys

CLOUDFLARE_URL = os.environ.get("CLOUDFLARE_AI_URL", "https://api.cloudflare.com/client/v4/accounts/78a079ab61bdbc152820916dcac9be98/ai/v1")
DEFAULT_AUTH = os.environ.get("CLOUDFLARE_AUTH_HEADER", "")

MODELS_LIST = {
    "object": "list",
    "data": [
        {
            "id": "@cf/meta/llama-3.3-70b-instruct-fp8-fast",
            "object": "model",
            "created": 1733788800,
            "owned_by": "cloudflare",
            "name": "Cloudflare Llama 3.3 70B Fast"
        },
        {
            "id": "@cf/meta/llama-3.1-8b-instruct",
            "object": "model",
            "created": 1721779200,
            "owned_by": "cloudflare",
            "name": "Cloudflare Llama 3.1 8B"
        },
        {
            "id": "@cf/deepseek-ai/deepseek-r1-distill-qwen-32b",
            "object": "model",
            "created": 1737331200,
            "owned_by": "cloudflare",
            "name": "Cloudflare DeepSeek R1 32B"
        },
        {
            "id": "@cf/qwen/qwen2.5-72b-instruct",
            "object": "model",
            "created": 1726704000,
            "owned_by": "cloudflare",
            "name": "Cloudflare Qwen 2.5 72B"
        }
    ]
}

def normalize_cloudflare_payload(payload_bytes: bytes) -> tuple[bytes, bool]:
    is_stream = False
    try:
        data = json.loads(payload_bytes.decode("utf-8"))
        is_stream = bool(data.get("stream", False))
        
        # Strip tools and tool_choice so Cloudflare answers directly instead of attempting tool execution
        if "tools" in data:
            del data["tools"]
        if "tool_choice" in data:
            del data["tool_choice"]
            
        if "messages" in data and isinstance(data["messages"], list):
            cleaned_messages = []
            for msg in data["messages"]:
                if not isinstance(msg, dict):
                    continue
                role = msg.get("role", "user")
                content = msg.get("content", "")
                
                # Cloudflare strictly requires content to be a flat string, NOT an array of parts
                if isinstance(content, list):
                    text_parts = []
                    for part in content:
                        if isinstance(part, dict):
                            if part.get("type") == "text":
                                text_parts.append(part.get("text", ""))
                            elif "text" in part:
                                text_parts.append(str(part["text"]))
                        elif isinstance(part, str):
                            text_parts.append(part)
                    content = "\n".join(text_parts)
                elif content is None:
                    content = ""
                else:
                    content = str(content)
                
                cleaned_messages.append({"role": role, "content": content})
            data["messages"] = cleaned_messages
        return json.dumps(data).encode("utf-8"), is_stream
    except Exception as e:
        return payload_bytes, is_stream

class CloudflareProxyHandler(BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        pass

    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "*")
        self.end_headers()

    def do_GET(self):
        if self.path.endswith("/models"):
            body = json.dumps(MODELS_LIST).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        else:
            self.send_response(404)
            self.end_headers()

    def do_POST(self):
        content_len = int(self.headers.get("Content-Length", 0))
        raw_body = self.rfile.read(content_len)
        post_body, is_stream = normalize_cloudflare_payload(raw_body)

        auth_header = self.headers.get("Authorization", DEFAULT_AUTH)
        if not auth_header.startswith("Bearer "):
            auth_header = DEFAULT_AUTH

        target_url = f"{CLOUDFLARE_URL}/chat/completions"
        req = urllib.request.Request(target_url, data=post_body, method="POST")
        req.add_header("Authorization", auth_header)
        req.add_header("Content-Type", "application/json")

        try:
            resp = urllib.request.urlopen(req)
            content_type = resp.headers.get("Content-Type", "application/json")
            self.send_response(resp.status)
            self.send_header("Content-Type", content_type)
            self.send_header("Access-Control-Allow-Origin", "*")
            
            if "text/event-stream" in content_type or is_stream:
                self.send_header("Cache-Control", "no-cache")
                self.send_header("Connection", "keep-alive")
                self.end_headers()
                while True:
                    chunk = resp.read(1024)
                    if not chunk:
                        break
                    self.wfile.write(chunk)
                    self.wfile.flush()
            else:
                resp_data = resp.read()
                self.send_header("Content-Length", str(len(resp_data)))
                self.end_headers()
                self.wfile.write(resp_data)
        except urllib.error.HTTPError as e:
            err_data = e.read()
            self.send_response(e.code)
            self.send_header("Content-Type", "application/json")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Content-Length", str(len(err_data)))
            self.end_headers()
            self.wfile.write(err_data)

def run():
    port = 7890
    server = HTTPServer(("127.0.0.1", port), CloudflareProxyHandler)
    print(f"Cloudflare Workers AI bridge listening on http://127.0.0.1:{port}")
    server.serve_forever()

if __name__ == "__main__":
    run()
