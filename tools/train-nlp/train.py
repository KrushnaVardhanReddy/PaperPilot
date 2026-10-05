import pandas as pd
import torch
import shutil
import zstandard as zstd
from sklearn.model_selection import train_test_split
from datasets import Dataset
from transformers import AutoTokenizer, AutoModelForSequenceClassification, Trainer, TrainingArguments
from optimum.onnxruntime import ORTModelForSequenceClassification, ORTQuantizer
from optimum.onnxruntime.configuration import AutoQuantizationConfig
import os

def main():
    # Load dataset
    df = pd.read_csv("tools/train-nlp/dataset.csv")

    labels = df['label'].unique().tolist()
    label2id = {label: i for i, label in enumerate(labels)}
    id2label = {i: label for i, label in enumerate(labels)}

    df['label'] = df['label'].map(label2id)

    train_df, val_df = train_test_split(df, test_size=0.2, random_state=42)

    train_dataset = Dataset.from_pandas(train_df)
    val_dataset = Dataset.from_pandas(val_df)

    # Use TinyBERT-4L-312D
    model_name = "huawei-noah/TinyBERT_General_4L_312D"
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModelForSequenceClassification.from_pretrained(
        model_name,
        num_labels=len(labels),
        id2label=id2label,
        label2id=label2id,
        ignore_mismatched_sizes=True
    )

    def tokenize_function(examples):
        return tokenizer(examples["query"], padding="max_length", truncation=True, max_length=64)

    tokenized_train = train_dataset.map(tokenize_function, batched=True)
    tokenized_val = val_dataset.map(tokenize_function, batched=True)

    training_args = TrainingArguments(
        output_dir="tools/train-nlp/results",
        num_train_epochs=1, # reduce for quick execution
        per_device_train_batch_size=16,
        per_device_eval_batch_size=16,
        warmup_steps=10,
        weight_decay=0.01,
        logging_dir="tools/train-nlp/logs",
        logging_steps=10,
        eval_strategy="epoch",
        save_strategy="epoch",
        load_best_model_at_end=True,
    )

    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_train,
        eval_dataset=tokenized_val,
    )

    print("Starting training...")
    trainer.train()

    temp_dir = "tools/train-nlp/temp_model"
    model.save_pretrained(temp_dir)
    tokenizer.save_pretrained(temp_dir)

    print("Exporting to ONNX...")
    output_dir = "tools/train-nlp/output"
    os.makedirs(output_dir, exist_ok=True)

    ort_model = ORTModelForSequenceClassification.from_pretrained(temp_dir, export=True)

    quantizer = ORTQuantizer.from_pretrained(ort_model)
    qconfig = AutoQuantizationConfig.avx2(is_static=False, per_channel=True)
    quantizer.quantize(save_dir=output_dir, quantization_config=qconfig)

    tokenizer.save_pretrained(output_dir)

    model_path = os.path.join(output_dir, "model_quantized.onnx")
    if not os.path.exists(model_path):
        model_path = os.path.join(output_dir, "model.onnx")

    zst_path = os.path.join(output_dir, "tinybert_int8.onnx.zst")
    print(f"Compressing {model_path} to {zst_path}...")
    with open(model_path, 'rb') as f_in:
        data = f_in.read()

    cctx = zstd.ZstdCompressor(level=19)
    compressed = cctx.compress(data)

    with open(zst_path, 'wb') as f_out:
        f_out.write(compressed)

    for f in os.listdir(output_dir):
        if f.endswith('.onnx'):
            os.remove(os.path.join(output_dir, f))

    print(f"Model exported and compressed to {output_dir}")

if __name__ == "__main__":
    main()
