"""
PaperPilot LoRA Merge & GGUF Exporter
Fuses the fine-tuned LoRA adapter into the base Qwen2.5-0.5B-Instruct weights
and saves the standalone FP16 model ready for native inference and GGUF quantization.
"""

import os
import sys
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from peft import PeftModel

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
BASE_MODEL_ID = "Qwen/Qwen2.5-0.5B-Instruct"
LORA_DIR = os.path.join(BASE_DIR, "output", "paperpilot-qwen-0.5b-lora")
MERGED_OUTPUT_DIR = os.path.join(BASE_DIR, "output", "paperpilot-qwen-0.5b-merged")

def merge_and_save():
    if not os.path.exists(LORA_DIR):
        print(f"❌ Error: LoRA adapter directory not found at {LORA_DIR}")
        sys.exit(1)

    print(f"📦 Loading base model in FP16 for clean weight merging: {BASE_MODEL_ID}")
    tokenizer = AutoTokenizer.from_pretrained(BASE_MODEL_ID, trust_remote_code=True)
    
    base_model = AutoModelForCausalLM.from_pretrained(
        BASE_MODEL_ID,
        torch_dtype=torch.float16,
        device_map="cpu",
        trust_remote_code=True,
    )

    print(f"🔗 Attaching LoRA adapter from {LORA_DIR}...")
    peft_model = PeftModel.from_pretrained(base_model, LORA_DIR)

    print("⚡ Fusing LoRA weights into base model (W = W_0 + B * A)...")
    merged_model = peft_model.merge_and_unload()

    print(f"💾 Saving merged standalone model to {MERGED_OUTPUT_DIR}...")
    os.makedirs(MERGED_OUTPUT_DIR, exist_ok=True)
    merged_model.save_pretrained(MERGED_OUTPUT_DIR, safe_serialization=True)
    tokenizer.save_pretrained(MERGED_OUTPUT_DIR)

    print("🎉 Merged model saved successfully!")
    print(f"   Files are at: {MERGED_OUTPUT_DIR}")

if __name__ == "__main__":
    merge_and_save()
