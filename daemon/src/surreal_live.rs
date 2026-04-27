//! Background task that subscribes to `LIVE SELECT * FROM artifact` and publishes
//! typed artifact change events to the daemon event bus.
//!
//! Each SurrealDB notification (CREATE, UPDATE, DELETE) produces a `LogEvent`
//! with a category that identifies the change type:
//!   - `"artifact.created"` — a new artifact record was inserted
//!   - `"artifact.updated"` — an existing artifact record was updated
//!   - `"artifact.deleted"` — an artifact record was removed
//!
//! The task owns the LIVE SELECT stream and reconnects with exponential backoff
//! (capped at 30s) when the stream ends or an error occurs.  It runs for the
//! lifetime of the daemon; callers retain the `JoinHandle` to join on shutdown.

use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use futures_util::StreamExt as _;
use orqa_engine_types::types::event::{EventLevel, EventSource, EventTier, LogEvent};
use orqa_graph::surreal::GraphDb;
use surrealdb::Notification;
use tokio::time::sleep;
use tracing::{error, info, warn};

use crate::event_bus::EventBus;

/// Maximum backoff delay between reconnect attempts.
const MAX_BACKOFF: Duration = Duration::from_secs(30);
/// Initial backoff delay on the first error.
const INITIAL_BACKOFF: Duration = Duration::from_millis(500);

/// Boxed pinned stream of LIVE SELECT notifications.
type LiveStream = Pin<
    Box<
        dyn futures_util::Stream<Item = Result<Notification<serde_json::Value>, surrealdb::Error>>
            + Send,
    >,
>;

/// Start the background LIVE SELECT subscription task.
///
/// Subscribes to `LIVE SELECT * FROM artifact` on `db` and publishes a `LogEvent`
/// to `bus` for each CREATE, UPDATE, or DELETE notification.  Reconnects with
/// exponential backoff on error or stream end.  Returns a `JoinHandle` that the
/// caller can await for clean shutdown.
pub fn start_live_updates(db: GraphDb, bus: Arc<EventBus>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        run_subscription_loop(db, bus).await;
    })
}

/// Inner loop: subscribe, drain notifications, reconnect on error.
///
/// Never returns — runs until the tokio runtime shuts down the task.
#[allow(clippy::too_many_lines)]
async fn run_subscription_loop(db: GraphDb, bus: Arc<EventBus>) {
    let mut backoff = INITIAL_BACKOFF;

    loop {
        match open_live_stream(&db).await {
            Ok(mut stream) => {
                info!(
                    subsystem = "surreal_live",
                    "[surreal-live] LIVE SELECT subscribed — watching artifact table"
                );
                backoff = INITIAL_BACKOFF;

                loop {
                    match stream.next().await {
                        Some(Ok(notification)) => {
                            handle_notification(&bus, notification);
                        }
                        Some(Err(e)) => {
                            warn!(
                                subsystem = "surreal_live",
                                error = %e,
                                "[surreal-live] notification stream error — reconnecting"
                            );
                            break;
                        }
                        None => {
                            warn!(
                                subsystem = "surreal_live",
                                "[surreal-live] notification stream closed — reconnecting"
                            );
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                error!(
                    subsystem = "surreal_live",
                    error = %e,
                    "[surreal-live] failed to open LIVE SELECT stream — will retry"
                );
            }
        }

        warn!(
            subsystem = "surreal_live",
            backoff_ms = backoff.as_millis(),
            "[surreal-live] backing off for {}ms before reconnect",
            backoff.as_millis()
        );
        sleep(backoff).await;
        backoff = (backoff * 2).min(MAX_BACKOFF);
    }
}

/// Open a LIVE SELECT stream against the artifact table.
///
/// Returns a `Box<Pin<dyn Stream>>` so the caller can store the stream across
/// loop iterations without dealing with lifetime-bound `impl Stream + '_` returns.
async fn open_live_stream(db: &GraphDb) -> Result<LiveStream, surrealdb::Error> {
    let mut response = db.0.query("LIVE SELECT * FROM artifact").await?;
    let stream = response.stream::<Notification<serde_json::Value>>(0)?;
    Ok(Box::pin(stream))
}

/// Classify a SurrealDB notification and publish the corresponding `LogEvent`.
///
/// CREATE → `"artifact.created"`, UPDATE → `"artifact.updated"`,
/// DELETE → `"artifact.deleted"`.  Uses `EventBus::next_ingest_id` to assign a
/// unique monotonic ID and the current wall-clock time as the timestamp.
fn handle_notification(bus: &Arc<EventBus>, notification: Notification<serde_json::Value>) {
    let (category, message) = match notification.action {
        surrealdb::types::Action::Create => {
            let id = extract_id(&notification.data);
            (
                "artifact.created".to_owned(),
                format!("artifact '{id}' created"),
            )
        }
        surrealdb::types::Action::Update => {
            let id = extract_id(&notification.data);
            (
                "artifact.updated".to_owned(),
                format!("artifact '{id}' updated"),
            )
        }
        surrealdb::types::Action::Delete => {
            let id = extract_id(&notification.data);
            (
                "artifact.deleted".to_owned(),
                format!("artifact '{id}' deleted"),
            )
        }
        // Killed means the LIVE query was terminated server-side; no artifact event to emit.
        surrealdb::types::Action::Killed => return,
    };

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);

    let event_id = bus.next_ingest_id();

    // Backpressure: EventBus uses a 10k broadcast ring buffer. When the buffer
    // is full the oldest event drops automatically — this matches the existing
    // daemon event bus semantics and requires no special handling here.
    bus.publish(LogEvent {
        id: event_id,
        timestamp,
        level: EventLevel::Info,
        source: EventSource::Daemon,
        tier: EventTier::default(),
        category,
        message,
        metadata: notification.data.clone(),
        session_id: None,
        fingerprint: None,
        message_template: None,
        correlation_id: None,
        stack_frames: None,
    });
}

/// Extract the artifact string ID from a SurrealDB notification payload.
///
/// Looks for an `"id"` field in the JSON object.  Falls back to `"<unknown>"` if
/// the field is absent or the payload is not an object, which can happen when
/// SurrealDB returns a partial record for a DELETE notification.
fn extract_id(data: &serde_json::Value) -> String {
    data.get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("<unknown>")
        .to_owned()
}
