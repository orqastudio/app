---

id: EPIC-064
title: "Close enforcement bootstrapping gap"
description: "The enforcement system is half-complete across all layers — rules declare enforcement entries but critical event types (stop) and actions (skill content injection) are not consumed by the plugin, and the Rust engine is disconnected from agent execution. This epic closes every gap so the system can enforce itself during its own development."
status: draft
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
deadline: null
milestone: MS-002
horizon: active
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on:
  - EPIC-050
blocks: []
research-refs:
  - RES-056
docs-required:
  - RES-056
docs-produced: []
scoring:
  dogfood-value: "5 — OrqaStudio cannot dogfood until the enforcement system enforces itself"
  user-facing: "4 — enforcement quality directly affects agent output quality"
  foundation: "5 — closes the circular dependency that undermines all governance"
  complexity: "3 — most infrastructure exists, this is wiring and extending"
  score: 4.5
relationships:
  - target: EPIC-050
    type: informed-by
    rationale: "EPIC-050 built the enforcement engine; this epic closes the gaps it left"
  - target: AD-048
    type: enforces
    rationale: "AD-048 requires enforcement to accompany rule promotion — this epic implements that requirement across all layers"
  - target: RES-056
    type: informed-by
    rationale: "Research document analyzing the bootstrapping gap drives this epic's scope"
  - target: IMPL-054
    type: informed-by
    rationale: "Orchestrator bypassing enforcement system — symptom this epic prevents"
  - target: IMPL-055
    type: informed-by
    rationale: "Missing graph integrity at write-time — symptom this epic prevents"
  - target: EPIC-050
    type: informs
    rationale: "Auto-generated inverse of informs relationship from EPIC-050"
  - target: RES-057
    type: informed-by
    rationale: "Integrity check alignment audit informed enforcement bootstrapping scope"
---
## Context

OrqaStudio's enforcement system (EPIC-050) built the infrastructure: a Claude Code plugin with rule-engine.mjs handling file/bash events, a Rust EnforcementEngine with full pattern matching, and process gates tracking workflow state. But critical gaps remain:

1. **Stop events**: 3 enforcement entries declared (RULE-001, RULE-044), none evaluated by the plugin
2. **Skill injection**: Plugin returns skill names, not content — agents get a list of IDs instead of actual knowledge
3. **Graph integrity at write-time**: RULE-045 declared enforcement entries but PostToolUse hook doesn't run integrity checks
4. **App-agent pipeline**: Rust enforcement engine is complete but not wired to agent execution
5. **Process gate / enforcement engine separation**: Two enforcement systems in Rust that don't compose

The consequence: **the orchestrator repeatedly violates behavioral rules** (IMPL-052 permission-seeking, IMPL-054 bypassing enforcement, IMPL-055 missing graph integrity) because the enforcement that should catch these violations isn't consuming the entries that declare them.

## Implementation Design

### Phase 1: Plugin Enforcement Completeness (CLI Context)

Close all gaps in the Claude Code plugin so enforcement entries are fully consumed.

#### Stop Event Support

Currently: Stop hook → stop-checklist.sh (static checklist, no rule evaluation)
Target: Stop hook → rule-engine.mjs (evaluates `event: stop` entries) + stop-checklist.sh (operational checklist)

**Approach**: Add stop event handling to rule-engine.mjs:
- Accept stop event context (no file_path — session-level context only)
- Evaluate enforcement entries with `event: stop` against session state
- Return warn/inject verdicts alongside stop-checklist.sh output
- Stop hook calls both scripts, merges output

#### Full Skill Content Injection

Currently: Returns `"**Read before implementing:**\n- skill-name-1\n- skill-name-2"`
Target: Reads skill SKILL.md files and returns their body content as systemMessage

**Approach**: In rule-engine.mjs `collectSkillIds()`:
- Resolve skill name → `.orqa/process/skills/{name}/SKILL.md`
- Read file, strip YAML frontmatter
- Return body content as systemMessage
- Deduplication already works via `.injected-skills.json`

#### Graph Integrity on PostToolUse

Currently: graph-guardian.mjs does basic checks
Target: After `.orqa/**/*.md` writes, run targeted bidirectional relationship verification

**Approach**: In graph-guardian.mjs:
- When modified file matches `.orqa/**/*.md`: parse frontmatter relationships
- For each relationship, check if target artifact has the inverse
- Return warning if inverses are missing (don't block — inform)
- Lightweight: single-file check, no full scan

### Phase 2: App Enforcement Pipeline (App Context)

Wire the Rust EnforcementEngine to agent execution so app-context enforcement achieves parity with CLI.

#### Connect Engine to Agent Tool Approval

The EnforcementEngine exists in `backend/src-tauri/src/domain/enforcement_engine.rs`. It needs to be called:
- Before tool execution in the agent loop (pre-tool-use evaluation)
- After tool execution (post-tool-use evaluation)
- At session boundaries (stop event evaluation)

#### Unify Process Gates and Enforcement Engine

Process gates (`process_gates.rs`) track workflow state. Enforcement engine (`enforcement_engine.rs`) evaluates patterns. Compose them:
- Enforcement entries can reference workflow state conditions
- Process gates can evaluate enforcement entries instead of hardcoded conditions
- Single evaluation pipeline: workflow state → enforcement entries → verdicts

## Tasks

### Phase 1: Plugin Completeness

- [ ] [TASK-411](TASK-411): Add stop event handling to rule-engine.mjs
- [ ] [TASK-412](TASK-412): Implement full skill content injection in rule-engine.mjs
- [ ] [TASK-413](TASK-413): Add bidirectional relationship checking to graph-guardian.mjs
- [ ] [TASK-414](TASK-414): Integration test — verify all declared enforcement entries are consumed

### Phase 2: App Enforcement Pipeline

- [ ] [TASK-415](TASK-415): Wire EnforcementEngine to agent tool approval pipeline
- [ ] [TASK-416](TASK-416): Unify process gates and enforcement engine evaluation

## Verification

1. All `event: stop` entries on RULE-001 and RULE-044 fire when the plugin's Stop hook runs
2. Skill injection returns actual skill content, not just skill names
3. Writing an `.orqa/` artifact with a one-sided relationship triggers a PostToolUse warning
4. Rust enforcement engine evaluates tool calls during agent execution in app context
5. `make verify` passes clean after all changes

## Out of Scope

- Prompt event enforcement (no rules declare it yet)
- Scan/lint event types in plugin (declarative only, handled by linters)
- Plugin distribution or registry
- Cross-project rule sharing
