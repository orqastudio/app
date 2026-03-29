---
id: "TASK-bf368dca"
type: "task"
title: "Audit existing architecture decisions against PD-f079c196/039/040"
description: "Review PD-7121ec20 through PD-306eccf1 to identify which decisions are superseded, affected, or made defunct by the graph-based knowledge injection (PD-f079c196), core graph firmware (PD-45f32bab), and task-first audit trail (PD-7fa3f280) decisions."
status: archived
created: "2026-03-12"
updated: "2026-03-12"
docs:
  - "DOC-28344cd7"
acceptance:
  - "Every AD from PD-7121ec20 to PD-306eccf1 has been reviewed"
  - "Superseded decisions have status superseded and superseded-by set"
  - "New decisions (PD-f079c196/039/040) have supersedes set where applicable"
  - "No one-sided supersessions exist (RULE-b10fe6d1 compliance)"
  - "Summary table of audit findings exists in this task's body"
relationships:
  - target: "EPIC-f079c196"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

[PD-f079c196](PD-f079c196) (graph-based knowledge injection), [PD-45f32bab](PD-45f32bab) (core graph firmware), and [PD-7fa3f280](PD-7fa3f280)
(task-first audit trail with configurable epic requirement) represent a significant
architectural shift. Several earlier decisions may now be:

- **Superseded**: The new decision replaces the old one entirely
- **Partially affected**: The old decision still holds but some aspects changed
- **Unaffected**: The old decision is independent of the new architecture

## Known Candidates for Review

These decisions are likely affected based on what [PD-f079c196](PD-f079c196)/039/040 change:

| Decision | Title | Likely Impact |
| ---------- | ------- | --------------- |
| [PD-c1e5a39e](PD-c1e5a39e) | Three-tier skill model | May be affected by graph-based skill discovery |
| [PD-48b310f9](PD-48b310f9) | Universal agent roles | Likely unaffected — roles are orthogonal to injection |
| [PD-859ed163](PD-859ed163) | SQLite for conversations only | Likely unaffected — governance stays file-based |
| [PD-80f39962](PD-80f39962) | Enforcement engine | May be affected by graph-based enforcement |
| [PD-a47f313a](PD-a47f313a) | Plugin architecture | May be affected by plugin reading graph |
| [PD-e8ea9fb9](PD-e8ea9fb9) | Companion plugin | Likely affected — plugin now reads graph |
| [PD-03d9007d](PD-03d9007d) | Rule enforcement entries | May be affected by self-enforcing rules |
| [PD-306eccf1](PD-306eccf1) | Capability-based agents | Likely unaffected — orthogonal to injection |

All other ADs should still be reviewed for completeness.

## How

1. Read each AD from [PD-7121ec20](PD-7121ec20) through [PD-306eccf1](PD-306eccf1)
2. Evaluate against [PD-f079c196](PD-f079c196)/039/040 changes
3. Mark superseded decisions with proper status and cross-references
4. Update both sides of any supersession in the same commit

## Verification

- All ADs reviewed and verdicts recorded in summary table below
- No one-sided supersessions
- All superseded decisions have correct status and cross-references

## Output

No full supersessions found. [PD-f079c196](PD-f079c196)/039/040 are additive/evolutionary, not replacements. Six ADs are partially affected:

| AD | Title | Verdict | Notes |
| ---- | ------- | --------- | ------- |
| [PD-7121ec20](PD-7121ec20) | Thick Backend Architecture | Unaffected | |
| [PD-4e7faf0e](PD-4e7faf0e) | IPC Boundary Design | Unaffected | |
| [PD-2d58941b](PD-2d58941b) | Error Propagation via Result Types | Unaffected | |
| [PD-ecc96aef](PD-ecc96aef) | Svelte 5 Runes Only | Unaffected | |
| [PD-75bb14ae](PD-75bb14ae) | SQLite for All Structured Persistence | Already superseded | By [PD-859ed163](PD-859ed163) prior to this audit |
| [PD-9a7d7256](PD-9a7d7256) | Component Purity | Unaffected | |
| [PD-09fc4e65](PD-09fc4e65) | Agent SDK Sidecar Integration | Unaffected | |
| [PD-fc4e9013](PD-fc4e9013) | Max Subscription Authentication | Unaffected | |
| [PD-39e2fb81](PD-39e2fb81) | Streaming Pipeline | Unaffected | |
| [PD-e4a3b5da](PD-e4a3b5da) | Tool Implementation as MCP | Unaffected | |
| [PD-d01b9e0a](PD-d01b9e0a) | Security Model | Unaffected | |
| [PD-5d0f8814](PD-5d0f8814) | Tauri Plugin Selections | Unaffected | |
| [PD-33e315cc](PD-33e315cc) | Frontend Library Selections | Unaffected | |
| [PD-b08f456d](PD-b08f456d) | Persistence Architecture | Already superseded | By [PD-859ed163](PD-859ed163) prior to this audit |
| [PD-0dfa4d52](PD-0dfa4d52) | Governance Artifact Format | Already superseded | By [PD-4ea9a290](PD-4ea9a290) prior to this audit |
| [PD-23e27cf5](PD-23e27cf5) | Onboarding Strategy | Unaffected | |
| [PD-af88bb69](PD-af88bb69) | Composability Principle | Unaffected | |
| [PD-85d45674](PD-85d45674) | Four-Zone Layout | Already superseded | By [PD-7cb83077](PD-7cb83077) prior to this audit |
| [PD-7cb83077](PD-7cb83077) | Three-Zone + Nav Sub-Panel Layout | Unaffected | |
| [PD-f9fbd1d7](PD-f9fbd1d7) | Filesystem-Driven Doc Browsing | Unaffected | |
| [PD-4ea9a290](PD-4ea9a290) | .orqa/ as Single Source of Truth | Partially affected | [PD-45f32bab](PD-45f32bab) adds firmware/project layering — core artifacts within .orqa/ are read-only firmware |
| [PD-45cfe1d1](PD-45cfe1d1) | Config-Driven Artifact Scanning | Partially affected | [PD-f079c196](PD-f079c196) adds docs/skills/sources schema fields the scanner must surface |
| [PD-3b986859](PD-3b986859) | Plans Merged Into Research Schema | Partially affected | [PD-f079c196](PD-f079c196) adds sources field to research schema |
| [PD-7d3d7521](PD-7d3d7521) | Native Search Engine | Unaffected | |
| [PD-02a2a97b](PD-02a2a97b) | Provider-Agnostic AI Integration | Unaffected | |
| [PD-306d7320](PD-306d7320) | Domain Service Extraction Pattern | Unaffected | |
| [PD-e711446e](PD-e711446e) | Vision Evolution | Unaffected | |
| [PD-c1e5a39e](PD-c1e5a39e) | Three-Tier Skill Loading | Partially affected | Tier 2 mechanism changes from orchestrator table to graph edges (task.docs/skills) |
| [PD-48b310f9](PD-48b310f9) | Universal Roles, Domain-Specific Skills | Unaffected | |
| [PD-26b0eb9f](PD-26b0eb9f) | Skill-Driven Project Initialisation | Partially affected | Must now configure workflow.epics-required during setup (PD-7fa3f280) |
| [PD-74a2cb7a](PD-74a2cb7a) | Pillars as First-Class Planning Artifacts | Unaffected | |
| [PD-859ed163](PD-859ed163) | SQLite for Conversation Persistence Only | Unaffected | |
| [PD-80f39962](PD-80f39962) | Core UI Boundary | Unaffected | |
| [PD-a47f313a](PD-a47f313a) | Schema-Driven Artifact Filtering | Partially affected | New schema fields from [PD-f079c196](PD-f079c196) appear as filter options; core schemas protected per [PD-45f32bab](PD-45f32bab) |
| [PD-e8ea9fb9](PD-e8ea9fb9) | Config-Driven Navigation Defaults | Unaffected | |
| [PD-03d9007d](PD-03d9007d) | Cross-Linking as Default Behaviour | Unaffected | |
| [PD-306eccf1](PD-306eccf1) | AI-Driven Cross-Artifact Search | Unaffected | |
