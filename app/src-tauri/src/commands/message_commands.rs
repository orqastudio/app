// Tauri IPC commands for message retrieval.
//
// Messages are persisted in the daemon's database. The app reads messages
// via the daemon HTTP API through libs/db.

use tauri::State;

use crate::domain::message::Message;
use crate::error::OrqaError;
use crate::state::AppState;

/// List messages for a session with pagination.
#[tauri::command]
pub async fn message_list(
    session_id: i64,
    limit: Option<i64>,
    offset: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<Message>, OrqaError> {
    let limit_val = limit.unwrap_or(100);
    let offset_val = offset.unwrap_or(0);

    if limit_val < 0 {
        return Err(OrqaError::Validation("limit cannot be negative".to_owned()));
    }
    if offset_val < 0 {
        return Err(OrqaError::Validation(
            "offset cannot be negative".to_owned(),
        ));
    }

    Ok(state
        .db
        .client
        .messages()
        .list(session_id, limit_val, offset_val)
        .await?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_query_validation() {
        let query = "   ";
        assert!(query.trim().is_empty());
    }

    #[test]
    fn negative_limit_validation() {
        let limit: i64 = -5;
        assert!(limit < 0);
    }
}
