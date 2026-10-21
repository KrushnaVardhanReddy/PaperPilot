# Spec 084: On-Device Document SLM (PaperPilot-Qwen-1.5B) — Dataset Generation, Colab Fine-Tuning & Offline GGUF Inference

## 1. Executive Summary & Vision
PaperPilot currently operates with two intent execution engines:
1. **Rule-Based NLP Engine (`paperpilot-nlp`)**: Fast (~1ms) and deterministic, but rigid with informal language, typos, or compound requests.
2. **Cloud LLM API Router (`paperpilot-ai`)**: Flexible, but requires internet, API keys (BYOK), token costs, and exposes document metadata to third-party endpoints.

Spec 084 establishes the **PaperPilot On-Device Small Language Model (SLM)**:
A specialized, fine-tuned 1.5B parameter model (`PaperPilot-Qwen-1.5B`) trained explicitly on PaperPilot's 52 tools, parameter configurations, and multi-step pipeline recipes. The model is quantized to **Q4_K_M GGUF (~1.1 GB)** and executed 100% offline via local llamafile/llama.cpp inference on CPU / Apple Silicon, delivering sub-100ms local token latency with zero external data transmission.

---

## 2. Architecture & Pipeline Overview

The pipeline is split into three clean stages:

```
[ Stage 1: Dataset Generation ] (Local / Python)
  ├── 52 Tool Schemas + Operation Registry
  ├── Synthetic Pair Generator (25,000+ ChatML examples)
  └── Exports `dataset/train.jsonl` and `dataset/val.jsonl`

[ Stage 2: QLoRA Fine-Tuning ] (Google Colab / Free T4 or A100 GPU)
  ├── Base: Qwen/Qwen2.5-1.5B-Instruct (or Llama-3.2-1B/3B)
  ├── Fast training via Unsloth & PEFT (rank=16, alpha=32)
  └── Auto-quantize and export `paperpilot-qwen-1.5b.Q4_K_M.gguf`

[ Stage 3: Local Engine Integration ] (PaperPilot Desktop & CLI)
  ├── Model Loader in `paperpilot-ai` (`~/.paperpilot/models/`)
  ├── Fallback Cascade: Rule NLP ➔ On-Device SLM ➔ Cloud BYOK
  └── 1-Click Model Download in Desktop Settings (`AiSettingsModal.svelte`)
```

---

## 3. Detailed Component Specifications

### 3.1 Stage 1: Synthetic Dataset Generator (`tools/slm-training/generate_dataset.py`)
Generates 25,000 high-diversity training pairs formatted in standard ChatML:
- **Instruction Categories**:
  1. *Single Tool Commands*: Rotate, Bates numbering, compression, password encryption, watermark, format conversion (e.g., PDF to DOCX/Markdown/JSON).
  2. *Complex Multi-Step Workflows*: Compound recipes combining 2 to 5 steps (e.g. Legal Discovery: `Bates ➔ Watermark ➔ Linearize`, Clean Scan: `Remove Blank ➔ Rotate ➔ Compress`).
  3. *Linguistic Noise & Robustness*: Typos, informal phrasing, implicit parameters, legal terminology, and FinTech domain expressions.
  4. *Negative & Guardrail Cases*: Refusal or clarifying questions for invalid operations (e.g. unsupported file types or ambiguous commands).
- **Target Output**: Strict JSON tool calls matching PaperPilot's internal `WorkflowStep` or MCP tool format.

### 3.2 Stage 2: Google Colab Training Notebook (`tools/slm-training/train_paperpilot_qwen.ipynb`)
A self-contained, reproducible notebook designed for free Google Colab (T4 GPU, 15GB VRAM):
1. **Environment Setup**:
   ```python
   !pip install "unsloth[colab-new] @ git+https://github.com/unslothai/unsloth.git"
   !pip install --no-deps trl peft accelerate bitsandbytes
   ```
2. **Model Loading**: Load `unsloth/Qwen2.5-1.5B-Instruct-bnb-4bit` (max seq length = 2048).
3. **Training Parameters**:
   - Optimizer: AdamW 8-bit
   - Learning rate: `2e-4` with cosine scheduler
   - Epochs: 2 to 3
   - Batch size: 4 with gradient accumulation 4 (effective batch 16)
   - Training time: ~30–45 minutes on Colab T4.
4. **Export & Quantization**:
   - Merge LoRA adapters into 16-bit.
   - Quantize directly to `paperpilot-qwen-1.5b-Q4_K_M.gguf`.
   - Upload artifact to Hugging Face Hub or Google Drive.

### 3.3 Stage 3: On-Device Inference & Desktop Integration
1. **Engine Dispatcher (`paperpilot-ai`)**:
   - Prioritizes offline local inference if `paperpilot-qwen-1.5b.Q4_K_M.gguf` is detected in `~/.paperpilot/models/`.
   - Spawns lightweight llamafile sidecar process on port 8081 (or embedded llama.cpp binding).
2. **Desktop UI Integration (`AiSettingsModal.svelte`)**:
   - Displays **"PaperPilot On-Device Neural Assistant"** card.
   - Shows status: `Not Downloaded` | `Downloading (1.1 GB)` | `Ready (Offline)`.
   - Toggle: `Prefer On-Device Model (100% Private, Zero Cloud Calls)`.

---

## 4. Verification & Evaluation Criteria
1. **Accuracy on Test Bench**:
   - Model must achieve $\ge 98\%$ JSON format validity.
   - Model must achieve $\ge 95\%$ tool ID accuracy on an unseen evaluation set of 1,000 diverse test prompts.
2. **Latency & Resource Budget**:
   - Model size: $\le 1.2\text{ GB}$ disk space.
   - Memory usage: $\le 1.8\text{ GB}$ RAM during active inference.
   - Time-to-first-token: $\le 200\text{ ms}$ on Apple Silicon / modern x86 CPU.
