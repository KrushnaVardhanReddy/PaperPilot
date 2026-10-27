# Spec 132: GGUF Quantization & Fast Local Execution for PaperPilot SLM (0.5B)

## 1. Overview
The fine-tuned on-device Small Language Model (`PaperPilot-Qwen-0.5B-Instruct`) was successfully trained and published in FP16 precision (`model.safetensors`, 988 MB). While mathematically exact, this size is heavier than desired for low-memory laptop environments and mobile devices.

This specification defines the conversion and quantization pipeline to produce standalone, memory-mapped **GGUF** artifacts (`Q4_K_M` and `Q8_0`) using `llama.cpp` tooling, upload them to the official Hugging Face repository (`kpakkiragari/PaperPilot-Qwen-0.5B-Instruct`), and update the desktop Tauri model downloader registry.

---

## 2. Technical Objectives
1. **Model Footprint Reduction**:
   - Original FP16: **988 MB**
   - Quantized `Q4_K_M` (4-bit k-quant): Target **~340–360 MB** (~65% footprint reduction)
   - Quantized `Q8_0` (8-bit): Target **~490–520 MB** (optional high-precision tier)
2. **Runtime Memory & Latency Targets**:
   - Active RAM footprint during inference: **< 400 MB**
   - Warm inference latency: **< 20 ms** per tool planning request
3. **Artifact Publishing & Distribution**:
   - Upload `paperpilot-qwen-0.5b-q4_k_m.gguf` to Hugging Face Hub under `kpakkiragari/PaperPilot-Qwen-0.5B-Instruct`.
   - Update `models/registry.json` and desktop Tauri fallbacks to download the GGUF artifact.
   - Maintain full backwards compatibility with zero changes required for user documents.

---

## 3. Architecture & Conversion Pipeline

```
  [ Local Merged FP16 Model ]
 (tools/slm-training/output/paperpilot-qwen-0.5b-merged/)
             │
             │ convert_hf_to_gguf.py
             ▼
  [ Unquantized GGUF (FP16) ] (~950 MB)
             │
             │ llama-quantize (Q4_K_M)
             ▼
  [ Quantized GGUF Artifact ] (~350 MB)
  (paperpilot-qwen-0.5b-q4_k_m.gguf)
             │
             │ push_to_hf.py
             ▼
[ Hugging Face Hub Release ]
             │
             ▼
[ models/registry.json Update ]
```

---

## 4. Verification Protocol
1. **Conversion Integrity**:
   - Verify GGUF header and tensor count match the Qwen2.5-0.5B architecture (24 transformer layers, hidden size 896, 14 attention heads, 2 key-value heads).
2. **Inference Verification**:
   - Test single-tool parameter generation (e.g. `rotate 90`, `compress level medium`).
   - Test compound multi-step chained parameter generation (e.g. `bates -> redact -> encrypt`).
   - Validate strict refusal behavior on malicious or non-document queries.
3. **Desktop Registry & UI**:
   - Run Vitest desktop unit tests (`pnpm --dir apps/desktop test`).
   - Confirm size badge reflects ~350 MB in `models/registry.json`.
