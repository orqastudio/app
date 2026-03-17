---
id: EPIC-052
title: Structured Thinking Enforcement
description: |
  Shift OrqaStudio's enforcement system from code-pattern regex matching to
  structured thinking process enforcement. Process gates inject thinking prompts
  at workflow transitions. Knowledge injection auto-loads domain skills based on
  file paths and prompt intent. Tooling ecosystem management delegates code quality
  to linters while OrqaStudio manages the full chain from documented intent to
  linter config to hook trigger.
status: completed
priority: P1
created: 2026-03-11
updated: 2026-03-12
deadline: null
horizon: null
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 4
relationships:
  - target: MS-002
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-191
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-192
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-193
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-194
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-195
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-196
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-197
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-198
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-199
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-200
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-201
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-202
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-203
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-204
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-205
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-206
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-207
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-342
    type: delivered-by
    rationale: Epic contains this task
  - target: DOC-002
    type: informed-by
    rationale: Referenced in documentation page Enforcement Architecture
  - target: MS-001
    type: delivers
  - target: EPIC-050
    type: depends-on
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-002
    type: grounded-by
  - target: RULE-041
    type: informed-by
  - target: RULE-042
    type: informed-by
  - target: RULE-043
    type: informed-by
  - target: EPIC-053
    type: depended-on-by
  - target: PILLAR-001
    type: grounded-by
---

## Context

OrqaStudio is a clarity engine. Its core value proposition is turning messy situations
into structured understanding through a defined thinking process. The enforcement system
should enforce THIS PROCESS — not replicate what linters already do.

Current state: 40 rules, 4 with regex enforcement (unwrap, --no-verify, TODO comments,
raw cargo). The Rust enforcement engine and CLI plugin both exist and work. But
enforcement is focused on code patterns that linters should handle.

The shift: OrqaStudio's enforcement system should be about injecting structured thinking
at the right moments. Code quality enforcement belongs in ESLint/clippy/svelte-check —
OrqaStudio should configure and invoke those tools, not regex-match their patterns.
OrqaStudio's unique enforcement is the PROCESS: understand → plan → document →
implement → review → learn.

## Implementation Design

### Four Enforcement Layers

**Layer 1 — Process Gates**: Enforce the structured thinking sequence at workflow
transitions. A WorkflowTracker tracks session events (reads, writes, searches, skills
loaded). Gates fire when transitions violate the expected sequence.

**Layer 2 — Knowledge Injection**: Auto-load domain skills when agents touch specific
code areas. Path-based injection maps file patterns to skills. New `inject` action in
enforcement entries returns skill content as `systemMessage`.

**Layer 3 — Tooling Ecosystem**: OrqaStudio manages linter configuration to match
documented standards. Skills describe how to configure tools. Rules reference linter
configs instead of regex-matching patterns linters already cover.

**Layer 4 — Prompt-Based Injection**: Interpret user prompts to determine needed skills
before work begins. Uses AI intent classification (CLI: fast model call; App: local
embeddings).

### Schema Changes

Add to enforcement schema:
- `inject` action type (alongside `block` and `warn`)
- `lint` event type (documents linter delegation)
- `skills` field on enforcement entries (list of skill names to inject)

### WorkflowTracker

Rust struct tracking per-session events: files read, files written, searches performed,
docs consulted, skills loaded, commands run. Process gates query this tracker to
determine if prerequisites are met before allowing transitions.

## Tasks

### Setup
- [ ] [TASK-191](TASK-191): Move rule-enforcement skill to core + rename plugin skill
- [ ] [TASK-192](TASK-192): Update plugin README

### Engine
- [ ] [TASK-193](TASK-193): Add `inject` action to enforcement schema
- [ ] [TASK-194](TASK-194): Add `lint` event type to enforcement schema

### Process Gates
- [ ] [TASK-195](TASK-195): Design WorkflowTracker domain type
- [ ] [TASK-196](TASK-196): Implement understand-first + docs-before-code gates
- [ ] [TASK-197](TASK-197): Implement plan-before-build + structure-before-code gates
- [ ] [TASK-198](TASK-198): Implement evidence-before-done + learn-after-doing gates

### Knowledge Injection
- [ ] [TASK-199](TASK-199): Implement skill injection in plugin rule-engine.mjs
- [ ] [TASK-200](TASK-200): Implement skill injection in Rust enforcement engine
- [ ] [TASK-201](TASK-201): Add injection map entries to relevant rules

### Tooling Ecosystem
- [ ] [TASK-202](TASK-202): Document linter-to-standard mapping
- [ ] [TASK-203](TASK-203): Consolidate code-pattern rules

### Prompt-Based Injection
- [ ] [TASK-204](TASK-204): Add prompt event handler to plugin rule-engine.mjs
- [ ] [TASK-205](TASK-205): Add prompt-based injection to Rust system prompt builder

### New Rules + Verification
- [ ] [TASK-206](TASK-206): Create [RULE-041](RULE-041), [RULE-042](RULE-042), [RULE-043](RULE-043)
- [ ] [TASK-207](TASK-207): Integration testing

## Out of Scope

- Replacing ESLint/clippy with OrqaStudio enforcement — linters stay as the code quality layer
- Runtime enforcement within the app's AI conversations (that's a separate streaming concern)
- Custom rule authoring UI (future epic)
