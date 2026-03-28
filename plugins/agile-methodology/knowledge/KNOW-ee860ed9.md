---
id: KNOW-ee860ed9
type: knowledge
title: Enforcement Patterns
description: Available enforcement mechanisms from strongest to weakest — validator checks, blocking hooks, warning hooks, stop hooks, thinking modes, agent prompts.
summary: "Available enforcement mechanisms from strongest to weakest — validator checks, blocking hooks, warning hooks, stop hooks, thinking modes, agent prompts."
status: active
relationships:
  - target: AGENT-7a06d10e
    type: employed-by
---

# Enforcement Patterns

## Available Mechanisms (strongest to weakest)

1. **Validator check** (`orqa enforce`) — fails the validation pipeline. Strongest: blocks commits via pre-commit hook.
2. **PreToolUse blocking hook** — prevents the action before it happens. Connector-specific.
3. **PostToolUse warning hook** — warns after the action. Non-blocking but visible.
4. **Stop hook check** — runs at session end. Catches accumulated violations.
5. **Thinking mode injection** — adds guidance to every prompt. Behavioral, not mechanical.
6. **Agent system prompt** — one-time context at session start. Weakest: easily overridden by conversation pressure.

## Choosing an Enforcement Level

- **Schema rules** (field requirements, types): validator check
- **Code quality rules** (no unwrap, no any): linter + pre-commit hook
- **Process rules** (documentation first, no stubs): PreToolUse hook
- **Behavioral rules** (never stop, trace to usage): prompt classification + behavioral rule injection + stop hook check
- **Safety rules** (no force push, no --no-verify): PreToolUse blocking hook

## Escalation Path

When enforcement fails (lesson recurrence >= 3):

- Prompt → hook (add mechanical check)
- Warning hook → blocking hook (prevent, not just warn)
- Hook → validator check (blocks the pipeline)
- Add to pre-commit hook chain if not already there

## Connector-Specific Enforcement

Each connector provides its own enforcement primitives. For Claude Code:

- `PreToolUse` hooks: can block Write/Edit/Bash
- `PostToolUse` hooks: can warn after Write/Edit/Bash
- `UserPromptSubmit` hooks: classify prompt type + inject behavioral rules
- `Stop` hooks: session-end checks
- `SubagentStop` hooks: review subagent work

## Linter Delegation

Per [RULE-42d17086](RULE-42d17086), OrqaStudio delegates code quality enforcement to linters. Do NOT add file-based enforcement for patterns that clippy or ESLint already catch. Instead:

- Add a `lint` entry in the rule's frontmatter documenting the delegation
- Ensure the linter rule is configured to match the documented standard
- Reference the linter rule name so violations can be traced to the standard

## Enforcement Frontmatter Format

Rules track enforcement in their frontmatter:

```yaml
enforcement:
  - event: file          # file write/edit
    action: block        # block | warn | inject
    paths: ["**/*.rs"]
    pattern: "unwrap\\(\\)"
    message: "Use Result<T,E> with thiserror instead of unwrap()"
  - event: lint
    tool: clippy
    rule: "clippy::unwrap_used"
    standard: "No unwrap() in production code"
  - event: hook
    trigger: PreToolUse
    tool: Bash
    pattern: "--no-verify"
    action: block
    message: "Never bypass pre-commit hooks"
```

## Gap Detection

An enforcement gap exists when:

- A rule has `enforcement: []` or no enforcement field
- A rule's enforcement is documentation-only (no mechanical check)
- A lesson recurs 3+ times without being blocked by tooling

Enforcement gaps in CRITICAL rules are always highest priority — never deferred.
