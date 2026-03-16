---
id: TASK-222
title: Audit existing architecture decisions against AD-038/039/040
description: "Review AD-001 through AD-037 to identify which decisions are superseded, affected, or made defunct by the graph-based knowledge injection (AD-038), core graph firmware (AD-039), and task-first audit trail (AD-040) decisions."
status: completed
created: 2026-03-12
updated: 2026-03-12
docs:
  - DOC-036
acceptance:
  - Every AD from AD-001 to AD-037 has been reviewed
  - Superseded decisions have status superseded and superseded-by set
  - New decisions (AD-038/039/040) have supersedes set where applicable
  - No one-sided supersessions exist (RULE-004 compliance)
  - "Summary table of audit findings exists in this task's body"
relationships:
  - target: EPIC-053
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-011
    type: grounded-by
  - target: SKILL-038
    type: grounded-by
  - target: TASK-343
    type: depended-on-by
---
## What

[AD-038](AD-038) (graph-based knowledge injection), [AD-039](AD-039) (core graph firmware), and [AD-040](AD-040)
(task-first audit trail with configurable epic requirement) represent a significant
architectural shift. Several earlier decisions may now be:

- **Superseded**: The new decision replaces the old one entirely
- **Partially affected**: The old decision still holds but some aspects changed
- **Unaffected**: The old decision is independent of the new architecture

## Known Candidates for Review

These decisions are likely affected based on what [AD-038](AD-038)/039/040 change:

| Decision | Title | Likely Impact |
|----------|-------|---------------|
| [AD-028](AD-028) | Three-tier skill model | May be affected by graph-based skill discovery |
| [AD-029](AD-029) | Universal agent roles | Likely unaffected — roles are orthogonal to injection |
| [AD-032](AD-032) | SQLite for conversations only | Likely unaffected — governance stays file-based |
| [AD-033](AD-033) | Enforcement engine | May be affected by graph-based enforcement |
| [AD-034](AD-034) | Plugin architecture | May be affected by plugin reading graph |
| [AD-035](AD-035) | Companion plugin | Likely affected — plugin now reads graph |
| [AD-036](AD-036) | Rule enforcement entries | May be affected by self-enforcing rules |
| [AD-037](AD-037) | Capability-based agents | Likely unaffected — orthogonal to injection |

All other ADs should still be reviewed for completeness.

## How

1. Read each AD from [AD-001](AD-001) through [AD-037](AD-037)
2. Evaluate against [AD-038](AD-038)/039/040 changes
3. Mark superseded decisions with proper status and cross-references
4. Update both sides of any supersession in the same commit

## Verification

- All ADs reviewed and verdicts recorded in summary table below
- No one-sided supersessions
- All superseded decisions have correct status and cross-references

## Output

No full supersessions found. [AD-038](AD-038)/039/040 are additive/evolutionary, not replacements. Six ADs are partially affected:

| AD | Title | Verdict | Notes |
|----|-------|---------|-------|
| [AD-001](AD-001) | Thick Backend Architecture | Unaffected | |
| [AD-002](AD-002) | IPC Boundary Design | Unaffected | |
| [AD-003](AD-003) | Error Propagation via Result Types | Unaffected | |
| [AD-004](AD-004) | Svelte 5 Runes Only | Unaffected | |
| [AD-005](AD-005) | SQLite for All Structured Persistence | Already superseded | By [AD-032](AD-032) prior to this audit |
| [AD-006](AD-006) | Component Purity | Unaffected | |
| [AD-007](AD-007) | Agent SDK Sidecar Integration | Unaffected | |
| [AD-008](AD-008) | Max Subscription Authentication | Unaffected | |
| [AD-009](AD-009) | Streaming Pipeline | Unaffected | |
| [AD-010](AD-010) | Tool Implementation as MCP | Unaffected | |
| [AD-011](AD-011) | Security Model | Unaffected | |
| [AD-012](AD-012) | Tauri Plugin Selections | Unaffected | |
| [AD-013](AD-013) | Frontend Library Selections | Unaffected | |
| [AD-014](AD-014) | Persistence Architecture | Already superseded | By [AD-032](AD-032) prior to this audit |
| [AD-015](AD-015) | Governance Artifact Format | Already superseded | By [AD-021](AD-021) prior to this audit |
| [AD-016](AD-016) | Onboarding Strategy | Unaffected | |
| [AD-017](AD-017) | Composability Principle | Unaffected | |
| [AD-018](AD-018) | Four-Zone Layout | Already superseded | By [AD-019](AD-019) prior to this audit |
| [AD-019](AD-019) | Three-Zone + Nav Sub-Panel Layout | Unaffected | |
| [AD-020](AD-020) | Filesystem-Driven Doc Browsing | Unaffected | |
| [AD-021](AD-021) | .orqa/ as Single Source of Truth | Partially affected | [AD-039](AD-039) adds firmware/project layering — core artifacts within .orqa/ are read-only firmware |
| [AD-022](AD-022) | Config-Driven Artifact Scanning | Partially affected | [AD-038](AD-038) adds docs/skills/sources schema fields the scanner must surface |
| [AD-023](AD-023) | Plans Merged Into Research Schema | Partially affected | [AD-038](AD-038) adds sources field to research schema |
| [AD-024](AD-024) | Native Search Engine | Unaffected | |
| [AD-025](AD-025) | Provider-Agnostic AI Integration | Unaffected | |
| [AD-026](AD-026) | Domain Service Extraction Pattern | Unaffected | |
| [AD-027](AD-027) | Vision Evolution | Unaffected | |
| [AD-028](AD-028) | Three-Tier Skill Loading | Partially affected | Tier 2 mechanism changes from orchestrator table to graph edges (task.docs/skills) |
| [AD-029](AD-029) | Universal Roles, Domain-Specific Skills | Unaffected | |
| [AD-030](AD-030) | Skill-Driven Project Initialisation | Partially affected | Must now configure workflow.epics-required during setup (AD-040) |
| [AD-031](AD-031) | Pillars as First-Class Planning Artifacts | Unaffected | |
| [AD-032](AD-032) | SQLite for Conversation Persistence Only | Unaffected | |
| [AD-033](AD-033) | Core UI Boundary | Unaffected | |
| [AD-034](AD-034) | Schema-Driven Artifact Filtering | Partially affected | New schema fields from [AD-038](AD-038) appear as filter options; core schemas protected per [AD-039](AD-039) |
| [AD-035](AD-035) | Config-Driven Navigation Defaults | Unaffected | |
| [AD-036](AD-036) | Cross-Linking as Default Behaviour | Unaffected | |
| [AD-037](AD-037) | AI-Driven Cross-Artifact Search | Unaffected | |
