//! Daemon-side issue group consumer.
//!
//! Subscribes to the central `EventBus` and, for every event carrying a
//! fingerprint, upserts the corresponding row in the `issue_groups` table via
//! the storage layer.  After each successful upsert the updated `IssueGroup`
//! is re-broadcast on a dedicated channel so SSE subscribers (OrqaDev devtools)
//! can render live updates in their reactive UI.
//!
//! This replaces the previous architecture where devtools fired one HTTP
//! `POST /issue-groups/upsert` per received event.  That pattern produced a
//! feedback loop:
//!
//!   1. Daemon emits a tracing event → `EventBusLayer` publishes to bus.
//!   2. Devtools SSE consumer receives the event → spawns an HTTP upsert task.
//!   3. Daemon handles the upsert → storage layer emits more tracing events.
//!   4. Those events flow back through step 1 → unbounded amplification.
//!
//! Moving the upsert into the daemon removes the HTTP round-trip entirely and
//! keeps issue-group computation co-located with the event source.

use std::sync::Arc;

use orqa_engine_types::types::event::{EventLevel, LogEvent};
use orqa_storage::repo::issue_groups::IssueGroup;
use orqa_storage::traits::IssueGroupRepository as _;
use orqa_storage::Storage;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use crate::event_bus::EventBus;

/// Capacity of the `IssueGroup` update broadcast channel.
///
/// Smaller than the main event bus because issue-group updates are bundled
/// per-fingerprint and therefore far less frequent than raw log events.
const UPDATE_BUS_CAPACITY: usize = 1024;

/// Spawn the background consumer that drains fingerprinted events from `bus`
/// and upserts them into `storage.issue_groups()`.
///
/// Returns a `broadcast::Sender<IssueGroup>` that SSE route handlers can
/// `subscribe()` to in order to stream updated groups to connected clients.
///
/// The task exits automatically when the event bus sender is dropped at
/// daemon shutdown.
pub fn spawn(bus: Arc<EventBus>, storage: Arc<Storage>) -> broadcast::Sender<IssueGroup> {
    let (update_tx, _) = broadcast::channel::<IssueGroup>(UPDATE_BUS_CAPACITY);
    let update_tx_task = update_tx.clone();

    tokio::spawn(async move {
        let mut rx = bus.subscribe();
        loop {
            match rx.recv().await {
                Ok(event) => {
                    if !should_track(&event) {
                        continue;
                    }
                    if let Some(group) = upsert_event(&storage, &event).await {
                        // Ignore send errors — zero subscribers just means no
                        // SSE clients are connected, which is fine.
                        let _ = update_tx_task.send(group);
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    warn!(
                        subsystem = "issue-group-consumer",
                        dropped = n,
                        "[issue-group-consumer] event bus lagged — {n} events missed"
                    );
                }
                Err(broadcast::error::RecvError::Closed) => {
                    info!(
                        subsystem = "issue-group-consumer",
                        "[issue-group-consumer] event bus closed — consumer exiting"
                    );
                    break;
                }
            }
        }
    });

    update_tx
}

/// Determine whether a log event should contribute to an issue group.
///
/// Only events with a fingerprint at WARN or ERROR severity are tracked.
/// Info/Debug/Perf events are noise for the "recurring issues" view and
/// would otherwise flood the table.
///
/// Events from subsystems on the upsert path are explicitly skipped to prevent
/// feedback loops: if the consumer or the storage layer emit an error while
/// processing an upsert, that error must not itself be upserted — it would
/// recurse on every failure.
fn should_track(event: &LogEvent) -> bool {
    if event.fingerprint.is_none() {
        return false;
    }
    if !matches!(event.level, EventLevel::Warn | EventLevel::Error) {
        return false;
    }
    // Skip categories that are emitted by the upsert path itself.  Any event
    // published here would trigger another upsert, which could fail and emit
    // the same event again — an infinite loop when the database is unhealthy.
    const SKIP_CATEGORIES: &[&str] = &["issue-group-consumer", "storage"];
    if SKIP_CATEGORIES.contains(&event.category.as_str()) {
        return false;
    }
    true
}

/// Upsert a single event into the issue groups table and return the updated
/// group on success.
///
/// Errors are logged but never returned — a failed upsert must not crash the
/// consumer task.  Returns `None` when the upsert fails or when the row cannot
/// be fetched back for broadcast.
async fn upsert_event(storage: &Arc<Storage>, event: &LogEvent) -> Option<IssueGroup> {
    let fingerprint = event.fingerprint.as_deref()?;
    let title = event
        .message_template
        .as_deref()
        .unwrap_or(&event.message)
        .to_owned();
    let component = event.source.to_string();
    let level = event.level.to_string();

    if let Err(e) = storage
        .issue_groups()
        .upsert(
            fingerprint,
            &title,
            &component,
            &level,
            event.timestamp,
            event.id,
        )
        .await
    {
        error!(
            subsystem = "issue-group-consumer",
            error = %e,
            fingerprint = %fingerprint,
            "[issue-group-consumer] upsert failed"
        );
        return None;
    }

    match storage.issue_groups().get(fingerprint).await {
        Ok(Some(group)) => Some(group),
        Ok(None) => {
            error!(
                subsystem = "issue-group-consumer",
                fingerprint = %fingerprint,
                "[issue-group-consumer] group missing after upsert"
            );
            None
        }
        Err(e) => {
            error!(
                subsystem = "issue-group-consumer",
                error = %e,
                fingerprint = %fingerprint,
                "[issue-group-consumer] get after upsert failed"
            );
            None
        }
    }
}
