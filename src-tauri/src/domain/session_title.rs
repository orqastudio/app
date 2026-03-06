use crate::domain::provider_event::StreamEvent;
use crate::sidecar::types::{SidecarRequest, SidecarResponse};
use crate::state::AppState;

/// Attempt to auto-generate a session title after the first successful turn.
///
/// Sends a `GenerateSummary` request to the sidecar and reads back the
/// `SummaryResult`. Only generates when the session has no manually-set title
/// and has no existing title. Emits `SessionTitleUpdated` on success.
pub fn maybe_auto_title(
    state: &tauri::State<'_, AppState>,
    session_id: i64,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) {
    use crate::repo::{message_repo, session_repo};

    let needs_title = match state.db.lock() {
        Ok(db) => match session_repo::get(&db, session_id) {
            Ok(session) => session.title.is_none() && !session.title_manually_set,
            Err(_) => false,
        },
        Err(_) => false,
    };

    if !needs_title {
        return;
    }

    let messages = match state.db.lock() {
        Ok(db) => match message_repo::list(&db, session_id, 10, 0) {
            Ok(msgs) => msgs,
            Err(_) => return,
        },
        Err(_) => return,
    };

    use crate::domain::message::MessageRole;
    let message_summaries: Vec<crate::sidecar::types::MessageSummary> = messages
        .iter()
        .filter_map(|m| {
            m.content
                .as_ref()
                .map(|c| crate::sidecar::types::MessageSummary {
                    role: match m.role {
                        MessageRole::User => "user".to_string(),
                        MessageRole::Assistant => "assistant".to_string(),
                        MessageRole::System => "system".to_string(),
                    },
                    content: c.clone(),
                })
        })
        .collect();

    if message_summaries.is_empty() {
        return;
    }

    let request = SidecarRequest::GenerateSummary {
        session_id,
        messages: message_summaries,
    };
    if state.sidecar.send(&request).is_err() {
        tracing::warn!("[stream] failed to send GenerateSummary request");
        return;
    }

    match state.sidecar.read_line() {
        Ok(Some(SidecarResponse::SummaryResult { summary, .. })) => {
            let title = summary.trim().to_string();
            if title.is_empty() {
                return;
            }
            if let Ok(db) = state.db.lock() {
                if let Ok(true) = session_repo::auto_update_title(&db, session_id, &title) {
                    let _ = on_event.send(StreamEvent::SessionTitleUpdated { session_id, title });
                }
            }
        }
        Ok(Some(other)) => {
            tracing::warn!(
                "[stream] unexpected response after GenerateSummary: {:?}",
                other
            );
        }
        Ok(None) => {
            tracing::warn!("[stream] sidecar EOF during GenerateSummary");
        }
        Err(e) => {
            tracing::warn!("[stream] failed to read GenerateSummary response: {e}");
        }
    }
}
