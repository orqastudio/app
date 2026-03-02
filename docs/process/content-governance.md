# Content Ownership: Docs, Agents, Skills, and Rules

**Created:** 2026-03-02

Forge uses five distinct layers for governance knowledge: documentation, agent instructions, skills, rules, and hooks. Each layer owns a specific type of content. Mixing them creates maintenance burden and drift -- when a standard changes in one place, stale copies in other layers remain undetected.

---

## The Five Layers

| Layer | Owns | Examples | Source of Truth For |
|-------|------|----------|---------------------|
| **Documentation (`docs/`)** | Functional and product knowledge: architecture decisions, coding standards, IPC contracts, UI specs | Architecture decisions, function size limits, IPC response format, component state tables | Yes -- code that doesn't match docs is wrong |
| **Agent Instructions (`.claude/agents/`)** | Process: how the agent works, which tools it uses, which docs to read first, when to delegate, verification steps | "Run clippy before committing", "Read docs/architecture/decisions.md first", "Delegate to test-engineer after implementation" | Process only -- agents reference docs, not restate them |
| **Skills (`.claude/skills/`)** | Domain knowledge: how a technology works, general patterns, reusable techniques not specific to Forge | How Svelte 5 runes work, how to structure a Rust module, how to write a cargo test | Technology patterns only -- skills must not contain Forge-specific architectural rules |
| **Rules (`.claude/rules/`)** | Enforcement: automated checks and behavioral constraints that apply across all agents | "No stubs", "Error ownership", "End-to-end completeness" | Behavioral constraints -- rules reference docs for the standards they enforce |
| **Hooks (`.claude/hooks/`)** | Automated rule implementation: shell scripts triggered by lifecycle events that enforce rules programmatically | Session-start checklist, skill loading protocol, pre-commit verification | Executable enforcement -- hooks are the mechanism through which rules are actively enforced at key lifecycle points |

---

## Content Placement Rules

### Documentation (`docs/`)

Documentation is the source of truth for **what** the system does and **how** it should be built.

- When a standard changes, change it **here**. Agent instructions that reference the doc pick up the change automatically.
- Never copy a rule from `docs/` into an agent file or skill -- reference the doc instead.
- Every architecture decision lives in `docs/architecture/decisions.md`. Agent files do not define decisions; they cite them.

### Agent Instructions (`.claude/agents/`)

Agent files define **process** -- the workflow an agent follows to do its job. They do not define the standards themselves.

**Correct agent content:**

```text
Read `docs/development/coding-standards.md` before writing any code.
Run cargo clippy and cargo fmt before committing.
Delegate to the test-engineer agent after implementation.
```

**Forbidden agent content:**

```text
Functions must be <= 50 lines.          <- Belongs in docs/development/coding-standards.md
No backwards compatibility shims.      <- Belongs in docs/development/coding-standards.md
IPC boundary: only invoke()...         <- Belongs in docs/architecture/decisions.md
```

### Skills (`.claude/skills/`)

Skills teach **how a technology works** -- patterns, idioms, and examples from the technology's own documentation and best practices. They are intentionally portable: a Svelte skill should be useful on any Svelte project, not just Forge.

**Correct skill content:**

```text
How Svelte 5 $state works.
How to write a Rust module with proper error handling.
How to structure a cargo test with test fixtures.
```

**Forbidden skill content:**

```text
IPC boundary: Tauri commands only.            <- Project rule, not technology knowledge
EmptyState component from $lib/components/    <- Forge-specific, not portable
All Rust functions must return Result.        <- Project architecture rule
```

### Rules (`.claude/rules/`)

Rules enforce behavioral constraints across all agents. They describe **how agents must behave**, not what the product does. Rules reference documentation for the standards they enforce -- they do not duplicate those standards.

**Correct rule content:**

```text
Before committing, verify all layers exist end-to-end.
All errors are your responsibility -- fix them, don't claim they pre-existed.
```

**Forbidden rule content:**

```text
The IPC response format is: Result<T, String>    <- Belongs in architecture docs
Functions must be <= 50 lines.                    <- Belongs in coding-standards.md
```

### Hooks (`.claude/hooks/`)

Hooks are **the mechanism through which rules are actively enforced**. Where rules define behavioral constraints as written instructions that agents should follow, hooks implement those constraints as executable shell scripts triggered at specific lifecycle events.

Think of it this way: a rule says "you must do X", a hook makes sure X actually happens.

**Relationship between rules and hooks:**

| Rule | Implemented By Hook | Trigger |
|------|-------------------|---------|
| `skill-enforcement.md` — Load relevant skills before coding | `skill-instructions-hook.sh` — Lists skills, requires LOAD/SKIP decision | `UserPromptSubmit` |
| `required-reading.md` — Read governing docs before implementing | `session-start-hook.sh` — Checks for session state, stale worktrees, stashes | `UserPromptSubmit` (first) |
| `testing-standards.md` — Run tests before committing | `pre-commit-reminder.sh` — Checklist: cargo test, clippy, npm check, no stubs | `Stop` |

**When to use a rule vs a hook:**

| Use a Rule When | Use a Hook When |
|----------------|-----------------|
| The constraint is judgement-based ("ensure error handling is comprehensive") | The constraint is checkable ("run clippy before committing") |
| Compliance requires context the agent must evaluate | Compliance can be verified or prompted by a script |
| The constraint applies situationally | The constraint should be enforced on every occurrence of the trigger event |
| The constraint is about quality of work | The constraint is about process discipline |

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
Read `docs/development/coding-standards.md` before writing any code.
All rules defined there apply to every commit.
```

### Skills containing architecture rules (wrong layer)

```text
# WRONG: Project rule in a skill file
## Key Rule
Never call invoke() directly in display components. Use stores.

# CORRECT: Technology pattern in skill, project rule in docs
## See Also
This skill covers Svelte 5 technology patterns. For Forge-specific
architectural constraints, see docs/architecture/decisions.md.
```

### Multiple agent files containing the same rule

When the same behavioral rule appears in two or more agent files, it will drift. Move the rule to `docs/` or `.claude/rules/` and replace both copies with a reference.

---

## Enforcement

### Periodic Audit

The `agent-maintainer` and `code-reviewer` include doc-layer compliance in their review checklists:

- Agent files reference docs for standards they cite, rather than restating them
- Skill files contain technology patterns, not Forge-specific rules
- Rule files enforce behavioral constraints, not product knowledge

### Change Process

When a standard needs updating:

1. Update it in `docs/` (the source of truth)
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
- Hooks need updating (new rules that should be enforced programmatically, new lifecycle triggers)

### Rule → Hook Promotion

When a rule is repeatedly violated (recurrence >= 2 in `docs/development/lessons.md`), consider whether it can be enforced by a hook. The promotion path:

1. Rule violation captured as an IMPL lesson
2. Lesson recurrence reaches threshold
3. `agent-maintainer` evaluates: is this enforceable at a lifecycle boundary?
4. If yes: write a hook that implements the rule, update this document's rule-to-hook mapping
5. If no: strengthen the rule's language, add to more agents' required reading

This loop ensures the governance system stays consistent as documentation evolves.

---

## Related Documents

- [Team Overview](/process/team) -- Agent directory and skill directory
- [Rules Reference](/process/rules) -- All enforcement rules and their purposes
- [Skills Log](/process/skills-log) -- Full skill inventory with provenance
- `docs/development/coding-standards.md` -- The standards all agents must follow
- `docs/architecture/decisions.md` -- Architecture decisions agents cite
- `.claude/rules/documentation-first.md` -- Documentation as source of truth for implementation
