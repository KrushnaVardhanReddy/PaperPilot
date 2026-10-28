"""Validate the inference evaluator using generated training labels, without ML dependencies."""

import json
from pathlib import Path
import subprocess
import sys

import pytest

from generate_dataset import generate_compound_workflow_record
from infer_local import default_cases, load_inference_model, validate_response


def test_default_benchmark_covers_single_compound_and_refusal_cases():
    cases = default_cases()
    assert sum(len(case.get("tools", [])) == 1 for case in cases) == 6
    assert sum(len(case.get("tools", [])) > 1 for case in cases) == 15
    assert sum(case.get("refusal", False) for case in cases) == 1


def test_compatible_training_label_passes_model_plan_validation():
    record = generate_compound_workflow_record(tools=["pdf_watermark", "pdf_encrypt"])
    validate_response(record["messages"][-1]["content"], {"tools": ["pdf_watermark", "pdf_encrypt"]})


def test_collapsing_compound_request_to_single_tool_fails():
    record = generate_compound_workflow_record(tools=["pdf_watermark", "pdf_encrypt"])
    first = json.loads(record["messages"][-1]["content"])[0]
    with pytest.raises(ValueError, match="array"):
        validate_response(json.dumps(first), {"tools": ["pdf_watermark", "pdf_encrypt"]})


def test_wrong_tool_order_fails_even_with_valid_schemas():
    record = generate_compound_workflow_record(tools=["pdf_watermark", "pdf_encrypt"])
    with pytest.raises(ValueError, match="tools/order"):
        validate_response(record["messages"][-1]["content"], {"tools": ["pdf_compress", "pdf_encrypt"]})


def test_missing_weights_fail_before_any_base_model_download(tmp_path):
    with pytest.raises(ValueError, match="Fine-tuned weights are missing"):
        load_inference_model(tmp_path / "missing")
    result = subprocess.run([sys.executable, str(Path(__file__).with_name("infer_local.py")),
                             "--model-dir", str(tmp_path / "missing"),
                             "--report", str(tmp_path / "report.json")], capture_output=True, text=True)
    assert result.returncode != 0
    assert "Fine-tuned weights are missing" in result.stderr
    assert not (tmp_path / "report.json").exists()
