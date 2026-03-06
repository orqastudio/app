---
title: "Enforcement Engine Architecture"
category: architecture
tags: []
created: 2026-03-05
updated: 2026-03-05
---

# Enforcement Engine Architecture

**Date:** 2026-03-05

The enforcement engine is Orqa Studio's mechanism for evaluating agent tool calls against behavioral rules in real time. Rules carry machine-readable YAML frontmatter declaring enforcement entries. The engine loads entries, evaluates patterns against every file write and bash command, records violations, and surfaces them in the UI. The same frontmatter auto-generates CLI-compatible hookify files so enforcement works in plain Claude Code sessions too.

---

## Single Source of Truth

Rule files in `.claude/rules/` are the single source of truth for both:

1. **Human-readable behavioral constraints** — injected into agent context as rule text
2. **Machine-readable enforcement entries** — YAML frontmatter declaring patterns to evaluate

There is no separate enforcement configuration file. Adding an enforcement entry to a rule's frontmatter activates it in both the app and (via generation) in the CLI.

---

## Rule Frontmatter Schema

```yaml
---
enforcement:
  - id: RULE-NNN-001
    description: "What this entry enforces"
    event: file | bash
    action: block | warn
    pattern: "regex pattern"
    scope: system | project
---
```

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique identifier. Format: `RULE-<rule-number>-<entry-number>`. Used for violation persistence and display. |
| `description` | string | Human-readable description shown in the enforcement panel. |
| `event` | `file` or `bash` | `file`: evaluates against file content on every write. `bash`: evaluates against the command string before execution. |
| `action` | `block` or `warn` | `block`: prevents the action, records a violation. `warn`: allows the action, records a violation with severity=warn. |
| `pattern` | string | Regular expression evaluated against the target (file content or bash command). |
| `scope` | `system` or `project` | `system`: applies to all projects. `project`: applies only to the current project's files. |

A rule file may declare zero or more enforcement entries. Rules with no frontmatter (or no `enforcement` key) are context-only — injected as text but not pattern-evaluated.

---

## Engine Design

### Startup

On app launch (and on project open), the enforcement engine:

1. Reads all `.md` files in `.claude/rules/`
2. Parses YAML frontmatter from each file using `yaml-front-matter`
3. Extracts the `enforcement` array from each parsed frontmatter block
4. Compiles all entries into an in-memory enforcement set, keyed by `id`

The compiled enforcement set is stored in `AppState` and shared across all Tauri command handlers.

### Evaluation

Every tool call that writes a file or executes a bash command passes through the enforcement engine before execution.

```text
Agent issues tool call
  → Tauri command handler receives the call
    → Enforcement engine evaluates the call:
        for each entry in enforcement_set where entry.event matches:
          if pattern matches target:
            record violation
            if action == block:
              return Err(EnforcementBlock { id, description, matched_text })
            if action == warn:
              emit EnforcementWarning event to frontend
        if no block: proceed with the tool call
```

**File evaluation:** The pattern is evaluated against the full file content after the write is staged but before it is committed to disk. This allows the pattern to see the complete modified file.

**Bash evaluation:** The pattern is evaluated against the raw command string before the process is spawned.

### Verdict Types

| Verdict | Action | User Sees |
|---------|--------|-----------|
| `block` | Tool call is rejected. File is not written. Command is not executed. | Error in tool call card: "Blocked by [rule description]" |
| `warn` | Tool call proceeds. Warning recorded. | Amber badge on tool call card: "Warning: [rule description]" |
| `pass` | No match. Tool call proceeds silently. | Nothing |

---

## Violation Persistence

Every block and warn verdict is written to the `violations` table in SQLite.

```sql
CREATE TABLE violations (
    id          TEXT PRIMARY KEY,       -- UUID
    rule_id     TEXT NOT NULL,          -- e.g. "RULE-004"
    entry_id    TEXT NOT NULL,          -- e.g. "RULE-004-001"
    session_id  TEXT NOT NULL,
    verdict     TEXT NOT NULL,          -- "block" | "warn"
    matched_text TEXT NOT NULL,         -- snippet that matched the pattern
    file_path   TEXT,                   -- for file events
    command     TEXT,                   -- for bash events
    created_at  TEXT NOT NULL
);
```

Violations are queryable by session, rule, entry, verdict, and time range. The enforcement panel reads this table directly via Tauri commands.

---

## CLI Backwards Compatibility

The app auto-generates hookify-compatible files from rule frontmatter so enforcement also works in plain Claude Code CLI sessions.

### Generated File Format

For each enforcement entry, the generator produces a file at `.claude/hookify.<rule-id>-<entry-id>.local.md`:

```markdown
---
event: file
action: block
conditions:
  - file_pattern: "**/*.ts"
    content_pattern: ": any"
---
Blocked by RULE-004-001: No `any` type in TypeScript files. Use proper types instead.
```

### Generation Rules

- Generated files are derived artifacts. Do not edit them directly.
- Edit the source rule's frontmatter; the generator re-creates the hookify files.
- Generated files are committed alongside their source rule — they are not gitignored.
- If a rule's enforcement entries are removed, the corresponding generated files are deleted.

### Hookify File Naming

`hookify.<lowercase-rule-id>-<lowercase-entry-id>.local.md`

Example: rule `RULE-004-001` produces `hookify.rule-004-001.local.md`.

---

## System vs Project Scope

| Scope | Meaning | Example |
|-------|---------|---------|
| `system` | Applies to all projects, all files | No `: any` in TypeScript — applies everywhere |
| `project` | Applies only to the currently open project | Project-specific naming conventions |

System-scoped entries are compiled unconditionally. Project-scoped entries are compiled when a project is open and their patterns are evaluated only against files within the project root.

---

## IPC Commands

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `list_violations` | `session_id?`, `rule_id?`, `verdict?` | `Vec<Violation>` | Query violations with optional filters |
| `get_violation` | `id: String` | `Violation` | Get a single violation by ID |
| `dismiss_violation` | `id: String` | `()` | Mark a violation as dismissed (soft delete) |
| `list_enforcement_entries` | — | `Vec<EnforcementEntry>` | List all compiled enforcement entries with their source rule |
| `get_enforcement_status` | — | `EnforcementStatus` | Summary: total entries, active violations, dismissed |

---

## Rust Module Structure

```text
src-tauri/src/
  enforcement/
    mod.rs              -- EnforcementEngine struct, public API
    parser.rs           -- YAML frontmatter extraction from rule files
    evaluator.rs        -- Pattern matching against file content and commands
    generator.rs        -- Hookify file generation from frontmatter
    types.rs            -- EnforcementEntry, Violation, EnforcementVerdict
  commands/
    enforcement.rs      -- Tauri command handlers for enforcement queries
  persistence/
    violations.rs       -- SQLite repository for the violations table
```

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | Violations are persisted and queryable over time, enabling recurrence tracking that feeds into the lesson promotion pipeline. |
| Process Governance | The enforcement engine is the primary mechanism through which governance rules become active constraints on agent behavior — making governance enforceable, not just documented. |

---

## Related Documents

- `docs/process/rules.md` — Rule inventory, frontmatter schema reference, when to create rules
- `docs/ui/enforcement-panel.md` — UI spec for the enforcement sidebar and violation display
- `docs/architecture/decisions.md` — AD-015: Governance artifact format
- `docs/process/content-governance.md` — Content ownership model (rules vs docs vs hooks)
