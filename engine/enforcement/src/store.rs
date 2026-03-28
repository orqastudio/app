// Enforcement rule repository for the orqa-engine crate.
//
// Loads all enforcement rule `.md` files from a directory and parses them into
// `EnforcementRule` values. This is the only module in the enforcement pipeline
// that performs filesystem I/O. The parsing logic lives in `parser`.

use std::path::Path;

use crate::parser::parse_rule_content;
use orqa_engine_types::error::EngineError;
use orqa_engine_types::types::enforcement::EnforcementRule;

/// Load all rule files from `rules_dir/*.md` and parse them into `EnforcementRule` values.
///
/// Files that fail to parse are logged as warnings and skipped — one bad rule file
/// must not prevent other rules from loading. Rules are returned sorted by name for
/// deterministic ordering. Returns an error only if the directory cannot be read.
pub fn load_rules(rules_dir: &Path) -> Result<Vec<EnforcementRule>, EngineError> {
    let read_dir = std::fs::read_dir(rules_dir).map_err(|e| {
        EngineError::FileSystem(std::io::Error::new(
            e.kind(),
            format!("cannot read rules directory '{}': {e}", rules_dir.display()),
        ))
    })?;

    let mut rules = Vec::new();

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!(
                    "[enforcement] cannot read rule file '{}': {e}",
                    path.display()
                );
                continue;
            }
        };

        match parse_rule_content(&name, &content) {
            Ok(rule) => {
                tracing::debug!(
                    "[enforcement] loaded rule '{}' ({} entries)",
                    rule.name,
                    rule.entries.len()
                );
                rules.push(rule);
            }
            Err(e) => {
                tracing::warn!("[enforcement] failed to parse '{}': {e}", path.display());
            }
        }
    }

    rules.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(rules)
}

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_engine_types::error::EngineError;

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
        assert!(matches!(err, EngineError::FileSystem(_)));
    }
}
