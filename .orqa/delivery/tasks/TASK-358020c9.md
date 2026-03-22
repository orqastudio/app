---
id: TASK-358020c9
title: Fix rescan stale data — refresh graph before integrity scan
description: Ensure the rescan button refreshes the artifact graph before running integrity checks so findings reflect current disk state.
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 1
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "Rescan button first refreshes the artifact graph, then runs integrity checks"
  - Findings reflect current disk state
relationships:
  - target: EPIC-6e774e50
    type: delivers
    rationale: Stale data on rescan undermines trust in integrity findings
---

## Scope

Update IntegrityWidget scan() function to call graph refresh before running integrity checks. Ensure the scan waits for the refresh to complete before proceeding.
