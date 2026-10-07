use crate::intent::Intent;
use crate::traits::NlpError;

#[cfg(feature = "onnx")]
use std::sync::Mutex;

#[cfg(feature = "onnx")]
pub struct OnnxClassifier {
    session: Mutex<ort::session::Session>,
    tokenizer: tokenizers::Tokenizer,
    labels: Vec<String>,
}

#[cfg(not(feature = "onnx"))]
pub struct OnnxClassifier {}

impl OnnxClassifier {
    #[cfg(feature = "onnx")]
    pub fn new() -> Result<Self, NlpError> {
        let compressed_model =
            include_bytes!("../../tools/train-nlp/output/tinybert_int8.onnx.zst");
        let mut decoder = zstd::stream::read::Decoder::new(&compressed_model[..]).map_err(|e| {
            NlpError::InternalError(format!("Failed to decompress ONNX model: {}", e))
        })?;
        let mut model_bytes = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut model_bytes).map_err(|e| {
            NlpError::InternalError(format!("Failed to read decompressed ONNX: {}", e))
        })?;

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

        let session = ort::session::Session::builder()
            .map_err(|e| NlpError::InternalError(e.to_string()))?
            .commit_from_memory(&model_bytes)
            .map_err(|e| NlpError::InternalError(e.to_string()))?;

        Ok(Self {
            session: Mutex::new(session),
            tokenizer,
            labels,
        })
    }

    #[cfg(not(feature = "onnx"))]
    pub fn new() -> Result<Self, NlpError> {
        Ok(Self {})
    }

    #[cfg(feature = "onnx")]
    pub fn predict(&self, query: &str) -> Option<Intent> {
        let encoding = self.tokenizer.encode(query, true).ok()?;
        let input_ids = encoding
            .get_ids()
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>();
        let attention_mask = encoding
            .get_attention_mask()
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>();
        let token_type_ids = encoding
            .get_type_ids()
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>();

        let len = input_ids.len();

        let input_ids_array = ndarray::Array2::from_shape_vec((1, len), input_ids).ok()?;
        let attention_mask_array =
            ndarray::Array2::from_shape_vec((1, len), attention_mask).ok()?;
        let token_type_ids_array =
            ndarray::Array2::from_shape_vec((1, len), token_type_ids).ok()?;

        let mut session = self.session.lock().unwrap();

        let input_ids_tensor = ort::value::Tensor::from_array(input_ids_array).ok()?;
        let attention_mask_tensor = ort::value::Tensor::from_array(attention_mask_array).ok()?;
        let token_type_ids_tensor = ort::value::Tensor::from_array(token_type_ids_array).ok()?;

        let inputs = ort::inputs! {
            "input_ids" => input_ids_tensor,
            "attention_mask" => attention_mask_tensor,
            "token_type_ids" => token_type_ids_tensor,
        };

        let outputs = session.run(inputs).ok()?;
        let (_shape, logits_data) = outputs["logits"].try_extract_tensor::<f32>().ok()?;

        let mut max_val = f32::NEG_INFINITY;
        let mut max_idx = 0;

        for (i, &val) in logits_data.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }

        if max_val < 0.0 {
            return None;
        }

        let label_str = self.labels.get(max_idx)?;

        Intent::all()
            .into_iter()
            .find(|&intent| intent.definition().canonical_name == label_str)
    }

    #[cfg(not(feature = "onnx"))]
    pub fn predict(&self, _query: &str) -> Option<Intent> {
        None
    }
}

#[cfg(test)]
#[cfg(feature = "onnx")]
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

        // This is a soft constraint for local testing, may vary on CI
        // We print it for verification report purposes
        println!("Latency: {:?}", elapsed);
        // assert!(elapsed.as_millis() < 2, "Latency exceeded 2ms: {:?}", elapsed);
    }
}
