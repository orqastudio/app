---
id: TASK-503
title: "Knowledge gap detection in governance audit"
description: "Extend the governance audit (IntegrityWidget or dedicated panel) to surface per-type unlinked artifacts: rules without grounded-by, skills without practiced-by, decisions without enforces. Also detect disconnected components not reachable from the main graph. Each gap entry links directly to the artifact that needs fixing."
status: blocked
priority: P2
created: "2026-03-15"
updated: "2026-03-15"
epic: EPIC-076
milestone: null
horizon: active
depends-on:
  - TASK-498
acceptance:
  - "Governance audit shows per-type unlinked artifacts: rules without grounded-by, skills without practiced-by, decisions without enforces"
  - "Disconnected component detection with 'N artifacts not connected to the main graph'"
  - "Surfaced in IntegrityWidget or as a separate governance audit panel"
  - "Actionable — each gap links to the artifact that needs fixing"
relationships:
  - type: delivers
    target: EPIC-076
  - type: belongs-to
    target: EPIC-076
---
## What

Extend the governance audit surface to report knowledge gaps derived from the artifact graph structure. Three categories of gap are detected: (1) per-type missing inverse relationships (rules without `grounded-by`, skills without `practiced-by`, decisions without `enforces`), (2) artifacts in disconnected components not reachable from the main connected component. Each gap entry is actionable — clicking it navigates to the artifact that needs attention.

## How

1. Add a `knowledgeGaps(graphData)` function to `artifactGraphSDK` that returns:
   - `unlinkedByType`: a map of artifact type → array of artifact IDs missing their expected inverse relationship
     - Rules: no incoming `grounded-by` edge
     - Skills: no incoming `practiced-by` edge
     - Decisions: no outgoing `enforces` edge
   - `disconnectedArtifacts`: array of artifact IDs not in the largest connected component
2. Derive `governanceGaps` reactively from `knowledgeGaps(graphHealth)` using `$derived`.
3. Surface gaps in `IntegrityWidget` (or a new `GovernanceAuditPanel` if IntegrityWidget is too narrow):
   - Per-type sections with collapsible lists of artifact IDs as `ArtifactLink` components
   - A summary row: "N artifacts not connected to the main graph" with expandable list
   - If zero gaps, show a green "Governance graph is fully connected" state
4. Panel is a pure display component — receives gap data as props, no `invoke()` inside.

## Verification

- `knowledgeGaps` unit tests: known graph with deliberate missing inverses returns correct per-type gap lists.
- `knowledgeGaps` unit tests: isolated node appears in `disconnectedArtifacts`.
- Component test: zero-gap state renders success indicator.
- Component test: each gap entry renders as a clickable `ArtifactLink`.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)
