use std::path::Path;

use ndarray::Axis;
use ort::value::TensorRef;

/// Error type for embedding operations.
#[derive(Debug, thiserror::Error)]
pub enum EmbedError {
    #[error("ONNX runtime error: {0}")]
    Ort(String),

    #[error("tokenizer error: {0}")]
    Tokenizer(String),

    #[error("model not found: {0}")]
    ModelNotFound(String),
}

/// ONNX-based text embedder using bge-small-en-v1.5.
///
/// Loads an ONNX model and tokenizer from disk, then embeds text into
/// 384-dimensional vectors using mean pooling over token embeddings.
pub struct Embedder {
    session: ort::session::Session,
    tokenizer: tokenizers::Tokenizer,
}

impl Embedder {
    /// Create a new embedder from model files in `model_dir`.
    ///
    /// Expects:
    /// - `model_dir/model.onnx` — the ONNX model
    /// - `model_dir/tokenizer.json` — the tokenizer config
    ///
    /// Uses DirectML execution provider for hardware acceleration (NPU/GPU/CPU).
    pub fn new(model_dir: &Path) -> Result<Self, EmbedError> {
        let model_path = model_dir.join("model.onnx");
        let tokenizer_path = model_dir.join("tokenizer.json");

        if !model_path.exists() {
            return Err(EmbedError::ModelNotFound(
                model_path.to_string_lossy().to_string(),
            ));
        }

        if !tokenizer_path.exists() {
            return Err(EmbedError::ModelNotFound(
                tokenizer_path.to_string_lossy().to_string(),
            ));
        }

        // Build ONNX session with DirectML for hardware acceleration.
        // DirectML auto-routes to NPU > GPU > CPU. If DirectML is not
        // available on this system, ort silently falls back to CPU.
        let session = ort::session::Session::builder()
            .map_err(|e| EmbedError::Ort(e.to_string()))?
            .with_execution_providers([ort::ep::DirectML::default().build()])
            .map_err(|e| EmbedError::Ort(e.to_string()))?
            .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3)
            .map_err(|e| EmbedError::Ort(e.to_string()))?
            .commit_from_file(&model_path)
            .map_err(|e| EmbedError::Ort(e.to_string()))?;

        let tokenizer = tokenizers::Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| EmbedError::Tokenizer(e.to_string()))?;

        Ok(Self { session, tokenizer })
    }

    /// Embed a batch of texts, returning a Vec of 384-dimensional vectors.
    ///
    /// Uses mean pooling over token embeddings (masked by attention mask)
    /// followed by L2 normalization.
    pub fn embed(&mut self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbedError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // Tokenize all texts
        let encodings = self
            .tokenizer
            .encode_batch(texts.to_vec(), true)
            .map_err(|e| EmbedError::Tokenizer(e.to_string()))?;

        let batch_size = encodings.len();
        let max_len = encodings
            .iter()
            .map(|e| e.get_ids().len())
            .max()
            .unwrap_or(0);

        if max_len == 0 {
            return Ok(vec![vec![0.0f32; 384]; batch_size]);
        }

        // Build flat input arrays with padding
        let total_elements = batch_size * max_len;
        let mut input_ids = vec![0i64; total_elements];
        let mut attention_mask = vec![0i64; total_elements];
        let mut token_type_ids = vec![0i64; total_elements];

        for (i, encoding) in encodings.iter().enumerate() {
            let ids = encoding.get_ids();
            let mask = encoding.get_attention_mask();
            let type_ids = encoding.get_type_ids();
            let row_offset = i * max_len;

            for (j, (&id, (&m, &t))) in
                ids.iter().zip(mask.iter().zip(type_ids.iter())).enumerate()
            {
                input_ids[row_offset + j] = id as i64;
                attention_mask[row_offset + j] = m as i64;
                token_type_ids[row_offset + j] = t as i64;
            }
        }

        let shape = [batch_size, max_len];

        // Create tensor references from flat arrays + shape
        let a_ids = TensorRef::from_array_view((shape, &*input_ids))
            .map_err(|e| EmbedError::Ort(e.to_string()))?;
        let a_mask = TensorRef::from_array_view((shape, &*attention_mask))
            .map_err(|e| EmbedError::Ort(e.to_string()))?;
        let a_type_ids = TensorRef::from_array_view((shape, &*token_type_ids))
            .map_err(|e| EmbedError::Ort(e.to_string()))?;

        // Run inference with named inputs
        let outputs = self
            .session
            .run(ort::inputs![
                "input_ids" => a_ids,
                "attention_mask" => a_mask,
                "token_type_ids" => a_type_ids,
            ])
            .map_err(|e| EmbedError::Ort(e.to_string()))?;

        // Extract embeddings from the output.
        // bge-small outputs last_hidden_state of shape [batch, seq_len, 384].
        // We mean-pool over the sequence dimension, masked by attention_mask.
        let output_array = outputs[0]
            .try_extract_array::<f32>()
            .map_err(|e| EmbedError::Ort(e.to_string()))?;

        let output_shape = output_array.shape(); // [batch, seq_len, hidden_dim]
        let hidden_dim = output_shape[2];

        let mut embeddings = Vec::with_capacity(batch_size);

        for i in 0..batch_size {
            let row_slice = output_array.index_axis(Axis(0), i);
            let row_offset = i * max_len;

            let mut sum = vec![0.0f32; hidden_dim];
            let mut count = 0.0f32;

            for j in 0..output_shape[1] {
                if attention_mask[row_offset + j] == 1 {
                    let token_embedding = row_slice.index_axis(Axis(0), j);
                    for (k, val) in token_embedding.iter().enumerate() {
                        sum[k] += val;
                    }
                    count += 1.0;
                }
            }

            // Mean pooling
            if count > 0.0 {
                for val in &mut sum {
                    *val /= count;
                }
            }

            // L2 normalize
            let norm: f32 = sum.iter().map(|x| x * x).sum::<f32>().sqrt();
            if norm > 0.0 {
                for val in &mut sum {
                    *val /= norm;
                }
            }

            embeddings.push(sum);
        }

        Ok(embeddings)
    }
}

/// Verify that the required model files exist in `model_dir`.
///
/// Returns `Ok(())` if both `model.onnx` and `tokenizer.json` exist.
/// Returns a descriptive error if either file is missing, telling the
/// user where to download the model.
pub fn ensure_model_exists(model_dir: &Path) -> Result<(), EmbedError> {
    let model_path = model_dir.join("model.onnx");
    let tokenizer_path = model_dir.join("tokenizer.json");

    if model_path.exists() && tokenizer_path.exists() {
        return Ok(());
    }

    std::fs::create_dir_all(model_dir)
        .map_err(|e| EmbedError::Ort(format!("failed to create model dir: {e}")))?;

    if !model_path.exists() {
        return Err(EmbedError::ModelNotFound(format!(
            "ONNX model not found. Download bge-small-en-v1.5 ONNX model to: {}",
            model_dir.display()
        )));
    }

    if !tokenizer_path.exists() {
        return Err(EmbedError::ModelNotFound(format!(
            "Tokenizer not found. Download tokenizer.json to: {}",
            model_dir.display()
        )));
    }

    Ok(())
}
