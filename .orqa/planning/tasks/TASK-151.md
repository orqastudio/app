---
id: TASK-151
title: Create epic readiness validator hook
description: Validation hook that checks docs-required paths exist on disk before an epic can move to ready status.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
acceptance:
  - Epic with missing docs-required items produces clear error
  - Epic with all docs-required items present passes validation
  - Epic with empty docs-required passes validation
  - Handles both artifact ID refs and file path refs
---
## What

[RULE-004](RULE-004) defines the documentation gate as NON-NEGOTIABLE — all `docs-required` items must exist before an epic moves from `draft` to `ready`. Currently orchestrator-enforced via manual checking. Automate it.

## How

1. Create a validation script that reads an epic's `docs-required` frontmatter array
2. For each entry: if it matches an artifact ID pattern (e.g., `[AD-001](AD-001)`), check if that artifact exists in the graph. If it is a file path, check if the file exists on disk.
3. If any required doc is missing, report which ones and exit non-zero
4. Can be a standalone script or integrated into the pre-commit hook when epic status changes

## Verification

- [ ] Epic with missing docs-required items produces clear error listing what's missing
- [ ] Epic with all docs-required items present passes validation
- [ ] Epic with empty docs-required passes validation
- [ ] Both artifact ID refs and file path refs are handled
