---
id: KNOW-2a846fb7
type: knowledge
title: OrqaStudio Enforcement Engine
domain: methodology/governance
description: Understands how OrqaStudio governance rules are mechanically enforced via the
status: active
created: 2026-03-11
updated: 2026-03-11
category: domain
version: 0.2.0
user-invocable: false
relationships:
  - target: DOC-9814ec3c
    type: synchronised-with
tier: "on-demand"
roles:
  - "implementer"
  - "reviewer"
tags:
  - "enforcement"
  - "engine"
  - "architecture"
priority: "P2"
summary: |
  OrqaStudio enforcement engine: how governance rules are mechanically enforced
  via the validation pipeline, hooks, and thinking modes.
---

# Rule Enforcement

OrqaStudio enforces governance rules through four layers, each serving a different purpose.

## Layer 1: Process Gates

Process gates enforce the structured thinking sequence at workflow transitions. They
fire when agents skip steps in the process: understand → plan → document → implement →
review → learn.

| Gate | Fires When | Effect |
| ------ | ----------- | -------- |
| understand-first | First code write with no prior research | Systems thinking prompt injected |
| docs-before-code | Code write without reading governing docs | Documentation prompt injected |
| plan-before-build | Code write without epic/task context | Planning prompt injected |
| evidence-before-done | Task complete without verification evidence | Review prompt injected |
| learn-after-doing | Session ending without lesson check | Learning prompt injected |

Gates fire once per session and inject thinking prompts as `systemMessage`. They
nudge rather than block.

## Layer 2: Knowledge Injection

When agents touch specific code areas, domain skills are auto-injected via enforcement
entries with `action: inject`. Path patterns map to skill names.

```yaml
enforcement:
  - event: file
    paths: ["app/src-tauri/src/domain/**"]
    action: inject
    skills: [orqa-domain-services, orqa-error-composition]
    message: "Domain service and error composition patterns loaded."
```

Injected skills are deduplicated per turn — editing two files in the same domain area
only injects the skills once.

## Layer 3: Tooling Ecosystem

OrqaStudio delegates code quality enforcement to linters (clippy, ESLint, svelte-check)
and documents the delegation via `event: lint` enforcement entries.

```yaml
enforcement:
  - event: lint
    pattern: "clippy::unwrap_used"
    action: warn
    message: "No unwrap() in production Rust code — enforced by clippy pedantic."
```

`lint` entries don't execute anything — they document that a standard is enforced by an
external tool. This closes the traceability gap: documented standard → linter rule →
hook trigger.

## Layer 4: Prompt-Based Injection

User prompts are classified for intent, and relevant domain skills are injected before
work begins. Uses AI classification (CLI: fast model) or semantic similarity (App: local
embeddings).

## Enforcement Entry Format

```yaml
enforcement:
  - event: file|bash|prompt|stop|lint
    pattern: "regex or linter rule name"
    paths: ["glob patterns"]
    action: block|warn|inject
    skills: [skill-name-1, skill-name-2]
    message: "Message shown when triggered"
```

### Event Types

| Event | Triggered By | Pattern Matched Against |
| ------- | ------------- | ------------------------ |
| `file` | Write, Edit tool calls | File content |
| `bash` | Bash tool calls | Command string |
| `prompt` | UserPromptSubmit hook | User message |
| `stop` | Stop hook | Session context |
| `lint` | Declarative — not executed | Documents linter delegation |

### Actions

| Action | Behavior |
| -------- | ---------- |
| `block` | Tool call denied with message |
| `warn` | Tool call proceeds, message shown as warning |
| `inject` | Skills loaded into context as systemMessage |

### Fields

| Field | Required | Description |
| ------- | ---------- | ------------- |
| `event` | Yes | Which event type triggers this entry |
| `pattern` | No | Regex (file/bash) or linter rule name (lint) |
| `paths` | No | Glob patterns restricting which files match |
| `action` | Yes | block, warn, or inject |
| `skills` | No | Skill names to inject (only for `action: inject`) |
| `message` | Yes | Message shown when triggered |

## Implementation

The app's Rust enforcement engine (`app/src-tauri/src/domain/enforcement_engine.rs`) is the
primary implementation. It sits above the agent layer and applies uniformly regardless
of which agent is running. The CLI companion plugin (`rule-engine.mjs`) is a compatibility
port that brings the same enforcement to Claude Code CLI users.
