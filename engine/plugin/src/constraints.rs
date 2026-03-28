//! Installation constraint checks for P5-26 (one-methodology) and P5-27 (one-per-stage).
//!
//! These checks are called by the installer before finalising any plugin installation.
//! Each check scans the currently installed plugins and validates that the incoming
//! plugin does not violate the methodology or stage-slot uniqueness constraints.

use std::path::Path;

use orqa_engine_types::error::EngineError;

use super::discovery::scan_plugins;
use super::manifest::{read_manifest, PluginManifest};

/// Violation returned when an installation constraint is breached.
///
/// Callers receive this as an `EngineError::Plugin` containing the violation message.
/// Structured here so constraint_check functions can produce typed errors before
/// converting to `EngineError`.
#[derive(Debug)]
pub struct ConstraintViolation {
    /// Human-readable explanation of the violation.
    pub message: String,
}

impl From<ConstraintViolation> for EngineError {
    fn from(v: ConstraintViolation) -> Self {
        Self::Plugin(v.message)
    }
}

/// P5-26: Enforce one-methodology-plugin-per-project.
///
/// Reads all currently installed plugins and checks whether any of them declare
/// `purpose` containing `"methodology"`. If so, and the incoming plugin is a
/// different methodology plugin, installation is rejected with a descriptive error.
/// Reinstalling the same methodology plugin succeeds (update, not conflict).
///
/// Non-methodology plugins pass this check unconditionally.
pub fn check_one_methodology(
    incoming: &PluginManifest,
    project_root: &Path,
) -> Result<(), ConstraintViolation> {
    // If the incoming plugin is not a methodology plugin, this constraint does not apply.
    if !incoming
        .install_constraints
        .purpose
        .iter()
        .any(|p| p == "methodology")
    {
        return Ok(());
    }

    // Scan installed plugins and look for an existing methodology plugin.
    let installed = scan_plugins(project_root);
    for plugin in &installed {
        // Skip the plugin being reinstalled — same name is an update, not a conflict.
        if plugin.name == incoming.name {
            continue;
        }

        let plugin_dir = Path::new(&plugin.path);
        if let Ok(manifest) = read_manifest(plugin_dir) {
            if manifest
                .install_constraints
                .purpose
                .iter()
                .any(|p| p == "methodology")
            {
                return Err(ConstraintViolation {
                    message: format!(
                        "cannot install methodology plugin '{}': project already has methodology \
                         plugin '{}' installed. Only one methodology plugin is allowed per project. \
                         Uninstall '{}' first.",
                        incoming.name, manifest.name, manifest.name
                    ),
                });
            }
        }
    }

    Ok(())
}

/// P5-27: Enforce one-workflow-plugin-per-stage.
///
/// Reads all currently installed plugins and checks whether any of them declare
/// the same `stage_slot` as the incoming plugin. If so, installation is rejected
/// with a descriptive error naming the conflicting plugin and the stage slot.
/// Reinstalling the same plugin (same name) succeeds.
///
/// Plugins without a `stage_slot` pass this check unconditionally.
pub fn check_one_per_stage(
    incoming: &PluginManifest,
    project_root: &Path,
) -> Result<(), ConstraintViolation> {
    // If the incoming plugin has no stage slot, this constraint does not apply.
    let Some(incoming_slot) = &incoming.install_constraints.stage_slot else {
        return Ok(());
    };

    // Scan installed plugins and look for an existing plugin filling the same slot.
    let installed = scan_plugins(project_root);
    for plugin in &installed {
        // Skip the plugin being reinstalled — same name is an update, not a conflict.
        if plugin.name == incoming.name {
            continue;
        }

        let plugin_dir = Path::new(&plugin.path);
        if let Ok(manifest) = read_manifest(plugin_dir) {
            if let Some(existing_slot) = &manifest.install_constraints.stage_slot {
                if existing_slot == incoming_slot {
                    return Err(ConstraintViolation {
                        message: format!(
                            "cannot install workflow plugin '{}': stage slot '{}' is already \
                             filled by '{}'. Only one workflow plugin may occupy each stage slot. \
                             Uninstall '{}' first.",
                            incoming.name,
                            incoming_slot,
                            manifest.name,
                            manifest.name
                        ),
                    });
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{PluginInstallConstraints, PluginManifest, PluginProvides};
    use std::path::PathBuf;

    fn make_manifest(name: &str, purpose: &[&str], stage_slot: Option<&str>) -> PluginManifest {
        PluginManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            display_name: None,
            description: None,
            provides: PluginProvides {
                schemas: vec![],
                views: vec![],
                widgets: vec![],
                relationships: vec![],
                sidecar: None,
                cli_tools: vec![],
                hooks: vec![],
                agents: vec![],
            },
            merge_decisions: vec![],
            install_constraints: PluginInstallConstraints {
                purpose: purpose.iter().map(|s| s.to_string()).collect(),
                stage_slot: stage_slot.map(String::from),
                affects_schema: false,
                affects_enforcement: false,
            },
        }
    }

    #[test]
    fn non_methodology_plugin_always_passes_methodology_check() {
        // A knowledge plugin has no purpose="methodology", so the check is a no-op.
        let incoming = make_manifest("@orqastudio/knowledge", &["knowledge"], None);
        let result = check_one_methodology(&incoming, &PathBuf::from("/nonexistent"));
        assert!(result.is_ok());
    }

    #[test]
    fn methodology_plugin_passes_when_no_existing_methodology() {
        // First methodology install — no installed plugins, check passes.
        let incoming = make_manifest("@orqastudio/plugin-agile-methodology", &["methodology"], None);
        let result = check_one_methodology(&incoming, &PathBuf::from("/nonexistent"));
        assert!(result.is_ok());
    }

    #[test]
    fn plugin_without_stage_slot_always_passes_stage_check() {
        // A knowledge plugin has no stageSlot, so the check is a no-op.
        let incoming = make_manifest("@orqastudio/knowledge", &["knowledge"], None);
        let result = check_one_per_stage(&incoming, &PathBuf::from("/nonexistent"));
        assert!(result.is_ok());
    }

    #[test]
    fn workflow_plugin_passes_when_no_existing_plugins() {
        // No installed plugins — stage slot is free.
        let incoming = make_manifest(
            "@orqastudio/plugin-agile-discovery",
            &["workflow"],
            Some("discovery"),
        );
        let result = check_one_per_stage(&incoming, &PathBuf::from("/nonexistent"));
        assert!(result.is_ok());
    }

    #[test]
    fn constraint_violation_converts_to_engine_error() {
        // ConstraintViolation should convert to EngineError::Plugin.
        let violation = ConstraintViolation {
            message: "test error".to_string(),
        };
        let err: EngineError = violation.into();
        assert!(matches!(err, EngineError::Plugin(_)));
        assert!(err.to_string().contains("test error"));
    }
}
