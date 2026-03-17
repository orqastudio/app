---

id: TASK-512
title: Validate existing artifacts against delivery type configuration
description: "Extend the integrity scanner to verify that delivery artifacts exist in their configured paths, that parent references point to valid parent types, and that no delivery directory contains artifacts whose type is not covered by the delivery type config. Current artifacts must pass validation cleanly."
status: blocked
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 2
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - Integrity scanner validates that delivery artifacts exist in their configured paths
  - Validates parent references point to valid parent types
  - Reports any artifacts in delivery directories not covered by the type config
  - Current artifacts pass validation cleanly
relationships:
  - target: EPIC-078
    type: delivers
  - target: TASK-510
    type: depends-on
---
## What

Once the delivery type config is in place (TASK-509) and the parent-child consistency checks read from it (TASK-510), this task adds a complementary validation pass that works the other direction: given the configured delivery types, scan the configured paths and verify that every artifact found belongs to a recognised type and that its parent field references an artifact of the correct parent type. Any artifact in a delivery directory that isn't covered by the config is reported as an anomaly.

## How

1. Add a `validate_delivery_type_coverage` function to the integrity scanner that:
   a. Iterates over each `DeliveryTypeConfig` entry.
   b. Reads all `.md` files in `config.path`.
   c. Parses each file's frontmatter to extract the `id` field and (if the type has a parent) the parent reference field (e.g. `epic`, `milestone`).
   d. Verifies the parent reference, if present, resolves to an artifact whose type key matches `config.parent`.
   e. Collects any files whose IDs do not match the expected ID prefix pattern for that type (derived from the type key, e.g. `"task"` → `"TASK-"`).
2. Emit structured `IntegrityFinding` entries for:
   - Artifacts with a parent reference pointing to the wrong type.
   - Files in a delivery directory whose ID prefix does not match the configured type.
3. Wire the new function into the existing integrity scan pipeline so its findings appear alongside existing check results.
4. Run the scanner against the live `.orqa/` directory — expect zero findings for current artifacts.
5. Add a unit test with a fixture set that includes one correctly-placed artifact, one mis-parented artifact, and one stray file — assert the scanner emits exactly the expected findings.
6. Run `make check` — zero warnings, zero type errors.

## Verification

- Running the scanner against the current repository produces zero new findings.
- Unit test with fixture data passes and asserts correct finding counts for each anomaly category.
- `IntegrityFinding` output includes the artifact ID, the configured type key, and a human-readable description of the violation.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)
