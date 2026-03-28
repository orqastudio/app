// Enforcement rules repository — delegates to the orqa-engine crate.
//
// Re-exposes `load_rules` from `orqa_engine::enforcement::store`, converting the
// engine-level error to the app-level `OrqaError`. This keeps the app's callers
// (enforcement_commands, tool_executor) unchanged while the implementation lives
// in the engine crate.

use std::path::Path;

use crate::domain::enforcement::EnforcementRule;
use crate::error::OrqaError;

/// Load all rule files from `rules_dir/*.md` and parse them.
///
/// Delegates to `orqa_engine::enforcement::store::load_rules`. Files that fail
/// to parse are logged as warnings and skipped — one bad rule file must not
/// prevent other rules from loading. Returns rules sorted by name.
pub fn load_rules(rules_dir: &Path) -> Result<Vec<EnforcementRule>, OrqaError> {
    orqa_engine::enforcement::store::load_rules(rules_dir).map_err(OrqaError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_empty_dir_returns_empty_vec() {
        let dir = tempfile::tempdir().expect("tempdir");
        let rules = load_rules(dir.path()).expect("should load");
        assert!(rules.is_empty());
    }

    #[test]
    fn load_skips_non_md_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("readme.txt"), "not a rule").expect("write");
        let rules = load_rules(dir.path()).expect("should load");
        assert!(rules.is_empty());
    }

    #[test]
    fn load_parses_valid_rule_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("no-stubs.md"),
            r#"---
scope: project
enforcement:
  - event: bash
    action: warn
    pattern: "TODO"
---
# No Stubs

Do not leave stub implementations.
"#,
        )
        .expect("write");

        let rules = load_rules(dir.path()).expect("should load");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].name, "no-stubs");
        assert_eq!(rules[0].entries.len(), 1);
    }

    #[test]
    fn load_returns_rules_sorted_by_name() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("z-rule.md"), "# Z Rule").expect("write");
        std::fs::write(dir.path().join("a-rule.md"), "# A Rule").expect("write");
        std::fs::write(dir.path().join("m-rule.md"), "# M Rule").expect("write");

        let rules = load_rules(dir.path()).expect("should load");
        assert_eq!(rules.len(), 3);
        assert_eq!(rules[0].name, "a-rule");
        assert_eq!(rules[1].name, "m-rule");
        assert_eq!(rules[2].name, "z-rule");
    }

    #[test]
    fn load_missing_dir_returns_error() {
        let result = load_rules(Path::new("/nonexistent/enforcement/rules/dir"));
        assert!(result.is_err());
        let err = result.expect_err("should be error");
        assert!(matches!(err, OrqaError::FileSystem(_)));
    }
}
