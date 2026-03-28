// System prompt utilities for the OrqaStudio app layer.
//
// Re-exports the filesystem-based prompt building functions from `orqa_engine::prompt`
// and provides app-layer utilities that require AppState (DB access, session messages).
// Pure prompt-building logic lives in the engine; app-specific context loading lives here.

pub use orqa_engine::prompt::{
    build_system_prompt, list_knowledge_catalog, read_governance_file, read_rules,
    resolve_system_prompt,
};

use crate::error::OrqaError;
use crate::state::AppState;

/// A condensed message record used for context injection into the system prompt.
#[derive(serde::Serialize)]
pub struct ContextMessage {
    /// Message role: "user" or "assistant".
    pub role: String,
    /// Text content of the message.
    pub content: String,
}

/// Load recent text messages from a session for context injection.
///
/// Returns up to 20 recent user/assistant text messages in chronological order.
/// Returns `None` if the database lock fails or the session has no qualifying messages.
pub fn load_context_messages(state: &AppState, session_id: i64) -> Option<Vec<ContextMessage>> {
    use crate::domain::message::{ContentType, MessageRole};
    use crate::repo::message_repo;

    let db = state.db.conn.lock().ok()?;
    let messages = message_repo::list(&db, session_id, 200, 0).ok()?;

    let context: Vec<ContextMessage> = messages
        .iter()
        .filter(|m| {
            matches!(m.role, MessageRole::User | MessageRole::Assistant)
                && m.content_type == ContentType::Text
                && m.content.is_some()
        })
        .rev()
        .take(20)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|m| ContextMessage {
            role: match m.role {
                MessageRole::User => "user".to_owned(),
                MessageRole::Assistant => "assistant".to_owned(),
                MessageRole::System => "system".to_owned(),
            },
            content: m.content.clone().unwrap_or_default(),
        })
        .collect();

    if context.is_empty() {
        None
    } else {
        Some(context)
    }
}

/// Load a summary of prior text messages in a session for context injection.
///
/// Returns `(message_count, total_chars, messages_json)` where only
/// `ContentType::Text` messages are included. The message with `exclude_id`
/// is skipped so the just-persisted user message is not counted.
pub fn load_context_summary(
    state: &AppState,
    session_id: i64,
    exclude_id: i64,
) -> Result<(i32, i64, String), OrqaError> {
    use crate::domain::message::ContentType;
    use crate::repo::message_repo;

    let db = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))?;

    let messages = message_repo::list(&db, session_id, 1000, 0)?;

    let mut message_count: i32 = 0;
    let mut total_chars: i64 = 0;
    let mut entries: Vec<serde_json::Value> = Vec::new();

    for msg in messages {
        if msg.id == exclude_id {
            continue;
        }
        if msg.content_type != ContentType::Text {
            continue;
        }
        if let Some(ref content) = msg.content {
            let role_str = match msg.role {
                crate::domain::message::MessageRole::User => "user",
                crate::domain::message::MessageRole::Assistant => "assistant",
                crate::domain::message::MessageRole::System => "system",
            };
            total_chars += content.len() as i64;
            message_count += 1;
            entries.push(serde_json::json!({
                "role": role_str,
                "content": content,
            }));
        }
    }

    let messages_json = serde_json::to_string(&entries).map_err(|e| {
        OrqaError::Serialization(format!("failed to serialize context messages: {e}"))
    })?;

    Ok((message_count, total_chars, messages_json))
}

/// Look up the persisted provider session UUID for the given session.
pub fn lookup_provider_session_id(
    state: &AppState,
    session_id: i64,
) -> Result<Option<String>, OrqaError> {
    use crate::repo::session_repo;

    let db = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))?;
    Ok(session_repo::get(&db, session_id)
        .ok()
        .and_then(|s| s.provider_session_id))
}
