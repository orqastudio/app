// Event emission and sidecar transport traits for the orqa-engine crate.
//
// Defines abstract interfaces for sending events to the frontend/consumer
// and for communicating with a sidecar process. The app implements these
// with Tauri-specific Channel<T> and sidecar manager; other access layers
// can implement them without Tauri.

/// Abstraction for sending streaming events to the frontend or event consumer.
///
/// Engine logic that needs to emit events (stream progress, errors) depends
/// on this trait rather than on Tauri's `Channel<T>`, keeping the engine
/// free of Tauri and IPC concerns.
pub trait EventEmitter: Send + Sync {
    /// Emit a named streaming event with a JSON payload string.
    ///
    /// The `event` parameter is the event discriminator (e.g. "text_delta").
    /// The `payload` parameter is the serialized event data. Returns an error
    /// if the event cannot be delivered to the consumer.
    fn emit_stream_event(
        &self,
        event: &str,
        payload: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Emit a terminal error event.
    ///
    /// Used to signal fatal streaming errors to the consumer. Returns an error
    /// if the notification cannot be delivered.
    fn emit_error(&self, error: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Abstraction for sending messages to and checking the status of a sidecar process.
///
/// The sidecar provides LLM inference. Engine logic that needs to communicate
/// with it (e.g. sending tool results back) depends on this trait rather than
/// on the concrete sidecar manager, keeping the engine free of process
/// management concerns.
pub trait SidecarTransport: Send + Sync {
    /// Send a JSON message to the sidecar over its stdin pipe.
    ///
    /// The `message` parameter is a serialized NDJSON line. Returns an error
    /// if the sidecar is not running or the write fails.
    fn send_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Return `true` if the sidecar process is currently connected and running.
    fn is_connected(&self) -> bool;
}
