"""Deterministic data and artifact-compatibility regressions (no model required)."""

import json
from pathlib import Path
import random
import subprocess
import sys

import pytest

from generate_dataset import (
    REFUSAL, SYSTEM_PROMPT, generate_single_tool_record,
    generate_compound_workflow_record, template_fields,
)
from templates import SINGLE_TOOL_TEMPLATES, COMPOUND_WORKFLOW_TEMPLATES, TemplateCategory
from tool_registry import TOOL_SCHEMAS
from workflow import CONTRACTS, validate_workflow

SCRIPT = Path(__file__).with_name("generate_dataset.py")


@pytest.fixture(autouse=True)
def reproducible_random():
    state = random.getstate()
    random.seed(84)
    yield
    random.setstate(state)


@pytest.fixture(scope="module")
def generated_dataset(tmp_path_factory):
    directory = tmp_path_factory.mktemp("dataset")
    subprocess.run([sys.executable, str(SCRIPT), "--count", "5000", "--seed", "84",
                    "--output-dir", str(directory)], check=True, cwd=directory)
    return [json.loads(line) for split in ("train", "val")
            for line in (directory / f"{split}.jsonl").read_text(encoding="utf-8").splitlines()]


def calls(record):
    value = json.loads(record["messages"][-1]["content"])
    return value if isinstance(value, list) else [value]


def test_all_generated_labels_obey_schema_and_artifact_types(generated_dataset):
    found = set()
    counts = {"single": 0, "compound": 0, "refusal": 0}
    for record in generated_dataset:
        messages = record["messages"]
        assert [message["role"] for message in messages] == ["system", "user", "assistant"]
        assert messages[0]["content"] == SYSTEM_PROMPT
        assert messages[1]["content"]
        if messages[-1]["content"] == REFUSAL:
            counts["refusal"] += 1
            continue
        steps = calls(record)
        validate_workflow(steps)
        found.update(step["tool"] for step in steps)
        counts["compound" if len(steps) > 1 else "single"] += 1
    assert found == set(CONTRACTS)
    for key, expected in (("single", .5), ("compound", .4), ("refusal", .1)):
        assert abs(counts[key] / len(generated_dataset) - expected) < .03, counts


@pytest.mark.parametrize("name", sorted(CONTRACTS))
@pytest.mark.parametrize("category", [TemplateCategory.DIRECT_IMPERATIVE, TemplateCategory.CONVERSATIONAL,
                                      TemplateCategory.DOMAIN_LEGAL_FINANCE, TemplateCategory.TYPO_NOISE])
def test_every_tool_and_linguistic_style_generates_valid_calls(name, category):
    record = generate_single_tool_record(tool_name=name, category=category)
    steps = calls(record)
    validate_workflow(steps)
    params = steps[0]["parameters"]
    assert params[CONTRACTS[name].output_parameter] in record["messages"][1]["content"]
    assert all(template_fields(template) <= TOOL_SCHEMAS[name]["properties"].keys()
               for template in SINGLE_TOOL_TEMPLATES[name][category])


@pytest.mark.parametrize("workflow", COMPOUND_WORKFLOW_TEMPLATES, ids=lambda w: w["id"])
def test_every_curated_recipe_is_compatible(workflow):
    record = generate_compound_workflow_record(workflow=workflow)
    steps = calls(record)
    assert [step["tool"] for step in steps] == workflow["tools"]
    assert 2 <= len(steps) <= 5
    validate_workflow(steps)
    current = steps[0]["parameters"].get("input", steps[0]["parameters"].get("input_a"))
    if current is None:
        current = steps[0]["parameters"]["inputs"][0]
    for step in steps:
        contract = CONTRACTS[step["tool"]]
        params = step["parameters"]
        if contract.input_parameters[0] == "inputs":
            assert current in params["inputs"]
        else:
            assert params[contract.input_parameters[0]] == current
        if not contract.preserves_input:
            current = params[contract.output_parameter]


def test_reports_preserve_original_pdf_for_following_steps():
    steps = calls(generate_compound_workflow_record(
        tools=["pdf_validate_signatures", "document_pii_scan", "pdf_watermark"]))
    assert len({step["parameters"]["input"] for step in steps}) == 1
    assert steps[0]["parameters"]["output"].endswith(".json")
    assert steps[1]["parameters"]["output"].endswith(".json")
    assert steps[2]["parameters"]["output"].endswith(".pdf")


def test_office_exports_are_typed():
    steps = calls(generate_compound_workflow_record(tools=["pdf_to_pptx", "office_extract_text"]))
    assert steps[0]["parameters"]["output"].endswith(".pptx")
    assert steps[1]["parameters"]["input"] == steps[0]["parameters"]["output"]
    assert steps[1]["parameters"]["output"].endswith(".txt")
    for legacy, modern in (("doc", "docx"), ("xls", "xlsx"), ("ppt", "pptx")):
        validate_workflow([
            {"tool": "office_modernize_legacy", "parameters": {"input": f"source.{legacy}", "output": f"modern.{modern}"}},
            {"tool": "office_to_markdown", "parameters": {"input": f"modern.{modern}", "output": "archive.md"}},
        ])


def test_compatible_random_sequences_cover_all_lengths():
    lengths = set()
    for _ in range(300):
        steps = calls(generate_compound_workflow_record())
        validate_workflow(steps)
        lengths.add(len(steps))
        for step in steps:
            contract = CONTRACTS[step["tool"]]
            if contract.input_parameters == ("inputs",):
                assert len(step["parameters"]["inputs"]) >= 2
    assert lengths == {2, 3, 4, 5}


@pytest.mark.parametrize("tools", [
    ["pdf_split", "pdf_compress"], ["pdf_to_markdown", "pdf_compress"],
    ["office_modernize_legacy", "pdf_to_markdown"], ["pdf_encrypt", "pdf_watermark"],
])
def test_incompatible_sequences_are_rejected(tools):
    with pytest.raises(ValueError):
        generate_compound_workflow_record(tools=tools)


@pytest.mark.parametrize("mutation", ["report_input", "future_input", "ghost_input", "wrong_extension",
                                      "unknown_parameter", "missing_parameter", "wrong_type", "unknown_tool"])
def test_invalid_plans_fail_before_execution(mutation):
    steps = calls(generate_compound_workflow_record(tools=["pdf_validate_signatures", "pdf_watermark"]))
    if mutation == "report_input":
        steps[1]["parameters"]["input"] = steps[0]["parameters"]["output"]
    elif mutation == "future_input":
        steps[0]["parameters"]["input"] = steps[1]["parameters"]["output"]
    elif mutation == "ghost_input":
        steps[1]["parameters"]["input"] = "/tmp/inter_missing.pdf"
    elif mutation == "wrong_extension":
        steps[0]["parameters"]["output"] = "report.pdf"
    elif mutation == "unknown_parameter":
        steps[1]["parameters"]["rotation"] = 90
    elif mutation == "missing_parameter":
        del steps[1]["parameters"]["text"]
    elif mutation == "wrong_type":
        steps[1]["parameters"]["opacity"] = "opaque"
    else:
        steps[0]["tool"] = "pdf_compare_visual"
    with pytest.raises(ValueError):
        validate_workflow(steps)


def test_encrypt_decrypt_uses_same_password():
    steps = calls(generate_compound_workflow_record(tools=["pdf_encrypt", "pdf_decrypt", "pdf_compress"]))
    validate_workflow(steps)
    steps[1]["parameters"]["password"] = "wrong"
    with pytest.raises(ValueError, match="password"):
        validate_workflow(steps)


def test_refusals_are_attached_to_actual_guardrail_templates():
    for _ in range(50):
        record = generate_single_tool_record(negative=True)
        assert record["messages"][-1]["content"] == REFUSAL
        assert record["messages"][1]["content"]


def test_catalog_excludes_unavailable_or_fixture_dependent_handlers():
    assert set(CONTRACTS) == set(TOOL_SCHEMAS) == set(SINGLE_TOOL_TEMPLATES)
    assert not {"pdf_extract_tables", "pdf_stamp", "pdf_overlay_underlay", "pdf_reverse",
                "pdf_invert_colors", "pdf_compare_visual"} & CONTRACTS.keys()
    for name, schema in TOOL_SCHEMAS.items():
        assert set(schema.get("required", [])) <= schema["properties"].keys(), name


@pytest.mark.parametrize("arguments", [["--count", "0"], ["--val-ratio", "1"],
                                       ["--negative-ratio", "-1"],
                                       ["--negative-ratio", ".8", "--compound-ratio", ".5"]])
def test_invalid_cli_configuration_is_rejected(tmp_path, arguments):
    result = subprocess.run([sys.executable, str(SCRIPT), "--output-dir", str(tmp_path / "bad"), *arguments],
                            capture_output=True, text=True)
    assert result.returncode != 0
    assert not (tmp_path / "bad").exists()


def test_seed_makes_data_reproducible_from_any_working_directory(tmp_path):
    for name in ("first", "second"):
        subprocess.run([sys.executable, str(SCRIPT), "--count", "50", "--seed", "84",
                        "--output-dir", str(tmp_path / name)], cwd=tmp_path, check=True)
    for split in ("train", "val"):
        assert (tmp_path / "first" / f"{split}.jsonl").read_bytes() == (tmp_path / "second" / f"{split}.jsonl").read_bytes()
