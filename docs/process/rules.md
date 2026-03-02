# Enforcement Rules Reference

**Date:** 2026-03-02

This page lists all 20 enforcement rules in `.claude/rules/` and 11 hookify enforcement files in `.claude/`, explains the rule injection mechanism, and clarifies the relationship between rules, hookify, and documentation.

---

## How Rules Work

Rules files in `.claude/rules/` are **automatically injected into every Claude Code session** -- both the orchestrator session and every subagent session. They enforce behavioral constraints that apply universally, regardless of the task or agent.

Rules are the last line of defense: they exist to catch violations that other mechanisms (docs, agent instructions, skills) might miss.

**Rules are NOT documentation.** They do not define product knowledge, coding standards, or architectural patterns. They point to the documentation that defines those things, and enforce that agents follow them.

---

## Rule Inventory

| # | Rule File | Purpose |
|---|-----------|---------|
| 1 | `vision-alignment.md` | Every feature must serve Pillar 1 (Self-Learning Loop) or Pillar 2 (Process Governance). Foundational principles are immutable. UX-first design. |
| 2 | `architecture-decisions.md` | Architecture decisions must be read before coding. Lists critical decisions (error propagation, IPC boundary, component purity, type safety, immutability, Svelte 5 only, SQLite for structured data). |
| 3 | `chunkhound-usage.md` | ChunkHound tools must be preferred over Grep/Glob for multi-file searches. Mandatory documentation review before implementation. |
| 4 | `coding-standards.md` | References `docs/development/coding-standards.md`. Rust standards (rustfmt, clippy pedantic, thiserror, no unwrap). TypeScript/Svelte standards (Svelte 5 runes only, strict TS, shadcn-svelte, no emoji in UI). 80%+ coverage. |
| 5 | `documentation-first.md` | Documentation is the source of truth. No code before docs exist. Re-read governing docs at the start of every phase. Bug investigation protocol. No deprecated documentation. |
| 6 | `end-to-end-completeness.md` | Every feature must span all 4 layers (Rust command + IPC type + Svelte component + store binding). The IPC boundary rule: Tauri `invoke()` is the ONLY interface. |
| 7 | `error-ownership.md` | All errors are the agent's responsibility. Never assume, always verify. No backwards compatibility shims. |
| 8 | `git-workflow.md` | Worktree lifecycle, branch naming, data loss prevention, stash policy, background process discipline, untracked files policy, post-merge verification. |
| 9 | `honest-reporting.md` | Reporting partial work as complete is worse than reporting it incomplete. The "Would It Work" test. Precise status categories (Done, Partially done, Scaffolded, Not started). |
| 10 | `lessons-learned.md` | Two learning loops: implementation lessons (`development/lessons.md`) and process retrospectives (`process/retrospectives.md`). Promotion pipeline at recurrence >= 2. Review agent output requirements. |
| 11 | `no-aliases-or-hacks.md` | Fix root causes, not symptoms. No alias entries, shims, normalizer maps, serde aliases to tolerate wrong names, or widened type unions. One canonical identifier per concept across Rust and TypeScript. |
| 12 | `no-stubs.md` | No hardcoded fake data, no-op handlers, always-success functions, or placeholder implementations. Stub scanner enforcement. Mandatory agent completion report structure (What Was Done, What Is NOT Done, Evidence, Smoke Test). |
| 13 | `pillar-alignment-docs.md` | Every feature/workflow/capability documentation page must include a Pillar Alignment section mapping to Pillar 1 and/or Pillar 2. |
| 14 | `plan-mode-compliance.md` | Every plan requires architectural compliance verification, systems architecture checklist, and UX-first design. Three-reviewer verification gate. Evidence requirements for Tauri commands, frontend components, and IPC wiring. |
| 15 | `required-reading.md` | Every agent must read its Required Reading documentation before implementation. Missing documents halt work. |
| 16 | `reusable-components.md` | Shared components (EmptyState, LoadingSpinner, ErrorDisplay, PageToolbar, StatusBadge, ProgressBar, Panel, CodeBlock, MarkdownRenderer, ConversationMessage, ToolCallCard) must be used. No inline equivalents. |
| 17 | `root-cleanliness.md` | Project root stays lean. Temporary files to `tmp/`. Documentation to `docs/`. Tools to `tools/`. Only configuration files that require root placement. |
| 18 | `sidebar-synchronization.md` | All sidebar files must be updated together whenever any page is added, moved, or removed. Canonical section order enforced. |
| 19 | `skill-enforcement.md` | Every agent must have a `skills:` list. `chunkhound` is universal. Agent-maintainer audits skill lists against Required Reading domains. |
| 20 | `testing-standards.md` | Test organization (Rust unit/integration, Vitest, Playwright E2E). 80%+ coverage. Mock only at adapter boundaries. Test isolation requirements. |

---

## Hookify Enforcement Files

In addition to the 20 rules above, Forge uses 11 hookify enforcement files (`.claude/hookify.*.local.md`) for real-time pattern-based enforcement. Where rules are written instructions injected into agent context, hookify files actively block or warn when a file edit or bash command matches a forbidden pattern.

**How hookify works:**

Each hookify file is a markdown file with YAML frontmatter that defines:

- **`event`** — `file` (triggers on file edits) or `bash` (triggers on bash commands)
- **`action`** — `block` (prevents the action) or `warn` (allows but displays a warning)
- **`conditions`** / **`pattern`** — The regex or glob pattern to match against

**Hookify Inventory:**

| # | Hookify File | Event | Action | Purpose |
|---|-------------|-------|--------|---------|
| 1 | `hookify.no-any-type.local.md` | file | block | Blocks `: any` type annotations in TypeScript files |
| 2 | `hookify.no-unwrap.local.md` | file | block | Blocks `unwrap()` and `expect()` in Rust production code |
| 3 | `hookify.no-todo-macro.local.md` | file | block | Blocks `todo!()` macro in Rust production code |
| 4 | `hookify.no-svelte4-patterns.local.md` | file | block | Blocks Svelte 4 patterns (`$:`, `export let`) in Svelte files |
| 5 | `hookify.no-todo-comments.local.md` | file | block | Blocks TODO/FIXME/HACK comments in production code |
| 6 | `hookify.no-console-log.local.md` | file | warn | Warns on `console.log` in TypeScript/Svelte files |
| 7 | `hookify.no-no-verify.local.md` | bash | block | Blocks `--no-verify` flag in git commands |
| 8 | `hookify.warn-destructive-git.local.md` | bash | warn | Warns on destructive git commands (`reset --hard`, `clean -fd`, `checkout .`) |
| 9 | `hookify.no-force-push.local.md` | bash | block | Blocks `git push --force` to main/master |
| 10 | `hookify.no-ts-ignore.local.md` | file | block | Blocks `@ts-ignore` and `@ts-nocheck` directives |
| 11 | `hookify.no-hardcoded-secrets.local.md` | file | block | Blocks patterns resembling hardcoded API keys and secrets |

**Relationship to rules:**

Rules define constraints as written instructions ("No `unwrap()` in production code"). Hookify files enforce a subset of those constraints in real-time by pattern-matching against file edits and commands. Not every rule has a corresponding hookify file — only rules with violations detectable by pattern matching are promoted to hookify enforcement.

| Rule | Enforced by Hookify |
|------|-------------------|
| `coding-standards.md` — No `unwrap()` in production | `hookify.no-unwrap.local.md` (block) |
| `coding-standards.md` — Svelte 5 runes only | `hookify.no-svelte4-patterns.local.md` (block) |
| `coding-standards.md` — Strict TypeScript, no `any` | `hookify.no-any-type.local.md` (block) |
| `no-stubs.md` — No TODO comments | `hookify.no-todo-comments.local.md` (block) |
| `git-workflow.md` — No `--no-verify` | `hookify.no-no-verify.local.md` (block) |
| `git-workflow.md` — No destructive git without approval | `hookify.warn-destructive-git.local.md` (warn) |
| `error-ownership.md` — No `todo!()` macro | `hookify.no-todo-macro.local.md` (block) |

---

## Rule Injection Mechanism

Claude Code automatically injects the contents of every `.md` file in `.claude/rules/` into the system prompt of every session. This happens before the user's first message. There is no explicit loading step -- rules are always active.

Because rules are injected verbatim, they must be:

- **Concise** -- verbose rules fill context and may be partially ignored
- **Enforcement-focused** -- behavioral constraints, not documentation
- **Reference-based** -- point to docs for the full standard, don't restate it

---

## Relationship to Documentation

Rules enforce the standards defined in `docs/`. They do not replace them.

| If you want to... | Use... |
|-------------------|--------|
| Define a coding standard | `docs/development/coding-standards.md` |
| Define an architectural decision | `docs/architecture/decisions.md` |
| Define an IPC contract | `docs/architecture/` or feature-specific docs |
| Enforce that agents follow a standard | `.claude/rules/` |
| Teach a technology pattern | `.claude/skills/` |

---

## When to Create a New Rule

Create a new rule when:

1. A behavioral constraint applies to ALL agents universally (not just one agent's process)
2. An implementation lesson has recurred enough times to warrant automatic enforcement (recurrence >= 2 per `development/lessons.md`)
3. A process change is significant enough that agents would violate it without automatic reminders

When creating a rule, also consider whether the constraint can be additionally enforced via hookify. If the violation is a specific pattern in code or commands (e.g., a forbidden function call, a banned flag), create a hookify file alongside the rule for real-time prevention.

Do NOT create a new rule when:

- The constraint applies only to one agent -- put it in that agent's instructions
- The constraint is a product/architecture standard -- put it in `docs/`
- The existing rules already cover the constraint -- extend an existing rule instead

---

## Rule Maintenance

Rules are maintained by the `agent-maintainer` agent. When documentation changes, the agent-maintainer reviews whether any rules need updating to stay consistent with the new docs.

The `code-reviewer` includes rule compliance in every code review:

- Does any committed code violate an enforcement rule?
- Do any rule files reference deleted or moved documentation pages?
- Are there new recurring patterns that should be promoted to rules?

---

## Related Documents

- [Content Governance](/process/content-governance) -- The six-layer ownership model
- [Team Overview](/process/team) -- Which agents load which skills and follow which rules
- [Process Retrospectives](/process/retrospectives) -- History of rule creation and governance changes
- [Implementation Lessons](/development/lessons) -- Individual patterns that may be promoted to rules
