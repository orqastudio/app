---
id: "EPIC-4440cdd4"
title: "Structured Thinking Enforcement"
description: "Shift OrqaStudio's enforcement system from code-pattern regex matching to

  structured thinking process enforcement. Process gates inject thinking prompts

  at workflow transitions. Knowledge injection auto-loads domain skills based on

  file paths and prompt intent. Tooling ecosystem management delegates code quality

  to linters while OrqaStudio manages the full chain from documented intent to

  linter config to hook trigger.\n"
status: "completed"
priority: "P1"
created: "2026-03-11"
updated: "2026-03-12"
deadline: null
horizon: null
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 4
relationships:
  - target: "MS-eea45fa8"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-fde17fff"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-835e2645"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-49c63248"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-4f45e5b9"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ff48daa1"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-15370e74"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-4fa7dd50"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-8232a533"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-6cd46196"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-439fa554"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-50b3aa55"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-05357368"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2bbc5077"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-24ef68ce"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2143a39a"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-965b2b81"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-34007190"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-d6727d2f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "MS-654badde"
    type: "fulfils"
  - target: "DOC-4db3a417"
    type: "documented-by"
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
- [ ] [TASK-fde17fff](TASK-fde17fff): Move rule-enforcement skill to core + rename plugin skill
- [ ] [TASK-835e2645](TASK-835e2645): Update plugin README

### Engine
- [ ] [TASK-49c63248](TASK-49c63248): Add `inject` action to enforcement schema
- [ ] [TASK-4f45e5b9](TASK-4f45e5b9): Add `lint` event type to enforcement schema

### Process Gates
- [ ] [TASK-ff48daa1](TASK-ff48daa1): Design WorkflowTracker domain type
- [ ] [TASK-15370e74](TASK-15370e74): Implement understand-first + docs-before-code gates
- [ ] [TASK-4fa7dd50](TASK-4fa7dd50): Implement plan-before-build + structure-before-code gates
- [ ] [TASK-8232a533](TASK-8232a533): Implement evidence-before-done + learn-after-doing gates

### Knowledge Injection
- [ ] [TASK-6cd46196](TASK-6cd46196): Implement skill injection in plugin rule-engine.mjs
- [ ] [TASK-439fa554](TASK-439fa554): Implement skill injection in Rust enforcement engine
- [ ] [TASK-50b3aa55](TASK-50b3aa55): Add injection map entries to relevant rules

### Tooling Ecosystem
- [ ] [TASK-05357368](TASK-05357368): Document linter-to-standard mapping
- [ ] [TASK-2bbc5077](TASK-2bbc5077): Consolidate code-pattern rules

### Prompt-Based Injection
- [ ] [TASK-24ef68ce](TASK-24ef68ce): Add prompt event handler to plugin rule-engine.mjs
- [ ] [TASK-2143a39a](TASK-2143a39a): Add prompt-based injection to Rust system prompt builder

### New Rules + Verification
- [ ] [TASK-965b2b81](TASK-965b2b81): Create [RULE-c95f4444](RULE-c95f4444), [RULE-f9d0279c](RULE-f9d0279c), [RULE-7f416d7d](RULE-7f416d7d)
- [ ] [TASK-34007190](TASK-34007190): Integration testing

## Out of Scope

- Replacing ESLint/clippy with OrqaStudio enforcement — linters stay as the code quality layer
- Runtime enforcement within the app's AI conversations (that's a separate streaming concern)
- Custom rule authoring UI (future epic)
