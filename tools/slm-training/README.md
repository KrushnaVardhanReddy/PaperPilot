# PaperPilot SLM Dataset Generator

This tool generates a synthetic dataset for fine-tuning the **PaperPilot-Qwen-0.6B** Small Language Model.
It transforms natural language user requests into precise ChatML JSON objects containing correct tool invocations matching the PaperPilot internal architecture.

## Tool Registry
The dataset engine covers 15 core PDF manipulation tools in its first wave:
`pdf_merge`, `pdf_split`, `pdf_compress`, `pdf_rotate`, `pdf_watermark`, `pdf_encrypt`, `pdf_decrypt`, `pdf_ocr`, `pdf_bates`, `pdf_extract_text`, `pdf_extract_tables`, `pdf_redact`, `pdf_sanitize`, `pdf_auto_rotate`, and `document_pii_scan`.

It generates synthetic prompts using various linguistic profiles, including:
- Direct imperatives.
- Conversational/casual speech.
- Legal & Finance domain terminology.
- Typo and noise robustness.
- Compound workflow sequences.

## Installation
Ensure you have `pydantic` and `pytest` installed:
```bash
pip install -r requirements.txt
```

## Running the Generator
To generate 25,000 ChatML examples across a train/val split (default 90/10):
```bash
python3 generate_dataset.py --count 25000 --output-dir dataset/
```

To see a small sample output in the terminal:
```bash
python3 generate_dataset.py --count 100 --output-dir /tmp/dataset --sample 3
```

## Running Tests
To ensure the integrity of the generated ChatML dataset, run the provided pytest suite:
```bash
python3 -m pytest test_dataset_generator.py -v
```
