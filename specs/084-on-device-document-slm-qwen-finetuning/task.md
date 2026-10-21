# Task Checklist: Spec 084 — On-Device Document SLM (PaperPilot-Qwen-1.5B)

## Phase 1: Synthetic Dataset Generator & Schema Alignment
- [ ] Create directory structure `tools/slm-training/`
- [ ] Implement `tools/slm-training/generate_dataset.py`:
  - [ ] Import and map all 52 tool definitions and parameter schemas from `apps/desktop/src/lib/state/pipeline.svelte.ts` and `paperpilot-mcp`
  - [ ] Implement query variation templates (formal legal, informal chat, typos, ambiguous parameters)
  - [ ] Generate 25,000 synthetic instruction pairs formatted in ChatML / JSONL
  - [ ] Implement 80/20 train/validation split (`dataset/train.jsonl`, `dataset/val.jsonl`)
  - [ ] Validate 100% JSON compliance of generated ground truth answers

## Phase 2: Google Colab Training Notebook (Unsloth + QLoRA)
- [ ] Author `tools/slm-training/train_paperpilot_qwen.ipynb`:
  - [ ] Cell 1: Environment installation (`unsloth`, `trl`, `peft`, `bitsandbytes`, `huggingface_hub`)
  - [ ] Cell 2: Base model loader (`unsloth/Qwen2.5-1.5B-Instruct-bnb-4bit`)
  - [ ] Cell 3: Dataset upload & formatting (loads `train.jsonl`)
  - [ ] Cell 4: LoRA adapter configuration (r=16, lora_alpha=32, target_modules)
  - [ ] Cell 5: Supervised Fine-Tuning Trainer setup with cosine scheduler
  - [ ] Cell 6: Evaluation on `val.jsonl` computing tool match accuracy
  - [ ] Cell 7: Direct GGUF Q4_K_M quantization & export
  - [ ] Cell 8: Hugging Face Hub / Google Drive automated upload

## Phase 3: Desktop & Engine Integration
- [ ] Add model discovery check in `paperpilot-ai` (`~/.paperpilot/models/paperpilot-qwen-1.5b.Q4_K_M.gguf`)
- [ ] Add execution pipeline cascade: Rule NLP $\to$ On-Device SLM $\to$ Cloud BYOK
- [ ] Add Model Manager Card in `apps/desktop/src/lib/components/settings/AiSettingsModal.svelte`:
  - [ ] 1-Click download button fetching quantized GGUF from Hugging Face release
  - [ ] Local model health check and latency gauge
- [ ] Unit & E2E verification:
  - [ ] Verify dataset generator produces valid ChatML JSONL
  - [ ] Verify mock or local GGUF execution in `paperpilot-ai`
