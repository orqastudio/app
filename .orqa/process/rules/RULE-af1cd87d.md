---
id: RULE-af1cd87d
type: rule
title: Behavioral Rule Enforcement Plan
description: "Defines enforcement strategies for rules that cannot be mechanically checked by linters, hooks, or tooling. Every behavioral rule has a defined enforcement mechanism: prompt injection, output validation, knowledge injection, or session hooks."
status: active
created: 2026-03-13
updated: 2026-03-13
enforcement:
  - mechanism: behavioral
    message: "Every behavioral rule maps to a defined enforcement strategy (prompt injection, output validation, knowledge injection, or session hooks); the orchestrator injects this rule as the authoritative reference"
  - mechanism: tool
    command: "orqa enforce"
    description: "Pipeline integrity tool verifies behavioral rule enforcement coverage"
summary: "Defines enforcement strategies for behavioral rules: (1) prompt injection at delegation time, (2) output validation via post-hoc checks, (3) knowledge injection based on file paths being modified, (4) session hooks at start/end boundaries. 19 behavioral rules mapped across 4 strategies."
tier: on-demand
roles: [orchestrator]
priority: P2
tags: [behavioral-enforcement, strategies, prompt-injection, knowledge-injection]
relationships:
  - target: AD-c6c2d9fb
    type: enforces
---
Rules that cannot be enforced by linters, hooks, or automated tooling still need a defined enforcement mechanism. Every behavioral rule maps to one of four strategies, and each strategy has a concrete implementation path.

## Enforcement Strategies

### Strategy 1: Prompt Injection

Rule content is injected into the agent's context at delegation time. The orchestrator includes the rule's constraints in the delegation prompt, making them part of the agent's active instructions.

| Rule | What is injected |
|------|-----------------|
| [RULE-87ba1b81](RULE-87ba1b81) | Delegation boundaries — orchestrator coordinates, doesn't implement |
| [RULE-0d29fc91](RULE-0d29fc91) | Search usage — prefer semantic search over Grep/Glob |
| [RULE-d2c2063a](RULE-d2c2063a) | Make targets — use make commands, not raw cargo/npm |
| [RULE-25baac14](RULE-25baac14) | ID semantics — IDs are identifiers, not priority rankings |
| [RULE-5965256d](RULE-5965256d) | Required reading — load governing docs before implementation |
| [RULE-dd5b69e6](RULE-dd5b69e6) | Knowledge loading — load knowledge before starting work |
| [RULE-d5d28fba](RULE-d5d28fba) | Structure before work — artifacts must exist before implementation |
| [RULE-ef822519](RULE-ef822519) | Context management — minimize orchestrator context window usage |

**Implementation**: The orchestrator's delegation template includes these rules by reference. The connector's prompt injector (`connectors/claude-code/src/hooks/prompt-injector.ts`) auto-injects relevant rule IDs when task artifacts are referenced.

### Strategy 2: Output Validation

Post-hoc checks on agent output for compliance signals. After an agent completes work, its output is checked for required sections and forbidden language.

| Rule | What is validated |
|------|------------------|
| [RULE-5dd9decd](RULE-5dd9decd) | Honest reporting — check for "What Is NOT Done" section in completion reports |
| [RULE-c603e90e](RULE-c603e90e) | Lessons learned — check for IMPL entries mentioned in review output |
| [RULE-8ee65d73](RULE-8ee65d73) | No deferred deliverables — check completion reports for deferral language ("handled by EPIC-NNN", "wired up later") |
| [RULE-dccf4226](RULE-dccf4226) | Plan compliance — check plan structure for required sections (Architectural Compliance, Systems Architecture Checklist) |

**Implementation**: A `PostToolUse` hook or stop hook scans agent output for compliance markers. Initially implemented as orchestrator self-checks; automated via plugin hooks as patterns stabilize.

### Strategy 3: Knowledge Injection

Domain knowledge is loaded into agent context before work begins on relevant files. The enforcement engine auto-injects knowledge based on file paths being modified.

| Rule | When injected |
|------|--------------|
| [RULE-05ae2ce7](RULE-05ae2ce7) | AD compliance — architecture knowledge injected when modifying cross-boundary code |
| [RULE-ec9462d8](RULE-ec9462d8) | Documentation first — documentation knowledge injected when creating new features |
| [RULE-4603207a](RULE-4603207a) | Enforcement before code — governance knowledge injected when modifying rules/lessons |
| [RULE-43f1bebc](RULE-43f1bebc) | Systems thinking — systems-thinking knowledge injected on all implementation work |
| [RULE-71352dc8](RULE-71352dc8) | UAT process — uat-process knowledge injected during review/testing phases |

**Implementation**: [RULE-e1f1afc1](RULE-e1f1afc1) defines the path-to-knowledge injection map. The connector's knowledge injector (`connectors/claude-code/src/hooks/knowledge-injector.ts`) triggers knowledge injection on Write/Edit.

### Strategy 4: Session Hooks

Plugin hooks that trigger at session boundaries (start, end, stop) to enforce workflow rules.

| Rule | When checked |
|------|-------------|
| [RULE-f609242f](RULE-f609242f) | Git workflow — session-start checks for stashes and untracked files; session-end verifies all changes committed |
| [RULE-30a223ca](RULE-30a223ca) | Session management — session-end checks for uncommitted changes and writes session state |

**Implementation**: The connector's `SessionStart` hook (`connectors/claude-code/hooks/scripts/session-start.sh`) and `Stop` hook (`connectors/claude-code/hooks/scripts/stop-checklist.sh`) enforce these checks.

## Coverage Summary

| Category | Rule Count | Strategy |
|----------|-----------|----------|
| Prompt injection | 8 rules | Delegation template + plugin injector |
| Output validation | 4 rules | Stop hook + orchestrator self-check |
| Knowledge injection | 5 rules | [RULE-e1f1afc1](RULE-e1f1afc1) enforcement entries + plugin PostToolUse |
| Session hooks | 2 rules | Plugin SessionStart + Stop hooks |
| **Total behavioral** | **19 rules** | |

## Verification

To verify behavioral enforcement coverage:
1. Run `node tools/verify-pipeline-integrity.mjs` — reports rules without enforcement chains
2. Run `node tools/verify-enforcement-rules.mjs` — reports agent capability compliance
3. Cross-reference: every rule in this plan should appear in the pipeline integrity tool's enforcement chain data

## Related Rules

- [RULE-e1f1afc1](RULE-e1f1afc1) (knowledge-injection) — implements Strategy 3 via path-to-knowledge mapping
- [RULE-42d17086](RULE-42d17086) (tooling-ecosystem) — distinguishes linter-enforceable from behavioral rules
- [RULE-998da8ea](RULE-998da8ea) (dogfood-mode) — enforcement gap priority on self-enforcing products
