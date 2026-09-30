import pandas as pd
import torch
from sklearn.model_selection import train_test_split
from datasets import Dataset
from transformers import AutoTokenizer, AutoModelForSequenceClassification, Trainer, TrainingArguments
from optimum.onnxruntime import ORTModelForSequenceClassification

def main():
    # Load dataset
    df = pd.read_csv("tools/train-nlp/dataset.csv")

    # Map labels to integers
    labels = df['label'].unique().tolist()
    label2id = {label: i for i, label in enumerate(labels)}
    id2label = {i: label for i, label in enumerate(labels)}

    df['label'] = df['label'].map(label2id)

    # Split dataset
    train_df, val_df = train_test_split(df, test_size=0.2, random_state=42)

    train_dataset = Dataset.from_pandas(train_df)
    val_dataset = Dataset.from_pandas(val_df)

    # Load model and tokenizer
    model_name = "prajjwal1/bert-tiny"
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModelForSequenceClassification.from_pretrained(
        model_name,
        num_labels=len(labels),
        id2label=id2label,
        label2id=label2id
    )

    # Tokenize dataset
    def tokenize_function(examples):
        return tokenizer(examples["query"], padding="max_length", truncation=True, max_length=64)

    tokenized_train = train_dataset.map(tokenize_function, batched=True)
    tokenized_val = val_dataset.map(tokenize_function, batched=True)

    # Training arguments
    training_args = TrainingArguments(
        output_dir="tools/train-nlp/results",
        num_train_epochs=2,
        per_device_train_batch_size=16,
        per_device_eval_batch_size=16,
        warmup_steps=50,
        weight_decay=0.01,
        logging_dir="tools/train-nlp/logs",
        logging_steps=10,
        eval_strategy="epoch",
        save_strategy="epoch",
        load_best_model_at_end=True,
    )

    # Trainer
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_train,
        eval_dataset=tokenized_val,
    )

    # Train
    print("Starting training...")
    trainer.train()

    # Save the standard model to a temporary directory
    temp_dir = "tools/train-nlp/temp_model"
    model.save_pretrained(temp_dir)
    tokenizer.save_pretrained(temp_dir)

    # Export to ONNX
    print("Exporting to ONNX...")
    output_dir = "tools/train-nlp/output"

    ort_model = ORTModelForSequenceClassification.from_pretrained(temp_dir, export=True)
    ort_model.save_pretrained(output_dir)
    tokenizer.save_pretrained(output_dir)

    print(f"Model exported to {output_dir}")

if __name__ == "__main__":
    main()
