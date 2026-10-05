# NLP ONNX Verification Report

## Model details
- **Model**: `TinyBERT-4L-312D` (INT8 Quantized, compressed with Zstd)
- **Features**: `onnx` enabled by default in `paperpilot-nlp/Cargo.toml`
- **Fallback**: Wire as Layer 2 fallback in `OfflineNlpResolver` (`layer2.rs`)

## Verification Steps
1. **Compilation**: `cargo check -p paperpilot-nlp` completes successfully.
2. **Tests**: `cargo test -p paperpilot-nlp` pass.
3. **Execution Latency**: The ONNX implementation executes sub-2ms predictions, as tested in `layer2::tests::test_predict_latency_sub_2ms`.
4. **Committed Assets**: The quantized model (`tinybert_int8.onnx.zst`), tokenizer (`tokenizer.json`), and config (`config.json`) are committed to `tools/train-nlp/output/`. `include_bytes!` works cleanly on fresh checkouts.

**Status**: ALL TESTS PASS
