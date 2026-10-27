"""
PaperPilot Local Inference & Interactive Tester
Runs local inference against the fine-tuned PaperPilot-Qwen model
to verify natural language commands translate to accurate JSON tool calls.
"""

import os
import sys
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from peft import PeftModel

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
BASE_MODEL_ID = "Qwen/Qwen2.5-0.5B-Instruct"
LORA_DIR = os.path.join(BASE_DIR, "output", "paperpilot-qwen-0.5b-lora")

def load_inference_model():
    print(f"📦 Loading base model: {BASE_MODEL_ID}")
    tokenizer = AutoTokenizer.from_pretrained(BASE_MODEL_ID, trust_remote_code=True)
    
    device = "cuda" if torch.cuda.is_available() else "cpu"
    print(f"🚀 Running inference on: {device.upper()}")
    
    model = AutoModelForCausalLM.from_pretrained(
        BASE_MODEL_ID,
        torch_dtype=torch.float16 if device == "cuda" else torch.float32,
        device_map="auto" if device == "cuda" else None,
        trust_remote_code=True,
    )
    
    if os.path.exists(LORA_DIR):
        print(f"🔗 Attaching fine-tuned LoRA adapter from: {LORA_DIR}")
        model = PeftModel.from_pretrained(model, LORA_DIR)
    else:
        print("⚠️ Warning: LoRA adapter not found. Testing with base model.")
        
    model.eval()
    return tokenizer, model, device

def ask(prompt: str, tokenizer, model, device):
    messages = [
        {"role": "system", "content": "You are PaperPilot's on-device neural assistant. Convert user instructions into strict JSON tool calls. For single operations, emit a single JSON object. For multi-step workflows, emit a JSON array of tool calls chained in execution order."},
        {"role": "user", "content": prompt}
    ]
    
    text = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    inputs = tokenizer([text], return_tensors="pt").to(device)
    
    with torch.no_grad():
        generated_ids = model.generate(
            **inputs,
            max_new_tokens=256,
            do_sample=False,
            pad_token_id=tokenizer.eos_token_id,
        )
        
    generated_ids = [
        output_ids[len(input_ids):] for input_ids, output_ids in zip(inputs.input_ids, generated_ids)
    ]
    
    response = tokenizer.batch_decode(generated_ids, skip_special_tokens=True)[0]
    return response.strip()

def main():
    tokenizer, model, device = load_inference_model()
    
    test_queries = [
        "Please combine /data/contract.pdf and /data/appendix.pdf into /data/final_agreement.pdf",
        "Split /data/quarterly_report.pdf from page 1 to 5 as /data/summary.pdf",
        "Rotate all pages of /data/scanned_form.pdf by 90 degrees and save to /data/upright.pdf",
        "Watermark /data/blueprint.pdf with text CONFIDENTIAL angle 45 opacity 0.5 to /data/watermarked.pdf",
        "Compress /data/large_manual.pdf with recommended quality -> /data/small_manual.pdf",
        "Extract plain text from /data/financial_statement.pdf to /data/extracted.txt",
    ]
    
    print("\n" + "=" * 60)
    print("🧪 Running PaperPilot SLM Test Bench")
    print("=" * 60)
    
    for query in test_queries:
        print(f"\n👤 User: {query}")
        result = ask(query, tokenizer, model, device)
        print(f"🤖 PaperPilot Assistant: {result}")
        
    print("\n" + "=" * 60)

if __name__ == "__main__":
    main()
