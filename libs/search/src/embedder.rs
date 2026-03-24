use std::path::Path;

use futures_util::StreamExt;
use ndarray::Axis;
use ort::value::TensorRef;
use tokio::io::AsyncWriteExt;

/// Tokenized batch output: (input_ids, attention_mask, token_type_ids, batch_size, max_len).
type TokenizedBatch = (Vec<i64>, Vec<i64>, Vec<i64>, usize, usize);

/// Error type for embedding operations.
#[derive(Debug, thiserror::Error)]
pub enum EmbedError {
    #[error("ONNX runtime error: {0}")]
    Ort(String),

    #[error("tokenizer error: {0}")]
    Tokenizer(String),

    #[error("model not found: {0}")]
    ModelNotFound(String),

    #[error("download error: {0}")]
    Download(String),
}

/// ONNX-based text embedder using all-MiniLM-L6-v2.
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

        // Build ONNX session with DirectML targeting the AMD Radeon 890M
        // iGPU (device_id 1). Device 0 is the NVIDIA RTX 5060 Ti dGPU.
        // If DirectML is not available, ort silently falls back to CPU.
        //
        // Level3 optimization restored with ort 2.0.0-rc.12 (ORT 1.24).
        // Previously Level1 was used as a workaround for rc.11 where Level3
        // forced embedding nodes to CPU while /embeddings/Add_1 stayed on DML,
        // causing E_INVALIDARG (0x80070057) from cross-device tensor transfers.
        // rc.12 bundles ORT 1.24 which should resolve the DirectML placement.
        let session = ort::session::Session::builder()
            .map_err(|e| EmbedError::Ort(e.to_string()))?
            .with_execution_providers([ort::ep::DirectML::default().with_device_id(1).build()])
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

        let (input_ids, attention_mask, token_type_ids, batch_size, max_len) =
            self.tokenize_batch(texts)?;

        if max_len == 0 {
            return Ok(vec![vec![0.0f32; 384]; batch_size]);
        }

        let output_array = self.run_inference(
            &input_ids,
            &attention_mask,
            &token_type_ids,
            batch_size,
            max_len,
        )?;
        let embeddings =
            postprocess_embeddings(&output_array, &attention_mask, batch_size, max_len);

        Ok(embeddings)
    }

    /// Tokenize a batch of texts into flat padded arrays.
    ///
    /// Returns `(input_ids, attention_mask, token_type_ids, batch_size, max_len)`.
    fn tokenize_batch(&self, texts: &[&str]) -> Result<TokenizedBatch, EmbedError> {
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
        let total = batch_size * max_len;

        let mut input_ids = vec![0i64; total];
        let mut attention_mask = vec![0i64; total];
        let mut token_type_ids = vec![0i64; total];

        for (i, encoding) in encodings.iter().enumerate() {
            let row_offset = i * max_len;
            for (j, (&id, (&m, &t))) in encoding
                .get_ids()
                .iter()
                .zip(
                    encoding
                        .get_attention_mask()
                        .iter()
                        .zip(encoding.get_type_ids().iter()),
                )
                .enumerate()
            {
                input_ids[row_offset + j] = id as i64;
                attention_mask[row_offset + j] = m as i64;
                token_type_ids[row_offset + j] = t as i64;
            }
        }

        Ok((
            input_ids,
            attention_mask,
            token_type_ids,
            batch_size,
            max_len,
        ))
    }

    /// Run ONNX inference on pre-tokenized inputs.
    fn run_inference(
        &mut self,
        input_ids: &[i64],
        attention_mask: &[i64],
        token_type_ids: &[i64],
        batch_size: usize,
        max_len: usize,
    ) -> Result<ndarray::ArrayD<f32>, EmbedError> {
        let shape = [batch_size, max_len];
        let a_ids = TensorRef::from_array_view((shape, input_ids))
            .map_err(|e| EmbedError::Ort(e.to_string()))?;
        let a_mask = TensorRef::from_array_view((shape, attention_mask))
            .map_err(|e| EmbedError::Ort(e.to_string()))?;
        let a_type_ids = TensorRef::from_array_view((shape, token_type_ids))
            .map_err(|e| EmbedError::Ort(e.to_string()))?;

        let outputs = self
            .session
            .run(ort::inputs![
                "input_ids" => a_ids,
                "attention_mask" => a_mask,
                "token_type_ids" => a_type_ids,
            ])
            .map_err(|e| EmbedError::Ort(e.to_string()))?;

        outputs[0]
            .try_extract_array::<f32>()
            .map(ndarray::ArrayBase::into_owned)
            .map_err(|e| EmbedError::Ort(e.to_string()))
    }
}

const HF_BASE_URL: &str =
    "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main";

/// Files needed for the embedder, with their HF paths relative to the repo root.
const MODEL_FILES: &[(&str, &str)] = &[
    ("model.onnx", "onnx/model.onnx"),
    ("tokenizer.json", "tokenizer.json"),
];

/// Ensure model files exist, downloading from Hugging Face if missing.
///
/// Downloads are streamed to a `.part` temp file, then renamed on completion
/// to avoid partial files from interrupted downloads.
///
/// `progress_cb` is called with (file_name, bytes_downloaded, total_bytes)
/// where total_bytes is `None` if the server didn't send Content-Length.
pub async fn ensure_model_exists<F>(model_dir: &Path, progress_cb: F) -> Result<(), EmbedError>
where
    F: Fn(&str, u64, Option<u64>),
{
    std::fs::create_dir_all(model_dir)
        .map_err(|e| EmbedError::Download(format!("failed to create model dir: {e}")))?;

    for &(local_name, hf_path) in MODEL_FILES {
        let local_path = model_dir.join(local_name);
        if local_path.exists() {
            continue;
        }

        let url = format!("{HF_BASE_URL}/{hf_path}");
        download_file(&url, &local_path, local_name, &progress_cb).await?;
    }

    Ok(())
}

/// Apply mean pooling and L2 normalization to ONNX output.
///
/// Takes the [batch, seq_len, hidden_dim] output array and returns one
/// normalized 384-d vector per sample.
fn postprocess_embeddings(
    output_array: &ndarray::ArrayD<f32>,
    attention_mask: &[i64],
    batch_size: usize,
    max_len: usize,
) -> Vec<Vec<f32>> {
    let output_shape = output_array.shape();
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

        if count > 0.0 {
            for val in &mut sum {
                *val /= count;
            }
        }

        let norm: f32 = sum.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in &mut sum {
                *val /= norm;
            }
        }

        embeddings.push(sum);
    }

    embeddings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ── postprocess_embeddings tests ────────────────────────────────────

    #[test]
    fn postprocess_single_sample_normalizes_to_unit_length() {
        // Create a simple [1, 3, 4] shaped output (batch=1, seq_len=3, hidden=4)
        let data = vec![
            1.0f32, 0.0, 0.0, 0.0, // token 0
            0.0, 1.0, 0.0, 0.0, // token 1
            0.0, 0.0, 1.0, 0.0, // token 2
        ];
        let output = ndarray::ArrayD::from_shape_vec(vec![1, 3, 4], data).unwrap();
        // All tokens are attended
        let attention_mask = vec![1i64, 1, 1];
        let batch_size = 1;
        let max_len = 3;

        let embeddings = postprocess_embeddings(&output, &attention_mask, batch_size, max_len);
        assert_eq!(embeddings.len(), 1);
        assert_eq!(embeddings[0].len(), 4);

        // Mean of [1,0,0,0], [0,1,0,0], [0,0,1,0] = [1/3, 1/3, 1/3, 0]
        // After L2 normalization: each nonzero component = 1/sqrt(3) * (1/3) / (1/3 * sqrt(3))
        let norm: f32 = embeddings[0].iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(
            (norm - 1.0).abs() < 1e-5,
            "embedding should be unit length, got norm={norm}"
        );
    }

    #[test]
    fn postprocess_with_masked_tokens_excludes_padding() {
        // [1, 4, 2] shaped: batch=1, seq_len=4, hidden=2
        let data = vec![
            1.0f32, 0.0, // token 0 (attended)
            0.0, 1.0, // token 1 (attended)
            9.9, 9.9, // token 2 (padding - should be ignored)
            9.9, 9.9, // token 3 (padding - should be ignored)
        ];
        let output = ndarray::ArrayD::from_shape_vec(vec![1, 4, 2], data).unwrap();
        let attention_mask = vec![1i64, 1, 0, 0];
        let batch_size = 1;
        let max_len = 4;

        let embeddings = postprocess_embeddings(&output, &attention_mask, batch_size, max_len);
        assert_eq!(embeddings.len(), 1);

        // Mean of [1,0], [0,1] = [0.5, 0.5], normalized = [1/sqrt(2), 1/sqrt(2)]
        let expected = 1.0f32 / 2.0f32.sqrt();
        assert!((embeddings[0][0] - expected).abs() < 1e-5);
        assert!((embeddings[0][1] - expected).abs() < 1e-5);
    }

    #[test]
    fn postprocess_batch_of_two() {
        // [2, 2, 3] shaped: batch=2, seq_len=2, hidden=3
        let data = vec![
            // Sample 0
            1.0f32, 0.0, 0.0, // token 0
            1.0, 0.0, 0.0, // token 1
            // Sample 1
            0.0, 0.0, 1.0, // token 0
            0.0, 0.0, 1.0, // token 1
        ];
        let output = ndarray::ArrayD::from_shape_vec(vec![2, 2, 3], data).unwrap();
        let attention_mask = vec![1i64, 1, 1, 1]; // all attended
        let batch_size = 2;
        let max_len = 2;

        let embeddings = postprocess_embeddings(&output, &attention_mask, batch_size, max_len);
        assert_eq!(embeddings.len(), 2);

        // Sample 0: mean of [1,0,0],[1,0,0] = [1,0,0], normalized = [1,0,0]
        assert!((embeddings[0][0] - 1.0).abs() < 1e-5);
        assert!(embeddings[0][1].abs() < 1e-5);
        assert!(embeddings[0][2].abs() < 1e-5);

        // Sample 1: mean of [0,0,1],[0,0,1] = [0,0,1], normalized = [0,0,1]
        assert!(embeddings[1][0].abs() < 1e-5);
        assert!(embeddings[1][1].abs() < 1e-5);
        assert!((embeddings[1][2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn postprocess_all_masked_produces_zero_vector() {
        // [1, 2, 3] shaped: batch=1, seq_len=2, hidden=3
        let data = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0];
        let output = ndarray::ArrayD::from_shape_vec(vec![1, 2, 3], data).unwrap();
        let attention_mask = vec![0i64, 0]; // nothing attended
        let batch_size = 1;
        let max_len = 2;

        let embeddings = postprocess_embeddings(&output, &attention_mask, batch_size, max_len);
        assert_eq!(embeddings.len(), 1);
        // All zeros (count=0, norm=0)
        for val in &embeddings[0] {
            assert!(val.abs() < 1e-10);
        }
    }

    // ── Embedder::new error path tests ──────────────────────────────────

    #[test]
    fn embedder_new_missing_model_file_returns_error() {
        let dir = PathBuf::from("nonexistent_model_dir_12345");
        let result = Embedder::new(&dir);
        assert!(result.is_err());
        match result {
            Err(EmbedError::ModelNotFound(_)) => {} // expected
            _ => panic!("expected ModelNotFound error"),
        }
    }

    #[test]
    fn embedder_new_missing_tokenizer_returns_error() {
        // Create a temp dir with model.onnx but no tokenizer.json
        let tmp = std::env::temp_dir().join("orqa_test_embedder_no_tokenizer");
        let _ = std::fs::create_dir_all(&tmp);
        let model_path = tmp.join("model.onnx");
        std::fs::write(&model_path, b"fake onnx").unwrap();

        let result = Embedder::new(&tmp);
        assert!(result.is_err());
        match result {
            Err(EmbedError::ModelNotFound(_)) => {} // expected
            _ => panic!("expected ModelNotFound error"),
        }

        // Cleanup
        let _ = std::fs::remove_dir_all(&tmp);
    }

    // ── EmbedError display tests ────────────────────────────────────────

    #[test]
    fn embed_error_display_messages() {
        let err = EmbedError::ModelNotFound("/path/to/model.onnx".to_string());
        assert_eq!(err.to_string(), "model not found: /path/to/model.onnx");

        let err = EmbedError::Tokenizer("bad token".to_string());
        assert_eq!(err.to_string(), "tokenizer error: bad token");

        let err = EmbedError::Ort("session failed".to_string());
        assert_eq!(err.to_string(), "ONNX runtime error: session failed");

        let err = EmbedError::Download("network error".to_string());
        assert_eq!(err.to_string(), "download error: network error");
    }

    // ── MODEL_FILES constant tests ──────────────────────────────────────

    #[test]
    fn model_files_has_expected_entries() {
        assert_eq!(MODEL_FILES.len(), 2);
        let local_names: Vec<&str> = MODEL_FILES.iter().map(|(name, _)| *name).collect();
        assert!(local_names.contains(&"model.onnx"));
        assert!(local_names.contains(&"tokenizer.json"));
    }
}

/// Download a single file from `url` to `dest`, streaming through a `.part` file.
async fn download_file<F>(
    url: &str,
    dest: &Path,
    display_name: &str,
    progress_cb: &F,
) -> Result<(), EmbedError>
where
    F: Fn(&str, u64, Option<u64>),
{
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.map_err(|e| {
        EmbedError::Download(format!("HTTP request failed for {display_name}: {e}"))
    })?;

    if !response.status().is_success() {
        return Err(EmbedError::Download(format!(
            "HTTP {} downloading {display_name} from {url}",
            response.status()
        )));
    }

    let total_size = response.content_length();
    let part_path = dest.with_extension("part");

    let mut file = tokio::fs::File::create(&part_path).await.map_err(|e| {
        EmbedError::Download(format!("failed to create {}: {e}", part_path.display()))
    })?;

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| {
            EmbedError::Download(format!("download stream error for {display_name}: {e}"))
        })?;
        file.write_all(&chunk)
            .await
            .map_err(|e| EmbedError::Download(format!("write error for {display_name}: {e}")))?;
        downloaded += chunk.len() as u64;
        progress_cb(display_name, downloaded, total_size);
    }

    file.flush()
        .await
        .map_err(|e| EmbedError::Download(format!("flush error for {display_name}: {e}")))?;
    drop(file);

    // Rename .part to final name (atomic on most filesystems)
    tokio::fs::rename(&part_path, dest).await.map_err(|e| {
        EmbedError::Download(format!(
            "failed to rename {} to {}: {e}",
            part_path.display(),
            dest.display()
        ))
    })?;

    Ok(())
}
