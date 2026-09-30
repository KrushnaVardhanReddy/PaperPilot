from transformers import AutoTokenizer
from optimum.onnxruntime import ORTModelForSequenceClassification
import torch

def main():
    model_path = "tools/train-nlp/output"

    tokenizer = AutoTokenizer.from_pretrained(model_path)
    model = ORTModelForSequenceClassification.from_pretrained(model_path)

    queries = [
        "merge these pdfs together",
        "put a password on this document",
        "extract page 5 from the file",
        "convert this to word"
    ]

    for query in queries:
        inputs = tokenizer(query, return_tensors="pt")
        outputs = model(**inputs)
        logits = outputs.logits
        predicted_class_idx = logits.argmax(-1).item()

        predicted_intent = model.config.id2label[predicted_class_idx]

        print(f"Query: '{query}' -> Intent: {predicted_intent}")

if __name__ == "__main__":
    main()
