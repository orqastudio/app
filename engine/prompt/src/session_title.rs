//! Session title generation for the orqa-engine crate.
//!
//! Provides the engine-level logic for generating a session title from
//! a set of message summaries. The actual LLM call is delegated to a
//! `SidecarClient` implementation provided by the access layer, keeping
//! the engine free of Tauri and process-management concerns.

use orqa_engine_types::traits::sidecar::SidecarClient;

/// Generate a session title from a list of message summaries.
///
/// Delegates LLM inference to the provided `SidecarClient`. The `messages`
/// slice should contain serialised role/content pairs from the current session.
/// Returns the trimmed title string on success, or an error if the sidecar
/// is unavailable or returns an unexpected response.
///
/// Returns `None` when the generated title is empty after trimming (the caller
/// should treat this as "no title generated" and skip the update).
pub async fn generate_session_title(
    client: &dyn SidecarClient,
    messages: &[String],
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let raw = client.generate_summary(messages).await?;
    let title = raw.trim().to_owned();
    if title.is_empty() {
        Ok(None)
    } else {
        Ok(Some(title))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stub sidecar that returns a fixed summary.
    struct StubSidecar {
        summary: String,
    }

    #[async_trait::async_trait]
    impl SidecarClient for StubSidecar {
        async fn generate_summary(
            &self,
            _messages: &[String],
        ) -> Result<String, Box<dyn std::error::Error>> {
            Ok(self.summary.clone())
        }
    }

    /// Stub sidecar that returns an empty summary.
    struct EmptySidecar;

    #[async_trait::async_trait]
    impl SidecarClient for EmptySidecar {
        async fn generate_summary(
            &self,
            _messages: &[String],
        ) -> Result<String, Box<dyn std::error::Error>> {
            Ok(String::new())
        }
    }

    /// Stub sidecar that always returns an error.
    struct ErrorSidecar;

    #[async_trait::async_trait]
    impl SidecarClient for ErrorSidecar {
        async fn generate_summary(
            &self,
            _messages: &[String],
        ) -> Result<String, Box<dyn std::error::Error>> {
            Err("sidecar unavailable".into())
        }
    }

    #[tokio::test]
    async fn returns_title_when_summary_non_empty() {
        let client = StubSidecar {
            summary: "  Discussing Rust ownership  ".to_owned(),
        };
        let result = generate_session_title(&client, &["user: hello".to_owned()])
            .await
            .expect("should succeed");
        assert_eq!(result, Some("Discussing Rust ownership".to_owned()));
    }

    #[tokio::test]
    async fn returns_none_when_summary_empty() {
        let client = EmptySidecar;
        let result = generate_session_title(&client, &["user: hello".to_owned()])
            .await
            .expect("should succeed");
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn propagates_sidecar_error() {
        let client = ErrorSidecar;
        let result = generate_session_title(&client, &["user: hello".to_owned()]).await;
        assert!(result.is_err());
    }
}
