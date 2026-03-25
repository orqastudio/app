---
id: "TASK-bf368dca"
type: "task"
title: "Audit existing architecture decisions against AD-f079c196/039/040"
description: "Review AD-7121ec20 through AD-306eccf1 to identify which decisions are superseded, affected, or made defunct by the graph-based knowledge injection (AD-f079c196), core graph firmware (AD-45f32bab), and task-first audit trail (AD-7fa3f280) decisions."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
docs:
  - "DOC-28344cd7"
acceptance:
  - "Every AD from AD-7121ec20 to AD-306eccf1 has been reviewed"
  - "Superseded decisions have status superseded and superseded-by set"
  - "New decisions (AD-f079c196/039/040) have supersedes set where applicable"
  - "No one-sided supersessions exist (RULE-b10fe6d1 compliance)"
  - "Summary table of audit findings exists in this task's body"
relationships:
  - target: "EPIC-f079c196"
    type: "delivers"
    rationale: "Task belongs to this epic"
---
## What

[AD-f079c196](AD-f079c196) (graph-based knowledge injection), [AD-45f32bab](AD-45f32bab) (core graph firmware), and [AD-7fa3f280](AD-7fa3f280)
(task-first audit trail with configurable epic requirement) represent a significant
architectural shift. Several earlier decisions may now be:

- **Superseded**: The new decision replaces the old one entirely
- **Partially affected**: The old decision still holds but some aspects changed
- **Unaffected**: The old decision is independent of the new architecture

## Known Candidates for Review

These decisions are likely affected based on what [AD-f079c196](AD-f079c196)/039/040 change:

| Decision | Title | Likely Impact |
|----------|-------|---------------|
| [AD-c1e5a39e](AD-c1e5a39e) | Three-tier skill model | May be affected by graph-based skill discovery |
| [AD-48b310f9](AD-48b310f9) | Universal agent roles | Likely unaffected — roles are orthogonal to injection |
| [AD-859ed163](AD-859ed163) | SQLite for conversations only | Likely unaffected — governance stays file-based |
| [AD-80f39962](AD-80f39962) | Enforcement engine | May be affected by graph-based enforcement |
| [AD-a47f313a](AD-a47f313a) | Plugin architecture | May be affected by plugin reading graph |
| [AD-e8ea9fb9](AD-e8ea9fb9) | Companion plugin | Likely affected — plugin now reads graph |
| [AD-03d9007d](AD-03d9007d) | Rule enforcement entries | May be affected by self-enforcing rules |
| [AD-306eccf1](AD-306eccf1) | Capability-based agents | Likely unaffected — orthogonal to injection |

All other ADs should still be reviewed for completeness.

## How

1. Read each AD from [AD-7121ec20](AD-7121ec20) through [AD-306eccf1](AD-306eccf1)
2. Evaluate against [AD-f079c196](AD-f079c196)/039/040 changes
3. Mark superseded decisions with proper status and cross-references
4. Update both sides of any supersession in the same commit

## Verification

- All ADs reviewed and verdicts recorded in summary table below
- No one-sided supersessions
- All superseded decisions have correct status and cross-references

## Output

No full supersessions found. [AD-f079c196](AD-f079c196)/039/040 are additive/evolutionary, not replacements. Six ADs are partially affected:

| AD | Title | Verdict | Notes |
|----|-------|---------|-------|
| [AD-7121ec20](AD-7121ec20) | Thick Backend Architecture | Unaffected | |
| [AD-4e7faf0e](AD-4e7faf0e) | IPC Boundary Design | Unaffected | |
| [AD-2d58941b](AD-2d58941b) | Error Propagation via Result Types | Unaffected | |
| [AD-ecc96aef](AD-ecc96aef) | Svelte 5 Runes Only | Unaffected | |
| [AD-75bb14ae](AD-75bb14ae) | SQLite for All Structured Persistence | Already superseded | By [AD-859ed163](AD-859ed163) prior to this audit |
| [AD-9a7d7256](AD-9a7d7256) | Component Purity | Unaffected | |
| [AD-09fc4e65](AD-09fc4e65) | Agent SDK Sidecar Integration | Unaffected | |
| [AD-fc4e9013](AD-fc4e9013) | Max Subscription Authentication | Unaffected | |
| [AD-39e2fb81](AD-39e2fb81) | Streaming Pipeline | Unaffected | |
| [AD-e4a3b5da](AD-e4a3b5da) | Tool Implementation as MCP | Unaffected | |
| [AD-d01b9e0a](AD-d01b9e0a) | Security Model | Unaffected | |
| [AD-5d0f8814](AD-5d0f8814) | Tauri Plugin Selections | Unaffected | |
| [AD-33e315cc](AD-33e315cc) | Frontend Library Selections | Unaffected | |
| [AD-b08f456d](AD-b08f456d) | Persistence Architecture | Already superseded | By [AD-859ed163](AD-859ed163) prior to this audit |
| [AD-0dfa4d52](AD-0dfa4d52) | Governance Artifact Format | Already superseded | By [AD-4ea9a290](AD-4ea9a290) prior to this audit |
| [AD-23e27cf5](AD-23e27cf5) | Onboarding Strategy | Unaffected | |
| [AD-af88bb69](AD-af88bb69) | Composability Principle | Unaffected | |
| [AD-85d45674](AD-85d45674) | Four-Zone Layout | Already superseded | By [AD-7cb83077](AD-7cb83077) prior to this audit |
| [AD-7cb83077](AD-7cb83077) | Three-Zone + Nav Sub-Panel Layout | Unaffected | |
| [AD-f9fbd1d7](AD-f9fbd1d7) | Filesystem-Driven Doc Browsing | Unaffected | |
| [AD-4ea9a290](AD-4ea9a290) | .orqa/ as Single Source of Truth | Partially affected | [AD-45f32bab](AD-45f32bab) adds firmware/project layering — core artifacts within .orqa/ are read-only firmware |
| [AD-45cfe1d1](AD-45cfe1d1) | Config-Driven Artifact Scanning | Partially affected | [AD-f079c196](AD-f079c196) adds docs/skills/sources schema fields the scanner must surface |
| [AD-3b986859](AD-3b986859) | Plans Merged Into Research Schema | Partially affected | [AD-f079c196](AD-f079c196) adds sources field to research schema |
| [AD-7d3d7521](AD-7d3d7521) | Native Search Engine | Unaffected | |
| [AD-02a2a97b](AD-02a2a97b) | Provider-Agnostic AI Integration | Unaffected | |
| [AD-306d7320](AD-306d7320) | Domain Service Extraction Pattern | Unaffected | |
| [AD-e711446e](AD-e711446e) | Vision Evolution | Unaffected | |
| [AD-c1e5a39e](AD-c1e5a39e) | Three-Tier Skill Loading | Partially affected | Tier 2 mechanism changes from orchestrator table to graph edges (task.docs/skills) |
| [AD-48b310f9](AD-48b310f9) | Universal Roles, Domain-Specific Skills | Unaffected | |
| [AD-26b0eb9f](AD-26b0eb9f) | Skill-Driven Project Initialisation | Partially affected | Must now configure workflow.epics-required during setup (AD-7fa3f280) |
| [AD-74a2cb7a](AD-74a2cb7a) | Pillars as First-Class Planning Artifacts | Unaffected | |
| [AD-859ed163](AD-859ed163) | SQLite for Conversation Persistence Only | Unaffected | |
| [AD-80f39962](AD-80f39962) | Core UI Boundary | Unaffected | |
| [AD-a47f313a](AD-a47f313a) | Schema-Driven Artifact Filtering | Partially affected | New schema fields from [AD-f079c196](AD-f079c196) appear as filter options; core schemas protected per [AD-45f32bab](AD-45f32bab) |
| [AD-e8ea9fb9](AD-e8ea9fb9) | Config-Driven Navigation Defaults | Unaffected | |
| [AD-03d9007d](AD-03d9007d) | Cross-Linking as Default Behaviour | Unaffected | |
| [AD-306eccf1](AD-306eccf1) | AI-Driven Cross-Artifact Search | Unaffected | |