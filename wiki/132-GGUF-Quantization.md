# 132 — PaperPilot SLM GGUF Quantization & On-Device Local Inference

## Overview
`PaperPilot-Qwen-0.5B-Instruct` is an on-device Small Language Model (SLM) specialized for private, air-gapped document intelligence and function-calling across 78+ PaperPilot tools.

While the baseline merged model was exported in FP16 precision (`model.safetensors`, 988 MB), Spec 132 introduces the production GGUF quantization pipeline to reduce the runtime memory footprint by 62% for resource-constrained laptops, desktop environments, and mobile clients.

---

## Quantization Architecture & Pipeline

```
  [ Fused FP16 Model ]
 (tools/slm-training/output/paperpilot-qwen-0.5b-merged/)
           │
           │  convert_hf_to_gguf.py
           ▼
  [ Unquantized GGUF (FP16) ] (~950 MB)
           │
           │  llama-quantize (Q4_K_M)
           ▼
[ Quantized GGUF Artifact ] (380 MB)
(paperpilot-qwen-0.5b-q4_k_m.gguf)
           │
           │  push_to_hf.py
           ▼
[ Hugging Face Hub Distribution ]
(kpakkiragari/PaperPilot-Qwen-0.5B-Instruct)
```

### Quantization Scheme: `Q4_K_M`
* **Attention & FFN Weights**: Quantized to 4-bit medium k-quants (`q4_K` / `q5_0`).
* **Normalization & Embeddings**: Kept in float32/q8_0 to prevent semantic drift.
* **Effective BPW (Bits Per Weight)**: ~6.35 BPW.
* **On-Disk Size**: **380 MB** (down from 988 MB).
* **RAM Footprint**: **< 400 MB** during active inference.

---

## Distribution & Desktop Registry
The model is published on Hugging Face Hub and registered in `models/registry.json`:

```json
{
  "id": "paperpilot-qwen-0.5b",
  "name": "PaperPilot Qwen 0.5B (Official)",
  "description": "Ultra-fast on-device neural assistant fine-tuned for 78+ PaperPilot tools & compound workflows. Zero cloud, sub-50ms.",
  "size_mb": 380,
  "download_url": "https://huggingface.co/kpakkiragari/PaperPilot-Qwen-0.5B-Instruct/resolve/main/paperpilot-qwen-0.5b-q4_k_m.gguf"
}
```

---

## Verification & Testing
1. **Model Weight Verification**:
   - GGUF metadata contains 290 tensors across 24 transformer layers (hidden size 896, 14 attention heads, 2 KV heads).
2. **Desktop Downloader & Settings Integration**:
   - Fallback defaults in `apps/desktop/src-tauri/src/ai_commands.rs` verified via `cargo check -p desktop -j 2`.
   - Desktop test suite verified: `pnpm --dir apps/desktop test` (20/20 test files, 113/113 tests passing).
3. **Security Audit**:
   - Semgrep OSS: 0 findings across 1,336 files.
   - Cargo Audit: 0 vulnerabilities.
