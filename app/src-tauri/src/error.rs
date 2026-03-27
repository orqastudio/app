use serde::Serialize;

/// Canonical error type for all OrqaStudio IPC commands.
///
/// Serialized as `{"code": "<variant>", "message": "<detail>"}` for the frontend.
/// The `Serialize` derive enables automatic conversion to `tauri::ipc::InvokeError`
/// via Tauri's blanket `impl<T: Serialize> From<T> for InvokeError`.
#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum OrqaError {
    #[error("not found: {0}")]
    #[serde(rename = "not_found")]
    NotFound(String),

    #[error("database error: {0}")]
    #[serde(rename = "database")]
    Database(String),

    #[error("file system error: {0}")]
    #[serde(rename = "file_system")]
    FileSystem(String),

    #[error("sidecar error: {0}")]
    #[serde(rename = "sidecar")]
    Sidecar(String),

    #[error("validation error: {0}")]
    #[serde(rename = "validation")]
    Validation(String),

    #[error("scan error: {0}")]
    #[serde(rename = "scan")]
    Scan(String),

    #[error("serialization error: {0}")]
    #[serde(rename = "serialization")]
    Serialization(String),

    #[error("permission denied: {0}")]
    #[serde(rename = "permission_denied")]
    PermissionDenied(String),

    #[error("search error: {0}")]
    #[serde(rename = "search")]
    Search(String),

    #[error("plugin error: {0}")]
    #[serde(rename = "plugin")]
    Plugin(String),
}

impl From<std::io::Error> for OrqaError {
    fn from(err: std::io::Error) -> Self {
        Self::FileSystem(err.to_string())
    }
}

impl From<serde_json::Error> for OrqaError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

/// Convert an engine-level error into an app-level error.
///
/// EngineError wraps the same underlying I/O and serialization errors that
/// OrqaError already handles, so we forward to the matching variant.
impl From<orqa_engine::error::EngineError> for OrqaError {
    fn from(err: orqa_engine::error::EngineError) -> Self {
        match err {
            orqa_engine::error::EngineError::FileSystem(e) => Self::FileSystem(e.to_string()),
            orqa_engine::error::EngineError::Serialization(e) => Self::Serialization(e.to_string()),
        }
    }
}

impl From<rusqlite::Error> for OrqaError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<orqa_search::SearchError> for OrqaError {
    fn from(err: orqa_search::SearchError) -> Self {
        Self::Search(err.to_string())
    }
}

impl From<orqa_search::store::StoreError> for OrqaError {
    fn from(err: orqa_search::store::StoreError) -> Self {
        Self::Search(err.to_string())
    }
}

impl From<orqa_search::embedder::EmbedError> for OrqaError {
    fn from(err: orqa_search::embedder::EmbedError) -> Self {
        Self::Search(err.to_string())
    }
}

impl From<orqa_search::chunker::ChunkError> for OrqaError {
    fn from(err: orqa_search::chunker::ChunkError) -> Self {
        Self::Search(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_not_found() {
        let err = OrqaError::NotFound("project 42".to_string());
        let json = serde_json::to_value(&err).expect("serialization should succeed");
        assert_eq!(json["code"], "not_found");
        assert_eq!(json["message"], "project 42");
    }

    #[test]
    fn serialize_database() {
        let err = OrqaError::Database("connection refused".to_string());
        let json = serde_json::to_value(&err).expect("serialization should succeed");
        assert_eq!(json["code"], "database");
        assert_eq!(json["message"], "connection refused");
    }

    #[test]
    fn serialize_file_system() {
        let err = OrqaError::FileSystem("no such file".to_string());
        let json = serde_json::to_value(&err).expect("serialization should succeed");
        assert_eq!(json["code"], "file_system");
        assert_eq!(json["message"], "no such file");
    }

    #[test]
    fn serialize_sidecar() {
        let err = OrqaError::Sidecar("process exited".to_string());
        let json = serde_json::to_value(&err).expect("serialization should succeed");
        assert_eq!(json["code"], "sidecar");
        assert_eq!(json["message"], "process exited");
    }

    #[test]
    fn serialize_validation() {
        let err = OrqaError::Validation("name is empty".to_string());
        let json = serde_json::to_value(&err).expect("serialization should succeed");
        assert_eq!(json["code"], "validation");
        assert_eq!(json["message"], "name is empty");
    }

    #[test]
    fn serialize_scan() {
        let err = OrqaError::Scan("tier 2 failed".to_string());
        let json = serde_json::to_value(&err).expect("serialization should succeed");
        assert_eq!(json["code"], "scan");
        assert_eq!(json["message"], "tier 2 failed");
    }

    #[test]
    fn serialize_serialization() {
        let err = OrqaError::Serialization("invalid utf-8".to_string());
        let json = serde_json::to_value(&err).expect("serialization should succeed");
        assert_eq!(json["code"], "serialization");
        assert_eq!(json["message"], "invalid utf-8");
    }

    #[test]
    fn serialize_permission_denied() {
        let err = OrqaError::PermissionDenied("path outside scope".to_string());
        let json = serde_json::to_value(&err).expect("serialization should succeed");
        assert_eq!(json["code"], "permission_denied");
        assert_eq!(json["message"], "path outside scope");
    }

    #[test]
    fn display_uses_thiserror_format() {
        let err = OrqaError::NotFound("session 99".to_string());
        assert_eq!(err.to_string(), "not found: session 99");

        let err = OrqaError::Database("timeout".to_string());
        assert_eq!(err.to_string(), "database error: timeout");
    }

    #[test]
    fn from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing file");
        let orqa_err = OrqaError::from(io_err);
        assert!(matches!(orqa_err, OrqaError::FileSystem(_)));
    }

    #[test]
    fn from_serde_json_error() {
        let json_err =
            serde_json::from_str::<serde_json::Value>("{{bad}}").expect_err("should fail to parse");
        let orqa_err = OrqaError::from(json_err);
        assert!(matches!(orqa_err, OrqaError::Serialization(_)));
    }

    #[test]
    fn all_variants_serialize_as_tagged_json() {
        let variants: Vec<OrqaError> = vec![
            OrqaError::NotFound("a".into()),
            OrqaError::Database("b".into()),
            OrqaError::FileSystem("c".into()),
            OrqaError::Sidecar("d".into()),
            OrqaError::Validation("e".into()),
            OrqaError::Scan("f".into()),
            OrqaError::Serialization("g".into()),
            OrqaError::PermissionDenied("h".into()),
            OrqaError::Search("i".into()),
            OrqaError::Plugin("j".into()),
        ];

        let expected_codes = [
            "not_found",
            "database",
            "file_system",
            "sidecar",
            "validation",
            "scan",
            "serialization",
            "permission_denied",
            "search",
            "plugin",
        ];

        for (variant, expected_code) in variants.iter().zip(expected_codes.iter()) {
            let json = serde_json::to_value(variant).expect("serialization should succeed");
            assert_eq!(
                json["code"].as_str(),
                Some(*expected_code),
                "variant {:?} should serialize with code {:?}",
                variant,
                expected_code
            );
            assert!(
                json["message"].is_string(),
                "variant {:?} should have a string message",
                variant
            );
        }
    }
}
