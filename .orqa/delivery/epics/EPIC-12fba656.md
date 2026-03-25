---
id: "EPIC-12fba656"
type: "epic"
title: "Close enforcement bootstrapping gap"
description: "The enforcement system can't enforce the project's principles because: (1) documentation is isolated from the artifact graph — agents can't traverse to the knowledge they need, (2) no grounding mechanism exists — agents lose purpose under implementation pressure, (3) the orchestrator has no delegation reference — work types aren't mapped to roles and skills, and (4) mechanical enforcement gaps remain — stop events, skill injection, and write-time integrity aren't wired. This epic closes all four gaps so the system can enforce itself during its own development."
status: "completed"
priority: "P1"
created: "2026-03-14"
updated: "2026-03-14"
deadline: null
horizon: "active"
scoring:
  impact: 5
  urgency: 5
  complexity: 5
  dependencies: 5
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---
## Context

During a heavy implementation session (31 tasks across 6 epics), the orchestrator lost awareness of the project's core principles — pillars, vision, architectural constraints. Investigation revealed four structural gaps:

1. **Documentation is isolated from the graph.** 55% of docs have zero relationships. Agents can't traverse from a skill to its deeper documentation. 8 significant duplications exist. 23 files contain stale phase references. ([RES-f664f528](RES-f664f528))
2. **No grounding mechanism exists.** Agent prompts are 100% procedural — they describe how to work but not why the work matters. Under implementation pressure, purpose evaporates because it was never anchored.
3. **The orchestrator has no delegation reference.** Work types aren't mapped to roles, skills, and grounding. Delegation is a judgement call instead of a lookup. The orchestrator doing implementation work is a sign of system failure, but nothing enforces this.
4. **Mechanical enforcement gaps remain.** Stop events aren't evaluated, skill injection returns names not content, write-time graph integrity isn't checked. ([RES-b0268020](RES-b0268020))

The consequence: **the system's principles are lost during implementation because the knowledge infrastructure that should maintain them is disconnected, ungrounded, and mechanically incomplete.**

## Implementation Design

### Phase 1: Documentation Restructuring

Fix the documentation so it's worth connecting to the graph. Delete duplicates, merge overlaps, remove stale content, clarify unfocused docs.

#### Deletions
- DOC-019 (architecture-overview) — stub, duplicates DOC-001
- DOC-054 (launch-timeline) — entirely outdated
- DOC-032 (process/rules) — duplicates RULE-dd5b69e6

#### Merges
- DOC-038 (governance-hub) → DOC-06224bf6 (governance)
- DOC-e42efeaf (guide/workflow) → DOC-db5b37dc (process/workflow)
- DOC-048 (component-inventory) → DOC-2c94f7ba (svelte-components)
- DOC-3d8ed14e (artifact-types) → DOC-28344cd7 (artifact-framework)

#### Restructures
- DOC-9814ec3c (coding-standards) — restructure as principles doc, not rule restatement
- DOC-d9cc1f84 (orchestration) — add purpose, reduce to delegation reference
- Remove all "Phase 2a/2b" references across 23 files
- Clarify or delete: DOC-051 (engagement-infrastructure), DOC-9010239f (metrics), DOC-045 (system-artifacts)

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
- Add `informed-by` relationships from rules/decisions to their documentation pages
- Link wireframe docs to epics via `docs-required`
- Add pillar alignment sections to UI and wireframe docs per RULE-05562ed4

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

- [ ] [TASK-ae0051a6](TASK-ae0051a6): Delete duplicate and stale documentation (DOC-019, DOC-054, DOC-032)
- [ ] [TASK-0ba4dedd](TASK-0ba4dedd): Merge overlapping documentation (4 merge pairs)
- [ ] [TASK-97d5ed5f](TASK-97d5ed5f): Restructure unfocused documentation and remove stale phase references

### Phase 2: Grounding and Graph Connectivity

- [ ] [TASK-1da659da](TASK-1da659da): Create grounding documents (5 role-area docs)
- [ ] [TASK-98f928c3](TASK-98f928c3): Create delegation reference document
- [ ] [TASK-60c0568d](TASK-60c0568d): Define Governance Steward agent
- [ ] [TASK-80951d17](TASK-80951d17): Connect documentation to graph (relationships on docs, skills, agents)

### Phase 3: Mechanical Enforcement (CLI Plugin)

- [ ] [TASK-982c5fbf](TASK-982c5fbf): Add stop event handling to rule-engine.mjs
- [ ] [TASK-c4b04937](TASK-c4b04937): Implement full skill content injection in rule-engine.mjs
- [ ] [TASK-f583bda2](TASK-f583bda2): Add bidirectional relationship checking to graph-guardian.mjs
- [ ] [TASK-a29adcd7](TASK-a29adcd7): Add grounding injection to plugin — resolve `grounded-by` on agent, inject content
- [ ] [TASK-40fb1279](TASK-40fb1279): Integration test — verify all enforcement entries are consumed

### Phase 4: App Enforcement Pipeline

- [ ] [TASK-8b51938b](TASK-8b51938b): Wire EnforcementEngine to agent tool approval pipeline
- [ ] [TASK-775e9f10](TASK-775e9f10): Unify process gates and enforcement engine evaluation
- [ ] [TASK-d4dade11](TASK-d4dade11): Surface violations in governance UI

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