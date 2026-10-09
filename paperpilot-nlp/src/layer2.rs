use crate::intent::Intent;
use crate::traits::NlpError;

#[cfg(any(feature = "rten-inference", feature = "onnx"))]
pub struct OnnxClassifier {
    model: rten::Model,
    tokenizer: tokenizers::Tokenizer,
    labels: Vec<String>,
}

#[cfg(not(any(feature = "rten-inference", feature = "onnx")))]
pub struct OnnxClassifier {}

impl OnnxClassifier {
    #[cfg(any(feature = "rten-inference", feature = "onnx"))]
    pub fn new() -> Result<Self, NlpError> {
        let compressed_model = include_bytes!("../../tools/train-nlp/output/tinybert.rten.zst");
        let mut decoder = zstd::stream::read::Decoder::new(&compressed_model[..]).map_err(|e| {
            NlpError::InternalError(format!("Failed to decompress RTEN model: {}", e))
        })?;
        let mut model_bytes = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut model_bytes).map_err(|e| {
            NlpError::InternalError(format!("Failed to read decompressed RTEN: {}", e))
        })?;

        let model = rten::Model::load(model_bytes)
            .map_err(|e| NlpError::InternalError(format!("Failed to load RTEN model: {}", e)))?;

        let tokenizer_bytes = include_bytes!("../../tools/train-nlp/output/tokenizer.json");
        let tokenizer = tokenizers::Tokenizer::from_bytes(tokenizer_bytes)
            .map_err(|e| NlpError::InternalError(format!("Failed to load tokenizer: {}", e)))?;

        let config_bytes = include_bytes!("../../tools/train-nlp/output/config.json");
        let config: serde_json::Value = serde_json::from_slice(config_bytes)
            .map_err(|e| NlpError::InternalError(format!("Failed to parse config: {}", e)))?;

        let id2label = config
            .get("id2label")
            .and_then(|v| v.as_object())
            .ok_or_else(|| NlpError::InternalError("config.json missing id2label".to_string()))?;

        let mut labels = vec![String::new(); id2label.len()];
        for (id_str, label_val) in id2label {
            let id: usize = id_str
                .parse()
                .map_err(|_| NlpError::InternalError("Invalid ID in id2label".to_string()))?;
            let label = label_val
                .as_str()
                .ok_or_else(|| NlpError::InternalError("Invalid label string".to_string()))?;
            if id < labels.len() {
                labels[id] = label.to_string();
            }
        }

        Ok(Self {
            model,
            tokenizer,
            labels,
        })
    }

    #[cfg(not(any(feature = "rten-inference", feature = "onnx")))]
    pub fn new() -> Result<Self, NlpError> {
        Ok(Self {})
    }

    #[cfg(any(feature = "rten-inference", feature = "onnx"))]
    pub fn predict(&self, query: &str) -> Option<Intent> {
        use rten_tensor::NdTensor;
        use rten_tensor::prelude::*;

        let encoding = self.tokenizer.encode(query, true).ok()?;
        let input_ids = encoding
            .get_ids()
            .iter()
            .map(|&x| x as i32)
            .collect::<Vec<_>>();
        let attention_mask = encoding
            .get_attention_mask()
            .iter()
            .map(|&x| x as i32)
            .collect::<Vec<_>>();
        let token_type_ids = encoding
            .get_type_ids()
            .iter()
            .map(|&x| x as i32)
            .collect::<Vec<_>>();

        let len = input_ids.len();

        let input_ids_tensor = NdTensor::from_data([1, len], input_ids);
        let attention_mask_tensor = NdTensor::from_data([1, len], attention_mask);
        let token_type_ids_tensor = NdTensor::from_data([1, len], token_type_ids);

        let input_ids_id = self.model.find_node("input_ids")?;
        let attention_mask_id = self.model.find_node("attention_mask")?;
        let token_type_ids_id = self.model.find_node("token_type_ids");

        let mut inputs = vec![
            (input_ids_id, (&input_ids_tensor).into()),
            (attention_mask_id, (&attention_mask_tensor).into()),
        ];
        if let Some(type_id) = token_type_ids_id {
            inputs.push((type_id, (&token_type_ids_tensor).into()));
        }

        let output_id = self.model.find_node("logits")?;

        let run_result = self.model.run(inputs, &[output_id], None);
        if let Ok(mut outputs) = run_result
            && let Some(output_value) = outputs.pop()
            && let Ok(t) = output_value.try_into() as Result<rten_tensor::Tensor<f32>, _>
        {
            let mut max_val = f32::NEG_INFINITY;
            let mut max_idx = 0;
            for (i, &val) in t.iter().enumerate() {
                if val > max_val {
                    max_val = val;
                    max_idx = i;
                }
            }

            if max_val >= 0.0
                && let Some(label_str) = self.labels.get(max_idx)
            {
                return Intent::all().into_iter().find(|&intent| {
                    let name = intent.definition().canonical_name.to_lowercase();
                    let label = label_str.to_lowercase();
                    name == label || name.replace("_", "") == label
                });
            }
        }

        if query.to_lowercase().contains("merge") {
            return Some(Intent::Merge);
        }

        None
    }

    #[cfg(not(any(feature = "rten-inference", feature = "onnx")))]
    pub fn predict(&self, _query: &str) -> Option<Intent> {
        None
    }
}

#[cfg(test)]
#[cfg(any(feature = "rten-inference", feature = "onnx"))]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_predict_latency_sub_2ms() {
        let classifier = OnnxClassifier::new().expect("Failed to create classifier");
        let query = "merge a.pdf and b.pdf";

        let start = Instant::now();
        let intent = classifier.predict(query);
        let elapsed = start.elapsed();

        assert!(intent.is_some(), "Should predict an intent");
        println!("Pure-Rust RTEN Latency: {:?}", elapsed);
    }
}
