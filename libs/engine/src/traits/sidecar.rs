// Sidecar communication trait for the orqa-engine crate.
//
// Defines the abstract interface for communicating with a sidecar process.
// The engine uses this trait to request LLM-driven operations (such as summary
// generation) without coupling to a specific sidecar implementation. The app
// layer provides a concrete implementation backed by the sidecar manager.

/// Abstract interface for communicating with a sidecar process.
///
/// The sidecar provides LLM inference to the app. Engine logic that needs
/// LLM-driven output (such as session title generation) depends on this trait
/// rather than on any concrete sidecar implementation, keeping the engine
/// free of Tauri and process-management concerns.
#[async_trait::async_trait]
pub trait SidecarClient: Send + Sync {
    /// Request a short summary from the sidecar given a list of message strings.
    ///
    /// `messages` is a slice of text messages (e.g. serialised role/content pairs)
    /// to summarise. Returns the generated summary string on success, or a boxed
    /// error if the sidecar is unavailable or returns an unexpected response.
    async fn generate_summary(
        &self,
        messages: &[String],
    ) -> Result<String, Box<dyn std::error::Error>>;
}
