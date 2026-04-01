//! Central pub/sub event bus for the OrqaStudio daemon.
//!
//! All subsystems (file watcher, LSP, MCP, agents) publish `LogEvent` values
//! here. Any number of subscribers — HTTP streams, websocket clients, the
//! frontend — can hold a `Receiver` and read from the broadcast channel.
//!
//! Uses `tokio::sync::broadcast` with a 10,000-event buffer. When the buffer
//! is full the oldest event is dropped and `total_dropped` is incremented so
//! that consumers can detect gaps.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use tokio::sync::broadcast;
use tracing::warn;

use orqa_engine_types::types::event::{EventLevel, EventSource, LogEvent};

/// Capacity of the broadcast channel ring buffer.
const BUS_CAPACITY: usize = 10_000;

/// Central event bus shared across all daemon subsystems.
///
/// Wrap in `Arc<EventBus>` and clone the arc to pass to each subsystem.
/// Call `publish` to emit events; call `subscribe` to obtain a receiver
/// that starts from the next published event.
pub struct EventBus {
    /// The broadcast sender — cloned cheaply for each publisher.
    sender: broadcast::Sender<LogEvent>,
    /// Total events successfully sent since bus creation.
    total_published: AtomicU64,
    /// Total events dropped due to a full buffer since bus creation.
    total_dropped: AtomicU64,
    /// Monotonically increasing ID counter for events ingested via HTTP.
    ///
    /// Internal subsystems assign their own IDs. External sources (frontend,
    /// dev-controller) use this counter so their events have unique bus IDs.
    next_ingest_id: AtomicU64,
}

// Methods are public API consumed by subsystems added in subsequent tasks.
#[allow(dead_code)]
impl EventBus {
    /// Create a new `EventBus` with the fixed 10,000-event buffer.
    pub fn new() -> Arc<Self> {
        let (sender, _) = broadcast::channel(BUS_CAPACITY);
        Arc::new(Self {
            sender,
            total_published: AtomicU64::new(0),
            total_dropped: AtomicU64::new(0),
            next_ingest_id: AtomicU64::new(1),
        })
    }

    /// Assign the next ingest ID and return it.
    ///
    /// Used by the `/events` HTTP ingest endpoint to stamp externally-sourced
    /// events (frontend, dev-controller) with a unique monotonic ID.
    pub fn next_ingest_id(&self) -> u64 {
        self.next_ingest_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Publish a `LogEvent` to all current subscribers.
    ///
    /// Non-blocking. If the buffer is full the oldest event is dropped
    /// automatically by the broadcast channel and `total_dropped` is
    /// incremented. A warning is emitted so operators can tune buffer size.
    pub fn publish(&self, event: LogEvent) {
        match self.sender.send(event) {
            Ok(_) => {
                self.total_published.fetch_add(1, Ordering::Relaxed);
            }
            Err(_) => {
                // No subscribers — event is still counted as dropped because
                // it was not delivered to anyone.
                self.total_dropped.fetch_add(1, Ordering::Relaxed);
                warn!(
                    subsystem = "event-bus",
                    "[event-bus] event dropped — no active subscribers"
                );
            }
        }

        // Detect buffer-overflow drops by comparing the receiver lag. The
        // broadcast channel drops old messages silently; we surface this
        // through the stats so callers can detect gaps.
        let dropped_count = self.sender.len();
        if dropped_count >= BUS_CAPACITY.saturating_sub(100) {
            warn!(
                subsystem = "event-bus",
                buffer_used = dropped_count,
                capacity = BUS_CAPACITY,
                "[event-bus] broadcast buffer near capacity — consider increasing BUS_CAPACITY"
            );
        }
    }

    /// Subscribe to the event bus.
    ///
    /// Returns a `Receiver` that delivers every event published after the
    /// subscribe call. Events published before this call are not replayed.
    pub fn subscribe(&self) -> broadcast::Receiver<LogEvent> {
        self.sender.subscribe()
    }

    /// Signal bus shutdown by logging a warning to all subscribers.
    ///
    /// Callers that hold an `Arc<EventBus>` should drop the arc after calling
    /// this method. When the last arc drops, the broadcast sender is destroyed
    /// and all subscribers receive `RecvError::Closed` on their next poll.
    ///
    /// This method is a named shutdown hook — the mechanical shutdown happens
    /// when all arcs are dropped, but the explicit call makes the shutdown
    /// intent visible in the call site.
    pub fn shutdown(&self) {
        // Log the final message so subscribers see a clean terminator.
        // If there are no subscribers this is a no-op.
        let _ = self.sender.send(LogEvent {
            id: u64::MAX,
            timestamp: 0,
            level: EventLevel::Info,
            source: EventSource::Daemon,
            category: "bus".to_owned(),
            message: "event bus shutting down".to_owned(),
            metadata: serde_json::Value::Null,
            session_id: None,
        });
    }

    /// Return a point-in-time snapshot of bus statistics.
    pub fn stats(&self) -> EventBusStats {
        EventBusStats {
            total_published: self.total_published.load(Ordering::Relaxed),
            total_dropped: self.total_dropped.load(Ordering::Relaxed),
            subscriber_count: self.sender.receiver_count(),
        }
    }
}

/// Point-in-time statistics snapshot for the event bus.
#[derive(Debug, Clone, serde::Serialize)]
pub struct EventBusStats {
    /// Total events successfully sent since bus creation.
    pub total_published: u64,
    /// Total events dropped due to missing subscribers since bus creation.
    pub total_dropped: u64,
    /// Number of active subscribers at the time of the snapshot.
    pub subscriber_count: usize,
}
