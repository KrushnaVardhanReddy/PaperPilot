"""Evaluate local fine-tuned SLM routing against single and compound requests.

This checks model-generated plans. Real tool execution is a separate MCP suite.
Model weights are always loaded locally; missing weights never fall back to the
untrained base model and never trigger a download.
"""

import argparse
import json
from pathlib import Path
import random
import time

from generate_dataset import REFUSAL, SYSTEM_PROMPT, generate_single_tool_record, generate_compound_workflow_record
from templates import COMPOUND_WORKFLOW_TEMPLATES
from workflow import validate_workflow

BASE_DIR = Path(__file__).resolve().parent
BASE_MODEL_ID = "Qwen/Qwen2.5-0.5B-Instruct"
LORA_DIR = BASE_DIR / "output" / "paperpilot-qwen-0.5b-lora"
MERGED_DIR = BASE_DIR / "output" / "paperpilot-qwen-0.5b-merged"


def load_inference_model(model_dir=None):
    directory = Path(model_dir) if model_dir else MERGED_DIR
    use_adapter = model_dir is None and not directory.is_dir() and LORA_DIR.is_dir()
    if not directory.is_dir() and not use_adapter:
        raise ValueError("Fine-tuned weights are missing. Supply --model-dir with a local merged model, or install the local LoRA adapter.")
    import torch
    from transformers import AutoModelForCausalLM, AutoTokenizer

    source = BASE_MODEL_ID if use_adapter else str(directory)
    tokenizer = AutoTokenizer.from_pretrained(source, local_files_only=True, trust_remote_code=False)
    device = "cuda" if torch.cuda.is_available() else "cpu"
    model = AutoModelForCausalLM.from_pretrained(
        source, local_files_only=True, trust_remote_code=False,
        torch_dtype=torch.float16 if device == "cuda" else torch.float32,
        device_map="auto" if device == "cuda" else None,
    )
    if use_adapter:
        from peft import PeftModel
        model = PeftModel.from_pretrained(model, str(LORA_DIR), local_files_only=True)
    model.eval()
    return tokenizer, model, device


def ask(prompt, tokenizer, model, device, max_new_tokens=1536):
    import torch
    messages = [{"role": "system", "content": SYSTEM_PROMPT}, {"role": "user", "content": prompt}]
    text = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    inputs = tokenizer([text], return_tensors="pt").to(device)
    with torch.no_grad():
        generated = model.generate(**inputs, max_new_tokens=max_new_tokens, do_sample=False,
                                   pad_token_id=tokenizer.eos_token_id)
    return tokenizer.decode(generated[0][inputs.input_ids.shape[1]:], skip_special_tokens=True).strip()


def validate_response(response, case):
    if case.get("refusal"):
        if response.strip() != REFUSAL:
            raise ValueError("Expected a refusal")
        return
    payload = json.loads(response)
    expected = case["tools"]
    if len(expected) == 1 and not isinstance(payload, dict):
        raise ValueError("Single operations must produce an object")
    if len(expected) > 1 and not isinstance(payload, list):
        raise ValueError("Compound requests must produce an array")
    steps = payload if isinstance(payload, list) else [payload]
    validate_workflow(steps)
    if [step["tool"] for step in steps] != expected:
        raise ValueError("Generated tools/order differ from the requested workflow")


def default_cases():
    state = random.getstate()
    random.seed(84)
    try:
        records = [generate_single_tool_record(name, negative=False) for name in
                   ("pdf_merge", "pdf_split", "pdf_rotate", "pdf_watermark", "pdf_compress", "pdf_extract_text")]
        records.extend(generate_compound_workflow_record(workflow=workflow) for workflow in COMPOUND_WORKFLOW_TEMPLATES)
        records.append(generate_single_tool_record(negative=True))
    finally:
        random.setstate(state)
    cases = []
    for record in records:
        content = record["messages"][-1]["content"]
        case = {"prompt": record["messages"][1]["content"], "refusal": content == REFUSAL}
        if content != REFUSAL:
            steps = json.loads(content)
            steps = steps if isinstance(steps, list) else [steps]
            case["tools"] = [step["tool"] for step in steps]
        cases.append(case)
    return cases


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--model-dir", type=Path)
    parser.add_argument("--cases", type=Path, help="Local JSON list of prompt/tools or prompt/refusal cases")
    parser.add_argument("--report", type=Path, default=Path("slm-inference-report.json"))
    parser.add_argument("--max-new-tokens", type=int, default=1536)
    args = parser.parse_args()
    if args.max_new_tokens < 1:
        parser.error("max-new-tokens must be positive")
    cases = json.loads(args.cases.read_text(encoding="utf-8")) if args.cases else default_cases()
    if not isinstance(cases, list) or not cases:
        parser.error("cases must be a nonempty list")
    try:
        tokenizer, model, device = load_inference_model(args.model_dir)
    except (ImportError, OSError, ValueError) as exc:
        parser.error(str(exc))
    results = []
    for index, case in enumerate(cases, 1):
        started = time.monotonic()
        response = ask(case["prompt"], tokenizer, model, device, args.max_new_tokens)
        result = {"case": index, "prompt": case["prompt"], "response": response,
                  "seconds": round(time.monotonic() - started, 3)}
        try:
            validate_response(response, case)
            result["passed"] = True
        except (ValueError, TypeError, KeyError) as exc:
            result.update(passed=False, error=str(exc))
        results.append(result)
        print(f"{index}/{len(cases)}: {'PASS' if result['passed'] else 'FAIL'}")
    passed = sum(result["passed"] for result in results)
    report = {"device": device, "model": str(args.model_dir or (LORA_DIR if not MERGED_DIR.exists() else MERGED_DIR)),
              "passed": passed, "total": len(results), "scope": "model plan generation; no tool execution",
              "results": results}
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(f"{passed}/{len(results)} cases passed. Report: {args.report}")
    return 0 if passed == len(results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
