---
id: "IDEA-c7da2e08"
type: discovery-idea
title: "Pipeline health dashboard — integrity surfacing, visualization, and process automation"
description: "Surface pipeline integrity checks on the app dashboard with scan/fix actions, add pipeline thread visualization, temporal trend analysis, and automate two process improvements: related idea surfacing during promotion and intent-based observation capture from user prompts."
status: completed
created: "2026-03-13"
updated: "2026-03-13"
horizon: "active"
research-needed:
  - "Should integrity checks run on app startup, on a schedule, or on-demand from the dashboard? → On-demand (Scan button), with optional auto-run on project load"
  - "What's the right UX — a health score, a warning banner, an expandable issue list? → Health score summary + expandable categorised issue list with drill-through"
  - "Should the Rust backend call the existing Node scripts or reimplement the checks natively? → Native Rust implementation using the existing ArtifactGraph — the graph already tracks orphans and broken refs"
  - "How does this interact with the pre-commit hook — are they the same checks presented differently? → Same logical checks, different presentation. Hook blocks commits, dashboard shows current state."
relationships:
  - target: "EPIC-82dd0bd2"
    type: "realises"
  - target: "PILLAR-c9e0a695"
    type: "grounded"
  - target: "PERSONA-c4afd86b"
    type: "benefits"
---
## Motivation

Pipeline integrity checks (`make verify`) currently only run from the CLI or pre-commit hook. Issues like missing reconciliation tasks, broken cross-references, or empty relationship arrays are invisible until someone runs the tool manually. Surfacing these on the app dashboard makes the artifact graph's health a first-class concern — visible at a glance, not hidden behind a terminal command.

Beyond integrity, the pipeline dashboard should show the flow of knowledge through the maturity pipeline — where are the bottlenecks? What observations are stuck? What enforcement gaps exist? And two process improvements discovered during this idea's own promotion should be implemented first so the rest of the work benefits from them.

## Bundled Ideas

- [IDEA-974623c0](IDEA-974623c0) — Auto-fix null relationship targets
- [IDEA-d8a65eee](IDEA-d8a65eee) — Pipeline thread visualization and flow analysis
- [IDEA-451b29b6](IDEA-451b29b6) — Graph-powered dashboard insights and trend analysis

## Promoted Observations (Precursors)

- [IMPL-f3629976](IMPL-f3629976) — Auto-surface related ideas during promotion → update [RULE-b10fe6d1](RULE-b10fe6d1)
- [IMPL-6a8f9612](IMPL-6a8f9612) — Prompt input intent inference → plugin prompt-submit hook

## Scope

### Phase 0: Process Automation (Precursors)
- Promote [IMPL-f3629976](IMPL-f3629976): update [RULE-b10fe6d1](RULE-b10fe6d1) with a mandatory "scan related ideas" step in the promotion procedure
- Promote [IMPL-6a8f9612](IMPL-6a8f9612): create a `user-prompt-submit` hook in the plugin that infers observation intent and auto-creates IMPL entries

### Phase 1: Integrity Engine (Backend)
- Native Rust integrity checks in the backend (link verification, relationship validation, reconciliation checks, schema compliance)
- IPC command to run checks on-demand and return categorised results
- Auto-fix logic for deterministic issues (null targets with obvious candidates, missing bidirectional inverses)
- Agent delegation packaging for non-deterministic issues

### Phase 2: Dashboard Integrity Widget (Frontend)
- Health score summary (pass/fail/warnings count)
- Categorised expandable issue list (broken links, missing relationships, schema violations, reconciliation gaps)
- Click-through to affected artifact
- Scan button (read-only) and Fix button (auto-fix + agent delegation)
- Fix result display (what was changed, what was delegated, delegation status)

### Phase 3: Pipeline Visualization
- Thread visualization by traversing relationship edges
- Bottleneck detection (stuck observations, disconnected enforcement)
- Flow direction analysis (forward pipeline movement vs backwards)
- Unresolved tension display (null targets with intended=false)

### Phase 4: Temporal Analytics
- Health trend sparklines (broken refs, orphans over time)
- Artifact velocity (status transitions per period)
- Staleness detection (artifacts not updated relative to dependents)
- "Attention needed" feed ranked by metric signals

## Research Answers

1. **Integrity check timing**: On-demand via Scan button. Optional auto-run on project load (configurable). Not on a schedule — the graph rebuilds on file changes anyway.
2. **UX pattern**: Health score card at top of dashboard → expandable categorised issue list below → click any issue to navigate to the artifact. Two action buttons: Scan (refresh) and Fix (auto-fix + delegate).
3. **Native vs Node scripts**: Native Rust. The `ArtifactGraph` already computes orphans and broken refs during its two-pass build. Extend it with relationship validation and reconciliation checks rather than shelling out to Node.
4. **Interaction with pre-commit hook**: Same logical checks, different context. The hook blocks bad commits. The dashboard shows current state and enables fixes. They share the same check definitions but run independently.