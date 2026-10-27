"""
================================================================================
PaperPilot Local SLM Fine-Tuning Engine: Interview & Architecture Guide
================================================================================

1. WHAT PROBLEM DOES THIS SOLVE? (The "Why")
   - Privacy & Air-gap: Users want an AI copilot to manipulate sensitive PDFs
     (merge, redact, compress, watermark, extract) without sending data to the cloud.
   - Resource Constraints: Cloud frontier models (like GPT-4) cost money and leak
     data. Standard open weights (7B - 70B) are too heavy for consumer laptops.
   - Solution: Fine-tune a compact 0.5B Small Language Model (Qwen2.5-0.5B-Instruct)
     strictly on tool invocation (ChatML JSON function calling) so it runs
     locally in real-time (<50ms latency, <1 GB memory footprint).

2. HOW DOES IT WORK? (Core ML / LLM Concepts)
   - Base Model: Qwen/Qwen2.5-0.5B-Instruct (state-of-the-art reasoning for sub-1B).
   - ChatML Format: Standard system/user/assistant conversational tokens (<|im_start|>,
     <|im_end|>) mapping natural user intent into exact PaperPilot CLI & MCP tool JSON.
   - QLoRA (Quantized Low-Rank Adaptation):
     * The original model weights (FP16/BF16) are frozen and quantized to 4-bit (NF4 -
       NormalFloat 4) using `bitsandbytes`. This shrinks base VRAM to ~400 MB.
     * Trainable low-rank adapter matrices (Rank r=16, Alpha=32) are injected into
       the attention and MLP projection layers (q, k, v, o, gate, up, down).
     * Only ~1-2% of total parameters are trained, cutting memory while preserving
       the base model's general English grammar and reasoning capabilities.
   - 8-bit Paged Optimizer: Uses `paged_adamw_8bit` to dynamically page optimizer
     states between GPU VRAM and CPU system RAM, preventing CUDA Out-Of-Memory (OOM).

3. END-TO-END PIPELINE:
   Step 1: Synthetic Dataset Generation (generate_dataset.py)
           Creates thousands of varied natural language queries mapped to tool schemas.
   Step 2: Supervised Fine-Tuning (train_local.py) -> [THIS SCRIPT]
           Trains QLoRA adapter weights using Hugging Face TRL (SFTTrainer) on consumer GPU.
   Step 3: Inference & Validation (infer_local.py)
           Loads base + adapter with dynamic temperature sampling to verify exact JSON output.
   Step 4: GGUF / llama.cpp / Candle Export (Optional)
           Fuses LoRA weights into base weights and quantizes to 4-bit GGUF for sub-50ms
           C++/Rust native air-gapped execution inside PaperPilot Desktop.
================================================================================
"""

import os
import sys
import torch
from datasets import load_dataset
from transformers import (
    AutoModelForCausalLM,
    AutoTokenizer,
    BitsAndBytesConfig,
    TrainingArguments,
)
from peft import LoraConfig, get_peft_model, prepare_model_for_kbit_training
from trl import SFTTrainer

# Paths
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
DATASET_DIR = os.path.join(BASE_DIR, "dataset")
TRAIN_FILE = os.path.join(DATASET_DIR, "train.jsonl")
VAL_FILE = os.path.join(DATASET_DIR, "val.jsonl")
OUTPUT_DIR = os.path.join(BASE_DIR, "output", "paperpilot-qwen-0.5b-lora")
MERGED_DIR = os.path.join(BASE_DIR, "output", "paperpilot-qwen-0.5b-merged")

MODEL_ID = "Qwen/Qwen2.5-0.5B-Instruct"

def check_hardware():
    if not torch.cuda.is_available():
        print("❌ Error: CUDA is not available. A local NVIDIA GPU is required.")
        sys.exit(1)
    device_name = torch.cuda.get_device_name(0)
    vram_gb = torch.cuda.get_device_properties(0).total_memory / (1024 ** 3)
    print(f"🚀 Training on GPU: {device_name} ({vram_gb:.2f} GB VRAM)")

def main():
    check_hardware()
    
    if not os.path.exists(TRAIN_FILE):
        print(f"❌ Error: Dataset file not found at {TRAIN_FILE}")
        print("Run: python3 generate_dataset.py --count 25000 --output-dir dataset/")
        sys.exit(1)

    print(f"📦 Loading base model & tokenizer: {MODEL_ID}")
    tokenizer = AutoTokenizer.from_pretrained(MODEL_ID, trust_remote_code=True)
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token

    # 4-bit Quantization Config (QLoRA)
    bnb_config = BitsAndBytesConfig(
        load_in_4bit=True,
        bnb_4bit_quant_type="nf4",
        bnb_4bit_compute_dtype=torch.bfloat16 if torch.cuda.is_bf16_supported() else torch.float16,
        bnb_4bit_use_double_quant=True,
    )

    model = AutoModelForCausalLM.from_pretrained(
        MODEL_ID,
        quantization_config=bnb_config,
        device_map="auto",
        trust_remote_code=True,
    )
    model = prepare_model_for_kbit_training(model)

    # LoRA Config
    peft_config = LoraConfig(
        r=16,
        lora_alpha=32,
        target_modules=["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"],
        lora_dropout=0.05,
        bias="none",
        task_type="CAUSAL_LM",
    )
    model = get_peft_model(model, peft_config)
    model.print_trainable_parameters()

    # Load ChatML Dataset
    print(f"📄 Loading dataset: {TRAIN_FILE}")
    dataset = load_dataset("json", data_files={"train": TRAIN_FILE, "validation": VAL_FILE})

    def format_chatml(example):
        formatted = tokenizer.apply_chat_template(
            example["messages"],
            tokenize=False,
            add_generation_prompt=False,
        )
        return {"text": formatted}

    dataset = dataset.map(format_chatml)

    from trl import SFTConfig, SFTTrainer

    # SFTConfig replaces TrainingArguments in modern trl
    training_args = SFTConfig(
        output_dir=OUTPUT_DIR,
        dataset_text_field="text",
        max_length=1024,
        per_device_train_batch_size=4,
        gradient_accumulation_steps=4,
        warmup_steps=20,
        num_train_epochs=3,
        learning_rate=2e-4,
        fp16=not torch.cuda.is_bf16_supported(),
        bf16=torch.cuda.is_bf16_supported(),
        logging_steps=10,
        eval_strategy="steps",
        eval_steps=50,
        save_strategy="steps",
        save_steps=100,
        save_total_limit=2,
        optim="paged_adamw_8bit",
        report_to="none",
    )

    trainer = SFTTrainer(
        model=model,
        train_dataset=dataset["train"],
        eval_dataset=dataset["validation"],
        args=training_args,
    )

    print("🔥 Starting local fine-tuning on RTX 4070...")
    trainer.train()

    print(f"💾 Saving LoRA adapter to {OUTPUT_DIR}...")
    trainer.model.save_pretrained(OUTPUT_DIR)
    tokenizer.save_pretrained(OUTPUT_DIR)

    print("🎉 Local training complete! Next: Merge & evaluate.")

if __name__ == "__main__":
    main()
