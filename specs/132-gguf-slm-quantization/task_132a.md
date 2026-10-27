# Task 132A: Local GGUF Quantization, Hub Upload, and Registry Synchronization

## Subtasks:
- [ ] 1. Install or clone conversion tools (`llama.cpp` Python converter / `gguf`).
- [ ] 2. Convert local fused FP16 weights (`tools/slm-training/output/paperpilot-qwen-0.5b-merged`) to GGUF format (`paperpilot-qwen-0.5b-f16.gguf`).
- [ ] 3. Quantize to `Q4_K_M` (`paperpilot-qwen-0.5b-q4_k_m.gguf`, target size ~350 MB).
- [ ] 4. Upload `paperpilot-qwen-0.5b-q4_k_m.gguf` to Hugging Face Hub (`kpakkiragari/PaperPilot-Qwen-0.5B-Instruct`).
- [ ] 5. Update `models/registry.json` with the new `.gguf` URL and 350 MB size badge.
- [ ] 6. Update `apps/desktop/src-tauri/src/ai_commands.rs` default download URL.
- [ ] 7. Validate desktop test suites (`pnpm --dir apps/desktop test`).
