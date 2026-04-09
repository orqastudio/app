// Project-scoped storage initialization for the OrqaStudio app.
//
// Wraps `orqa_storage::Storage::open` with app-layer error conversion.
// The old `init_db` / `init_memory_db` functions are gone — storage is now
// managed entirely by the `engine/storage` crate.

use std::path::Path;
use std::sync::Arc;

use orqa_storage::Storage;

use crate::error::OrqaError;

/// Open (or create) the project-scoped database at `{project_root}/.state/orqa.db`.
///
/// Creates `.state/` if absent, applies PRAGMAs, and runs pending migrations.
/// Returns an `Arc<Storage>` that commands clone out of state for each operation.
pub async fn open_project_storage(project_root: &Path) -> Result<Arc<Storage>, OrqaError> {
    Storage::open(project_root)
        .await
        .map(Arc::new)
        .map_err(|e| OrqaError::Database(e.to_string()))
}
