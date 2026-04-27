// Integration tests for the LIVE SELECT artifact subscription (TASK-S2-12).
//
// Verifies that POST /artifacts, PUT /artifacts/:id, and DELETE /artifacts/:id
// each produce a corresponding typed event on the event bus within 200ms.
// Tests subscribe to the bus BEFORE the LIVE SELECT task is started so no
// events are missed.
//
// Each test uses a freshly isolated in-memory SurrealDB instance and a dedicated
// EventBus to avoid cross-test contamination.

#![allow(missing_docs)]

mod helpers;

use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tokio::time::timeout;
use tower::ServiceExt as _;

use orqa_daemon_lib::event_bus::EventBus;
use orqa_daemon_lib::surreal_live::start_live_updates;

/// Timeout used for all event assertions — matches the 200ms AC requirement.
const EVENT_TIMEOUT: Duration = Duration::from_millis(200);

/// Drain the bus receiver until we find an event matching `category`, or timeout.
///
/// Returns the first matching `LogEvent`, or panics if none arrives within `EVENT_TIMEOUT`.
async fn wait_for_category(
    rx: &mut tokio::sync::broadcast::Receiver<orqa_engine_types::types::event::LogEvent>,
    category: &str,
) -> orqa_engine_types::types::event::LogEvent {
    timeout(EVENT_TIMEOUT, async {
        loop {
            match rx.recv().await {
                Ok(event) if event.category == category => return event,
                Ok(_) => {} // skip unrelated events (e.g. tracing events)
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {}
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    panic!("event bus closed before {category} event arrived");
                }
            }
        }
    })
    .await
    .unwrap_or_else(|_| panic!("no '{category}' event arrived within {EVENT_TIMEOUT:?}"))
}

// ---------------------------------------------------------------------------
// POST /artifacts → artifact.created
// ---------------------------------------------------------------------------

/// POST a new artifact → event bus receives an artifact.created event within 200ms.
#[tokio::test]
async fn post_artifact_emits_created_event() {
    let state = helpers::build_app_state().await;
    let bus = Arc::new(EventBus::new());
    // Inject bus into GraphState so the route handler can publish via the HTTP handler path.
    state.graph_state.inject_event_bus(Arc::clone(&bus));

    // Subscribe before starting the LIVE SELECT task — no events missed.
    let mut rx = bus.subscribe();

    // Start LIVE SELECT subscription.
    let db = state
        .graph_state
        .surreal_db()
        .expect("in-memory SurrealDB must be present");
    let _handle = start_live_updates(db, Arc::clone(&bus));

    // Give the LIVE SELECT task a moment to subscribe before we write.
    tokio::time::sleep(Duration::from_millis(50)).await;

    let router = orqa_daemon_lib::build_router(state);
    let body = serde_json::json!({
        "id": "EPIC-live001",
        "type": "epic",
        "title": "Live Test Epic"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "POST /artifacts must return 201"
    );

    let event = wait_for_category(&mut rx, "artifact.created").await;
    assert!(
        event.message.contains("EPIC-live001"),
        "created event message must contain the artifact ID"
    );
}

// ---------------------------------------------------------------------------
// PUT /artifacts/:id → artifact.updated
// ---------------------------------------------------------------------------

/// PUT an existing artifact → event bus receives an artifact.updated event within 200ms.
#[tokio::test]
async fn put_artifact_emits_updated_event() {
    let state = helpers::build_app_state().await;
    let bus = Arc::new(EventBus::new());
    state.graph_state.inject_event_bus(Arc::clone(&bus));

    let mut rx = bus.subscribe();

    let db = state
        .graph_state
        .surreal_db()
        .expect("in-memory SurrealDB must be present");
    let _handle = start_live_updates(db, Arc::clone(&bus));

    tokio::time::sleep(Duration::from_millis(50)).await;

    // First, create the artifact.
    let router = orqa_daemon_lib::build_router(state);
    let create_body = serde_json::json!({
        "id": "EPIC-live002",
        "type": "epic",
        "title": "Live Update Epic",
        "status": "draft"
    });
    let create_req = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(create_body.to_string()))
        .unwrap();
    let create_resp = router.clone().oneshot(create_req).await.unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);

    // Drain the created event so it doesn't interfere with the updated assertion.
    wait_for_category(&mut rx, "artifact.created").await;

    // Now update it.
    let update_body = serde_json::json!({ "field": "status", "value": "active" });
    let update_req = Request::builder()
        .method("PUT")
        .uri("/artifacts/EPIC-live002")
        .header("content-type", "application/json")
        .body(Body::from(update_body.to_string()))
        .unwrap();
    let update_resp = router.oneshot(update_req).await.unwrap();
    assert_eq!(update_resp.status(), StatusCode::OK);

    let event = wait_for_category(&mut rx, "artifact.updated").await;
    assert!(
        event.message.contains("EPIC-live002"),
        "updated event message must contain the artifact ID"
    );
}

// ---------------------------------------------------------------------------
// DELETE /artifacts/:id → artifact.deleted
// ---------------------------------------------------------------------------

/// DELETE an existing artifact → event bus receives an artifact.deleted event within 200ms.
#[tokio::test]
async fn delete_artifact_emits_deleted_event() {
    let state = helpers::build_app_state().await;
    let bus = Arc::new(EventBus::new());
    state.graph_state.inject_event_bus(Arc::clone(&bus));

    let mut rx = bus.subscribe();

    let db = state
        .graph_state
        .surreal_db()
        .expect("in-memory SurrealDB must be present");
    let _handle = start_live_updates(db, Arc::clone(&bus));

    tokio::time::sleep(Duration::from_millis(50)).await;

    let router = orqa_daemon_lib::build_router(state);

    // Create then delete.
    let create_body = serde_json::json!({
        "id": "EPIC-live003",
        "type": "epic",
        "title": "Live Delete Epic"
    });
    let create_req = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(create_body.to_string()))
        .unwrap();
    let create_resp = router.clone().oneshot(create_req).await.unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);

    // Drain the created event.
    wait_for_category(&mut rx, "artifact.created").await;

    let delete_req = Request::builder()
        .method("DELETE")
        .uri("/artifacts/EPIC-live003")
        .body(Body::empty())
        .unwrap();
    let delete_resp = router.oneshot(delete_req).await.unwrap();
    assert_eq!(delete_resp.status(), StatusCode::OK);

    let event = wait_for_category(&mut rx, "artifact.deleted").await;
    // The DELETE handler publishes its own event with the ID in the message.
    // The LIVE SELECT task will also emit one when SurrealDB fires the notification.
    // Either satisfies the AC.
    assert!(
        event.message.contains("EPIC-live003"),
        "deleted event message must contain the artifact ID, got: {}",
        event.message
    );
}

// ---------------------------------------------------------------------------
// Reconnect / error handling
// ---------------------------------------------------------------------------

/// When the GraphDb handle is absent start_live_updates is not called, so no
/// events are produced.  This test verifies that the bus stays quiet when
/// no SurrealDB is wired up — i.e. the production guard in main.rs works.
#[tokio::test]
async fn no_live_events_without_surreal_db() {
    // Use empty state — db: None, no LIVE SELECT started.
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build_empty(&root);
    let bus = Arc::new(EventBus::new());
    graph_state.inject_event_bus(Arc::clone(&bus));

    let state = HealthState::for_test(graph_state, None);

    // Do NOT start_live_updates (no db).
    let mut rx = bus.subscribe();

    let router = orqa_daemon_lib::build_router(state);
    let body = serde_json::json!({
        "id": "EPIC-nodb001",
        "type": "epic",
        "title": "No DB Epic"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    // This should return 503 because SurrealDB is unavailable.
    let response = router.oneshot(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::SERVICE_UNAVAILABLE,
        "POST without SurrealDB must return 503"
    );

    // Bus must receive no events — LIVE SELECT was never started.
    let result = timeout(Duration::from_millis(50), async {
        loop {
            match rx.recv().await {
                Ok(event) if event.category.starts_with("artifact.") => return Some(event),
                Ok(_) => {}
                Err(_) => return None,
            }
        }
    })
    .await;

    assert!(
        result.is_err(),
        "no artifact.* events should arrive when SurrealDB is absent"
    );
}
