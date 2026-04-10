// Error type for the orqa-db HTTP client.
//
// Three failure modes: network transport errors from reqwest, structured HTTP
// error responses from the daemon (non-2xx with JSON body), and deserialization
// failures when the response body cannot be parsed into the expected type.

/// Errors that can occur when calling the daemon storage API.
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    /// A network-level error from reqwest (connection refused, timeout, etc.).
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    /// The daemon returned a non-2xx HTTP status with a structured error body.
    ///
    /// The `code` field is the machine-readable error code from the daemon's JSON
    /// response body (`{ "error": "...", "code": "..." }`).
    #[error("http {status}: {code} — {error}")]
    Http {
        /// HTTP status code.
        status: u16,
        /// Machine-readable error code from the daemon response body.
        code: String,
        /// Human-readable error message from the daemon response body.
        error: String,
    },

    /// The response body could not be deserialized into the expected type.
    #[error("deserialization error: {0}")]
    Deserialization(String),
}
