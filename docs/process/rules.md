---
title: "Enforcement Rules Reference"
category: process
tags: []
created: 2026-03-02
updated: 2026-03-05
---

# Enforcement Rules Reference

**Date:** 2026-03-05

This page describes the rule enforcement model, lists all active rules in `.claude/rules/`, and explains how rules, frontmatter, and generated hooks relate to each other.

---

## How Rules Work

Rule files in `.claude/rules/` serve two purposes simultaneously:

1. **Context injection** — Every `.md` file in `.claude/rules/` is automatically injected into every Claude Code session as part of the system prompt. Rules describe behavioral constraints that agents must follow.

2. **Enforcement definitions** — Each rule file carries YAML frontmatter that declares machine-readable enforcement entries. The app reads this frontmatter to power the enforcement engine (pattern evaluation, block/warn verdicts, violation persistence) and to auto-generate CLI-compatible hook scripts.

The rule file IS the enforcement definition. There is no separate enforcement configuration — the frontmatter and the human-readable rule body are one artifact.

---

## Rule Frontmatter Schema

```yaml
---
enforcement:
  - id: RULE-NNN-001          # Unique identifier for this enforcement entry
    description: "Brief description of what is being enforced"
    event: file | bash         # What triggers this check
    action: block | warn       # What happens on a match
    pattern: "regex pattern"   # Pattern to match against file content or command text
    scope: system | project    # Whether this rule applies everywhere or only to a specific project
---
```

A single rule file can declare multiple enforcement entries — one per pattern it enforces. Rules without frontmatter enforcement entries are context-only: they inform agent behavior but are not programmatically enforced.

---

## Enforcement Engine

When the app loads a project, it reads YAML frontmatter from all rule files in `.claude/rules/` and compiles them into an in-memory enforcement set.

**Evaluation flow:**

```text
Agent tool call (file write or bash command)
  → Enforcement engine receives the action
    → For each enforcement entry matching the event type:
        → Evaluate pattern against file content or command text
          → Match found:
              → block: action is prevented, violation is recorded
              → warn:  action proceeds, warning is surfaced, violation is recorded
          → No match: action proceeds silently
```

**Violation persistence:** Every block and warn event is written to the `violations` SQLite table with the rule ID, entry ID, matched text snippet, timestamp, and session ID. The enforcement panel queries this table for display.

---

## CLI Backwards Compatibility

The same frontmatter that drives the app's enforcement engine also generates hookify-compatible `.local.md` files in `.claude/`. These generated files allow the enforcement rules to work in plain Claude Code CLI sessions without running the OrqaStudio™ app.

**Generation:** When rule frontmatter changes, the app re-generates the corresponding hookify files. The generated files are named `hookify.<rule-id>-<entry-id>.local.md`.

**The hookify files are derived artifacts** — they are generated from rule frontmatter and should not be edited directly. Edit the rule's frontmatter; the hookify files update automatically.

---

## Rule Inventory

| # | Rule File | Enforcement Entries | Purpose |
|---|-----------|--------------------:|---------|
| 1 | `vision-alignment.md` | 0 | Every feature must serve Pillar 1 or Pillar 2. Foundational principles are immutable. UX-first design. Context-only — requires judgment, cannot be pattern-matched. |
| 2 | `architecture-decisions.md` | 0 | Architecture decisions must be read before coding. Lists critical decisions (error propagation, IPC boundary, component purity, type safety, immutability, Svelte 5 only, SQLite for structured data). Context-only. |
| 3 | `chunkhound-usage.md` | 0 | ChunkHound tools must be preferred over Grep/Glob for multi-file searches. Mandatory documentation review before implementation. Context-only. |
| 4 | `coding-standards.md` | 3 | References `docs/development/coding-standards.md`. Rust: no `unwrap()`, no `todo!()`. TypeScript: no `: any`, no Svelte 4 patterns. Pattern-enforced via frontmatter. |
| 5 | `documentation-first.md` | 0 | Documentation is the source of truth. No code before docs exist. Re-read governing docs at the start of every phase. Context-only. |
| 6 | `end-to-end-completeness.md` | 0 | Every feature must span all 4 layers (Rust command + IPC type + Svelte component + store binding). Context-only. |
| 7 | `error-ownership.md` | 1 | All errors are the agent's responsibility. Pattern-enforced: blocks `todo!()` macro in Rust. |
| 8 | `git-workflow.md` | 3 | Worktree lifecycle, data loss prevention, stash policy, background process discipline. Pattern-enforced: no `--no-verify`, warn destructive git commands, block force push. |
| 9 | `honest-reporting.md` | 0 | Partial work must not be reported as complete. The "Would It Work" test. Context-only. |
| 10 | `lessons-learned.md` | 0 | Two learning loops: implementation lessons and process retrospectives. Promotion pipeline. Context-only. |
| 11 | `no-aliases-or-hacks.md` | 0 | Fix root causes, not symptoms. No alias entries, shims, normalizer maps. Context-only. |
| 12 | `no-stubs.md` | 1 | No hardcoded fake data or no-op handlers. Pattern-enforced: blocks TODO/FIXME/HACK comments in production code. |
| 13 | `pillar-alignment-docs.md` | 0 | Every feature/workflow/capability doc must include a Pillar Alignment section. Context-only. |
| 14 | `plan-mode-compliance.md` | 0 | Every plan requires architectural compliance verification, systems architecture checklist, and UX-first design. Context-only. |
| 15 | `required-reading.md` | 0 | Every agent must read its Required Reading documentation before implementation. Context-only. |
| 16 | `reusable-components.md` | 0 | Shared components must be used — no inline equivalents. Context-only. |
| 17 | `root-cleanliness.md` | 0 | Project root stays lean. Temporary files to `tmp/`. Context-only. |
| 18 | `skill-enforcement.md` | 0 | Every agent must have a `skills:` list. `chunkhound` is universal. Context-only. |
| 19 | `testing-standards.md` | 0 | Test organization, 80%+ coverage, mock only at adapter boundaries. Context-only. |

---

## Context-Only vs Pattern-Enforced

| Type | How It Works | Example |
|------|--------------|---------|
| **Context-only** | Injected into agent system prompt. Agents read and follow it. Violations are caught during code review. | "Documentation must exist before implementation begins" |
| **Pattern-enforced** | Has frontmatter enforcement entries. App evaluates patterns against every file write and bash command. Violations are blocked or warned in real time. | Block `: any` type in TypeScript files |

Context-only rules require judgment that pattern matching cannot provide. They are enforced through agent training (the rule text itself) and the code review gate (`code-reviewer` agent). Pattern-enforced rules catch violations at the moment they happen — before a commit, before a review.

---

## When to Create a New Rule

Create a new rule when:

1. A behavioral constraint applies to all agents universally, not just one agent's process
2. An implementation lesson has recurred enough times to warrant automatic enforcement (recurrence >= 2 per `.orqa/lessons/`)
3. A process change is significant enough that agents would violate it without automatic reminders

When creating a rule, also decide whether the constraint is pattern-enforceable. If the violation is a specific pattern in file content or a banned command flag, add frontmatter enforcement entries. The generated hookify files will be created automatically.

Do NOT create a new rule when:

- The constraint applies only to one agent — put it in that agent's instructions
- The constraint is a product or architecture standard — put it in `docs/`
- Existing rules already cover the constraint — extend an existing rule instead

---

## Rule Maintenance

Rules are maintained by the `agent-maintainer` agent. When documentation changes, the agent-maintainer reviews whether any rules need updating to stay consistent with the new docs.

The `code-reviewer` includes rule compliance in every code review:

- Does any committed code violate an enforcement rule?
- Do any rule files reference deleted or moved documentation pages?
- Are there new recurring patterns that should be promoted to rules?

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | N/A — this page describes the rule structure, not a learning feature. The lessons-learned rule drives learning loop behavior. |
| Process Governance | Rule files are the primary mechanism through which behavioral standards are enforced on all agents. This page documents that enforcement model, making governance tangible and auditable. |

---

## Related Documents

- [Content Governance](content-governance.md) — The six-layer ownership model (docs, agents, skills, rules, hooks, enforcement)
- [Team Overview](team.md) — Which agents load which skills and follow which rules
- [Process Retrospectives](retrospectives.md) — History of rule creation and governance changes
- `.orqa/lessons/` — Individual patterns that may be promoted to rules
- `docs/architecture/enforcement.md` — Technical architecture of the enforcement engine
