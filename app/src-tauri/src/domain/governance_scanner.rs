// Governance scanner — delegates to the orqa-engine crate.
//
// Re-exports `scan_governance` from `orqa_engine::enforcement::scanner`, converting
// the engine-level error to the app-level `OrqaError`. This keeps the app's callers
// unchanged while the implementation lives in the engine crate.

use std::path::Path;

use orqa_validation::settings::ArtifactEntry;

use crate::domain::governance::GovernanceScanResult;
use crate::error::OrqaError;

/// Scan a project directory for governance files across the areas defined in `artifacts`.
///
/// Delegates to `orqa_engine::enforcement::scanner::scan_governance`. Returns
/// `OrqaError::Validation` if the path does not exist, `OrqaError::FileSystem`
/// for I/O errors.
pub fn scan_governance(
    project_path: &Path,
    artifacts: &[ArtifactEntry],
) -> Result<GovernanceScanResult, OrqaError> {
    orqa_engine::enforcement::scanner::scan_governance(project_path, artifacts)
        .map_err(OrqaError::from)
}
