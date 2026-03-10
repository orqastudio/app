---
title: "Content Ownership: Docs, Agents, Skills, and Rules"
description: "Ownership model for documentation, agent definitions, skills, and rules across the project."
created: 2026-03-02
updated: 2026-03-09
---

**Created:** 2026-03-02

OrqaStudio™ uses six distinct layers for governance knowledge: documentation, agent instructions, skills, rules, hooks, and hookify. Each layer owns a specific type of content. Mixing them creates maintenance burden and drift -- when a standard changes in one place, stale copies in other layers remain undetected.

---

## The Six Layers

| Layer | Owns | Examples | Source of Truth For |
|-------|------|----------|---------------------|
| **Documentation (`.orqa/documentation/`)** | Functional and product knowledge: architecture decisions, coding standards, IPC contracts, UI specs | Architecture decisions, function size limits, IPC response format, component state tables | Yes — code that doesn't match docs is wrong |
| **Agent Instructions (`.orqa/team/agents/`)** | Process: how the agent works, which tools it uses, which docs to read first, when to delegate, verification steps | "Run clippy before committing", "Read `.orqa/documentation/architecture/decisions.md` first", "Delegate to test-engineer after implementation" | Process only — agents reference docs, not restate them |
| **Skills (`.orqa/team/skills/`)** | Domain knowledge: how a technology works, general patterns, reusable techniques not specific to OrqaStudio | How Svelte 5 runes work, how to structure a Rust module, how to write a cargo test | Technology patterns only — skills must not contain OrqaStudio-specific architectural rules |
| **Rules (`.orqa/governance/rules/`)** | Enforcement: automated checks and behavioral constraints that apply across all agents | "No stubs", "Error ownership", "End-to-end completeness" | Behavioral constraints — rules reference docs for the standards they enforce |
| **Hooks (`.orqa/governance/hooks/`)** | Automated rule implementation: shell scripts triggered by lifecycle events that enforce rules programmatically | Session-start checklist, skill loading protocol, pre-commit verification | Executable enforcement — hooks are the mechanism through which rules are actively enforced at key lifecycle points |
| **Hookify (`.orqa/hookify/`)** | Real-time enforcement: pattern-based blocks and warnings triggered by file edits and bash commands | Block `any` type in TypeScript, block `todo!()` in Rust, warn on destructive git commands, block `--no-verify` | Action-level constraints — hookify files are the active enforcement mechanism that catches violations at the moment they happen, not at lifecycle boundaries |

---

## Content Placement Rules

### Documentation (`.orqa/documentation/`)

Documentation is the source of truth for **what** the system does and **how** it should be built.

- When a standard changes, change it **here**. Agent instructions that reference the doc pick up the change automatically.
- Never copy a rule from `.orqa/documentation/` into an agent file or skill — reference the doc instead.
- Every architecture decision lives in `.orqa/documentation/architecture/decisions.md`. Agent files do not define decisions; they cite them.

### Agent Instructions (`.orqa/team/agents/`)

Agent files define **process** — the workflow an agent follows to do its job. They do not define the standards themselves.

**Correct agent content:**

```text
Read `.orqa/documentation/development/coding-standards.md` before writing any code.
Run cargo clippy and cargo fmt before committing.
Delegate to the test-engineer agent after implementation.
```

**Forbidden agent content:**

```text
Functions must be <= 50 lines.          <- Belongs in .orqa/documentation/development/coding-standards.md
No backwards compatibility shims.      <- Belongs in .orqa/documentation/development/coding-standards.md
IPC boundary: only invoke()...         <- Belongs in .orqa/documentation/architecture/decisions.md
```

### Skills (`.orqa/team/skills/`)

Skills teach **how a technology works** -- patterns, idioms, and examples from the technology's own documentation and best practices. They are intentionally portable: a Svelte skill should be useful on any Svelte project, not just OrqaStudio.

**Correct skill content:**

```text
How Svelte 5 $state works.
How to write a Rust module with proper error handling.
How to structure a cargo test with test fixtures.
```

**Forbidden skill content:**

```text
IPC boundary: Tauri commands only.            <- Project rule, not technology knowledge
EmptyState component from $lib/components/    <- OrqaStudio-specific, not portable
All Rust functions must return Result.        <- Project architecture rule
```

### Rules (`.orqa/governance/rules/`)

Rules enforce behavioral constraints across all agents. They describe **how agents must behave**, not what the product does. Rules reference documentation for the standards they enforce -- they do not duplicate those standards.

**Correct rule content:**

```text
Before committing, verify all layers exist end-to-end.
All errors are your responsibility -- fix them, don't claim they pre-existed.
```

**Forbidden rule content:**

```text
The IPC response format is: Result<T, String>    <- Belongs in .orqa/documentation/architecture/decisions.md
Functions must be <= 50 lines.                    <- Belongs in .orqa/documentation/development/coding-standards.md
```

### Hooks (`.orqa/governance/hooks/`)

Hooks are **the mechanism through which rules are actively enforced**. Where rules define behavioral constraints as written instructions that agents should follow, hooks implement those constraints as executable shell scripts triggered at specific lifecycle events.

Think of it this way: a rule says "you must do X", a hook makes sure X actually happens.

**Relationship between rules and hooks:**

| Rule | Implemented By Hook | Trigger |
|------|-------------------|---------|
| `skill-enforcement.md` — Load relevant skills before coding | `skill-instructions-hook.sh` — Lists skills, requires LOAD/SKIP decision | `UserPromptSubmit` |
| `required-reading.md` — Read governing docs before implementing | `session-start-hook.sh` — Checks for session state, stale worktrees, stashes | `UserPromptSubmit` (first) |
| `testing-standards.md` — Run tests before committing | `pre-commit-reminder.sh` — Checklist: cargo test, clippy, npm check, no stubs | `Stop` |

**When to use a rule vs a hook vs hookify:**

| Use a Rule When | Use a Hook When | Use Hookify When |
|----------------|-----------------|------------------|
| The constraint is judgement-based ("ensure error handling is comprehensive") | The constraint is checkable at a lifecycle boundary ("run clippy before committing") | The constraint is a specific code or command pattern ("`any` type in TypeScript") |
| Compliance requires context the agent must evaluate | Compliance can be verified or prompted by a script | Compliance can be pattern-matched in real-time |
| The constraint applies situationally | The constraint should fire on every lifecycle event | The constraint should block or warn on every file edit or command |
| The constraint is about quality of work | The constraint is about process discipline | The constraint is about preventing specific violations |

**Hook lifecycle events:**

| Event | When It Fires | Use For |
|-------|--------------|---------|
| `UserPromptSubmit` | Every time the user sends a message | Session setup, skill loading, context checks |
| `Stop` | When the agent finishes a response | Pre-commit checklists, session state reminders |

**Correct hook content:**

```bash
# Enforce the skill-loading rule programmatically
echo "Skills to evaluate: chunkhound, planning, svelte, typescript, tailwind"
echo "For each: LOAD (with reason) or SKIP (with reason)"
echo "Documentation-first: verify docs exist for the feature area before coding."
```

**Forbidden hook content:**

```bash
# WRONG: Implementing business logic in a hook
# Hooks enforce process, they don't make product decisions
echo "The IPC boundary uses invoke() only"  <- Belongs in architecture docs
echo "Functions must be <= 50 lines"         <- Belongs in coding-standards.md
```

**Not every rule needs a hook.** Hooks are appropriate when enforcement can be automated at a lifecycle boundary. Many rules are best left as written instructions that agents internalize — over-automating creates brittle process.

### Hookify (`.orqa/hookify/`)

Hookify files (`.orqa/hookify/`) are **real-time enforcement** — pattern-based blocks and warnings that fire the moment an agent edits a file or runs a bash command. Where hooks enforce at lifecycle boundaries (session start, stop), hookify enforces at the action level (every file write, every command execution). CLI tools may read these from a `.claude/hookify.*.local.md` symlink layer if present.

**How hookify files work:**

Hookify files are markdown files with YAML frontmatter that define:

- **`event`** — What triggers the rule: `file` (file edit) or `bash` (bash command)
- **`action`** — What happens on match: `block` (prevents the action) or `warn` (allows but shows a warning)
- **`conditions`** / **`pattern`** — The regex or glob pattern that triggers the rule

**The enforcement model:**

| Action | Behavior |
|--------|----------|
| `block` | The action is prevented entirely. The agent cannot proceed until the violation is resolved. |
| `warn` | The action is allowed, but a warning is displayed. The agent should address the concern. |

**Relationship between rules, hooks, and hookify:**

| Layer | Enforces | When | How |
|-------|----------|------|-----|
| Rules (`.orqa/governance/rules/`) | Behavioral constraints | Read at session start, apply throughout | Written instructions agents follow |
| Hooks (`.orqa/governance/hooks/`) | Process discipline | Lifecycle events (session start, stop) | Shell scripts that run automatically |
| Hookify (`.orqa/hookify/`) | Code/command violations | Every file edit, every bash command | Pattern matching that blocks or warns |

Think of it this way: a rule says "don't do X", a hook reminds you about X at lifecycle boundaries, and hookify actively prevents X from happening in real-time.

**Correct hookify content:**

```yaml
---
event: file
action: block
conditions:
  - file_pattern: "**/*.ts"
    content_pattern: ": any"
---
Block usage of `any` type in TypeScript files. Use proper types instead.
```

**Forbidden hookify content:**

```yaml
# WRONG: Using hookify for things that need judgement
# Hookify is for concrete pattern matches, not subjective quality
---
event: file
action: block
conditions:
  - content_pattern: "function"   # Too broad — blocks all functions
---
```

**Not every rule needs a hookify file.** Hookify is appropriate when a violation can be detected by pattern matching against file content or command text. Rules that require context, judgement, or multi-file analysis should remain as written instructions or lifecycle hooks.

---

## Anti-Patterns

### Agent files restating coding standards (duplication)

```text
# WRONG: Restating rules in agent file
## CRITICAL Rules
1. Functions must be <= 50 lines
2. No unwrap() in production code
3. Zero clippy warnings

# CORRECT: Referencing the doc
Read `.orqa/documentation/development/coding-standards.md` before writing any code.
All rules defined there apply to every commit.
```

### Skills containing architecture rules (wrong layer)

```text
# WRONG: Project rule in a skill file
## Key Rule
Never call invoke() directly in display components. Use stores.

# CORRECT: Technology pattern in skill, project rule in docs
## See Also
This skill covers Svelte 5 technology patterns. For OrqaStudio-specific
architectural constraints, see .orqa/documentation/architecture/decisions.md.
```

### Multiple agent files containing the same rule

When the same behavioral rule appears in two or more agent files, it will drift. Move the rule to `docs/` or `.orqa/rules/` and replace both copies with a reference.

---

## Enforcement

### Periodic Audit

The `agent-maintainer` and `code-reviewer` include doc-layer compliance in their review checklists:

- Agent files reference docs for standards they cite, rather than restating them
- Skill files contain technology patterns, not OrqaStudio-specific rules
- Rule files enforce behavioral constraints, not product knowledge

### Change Process

When a standard needs updating:

1. Update it in `.orqa/documentation/` (the source of truth)
2. Verify agent files and rules that reference it are still accurate
3. Do NOT update agent files or skills to restate the new content -- the references are correct by design

---

## Review Gate

After implementation, independent review agents evaluate each phase before it is considered complete:

| Review Agent | Evaluates |
|---|---|
| `code-reviewer` | Code quality: clippy, rustfmt, ESLint, svelte-check, no stubs, coverage, doc layer compliance |
| `qa-tester` | Functional correctness: does it behave as documented, not just compile |
| `ux-reviewer` | UX/accessibility: labels match docs, states are complete, no jargon in the UI |
| `agent-maintainer` | Governance audits: content layer compliance, reading list completeness |

---

## Documentation-Change Feedback Loop

When the `documentation-writer` agent makes changes to any documentation page, it triggers the `agent-maintainer` to review whether:

- Agent Required Reading lists need updating (new pages or moved pages)
- Rules need updating (new constraints documented, old ones removed)
- Skills need updating (new technology patterns documented)
- Hooks need updating (new rules that should be enforced programmatically at lifecycle boundaries)
- Hookify files need updating (new rules that can be pattern-matched against file edits or commands)

### Rule → Hook / Hookify Promotion

When a rule is repeatedly violated (recurrence >= 2 in `.orqa/lessons/`), consider whether it can be enforced by a hook or hookify file. The promotion path:

1. Rule violation captured as an IMPL lesson
2. Lesson recurrence reaches threshold
3. `agent-maintainer` evaluates: can this be pattern-matched against file content or commands?
4. If yes (specific pattern): write a hookify file that blocks or warns on the pattern
5. If no but enforceable at lifecycle boundary: write a hook that implements the rule
6. If neither: strengthen the rule's language, add to more agents' required reading

The key distinction: hookify is for violations that can be caught by matching a specific pattern in a file edit or bash command. Hooks are for enforcement that requires running a script at a lifecycle boundary (session start, stop). Rules remain for constraints that require judgement or context.

This loop ensures the governance system stays consistent as documentation evolves.

---

## Related Documents

- [Team Overview](/process/team) -- Agent directory and skill directory
- [Rules Reference](/process/rules) -- All enforcement rules and their purposes
- [Skills Log](/process/skills-log) -- Full skill inventory with provenance
- `.orqa/documentation/development/coding-standards.md` -- The standards all agents must follow
- `.orqa/documentation/architecture/decisions.md` -- Architecture decisions agents cite
- `.orqa/governance/rules/documentation-first.md` — Documentation as source of truth for implementation
