---
id: EPIC-064
title: Close enforcement bootstrapping gap
description: "The enforcement system can't enforce the project's principles because: (1) documentation is isolated from the artifact graph — agents can't traverse to the knowledge they need, (2) no grounding mechanism exists — agents lose purpose under implementation pressure, (3) the orchestrator has no delegation reference — work types aren't mapped to roles and skills, and (4) mechanical enforcement gaps remain — stop events, skill injection, and write-time integrity aren't wired. This epic closes all four gaps so the system can enforce itself during its own development."
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
deadline: null
milestone: MS-002
horizon: active
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on: []
blocks: []
research-refs:
  - RES-056
  - RES-062
docs-required:
  - RES-056
  - RES-062
docs-produced: []
scoring:
  dogfood-value: 5 — OrqaStudio cannot dogfood until the enforcement system enforces itself
  user-facing: 4 — enforcement quality directly affects agent output quality
  foundation: 5 — closes the circular dependency that undermines all governance
  complexity: 3 — most infrastructure exists, this is wiring and extending
  score: 4.5
relationships:
  - target: EPIC-050
    type: informed-by
    rationale: EPIC-050 built the enforcement engine; this epic closes the gaps it left
  - target: AD-048
    type: enforces
    rationale: AD-048 requires enforcement to accompany rule promotion — this epic implements that requirement across all layers
  - target: RES-056
    type: informed-by
    rationale: Research document analyzing the bootstrapping gap drives this epic's scope
  - target: IMPL-054
    type: informed-by
    rationale: Orchestrator bypassing enforcement system — symptom this epic prevents
  - target: IMPL-055
    type: informed-by
    rationale: Missing graph integrity at write-time — symptom this epic prevents
  - target: EPIC-050
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-050
  - target: RES-057
    type: informed-by
    rationale: Integrity check alignment audit informed enforcement bootstrapping scope
  - target: EPIC-066
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from EPIC-066
  - target: MS-002
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-411
    type: contains
    rationale: Epic contains this task
  - target: TASK-412
    type: contains
    rationale: Epic contains this task
  - target: TASK-413
    type: contains
    rationale: Epic contains this task
  - target: TASK-414
    type: contains
    rationale: Epic contains this task
  - target: TASK-415
    type: contains
    rationale: Epic contains this task
  - target: TASK-416
    type: contains
    rationale: Epic contains this task
  - target: TASK-411
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-411"
  - target: TASK-415
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-415"
  - target: TASK-412
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-412"
  - target: TASK-416
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-416"
  - target: TASK-414
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-414"
  - target: TASK-413
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-413"
  - target: TASK-190
    type: contains
    rationale: Absorbed from EPIC-050 — surface violations in governance UI
  - target: TASK-190
    type: delivered-by
    rationale: Absorbed from EPIC-050 — surface violations in governance UI
  - target: TASK-460
    type: contains
    rationale: Phase 1 — delete duplicate documentation
  - target: TASK-460
    type: delivered-by
    rationale: Phase 1 — delete duplicate documentation
  - target: TASK-461
    type: contains
    rationale: Phase 1 — merge overlapping documentation
  - target: TASK-461
    type: delivered-by
    rationale: Phase 1 — merge overlapping documentation
  - target: TASK-462
    type: contains
    rationale: Phase 1 — restructure unfocused documentation
  - target: TASK-462
    type: delivered-by
    rationale: Phase 1 — restructure unfocused documentation
  - target: TASK-463
    type: contains
    rationale: Phase 2 — create grounding documents
  - target: TASK-463
    type: delivered-by
    rationale: Phase 2 — create grounding documents
  - target: TASK-464
    type: contains
    rationale: Phase 2 — create delegation reference
  - target: TASK-464
    type: delivered-by
    rationale: Phase 2 — create delegation reference
  - target: TASK-465
    type: contains
    rationale: Phase 2 — define Governance Steward agent
  - target: TASK-465
    type: delivered-by
    rationale: Phase 2 — define Governance Steward agent
  - target: TASK-466
    type: contains
    rationale: Phase 2 — connect documentation to graph
  - target: TASK-466
    type: delivered-by
    rationale: Phase 2 — connect documentation to graph
  - target: TASK-467
    type: contains
    rationale: Phase 3 — grounding injection in plugin
  - target: TASK-467
    type: delivered-by
    rationale: Phase 3 — grounding injection in plugin
  - target: IDEA-095
    type: informed-by
    rationale: Documentation-as-graph-knowledge idea drove the expanded scope
  - target: RES-062
    type: informed-by
    rationale: Documentation audit findings shaped the restructuring plan
  - target: EPIC-073
    type: informs
    rationale: "Auto-generated inverse of informs relationship from EPIC-073"
  - target: EPIC-075
    type: informs
    rationale: "Auto-generated inverse of informs relationship from EPIC-075"
---
## Context

During a heavy implementation session (31 tasks across 6 epics), the orchestrator lost awareness of the project's core principles — pillars, vision, architectural constraints. Investigation revealed four structural gaps:

1. **Documentation is isolated from the graph.** 55% of docs have zero relationships. Agents can't traverse from a skill to its deeper documentation. 8 significant duplications exist. 23 files contain stale phase references. ([RES-062](RES-062))
2. **No grounding mechanism exists.** Agent prompts are 100% procedural — they describe how to work but not why the work matters. Under implementation pressure, purpose evaporates because it was never anchored.
3. **The orchestrator has no delegation reference.** Work types aren't mapped to roles, skills, and grounding. Delegation is a judgement call instead of a lookup. The orchestrator doing implementation work is a sign of system failure, but nothing enforces this.
4. **Mechanical enforcement gaps remain.** Stop events aren't evaluated, skill injection returns names not content, write-time graph integrity isn't checked. ([RES-056](RES-056))

The consequence: **the system's principles are lost during implementation because the knowledge infrastructure that should maintain them is disconnected, ungrounded, and mechanically incomplete.**

## Implementation Design

### Phase 1: Documentation Restructuring

Fix the documentation so it's worth connecting to the graph. Delete duplicates, merge overlaps, remove stale content, clarify unfocused docs.

#### Deletions
- DOC-019 (architecture-overview) — stub, duplicates DOC-001
- DOC-054 (launch-timeline) — entirely outdated
- DOC-032 (process/rules) — duplicates RULE-026

#### Merges
- DOC-038 (governance-hub) → DOC-039 (governance)
- DOC-082 (guide/workflow) → DOC-035 (process/workflow)
- DOC-048 (component-inventory) → DOC-016 (svelte-components)
- DOC-081 (artifact-types) → DOC-036 (artifact-framework)

#### Restructures
- DOC-021 (coding-standards) — restructure as principles doc, not rule restatement
- DOC-030 (orchestration) — add purpose, reduce to delegation reference
- Remove all "Phase 2a/2b" references across 23 files
- Clarify or delete: DOC-051 (engagement-infrastructure), DOC-029 (metrics), DOC-045 (system-artifacts)

### Phase 2: Grounding Documents and Delegation Reference

Create grounding documents distilled from restructured docs. Create the orchestrator's delegation reference. Connect everything to the graph.

#### Grounding Documents (one per role area)
- `grounding/product-purpose.md` — mission, pillars, identity (grounds: Orchestrator, Planner, Writer)
- `grounding/code-principles.md` — what "good code" means (grounds: Implementer, Reviewer)
- `grounding/artifact-principles.md` — what "good artifacts" look like (grounds: Orchestrator, Writer, Researcher)
- `grounding/design-principles.md` — UX principles (grounds: Designer)
- `grounding/research-principles.md` — evidence standards (grounds: Researcher)

Each is 30-50 lines. Answers three questions: why this role exists, what "good" looks like, what goes wrong under pressure.

#### Delegation Reference
A new doc in `documentation/reference/delegation.md` — the orchestrator's lookup table:
- Maps every work type to: agent role, required skills, grounding document
- Connected to orchestrator via `grounded-by`
- Makes "if the orchestrator is writing anything other than coordination output, the system has failed" explicit and actionable
- Includes the Governance Steward as the owner of ALL `.orqa/` artifact creation and maintenance

#### Governance Steward Agent
A new specialist agent (`governance-steward.md`) that owns all `.orqa/` artifact work:
- **Purpose**: Create and maintain governance artifacts with correct frontmatter, relationships, pillar alignment, and schema compliance
- **Grounding**: artifact-principles, product-purpose
- **Skills**: orqa-governance, orqa-documentation, orqa-schema-compliance, migration-tooling
- **Key behavior**: automatically maintains bidirectional relationships, validates against schema.json before writing, enforces pillar alignment on every artifact
- **Capabilities**: file_read, file_write, file_edit, file_search, content_search, code_search_regex
- The orchestrator provides content intent; the steward handles the writing with full graph discipline

#### Graph Connectivity
- Add `grounded-by` relationships on all agent definitions → their grounding docs
- Add `informs`/`informed-by` relationships between skills and their documentation
- Add `documented-by` relationships from rules/decisions to their documentation pages
- Link wireframe docs to epics via `docs-required`
- Add pillar alignment sections to UI and wireframe docs per RULE-021

### Phase 3: Mechanical Enforcement (CLI Plugin)

Close gaps in the Claude Code plugin so enforcement entries are fully consumed.

#### Stop Event Support
Stop hook → rule-engine.mjs (evaluates `event: stop` entries) + stop-checklist.sh

#### Full Skill Content Injection
rule-engine.mjs reads SKILL.md files and returns body content as systemMessage, not just names.

#### Graph Integrity on PostToolUse
graph-guardian.mjs checks bidirectional relationship inverses after `.orqa/**/*.md` writes.

#### Grounding Injection
Plugin resolves `grounded-by` relationships on agent definitions and injects target content at session initialization. This is the mechanical implementation of the grounding pattern.

### Phase 4: App Enforcement Pipeline

Wire the Rust EnforcementEngine to agent execution for app-context enforcement parity.

## Tasks

### Phase 1: Documentation Restructuring

- [ ] [TASK-460](TASK-460): Delete duplicate and stale documentation (DOC-019, DOC-054, DOC-032)
- [ ] [TASK-461](TASK-461): Merge overlapping documentation (4 merge pairs)
- [ ] [TASK-462](TASK-462): Restructure unfocused documentation and remove stale phase references

### Phase 2: Grounding and Graph Connectivity

- [ ] [TASK-463](TASK-463): Create grounding documents (5 role-area docs)
- [ ] [TASK-464](TASK-464): Create delegation reference document
- [ ] [TASK-465](TASK-465): Define Governance Steward agent
- [ ] [TASK-466](TASK-466): Connect documentation to graph (relationships on docs, skills, agents)

### Phase 3: Mechanical Enforcement (CLI Plugin)

- [ ] [TASK-411](TASK-411): Add stop event handling to rule-engine.mjs
- [ ] [TASK-412](TASK-412): Implement full skill content injection in rule-engine.mjs
- [ ] [TASK-413](TASK-413): Add bidirectional relationship checking to graph-guardian.mjs
- [ ] [TASK-467](TASK-467): Add grounding injection to plugin — resolve `grounded-by` on agent, inject content
- [ ] [TASK-414](TASK-414): Integration test — verify all enforcement entries are consumed

### Phase 4: App Enforcement Pipeline

- [ ] [TASK-415](TASK-415): Wire EnforcementEngine to agent tool approval pipeline
- [ ] [TASK-416](TASK-416): Unify process gates and enforcement engine evaluation
- [ ] [TASK-190](TASK-190): Surface violations in governance UI

## Verification

1. Documentation tree has zero duplicates, zero stale phase references, zero orphaned docs
2. Every agent definition has `grounded-by` relationships to its grounding documents
3. The orchestrator prompt includes purpose, pillars, and delegation reference
4. The plugin injects grounding content at session initialization
5. Skill injection returns actual content, not just names
6. Writing an `.orqa/` artifact with a one-sided relationship triggers a PostToolUse warning
7. All `event: stop` enforcement entries fire during the Stop hook
8. `make verify` passes clean after all changes

## Out of Scope

- Prompt event enforcement (no rules declare it yet)
- Scan/lint event types in plugin (declarative only, handled by linters)
- Plugin distribution or registry
- Cross-project rule sharing
- App-native grounding injection (plugin handles CLI; app will follow in a later epic)
