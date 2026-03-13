---
id: IDEA-077
title: "Surface data integrity issues on the dashboard"
description: "Run pipeline integrity checks (verify-links, verify-pipeline-integrity) and display results on the app dashboard. Makes broken links, missing relationships, reconciliation gaps, and schema violations visible without running CLI commands."
status: captured
created: "2026-03-13"
updated: "2026-03-13"
pillars:
  - PILLAR-001
  - PILLAR-003
research-needed:
  - "Should integrity checks run on app startup, on a schedule, or on-demand from the dashboard?"
  - "What's the right UX — a health score, a warning banner, an expandable issue list?"
  - "Should the Rust backend call the existing Node scripts or reimplement the checks natively?"
  - "How does this interact with the pre-commit hook — are they the same checks presented differently?"
promoted-to: null
---

## Motivation

Pipeline integrity checks (`make verify`) currently only run from the CLI or pre-commit hook. Issues like missing reconciliation tasks, broken cross-references, or empty relationship arrays are invisible until someone runs the tool manually. Surfacing these on the app dashboard makes the artifact graph's health a first-class concern — visible at a glance, not hidden behind a terminal command.

## Sketch

Dashboard widget showing:
- Overall health indicator (pass/fail/warnings count)
- Categorised issues: broken links, missing relationships, schema violations, reconciliation gaps
- Click-through to the affected artifact
- Optional: trend over time (are we improving or accumulating debt?)
