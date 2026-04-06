// Axum middleware for correlation ID propagation.
//
// Reads `x-request-id` from the incoming request headers. If no header is
// present a new correlation ID is generated via `new_correlation_id`. The ID
// is then set on the task-local context for the duration of the request so
// every log event published inside a route handler carries the same ID. The
// same ID is written back to the response as `x-request-id` so callers can
// correlate their own logs with daemon logs.

use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;

use crate::correlation::{new_correlation_id, with_correlation_id};

/// Axum middleware layer that extracts or generates a correlation ID for each
/// request, runs the handler inside a `with_correlation_id` scope, and then
/// mirrors the ID back in the response headers.
///
/// Install via `axum::middleware::from_fn(correlation_id_middleware)` on the
/// router after all route definitions.
pub async fn correlation_id_middleware(req: Request, next: Next) -> Response {
    // Use the caller-supplied ID if present; otherwise generate a fresh one.
    let id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map_or_else(new_correlation_id, ToOwned::to_owned);

    // Run the handler inside the correlation scope so every log event emitted
    // during this request sees the same ID via `current_correlation_id`.
    let mut response = with_correlation_id(id.clone(), next.run(req)).await;

    // Echo the ID back so HTTP clients can correlate their own traces.
    if let Ok(header_val) = HeaderValue::from_str(&id) {
        response.headers_mut().insert("x-request-id", header_val);
    }

    response
}
