"""
PaperPilot - Hugging Face Model Card and Publisher
Uploads the merged PaperPilot-Qwen-0.5B model to Hugging Face Hub.
"""

import os
import sys

# Load repo root .env if present
REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
env_file = os.path.join(REPO_ROOT, ".env")
if os.path.exists(env_file):
    with open(env_file, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                key, _, val = line.partition("=")
                key = key.strip()
                val = val.strip().strip('"').strip("'")
                if key and key not in os.environ:
                    os.environ[key] = val

from huggingface_hub import HfApi, create_repo
from transformers import AutoModelForCausalLM, AutoTokenizer

MODEL_DIR = os.path.join(os.path.dirname(__file__), "output", "paperpilot-qwen-0.5b-merged")
REPO_NAME = "PaperPilot-Qwen-0.5B-Instruct"

MODEL_CARD = """---
language:
- en
license: apache-2.0
library_name: transformers
tags:
- pdf
- document-ai
- agent
- tool-calling
- on-device
- qwen2.5
pipeline_tag: text-generation
---

# PaperPilot-Qwen-0.5B-Instruct 📄✈️

**PaperPilot-Qwen-0.5B-Instruct** is an on-device, lightweight specialized small language model (SLM) fine-tuned on top of `Qwen/Qwen2.5-0.5B-Instruct`.

It is engineered specifically for **private, air-gapped document intelligence and tool-calling**, translating natural language user requests into single operations or multi-step chained pipelines across **78+ PaperPilot PDF and document tools**.

## Key Capabilities
- **Exact Schema Tool-Calling**: Emits valid JSON tool calls for single and multi-step PDF workflows.
- **Dynamic Parameter Chaining**: Automatically chains intermediate file paths (`/tmp/inter_1.pdf` -> `/tmp/inter_2.pdf` -> `output.pdf`) for complex multi-tool goals.
- **Ultra-Lightweight & Fast**: Sub-50ms token generation; can run on-device via WebGPU/WASM, Tauri desktop, or llama.cpp.
- **Strict Privacy**: 100% offline, zero cloud telemetry.

## Usage Example

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model_id = "KrushnaVardhanReddy/PaperPilot-Qwen-0.5B-Instruct"
tokenizer = AutoTokenizer.from_pretrained(model_id)
model = AutoModelForCausalLM.from_pretrained(model_id, torch_dtype="auto", device_map="auto")

messages = [
    {
        "role": "system",
        "content": "You are PaperPilot's on-device neural assistant. Convert user instructions into strict JSON tool calls. For single operations, emit a single JSON object. For multi-step workflows, emit a JSON array of tool calls chained in execution order."
    },
    {
        "role": "user",
        "content": "Combine document1.pdf and document2.pdf into merged.pdf and then compress it."
    }
]

text = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
inputs = tokenizer([text], return_tensors="pt").to(model.device)
outputs = model.generate(**inputs, max_new_tokens=256, do_sample=False)
print(tokenizer.decode(outputs[0][len(inputs.input_ids[0]):], skip_special_tokens=True))
```

## License
Apache-2.0 (Open Source)
"""

def push_to_hub(repo_id: str):
    token = os.environ.get("HF_TOKEN")
    if not token:
        print("❌ Error: HF_TOKEN environment variable not found.")
        print("Set your write token: export HF_TOKEN=hf_...")
        sys.exit(1)

    print(f"🚀 Creating Hugging Face repository: {repo_id}")
    api = HfApi(token=token)
    create_repo(repo_id=repo_id, token=token, exist_ok=True, repo_type="model")

    # Write README.md model card into model directory
    readme_path = os.path.join(MODEL_DIR, "README.md")
    with open(readme_path, "w", encoding="utf-8") as f:
        f.write(MODEL_CARD)

    print(f"📦 Uploading model files from {MODEL_DIR} to https://huggingface.co/{repo_id}...")
    api.upload_folder(
        folder_path=MODEL_DIR,
        repo_id=repo_id,
        repo_type="model",
        token=token,
    )

    gguf_path = os.path.join(os.path.dirname(MODEL_DIR), "paperpilot-qwen-0.5b-q4_k_m.gguf")
    if os.path.exists(gguf_path):
        print(f"📦 Uploading quantized GGUF ({os.path.getsize(gguf_path) // (1024*1024)} MB) to https://huggingface.co/{repo_id}...")
        api.upload_file(
            path_or_fileobj=gguf_path,
            path_in_repo="paperpilot-qwen-0.5b-q4_k_m.gguf",
            repo_id=repo_id,
            repo_type="model",
            token=token,
        )

    print(f"🎉 Model published successfully: https://huggingface.co/{repo_id}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        repo_id = sys.argv[1]
    else:
        repo_id = f"KrushnaVardhanReddy/{REPO_NAME}"
    push_to_hub(repo_id)
