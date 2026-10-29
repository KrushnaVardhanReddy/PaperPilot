"""Artifact contracts for the training catalog, independent of model/runtime code.

MCP JSON schemas describe parameters, not the bytes written to output paths.
These contracts follow the actual handlers (e.g. pdf_compare writes JSON).
"""

from dataclasses import dataclass
from pathlib import PurePosixPath

from jsonschema import Draft202012Validator


@dataclass(frozen=True)
class ArtifactContract:
    input_kinds: tuple[str, ...] = ("pdf",)
    output_kind: str = "pdf"
    input_parameters: tuple[str, ...] = ("input",)
    output_parameter: str = "output"
    preserves_input: bool = False
    terminal: bool = False


_PDF_TO_PDF = (
    "pdf_compress", "pdf_rotate", "pdf_watermark", "pdf_ocr", "pdf_bates",
    "pdf_redact", "pdf_sanitize", "pdf_auto_rotate", "pdf_nup", "pdf_booklet",
    "pdf_scale_pages", "pdf_crop", "pdf_reorder_pages", "pdf_delete_pages",
    "pdf_stitch_canvas", "pdf_attachments",
    "pdf_remove_signatures", "pdf_flatten", "pdf_header_footer", "pdf_linearize",
    "pdf_repair", "pdf_extract_pages", "pdf_remove_blank", "pdf_page_numbers",
)
CONTRACTS = {name: ArtifactContract() for name in _PDF_TO_PDF}
CONTRACTS.update({
    "pdf_merge": ArtifactContract(input_parameters=("inputs",)),
    "pdf_split": ArtifactContract(output_kind="directory", output_parameter="output_dir", terminal=True),
    "pdf_extract_images": ArtifactContract(output_kind="directory", output_parameter="output_dir", terminal=True),
    "pdf_images_to_pdf": ArtifactContract(input_kinds=("png", "jpg"), input_parameters=("inputs",)),
    "pdf_encrypt": ArtifactContract(output_kind="encrypted_pdf"),
    "pdf_decrypt": ArtifactContract(input_kinds=("encrypted_pdf",)),
    "pdf_extract_text": ArtifactContract(output_kind="txt", preserves_input=True),
    "pdf_validate_signatures": ArtifactContract(output_kind="json", preserves_input=True),
    "document_pii_scan": ArtifactContract(output_kind="json", preserves_input=True),
    "pdf_compare": ArtifactContract(output_kind="json", input_parameters=("input_a", "input_b"), preserves_input=True),
    "pdf_to_docx": ArtifactContract(output_kind="docx"),
    "pdf_to_xlsx": ArtifactContract(output_kind="xlsx"),
    "pdf_to_pptx": ArtifactContract(output_kind="pptx"),
    "pdf_to_markdown": ArtifactContract(output_kind="md"),
    "pdf_to_html": ArtifactContract(output_kind="html"),
    "office_to_markdown": ArtifactContract(input_kinds=("docx", "xlsx", "pptx"), output_kind="md"),
    "office_extract_text": ArtifactContract(input_kinds=("docx", "xlsx", "pptx"), output_kind="txt", preserves_input=True),
    "office_modernize_legacy": ArtifactContract(input_kinds=("doc", "xls", "ppt"), output_kind="docx"),
})


def output_kind(name, parameters):
    if name == "office_modernize_legacy":
        suffix = PurePosixPath(parameters["input"]).suffix.lower()
        return {".doc": "docx", ".xls": "xlsx", ".ppt": "pptx"}[suffix]
    return CONTRACTS[name].output_kind


def artifact_path(stem, kind):
    if kind == "directory":
        return stem
    return f"{stem}.{('pdf' if kind == 'encrypted_pdf' else kind)}"


def path_kind(path):
    return PurePosixPath(path).suffix.lower().lstrip(".")


def validate_workflow(steps, schemas=None):
    """Reject schema drift, nonexistent/future intermediates, and incompatible bytes.

    External source paths need not exist during synthetic data generation. Once a
    path is produced, its semantic type overrides its extension for all later steps.
    """
    if schemas is None:
        from tool_registry import TOOL_SCHEMAS
        schemas = TOOL_SCHEMAS
    if not isinstance(steps, list) or not steps:
        raise ValueError("Expected a nonempty tool-call array")
    produced = {}
    planned_outputs = set()
    for step in steps:
        if not isinstance(step, dict) or set(step) != {"tool", "parameters"}:
            raise ValueError("Each step must contain only tool and parameters")
        name = step["tool"]
        if not isinstance(name, str) or name not in CONTRACTS or name not in schemas:
            raise ValueError(f"Unknown or unregistered training tool: {name}")
        params = step["parameters"]
        # Reject undeclared parameters even when the server tolerates extras.
        schema = dict(schemas[name], additionalProperties=False)
        errors = list(Draft202012Validator(schema).iter_errors(params))
        if errors:
            raise ValueError(f"{name}: {errors[0].message}")
        contract = CONTRACTS[name]
        output = params.get(contract.output_parameter)
        if not isinstance(output, str) or not output:
            raise ValueError(f"{name}: missing artifact output")
        if output in planned_outputs:
            raise ValueError(f"Duplicate output path: {output}")
        planned_outputs.add(output)

    password_by_path = {}
    for index, step in enumerate(steps):
        name, params = step["tool"], step["parameters"]
        contract = CONTRACTS[name]
        if contract.terminal and index != len(steps) - 1:
            raise ValueError(f"{name}: directory outputs cannot feed a scalar tool")
        for key in contract.input_parameters:
            paths = params.get(key)
            paths = paths if isinstance(paths, list) else [paths]
            if not paths or any(not isinstance(path, str) or not path for path in paths):
                raise ValueError(f"{name}: missing input artifact {key}")
            for path in paths:
                if path in planned_outputs and path not in produced:
                    raise ValueError(f"{name}: input refers to a future output: {path}")
                if path.startswith("/tmp/inter_") and path not in produced:
                    raise ValueError(f"{name}: intermediate has not been produced: {path}")
                kind = produced.get(path, path_kind(path))
                if name == "pdf_decrypt" and path not in produced and kind == "pdf":
                    kind = "encrypted_pdf"  # An external encrypted PDF still ends in .pdf.
                if kind not in contract.input_kinds:
                    raise ValueError(f"{name}: {path} is {kind}, expected {contract.input_kinds}")
                if name == "pdf_decrypt" and path in password_by_path:
                    if params["password"] != password_by_path[path]:
                        raise ValueError("Decryption password differs from preceding encryption")
        output = params[contract.output_parameter]
        kind = output_kind(name, params)
        if kind != "directory" and path_kind(output) != ("pdf" if kind == "encrypted_pdf" else kind):
            raise ValueError(f"{name}: output extension does not match {kind}")
        if kind == "directory" and PurePosixPath(output).suffix:
            raise ValueError(f"{name}: output_dir must name a directory, not a file")
        produced[output] = kind
        if name == "pdf_encrypt":
            password_by_path[output] = params["password"]
    return produced
