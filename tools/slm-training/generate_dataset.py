import argparse
import json
import random
import os
import string
from tool_registry import TOOLS_REGISTRY
from templates import SINGLE_TOOL_TEMPLATES, COMPOUND_WORKFLOW_TEMPLATES

SYSTEM_PROMPT = "You are PaperPilot's on-device neural assistant. Convert user instructions into strict JSON tool calls. For single operations, emit a single JSON object. For multi-step workflows, emit a JSON array of tool calls chained in execution order."

def random_string(length=8):
    return ''.join(random.choices(string.ascii_lowercase, k=length))

def generate_random_param(param_type: str, param_name: str) -> any:
    if param_type == "string":
        if param_name in ["input", "output", "output_dir"]:
            return f"/data/{random_string()}.pdf"
        elif param_name == "password":
            return random_string(10)
        elif param_name == "prefix":
            return f"BATES-{random.randint(100, 999)}-"
        elif param_name == "format":
            return random.choice(["json", "csv"])
        elif param_name == "pages":
            return random.choice(["all", "1-5", "1,3,5"])
        elif param_name == "text":
            return random.choice(["CONFIDENTIAL", "DRAFT", "DO NOT COPY"])
        elif param_name == "color":
            return random.choice(["#FF0000", "gray", "red", "#000000"])
        elif param_name == "quality":
            return random.choice(["low", "medium", "high", "50", "80"])
        else:
            return random_string()
    elif param_type == "number" or param_type == "integer":
        if param_name == "opacity":
            return round(random.uniform(0.1, 1.0), 2)
        elif param_name == "rotation" or param_name == "angle":
            return random.choice([0, 45, 90, 180, 270])
        elif param_name == "start_number":
            return random.randint(1, 1000)
        elif param_name == "padding":
            return random.randint(3, 8)
        elif param_name == "font_size":
            return random.randint(10, 72)
        elif param_name == "page":
            return random.randint(1, 50)
        elif param_name in ["x", "y", "width", "height"]:
            return round(random.uniform(10.0, 500.0), 1)
        else:
            return random.randint(1, 100)
    elif param_type == "boolean":
        return random.choice([True, False])
    elif param_type == "array":
        # Specific handling for pdf_merge
        if param_name == "inputs":
            return [f"/data/file{i}.pdf" for i in range(random.randint(2, 5))]
        return []
    return None

def format_prompt(template: str, params: dict) -> str:
    # Safely formats the template with string representations of params
    # We might have array which needs comma separation in text
    str_params = {}
    for k, v in params.items():
        if isinstance(v, list):
            str_params[k] = ", ".join(map(str, v))
        else:
            str_params[k] = str(v)
    return template.format(**str_params)

def generate_single_tool_record() -> dict:
    tool_name = random.choice(list(SINGLE_TOOL_TEMPLATES.keys()))
    tool_def = TOOLS_REGISTRY[tool_name]

    cats = list(SINGLE_TOOL_TEMPLATES[tool_name].keys())

    # Decide if this should be a negative guardrail example
    if "negative_guardrails" in cats and random.random() < 0.15:
        category = "negative_guardrails"
    else:
        categories = [c for c in cats if c != "negative_guardrails"]
        if not categories:
            categories = cats
        category = random.choice(categories)

    template = random.choice(SINGLE_TOOL_TEMPLATES[tool_name][category])

    params = {}
    for prop_name, prop_def in tool_def.properties.items():
        params[prop_name] = generate_random_param(prop_def.type, prop_name)

    user_prompt = format_prompt(template, params)

    if category == "negative_guardrails":
        assistant_content = "I cannot assist with that request."
    else:
        assistant_content = json.dumps({"tool": tool_name, "parameters": params})

    return {
        "messages": [
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": user_prompt},
            {"role": "assistant", "content": assistant_content}
        ]
    }



def generate_compound_workflow_record() -> dict:
    is_curated = random.random() < 0.5

    if is_curated:
        workflow = random.choice(COMPOUND_WORKFLOW_TEMPLATES)
        template = random.choice(workflow["templates"])
        tools_to_use = workflow["tools"]
    else:
        num_tools = random.randint(2, 5)
        tools_to_use = random.sample(list(TOOLS_REGISTRY.keys()), num_tools)

        phrasing_type = random.choice(["sequential", "inverted", "comma_list", "pipe"])
        if phrasing_type == "sequential":
            if num_tools == 2:
                template = f"First run {tools_to_use[0]} on {{input}}, and then {tools_to_use[1]} saving to {{output}}."
            elif num_tools == 3:
                template = f"First run {tools_to_use[0]} on {{input}}, then {tools_to_use[1]}, and finally {tools_to_use[2]} outputting to {{output}}."
            elif num_tools == 4:
                template = f"First {tools_to_use[0]} {{input}}, then {tools_to_use[1]}, then {tools_to_use[2]}, and finally {tools_to_use[3]} to {{output}}."
            else:
                template = f"First {tools_to_use[0]} {{input}}, then {tools_to_use[1]}, then {tools_to_use[2]}, then {tools_to_use[3]}, and finally {tools_to_use[4]} saving to {{output}}."
        elif phrasing_type == "inverted":
            if num_tools == 2:
                template = f"Before saving to {{output}} and running {tools_to_use[1]}, please apply {tools_to_use[0]} to {{input}}."
            else:
                template = f"Before saving to {{output}} and running {tools_to_use[-1]}, please apply {', '.join(tools_to_use[:-1])} to {{input}}."
        elif phrasing_type == "comma_list":
            template = f"Please process {{input}} with {', '.join(tools_to_use)}, outputting to {{output}}."
        else: # pipe
            template = f"{{input}} | {' | '.join(tools_to_use)} -> {{output}}"

    # Generate generic input/output for the workflow prompt
    main_input = f"/data/wf_in_{random_string(4)}.pdf"
    main_output = f"/data/wf_out_{random_string(4)}.pdf"
    main_inputs = [f"/data/wf_in1.pdf", f"/data/wf_in2.pdf"]

    if "{inputs}" in template:
        user_prompt = template.format(input=main_input, output=main_output, inputs=", ".join(main_inputs))
    else:
        user_prompt = template.format(input=main_input, output=main_output)

    # Generate steps
    steps = []

    for idx, tool_name in enumerate(tools_to_use):
        tool_def = TOOLS_REGISTRY[tool_name]
        params = {}
        for prop_name, prop_def in tool_def.properties.items():
            if prop_name in ["input", "input_a", "input_b"]:
                if idx == 0 and "{inputs}" not in template:
                    params[prop_name] = main_input
                elif idx == 0 and "{inputs}" in template:
                    # In case a template says {inputs} but the tool asks for "input", just give it main_input
                    params[prop_name] = main_input
                else:
                    # For input_a and input_b, just use the intermediate file for one and random for the other if not first
                    if prop_name == "input_b" and "input_a" in params:
                        params[prop_name] = f"/data/other_file_{random_string(4)}.pdf"
                    else:
                        params[prop_name] = f"/tmp/inter_{idx}.pdf"
            elif prop_name == "inputs":
                if idx == 0:
                    params[prop_name] = main_inputs
                else:
                    params[prop_name] = [f"/tmp/inter_{idx}.pdf"]
            elif prop_name == "output":
                pass # Handled below to ensure it's always added
            elif prop_name == "text":
                params[prop_name] = "CONFIDENTIAL"
            else:
                params[prop_name] = generate_random_param(prop_def.type, prop_name)

        # Force output to be present for every tool in the workflow to avoid chained test failure
        # (some tools like pdf_merge might not have an "output" property required in the same way depending on definition, but realistically SLM outputs need it)
        if "output" in tool_def.properties:
            if idx == len(tools_to_use) - 1:
                params["output"] = main_output
            else:
                params["output"] = f"/tmp/inter_{idx+1}.pdf"
        elif "output_dir" in tool_def.properties:
            if idx == len(tools_to_use) - 1:
                params["output_dir"] = main_output
            else:
                params["output_dir"] = f"/tmp/inter_{idx+1}.pdf"
        else:
            # Force add output anyway since we need to chain it
            if idx == len(tools_to_use) - 1:
                params["output"] = main_output
            else:
                params["output"] = f"/tmp/inter_{idx+1}.pdf"

        steps.append({"tool": tool_name, "parameters": params})

    assistant_content = json.dumps(steps)

    return {
        "messages": [
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": user_prompt},
            {"role": "assistant", "content": assistant_content}
        ]
    }

def main():
    parser = argparse.ArgumentParser(description="Generate synthetic ChatML dataset for PaperPilot SLM.")
    parser.add_argument("--count", type=int, default=25000, help="Number of records to generate.")
    parser.add_argument("--output-dir", type=str, default="dataset/", help="Directory to save the dataset.")
    parser.add_argument("--val-ratio", type=float, default=0.1, help="Ratio of validation data.")
    parser.add_argument("--sample", type=int, default=0, help="Print N sample items to stdout.")
    parser.add_argument("--compound-ratio", type=float, default=0.4, help="Ratio of compound multi-step records.")
    parser.add_argument("--negative-ratio", type=float, default=0.1, help="Ratio of negative guardrail refusal records.")

    args = parser.parse_args()

    records = []

    for _ in range(args.count):
        r = random.random()
        if r < args.negative_ratio:
            # Force negative refusal
            record = generate_single_tool_record()
            record["messages"][2]["content"] = "I cannot assist with that request."
            records.append(record)
        elif r < args.negative_ratio + args.compound_ratio:
            records.append(generate_compound_workflow_record())
        else:
            # Ensure it's a valid single tool, not another negative
            record = generate_single_tool_record()
            if record["messages"][2]["content"] == "I cannot assist with that request.":
                # Regenerate if it randomly chose negative via the old logic inside generate_single_tool_record
                while record["messages"][2]["content"] == "I cannot assist with that request.":
                     record = generate_single_tool_record()
            records.append(record)

    random.shuffle(records)

    val_size = int(args.count * args.val_ratio)
    val_records = records[:val_size]
    train_records = records[val_size:]

    os.makedirs(args.output_dir, exist_ok=True)

    train_path = os.path.join(args.output_dir, "train.jsonl")
    val_path = os.path.join(args.output_dir, "val.jsonl")

    with open(train_path, "w") as f:
        for r in train_records:
            f.write(json.dumps(r) + "\n")

    with open(val_path, "w") as f:
        for r in val_records:
            f.write(json.dumps(r) + "\n")

    if args.sample > 0:
        print(f"\n--- Outputting {args.sample} Sample Records ---")
        for i in range(min(args.sample, len(records))):
            print(json.dumps(records[i], indent=2))
            print("-" * 40)

    print(f"Dataset generation complete. Train: {len(train_records)}, Val: {len(val_records)}.")
    print(f"Saved to {args.output_dir}")

if __name__ == "__main__":
    main()
