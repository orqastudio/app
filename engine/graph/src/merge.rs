//! Three-way merge helper for artifact frontmatter fields.
//!
//! This module provides field-level three-way merge between:
//! - `base`  — the common ancestor (plugin manifest hash or export snapshot)
//! - `ours`  — the current SurrealDB state (theirs in the DB)
//! - `theirs` — the incoming import file
//!
//! Merge strategy (per field):
//! - If ours == base: theirs wins (ours did not change, accept the incoming value)
//! - If theirs == base: ours wins (theirs did not change, keep the existing value)
//! - If ours == theirs: no conflict (both sides changed to the same value)
//! - If all three differ: true conflict — field is unresolvable
//!
//! The merge operates on `serde_json::Value` maps. Fields not present in the
//! base are treated as absent (which is considered equal to `Value::Null`).
//! Fields only in `theirs` that are absent in both base and ours are added
//! (new field introduction — no conflict).

use std::collections::BTreeMap;

use serde_json::Value;

/// Result of merging a single field.
#[derive(Debug, Clone)]
pub enum FieldMerge {
    /// Both sides agree — no conflict. Contains the resolved value.
    Resolved(Value),
    /// True conflict — both sides changed the field to different values.
    Conflict {
        /// Field name.
        field: String,
        /// Value stored in the database (our side).
        ours: Value,
        /// Value coming from the import file (their side).
        theirs: Value,
        /// The common ancestor value.
        base: Value,
    },
}

/// Result of a full three-way merge of two artifact field maps.
#[derive(Debug, Clone)]
pub struct MergeResult {
    /// All resolved fields (conflicting fields are excluded from this map).
    pub resolved: BTreeMap<String, Value>,
    /// Fields that could not be auto-resolved (one per conflicted field).
    pub conflicts: Vec<FieldMerge>,
}

impl MergeResult {
    /// Returns true if the merge has no unresolvable conflicts.
    pub fn is_clean(&self) -> bool {
        self.conflicts.is_empty()
    }
}

/// Perform a three-way merge of artifact frontmatter field maps.
///
/// Iterates the union of keys across all three maps. For each key:
/// - `ours` unchanged from `base` → accept `theirs`
/// - `theirs` unchanged from `base` → keep `ours`
/// - both changed to the same value → accept it (idempotent)
/// - all three differ → record as conflict
///
/// `base` may be empty (`{}`) when no ancestor is known. In that case, any
/// field present in both `ours` and `theirs` with different values is a
/// conflict (since there is no shared history to mediate). Fields only in one
/// side are accepted from that side.
pub fn three_way_merge(
    base: &BTreeMap<String, Value>,
    ours: &BTreeMap<String, Value>,
    theirs: &BTreeMap<String, Value>,
) -> MergeResult {
    let null = Value::Null;
    let mut resolved: BTreeMap<String, Value> = BTreeMap::new();
    let mut conflicts: Vec<FieldMerge> = Vec::new();

    // Collect all keys from all three maps.
    let mut all_keys: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for k in base.keys() {
        all_keys.insert(k.as_str());
    }
    for k in ours.keys() {
        all_keys.insert(k.as_str());
    }
    for k in theirs.keys() {
        all_keys.insert(k.as_str());
    }

    for key in all_keys {
        let base_val = base.get(key).unwrap_or(&null);
        let our_val = ours.get(key).unwrap_or(&null);
        let their_val = theirs.get(key).unwrap_or(&null);

        if our_val == their_val {
            // Both sides agree — no conflict regardless of base.
            if our_val != &null {
                resolved.insert(key.to_owned(), our_val.clone());
            }
        } else if our_val == base_val {
            // Ours unchanged — accept theirs.
            if their_val != &null {
                resolved.insert(key.to_owned(), their_val.clone());
            }
        } else if their_val == base_val {
            // Theirs unchanged — keep ours.
            if our_val != &null {
                resolved.insert(key.to_owned(), our_val.clone());
            }
        } else {
            // All three differ — true conflict.
            conflicts.push(FieldMerge::Conflict {
                field: key.to_owned(),
                ours: our_val.clone(),
                theirs: their_val.clone(),
                base: base_val.clone(),
            });
        }
    }

    MergeResult {
        resolved,
        conflicts,
    }
}

/// Parse a JSON object value into a `BTreeMap<String, Value>`.
///
/// Returns an empty map if the value is not an object or is absent.
pub fn value_to_map(v: &Value) -> BTreeMap<String, Value> {
    match v {
        Value::Object(m) => m.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        _ => BTreeMap::new(),
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn map(v: &Value) -> BTreeMap<String, Value> {
        value_to_map(v)
    }

    #[test]
    fn clean_merge_ours_changed() {
        // Base: status=active; ours: status=active (unchanged); theirs: status=done
        let base = map(&json!({"status": "active"}));
        let ours = map(&json!({"status": "active"}));
        let theirs = map(&json!({"status": "done"}));
        let result = three_way_merge(&base, &ours, &theirs);
        assert!(result.is_clean());
        assert_eq!(result.resolved["status"], "done");
    }

    #[test]
    fn clean_merge_theirs_changed() {
        // Base: status=active; ours: status=in-progress; theirs: status=active (unchanged)
        let base = map(&json!({"status": "active"}));
        let ours = map(&json!({"status": "in-progress"}));
        let theirs = map(&json!({"status": "active"}));
        let result = three_way_merge(&base, &ours, &theirs);
        assert!(result.is_clean());
        assert_eq!(result.resolved["status"], "in-progress");
    }

    #[test]
    fn clean_merge_both_changed_same_value() {
        let base = map(&json!({"status": "active"}));
        let ours = map(&json!({"status": "done"}));
        let theirs = map(&json!({"status": "done"}));
        let result = three_way_merge(&base, &ours, &theirs);
        assert!(result.is_clean());
        assert_eq!(result.resolved["status"], "done");
    }

    #[test]
    fn conflict_detected_when_all_differ() {
        let base = map(&json!({"status": "active", "priority": "medium"}));
        let ours = map(&json!({"status": "archived", "priority": "high"}));
        let theirs = map(&json!({"status": "in-progress", "priority": "critical"}));
        let result = three_way_merge(&base, &ours, &theirs);
        assert!(!result.is_clean());
        assert_eq!(result.conflicts.len(), 2);
    }

    #[test]
    fn no_base_both_sides_differ_is_conflict() {
        // No base knowledge — both sides mutated the same field differently.
        let base = map(&json!({}));
        let ours = map(&json!({"status": "active"}));
        let theirs = map(&json!({"status": "archived"}));
        let result = three_way_merge(&base, &ours, &theirs);
        assert!(!result.is_clean());
        assert_eq!(result.conflicts.len(), 1);
    }

    #[test]
    fn new_field_in_theirs_accepted() {
        // Field not in base or ours — theirs adds it.
        let base = map(&json!({"title": "A"}));
        let ours = map(&json!({"title": "A"}));
        let theirs = map(&json!({"title": "A", "priority": "high"}));
        let result = three_way_merge(&base, &ours, &theirs);
        assert!(result.is_clean());
        assert_eq!(result.resolved["priority"], "high");
    }
}
