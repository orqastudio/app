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

use orqa_engine_types::types::event::{EventLevel, EventSource, LogEvent};
use tokio::sync::broadcast;

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
    /// incremented.
    ///
    /// This function MUST NOT call any tracing macros (info!, warn!, etc.).
    /// The `EventBusLayer` in the tracing subscriber calls this function, so
    /// any tracing emission here would cause infinite recursion:
    ///   tracing event → EventBusLayer::on_event → publish → tracing warn →
    ///   EventBusLayer::on_event → publish → ...
    ///
    /// Dropped-event counts are exposed via `EventBusStats` on GET /health
    /// so operators can detect saturation without any tracing calls here.
    pub fn publish(&self, event: LogEvent) {
        match self.sender.send(event) {
            Ok(_) => {
                self.total_published.fetch_add(1, Ordering::Relaxed);
            }
            Err(_) => {
                // No subscribers or all subscribers have been dropped.
                // Increment the dropped counter so /health stats reflect this.
                // Do NOT emit a tracing event here — that would recurse.
                self.total_dropped.fetch_add(1, Ordering::Relaxed);
            }
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
            fingerprint: None,
            message_template: None,
            correlation_id: None,
            stack_frames: None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_engine_types::types::event::{EventLevel, EventSource, LogEvent};

    /// Build a minimal LogEvent for use in tests.
    fn make_event(id: u64, message: &str) -> LogEvent {
        LogEvent {
            id,
            timestamp: 1_000_000 + id as i64,
            level: EventLevel::Info,
            source: EventSource::Daemon,
            category: "test".to_owned(),
            message: message.to_owned(),
            metadata: serde_json::Value::Null,
            session_id: None,
            fingerprint: None,
            message_template: None,
            correlation_id: None,
            stack_frames: None,
        }
    }

    /// A fresh bus has zero published and zero dropped events.
    #[test]
    fn new_bus_has_clean_stats() {
        let bus = EventBus::new();
        let stats = bus.stats();
        assert_eq!(stats.total_published, 0, "published should start at 0");
        assert_eq!(stats.total_dropped, 0, "dropped should start at 0");
        assert_eq!(stats.subscriber_count, 0, "no subscribers at construction");
    }

    /// Publishing when a subscriber exists delivers the event with correct fields.
    #[tokio::test]
    async fn publish_with_subscriber_delivers_event() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe();
        let event = make_event(1, "hello");

        bus.publish(event.clone());

        let received = rx.try_recv().expect("subscriber should receive the event");
        assert_eq!(received.id, 1);
        assert_eq!(received.message, "hello");
        assert_eq!(received.source, EventSource::Daemon);

        let stats = bus.stats();
        assert_eq!(stats.total_published, 1);
        assert_eq!(stats.total_dropped, 0);
    }

    /// Publishing when no subscribers exist increments the drop counter.
    #[test]
    fn publish_without_subscribers_increments_dropped() {
        let bus = EventBus::new();
        // No subscriber — no rx created.
        bus.publish(make_event(1, "dropped"));

        let stats = bus.stats();
        assert_eq!(
            stats.total_published, 0,
            "no subscriber means nothing was sent"
        );
        assert_eq!(stats.total_dropped, 1, "drop counter must increment");
    }

    /// Stats correctly reflect both published and dropped counts across
    /// a sequence of mixed operations.
    #[tokio::test]
    async fn stats_accurately_track_published_and_dropped() {
        let bus = EventBus::new();

        // Publish 3 events to a live subscriber.
        let rx = bus.subscribe();
        for i in 0..3 {
            bus.publish(make_event(i, "live"));
        }

        // Drop the subscriber, then publish 2 more events — both should drop.
        drop(rx);
        for i in 3..5 {
            bus.publish(make_event(i, "after drop"));
        }

        let stats = bus.stats();
        assert_eq!(stats.total_published, 3);
        assert_eq!(stats.total_dropped, 2);
    }

    /// next_ingest_id returns strictly increasing values across calls.
    #[test]
    fn next_ingest_id_is_monotonically_increasing() {
        let bus = EventBus::new();
        let id1 = bus.next_ingest_id();
        let id2 = bus.next_ingest_id();
        let id3 = bus.next_ingest_id();
        assert!(id1 < id2, "id2 must be greater than id1");
        assert!(id2 < id3, "id3 must be greater than id2");
    }

    /// Multiple subscribers all receive the same published event.
    #[tokio::test]
    async fn multiple_subscribers_each_receive_event() {
        let bus = EventBus::new();
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        let mut rx3 = bus.subscribe();

        bus.publish(make_event(42, "broadcast"));

        let e1 = rx1.try_recv().expect("subscriber 1 should receive");
        let e2 = rx2.try_recv().expect("subscriber 2 should receive");
        let e3 = rx3.try_recv().expect("subscriber 3 should receive");

        // All three must see the same event identity and content.
        assert_eq!(e1.id, 42);
        assert_eq!(e2.id, 42);
        assert_eq!(e3.id, 42);
        assert_eq!(e1.message, "broadcast");
        assert_eq!(e2.message, "broadcast");
        assert_eq!(e3.message, "broadcast");

        assert_eq!(
            bus.stats().total_published,
            1,
            "counted once, not per subscriber"
        );
    }

    /// A subscriber created AFTER a publish does not receive the earlier event.
    /// The bus is not a replay log — subscribers only get events published after
    /// they subscribe.
    #[tokio::test]
    async fn late_subscriber_does_not_replay_past_events() {
        let bus = EventBus::new();

        // Publish before any subscriber exists.
        bus.publish(make_event(1, "before subscriber"));

        // Subscribe after the publish.
        let mut rx = bus.subscribe();

        // The subscriber queue must be empty.
        assert!(
            rx.try_recv().is_err(),
            "late subscriber must not receive events published before it subscribed"
        );
    }

    /// shutdown sends a sentinel event that subscribers can detect as the bus
    /// terminator. The sentinel uses id=u64::MAX and category="bus".
    #[tokio::test]
    async fn shutdown_sends_terminator_to_subscribers() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe();

        bus.shutdown();

        let event = rx
            .try_recv()
            .expect("shutdown must deliver a terminator event");
        assert_eq!(
            event.id,
            u64::MAX,
            "shutdown event must use id=u64::MAX sentinel"
        );
        assert_eq!(event.category, "bus");
        assert!(
            event.message.contains("shutting down"),
            "shutdown message must describe intent"
        );
    }
}
