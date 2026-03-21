---
id: "TASK-4bc73133"
title: "Audit existing architecture decisions against AD-2783985c/039/040"
description: "Review AD-e513c9e4 through AD-0f6286cd to identify which decisions are superseded, affected, or made defunct by the graph-based knowledge injection (AD-2783985c), core graph firmware (AD-e8ab8572), and task-first audit trail (AD-6dfbba70) decisions."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
docs:
  - "DOC-01ddd8aa"
acceptance:
  - "Every AD from AD-e513c9e4 to AD-0f6286cd has been reviewed"
  - "Superseded decisions have status superseded and superseded-by set"
  - "New decisions (AD-2783985c/039/040) have supersedes set where applicable"
  - "No one-sided supersessions exist (RULE-7b770593 compliance)"
  - "Summary table of audit findings exists in this task's body"
relationships:
  - target: "EPIC-dc1e3e4b"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-dd9c8538"
    type: "depended-on-by"
---
## What

[AD-2783985c](AD-2783985c) (graph-based knowledge injection), [AD-e8ab8572](AD-e8ab8572) (core graph firmware), and [AD-6dfbba70](AD-6dfbba70)
(task-first audit trail with configurable epic requirement) represent a significant
architectural shift. Several earlier decisions may now be:

- **Superseded**: The new decision replaces the old one entirely
- **Partially affected**: The old decision still holds but some aspects changed
- **Unaffected**: The old decision is independent of the new architecture

## Known Candidates for Review

These decisions are likely affected based on what [AD-2783985c](AD-2783985c)/039/040 change:

| Decision | Title | Likely Impact |
|----------|-------|---------------|
| [AD-53e80192](AD-53e80192) | Three-tier skill model | May be affected by graph-based skill discovery |
| [AD-774cc3d0](AD-774cc3d0) | Universal agent roles | Likely unaffected — roles are orthogonal to injection |
| [AD-2aa4d6db](AD-2aa4d6db) | SQLite for conversations only | Likely unaffected — governance stays file-based |
| [AD-0291fa65](AD-0291fa65) | Enforcement engine | May be affected by graph-based enforcement |
| [AD-89391ab6](AD-89391ab6) | Plugin architecture | May be affected by plugin reading graph |
| [AD-2f1991c2](AD-2f1991c2) | Companion plugin | Likely affected — plugin now reads graph |
| [AD-d8ea4d2b](AD-d8ea4d2b) | Rule enforcement entries | May be affected by self-enforcing rules |
| [AD-0f6286cd](AD-0f6286cd) | Capability-based agents | Likely unaffected — orthogonal to injection |

All other ADs should still be reviewed for completeness.

## How

1. Read each AD from [AD-e513c9e4](AD-e513c9e4) through [AD-0f6286cd](AD-0f6286cd)
2. Evaluate against [AD-2783985c](AD-2783985c)/039/040 changes
3. Mark superseded decisions with proper status and cross-references
4. Update both sides of any supersession in the same commit

## Verification

- All ADs reviewed and verdicts recorded in summary table below
- No one-sided supersessions
- All superseded decisions have correct status and cross-references

## Output

No full supersessions found. [AD-2783985c](AD-2783985c)/039/040 are additive/evolutionary, not replacements. Six ADs are partially affected:

| AD | Title | Verdict | Notes |
|----|-------|---------|-------|
| [AD-e513c9e4](AD-e513c9e4) | Thick Backend Architecture | Unaffected | |
| [AD-a334623b](AD-a334623b) | IPC Boundary Design | Unaffected | |
| [AD-1ad08e5f](AD-1ad08e5f) | Error Propagation via Result Types | Unaffected | |
| [AD-8d552e96](AD-8d552e96) | Svelte 5 Runes Only | Unaffected | |
| [AD-dffc3d30](AD-dffc3d30) | SQLite for All Structured Persistence | Already superseded | By [AD-2aa4d6db](AD-2aa4d6db) prior to this audit |
| [AD-61087142](AD-61087142) | Component Purity | Unaffected | |
| [AD-dc919e52](AD-dc919e52) | Agent SDK Sidecar Integration | Unaffected | |
| [AD-22650b3b](AD-22650b3b) | Max Subscription Authentication | Unaffected | |
| [AD-fcd55d44](AD-fcd55d44) | Streaming Pipeline | Unaffected | |
| [AD-0dbba717](AD-0dbba717) | Tool Implementation as MCP | Unaffected | |
| [AD-5d9ac6bd](AD-5d9ac6bd) | Security Model | Unaffected | |
| [AD-834fc71a](AD-834fc71a) | Tauri Plugin Selections | Unaffected | |
| [AD-afc78f6e](AD-afc78f6e) | Frontend Library Selections | Unaffected | |
| [AD-8b91f5a4](AD-8b91f5a4) | Persistence Architecture | Already superseded | By [AD-2aa4d6db](AD-2aa4d6db) prior to this audit |
| [AD-69072318](AD-69072318) | Governance Artifact Format | Already superseded | By [AD-6ce44025](AD-6ce44025) prior to this audit |
| [AD-4047ceb1](AD-4047ceb1) | Onboarding Strategy | Unaffected | |
| [AD-1d928079](AD-1d928079) | Composability Principle | Unaffected | |
| [AD-40b3de0a](AD-40b3de0a) | Four-Zone Layout | Already superseded | By [AD-4f5277f0](AD-4f5277f0) prior to this audit |
| [AD-4f5277f0](AD-4f5277f0) | Three-Zone + Nav Sub-Panel Layout | Unaffected | |
| [AD-8b7c4ac5](AD-8b7c4ac5) | Filesystem-Driven Doc Browsing | Unaffected | |
| [AD-6ce44025](AD-6ce44025) | .orqa/ as Single Source of Truth | Partially affected | [AD-e8ab8572](AD-e8ab8572) adds firmware/project layering — core artifacts within .orqa/ are read-only firmware |
| [AD-0c56aa90](AD-0c56aa90) | Config-Driven Artifact Scanning | Partially affected | [AD-2783985c](AD-2783985c) adds docs/skills/sources schema fields the scanner must surface |
| [AD-c3700062](AD-c3700062) | Plans Merged Into Research Schema | Partially affected | [AD-2783985c](AD-2783985c) adds sources field to research schema |
| [AD-99c2a969](AD-99c2a969) | Native Search Engine | Unaffected | |
| [AD-ff88ecea](AD-ff88ecea) | Provider-Agnostic AI Integration | Unaffected | |
| [AD-6cd1ff6f](AD-6cd1ff6f) | Domain Service Extraction Pattern | Unaffected | |
| [AD-44a033cf](AD-44a033cf) | Vision Evolution | Unaffected | |
| [AD-53e80192](AD-53e80192) | Three-Tier Skill Loading | Partially affected | Tier 2 mechanism changes from orchestrator table to graph edges (task.docs/skills) |
| [AD-774cc3d0](AD-774cc3d0) | Universal Roles, Domain-Specific Skills | Unaffected | |
| [AD-6f0dea5e](AD-6f0dea5e) | Skill-Driven Project Initialisation | Partially affected | Must now configure workflow.epics-required during setup (AD-6dfbba70) |
| [AD-47c41f0c](AD-47c41f0c) | Pillars as First-Class Planning Artifacts | Unaffected | |
| [AD-2aa4d6db](AD-2aa4d6db) | SQLite for Conversation Persistence Only | Unaffected | |
| [AD-0291fa65](AD-0291fa65) | Core UI Boundary | Unaffected | |
| [AD-89391ab6](AD-89391ab6) | Schema-Driven Artifact Filtering | Partially affected | New schema fields from [AD-2783985c](AD-2783985c) appear as filter options; core schemas protected per [AD-e8ab8572](AD-e8ab8572) |
| [AD-2f1991c2](AD-2f1991c2) | Config-Driven Navigation Defaults | Unaffected | |
| [AD-d8ea4d2b](AD-d8ea4d2b) | Cross-Linking as Default Behaviour | Unaffected | |
| [AD-0f6286cd](AD-0f6286cd) | AI-Driven Cross-Artifact Search | Unaffected | |
