import argparse
import base64
import json
from pathlib import Path
import random
import string
from string import Formatter

from tool_registry import TOOLS_REGISTRY
from templates import SINGLE_TOOL_TEMPLATES, COMPOUND_WORKFLOW_TEMPLATES, TemplateCategory
from workflow import CONTRACTS, artifact_path, output_kind, validate_workflow

SYSTEM_PROMPT = "You are PaperPilot's on-device neural assistant. Convert user instructions into strict JSON tool calls. For single operations, emit a single JSON object. For multi-step workflows, emit a JSON array of tool calls chained in execution order."
REFUSAL = "I cannot assist with that request."


def random_string(length=8):
    return ''.join(random.choices(string.ascii_lowercase, k=length))


def generate_random_param(param_type, param_name):
    # Values obey the actual tool semantics, rather than arbitrary strings in
    # enum-like fields or page ranges unrelated to the request.
    values = {
        "password": "Secret-" + random_string(8), "prefix": "CASE-2026-",
        "pages": "all", "angle": 90, "rotation": 90, "start_number": 1,
        "padding": 4, "font_size": 24, "page": 1, "x": 50, "y": 50,
        "width": 100, "height": 30, "opacity": 0.3, "text": "CONFIDENTIAL",
        "color": "gray", "quality": "medium", "box": "20,20,592,772",
        "order": "2,1", "position": "bottom", "format": "{page}",
        "action": "add", "file_name": "note.txt", "description": "Review note",
        "file_data_base64": base64.b64encode(b"PaperPilot review note").decode(),
    }
    if param_name in values:
        return values[param_name]
    if param_type == "boolean":
        return True
    if param_type in ("number", "integer"):
        return 1
    if param_type == "array":
        return []
    return random_string()


OPTIONAL_PARAMETERS = {
    "pdf_watermark": ("angle", "opacity", "color", "font_size"),
    "pdf_compress": ("quality",),
    "pdf_extract_text": ("pages",),
    "pdf_sanitize": ("remove_metadata", "remove_attachments", "remove_scripts"),
    "document_pii_scan": ("scan_ssn", "scan_credit_cards", "scan_emails", "scan_phones", "scan_iban"),
    "pdf_attachments": ("action", "file_name", "file_data_base64", "description"),
}


def template_fields(template):
    return {name for _, name, _, _ in Formatter().parse(template) if name}


def format_prompt(template, params):
    return template.format(**{key: ", ".join(value) if isinstance(value, list) else str(value)
                              for key, value in params.items()})


def _parameters(name, fields=()):
    definition = TOOLS_REGISTRY[name]
    names = set(definition.required) | set(fields) | set(OPTIONAL_PARAMETERS.get(name, ()))
    contract = CONTRACTS[name]
    paths = set(contract.input_parameters) | {contract.output_parameter}
    params = {key: generate_random_param(prop.type, key)
              for key, prop in definition.properties.items() if key in names and key not in paths}
    if name == "pdf_delete_pages":
        params["pages"] = "1"
    if name == "pdf_extract_pages":
        params["pages"] = "1-2"
    return params


def _settings(steps):
    settings = []
    for index, step in enumerate(steps, 1):
        contract = CONTRACTS[step["tool"]]
        paths = set(contract.input_parameters) | {contract.output_parameter}
        params = {key: value for key, value in step["parameters"].items() if key not in paths}
        if params:
            settings.append(f"step {index} ({step['tool']}): " + ", ".join(
                f"{key}={json.dumps(value)}" for key, value in params.items()))
        if contract.preserves_input and index != len(steps):
            settings.append(f"step {index} report={step['parameters'][contract.output_parameter]}")
        # Additional inputs must be named in the request, not invented by the label.
        for key in contract.input_parameters:
            if key in ("input_b", "inputs"):
                settings.append(f"step {index} {key}={json.dumps(step['parameters'][key])}")
    return " Settings: " + "; ".join(settings) + "." if settings else ""


def _record(prompt, content):
    return {"messages": [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": prompt},
        {"role": "assistant", "content": content},
    ]}


def generate_single_tool_record(tool_name=None, category=None, negative=None):
    if negative is True:
        candidates = [name for name, cats in SINGLE_TOOL_TEMPLATES.items()
                      if TemplateCategory.NEGATIVE_GUARDRAILS in cats]
    else:
        candidates = list(SINGLE_TOOL_TEMPLATES)
    name = tool_name or random.choice(candidates)
    categories = SINGLE_TOOL_TEMPLATES[name]
    if category is None:
        refuse = negative is True or (negative is None and random.random() < 0.15
                                     and TemplateCategory.NEGATIVE_GUARDRAILS in categories)
        choices = ([TemplateCategory.NEGATIVE_GUARDRAILS] if refuse else
                   [key for key in categories if key != TemplateCategory.NEGATIVE_GUARDRAILS])
        category = random.choice(choices)
    template = random.choice(categories[category])
    contract = CONTRACTS[name]
    params = _parameters(name, template_fields(template))
    for key in contract.input_parameters:
        kind = contract.input_kinds[0]
        params[key] = ([artifact_path(f"/data/input_{i}", kind) for i in (1, 2)] if key == "inputs"
                       else artifact_path(f"/data/{key}_{random_string(4)}", kind))
    params[contract.output_parameter] = artifact_path("/data/result_" + random_string(4), output_kind(name, params))
    step = {"tool": name, "parameters": params}
    validate_workflow([step])
    prompt = format_prompt(template, params)
    if category == TemplateCategory.NEGATIVE_GUARDRAILS:
        return _record(prompt, REFUSAL)
    if params[contract.output_parameter] not in prompt:
        prompt += f" Save the result to {params[contract.output_parameter]}."
    return _record(prompt + _settings([step]), json.dumps(step))


def _dynamic_tools():
    length = random.randint(2, 5)
    first = random.choice([
        name for name, contract in CONTRACTS.items() if not contract.terminal
        and (contract.preserves_input or any(contract.output_kind in next_contract.input_kinds
                                             for next_contract in CONTRACTS.values()))
    ])
    tools = [first]
    kind = CONTRACTS[first].input_kinds[0]
    for index in range(length):
        if index:
            choices = [name for name, contract in CONTRACTS.items()
                       if name not in tools and kind in contract.input_kinds
                       and (not contract.terminal or index == length - 1)]
            if not choices:
                break
            tools.append(random.choice(choices))
        contract = CONTRACTS[tools[-1]]
        if not contract.preserves_input:
            kind = contract.output_kind
        # An exported Markdown/HTML artifact has no consumer in this catalog.
        if not any(kind in c.input_kinds for c in CONTRACTS.values()):
            break
    return tools


def generate_compound_workflow_record(workflow=None, tools=None):
    if workflow is None and tools is None and random.random() < 0.5:
        workflow = random.choice(COMPOUND_WORKFLOW_TEMPLATES)
    names = list(tools or (workflow["tools"] if workflow else _dynamic_tools()))
    if not 2 <= len(names) <= 5:
        raise ValueError("Compound workflows must contain 2 to 5 tools")
    token = random_string(6)
    first_contract = CONTRACTS[names[0]]
    current_kind = first_contract.input_kinds[0]
    main_input = artifact_path(f"/data/wf_in_{token}", current_kind)
    current_input = main_input
    extra_input = f"/data/reference_{token}.pdf"
    main_inputs = [main_input, artifact_path(f"/data/second_{token}", current_kind)]
    password = "Secret-" + random_string(8)
    steps = []
    for index, name in enumerate(names):
        contract = CONTRACTS[name]
        if current_kind not in contract.input_kinds:
            raise ValueError(f"{name} cannot consume {current_kind}")
        params = _parameters(name)
        for key in contract.input_parameters:
            if key == "inputs":
                params[key] = main_inputs if index == 0 else [current_input, artifact_path(f"/data/extra_{token}_{index}", current_kind)]
            else:
                params[key] = extra_input if key == "input_b" else current_input
        if "password" in params:
            params["password"] = password
        kind = output_kind(name, params)
        stem = f"/data/wf_out_{token}" if index == len(names) - 1 else f"/tmp/inter_{token}_{index + 1}"
        output = artifact_path(stem, kind)
        params[contract.output_parameter] = output
        steps.append({"tool": name, "parameters": params})
        if not contract.preserves_input:
            current_input, current_kind = output, kind
    validate_workflow(steps)
    main_output = steps[-1]["parameters"][CONTRACTS[names[-1]].output_parameter]
    if workflow:
        template = random.choice(workflow["templates"])
    else:
        template = random.choice([
            "First run " + ", then ".join(names) + " on {input}, saving the final result to {output}.",
            "Before saving to {output}, run " + ", then ".join(names) + " on {input} in that order.",
            "{input} | " + " | ".join(names) + " -> {output}",
        ])
    prompt = format_prompt(template, {"input": main_input, "inputs": main_inputs,
                                     "input_b": extra_input, "output": main_output})
    return _record(prompt + _settings(steps), json.dumps(steps))


def main():
    parser = argparse.ArgumentParser(description="Generate schema-checked PaperPilot ChatML training data.")
    parser.add_argument("--count", type=int, default=25000)
    parser.add_argument("--output-dir", type=Path, default=Path("dataset"))
    parser.add_argument("--val-ratio", type=float, default=0.1)
    parser.add_argument("--sample", type=int, default=0)
    parser.add_argument("--compound-ratio", type=float, default=0.4)
    parser.add_argument("--negative-ratio", type=float, default=0.1)
    parser.add_argument("--seed", type=int)
    args = parser.parse_args()
    if args.count < 1 or not 0 <= args.val_ratio < 1 or args.sample < 0:
        parser.error("count must be positive, sample nonnegative, and val-ratio in [0, 1)")
    if not (0 <= args.compound_ratio <= 1 and 0 <= args.negative_ratio <= 1
            and args.compound_ratio + args.negative_ratio <= 1):
        parser.error("compound/negative ratios must be in [0, 1] and sum to at most 1")
    random.seed(args.seed)
    records = []
    for _ in range(args.count):
        draw = random.random()
        if draw < args.negative_ratio:
            record = generate_single_tool_record(negative=True)
        elif draw < args.negative_ratio + args.compound_ratio:
            record = generate_compound_workflow_record()
        else:
            record = generate_single_tool_record(negative=False)
        records.append(record)
    random.shuffle(records)
    val_size = int(args.count * args.val_ratio)
    args.output_dir.mkdir(parents=True, exist_ok=True)
    for name, split in (("train", records[val_size:]), ("val", records[:val_size])):
        with (args.output_dir / f"{name}.jsonl").open("w", encoding="utf-8") as handle:
            for record in split:
                handle.write(json.dumps(record) + "\n")
    for record in records[:args.sample]:
        print(json.dumps(record, indent=2))
    print(f"Dataset generation complete. Train: {args.count - val_size}, Val: {val_size}.")


if __name__ == "__main__":
    main()
