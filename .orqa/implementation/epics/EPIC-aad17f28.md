---
id: EPIC-aad17f28
type: epic
title: "Rule enforcement generation"
description: "Complete the daemon-based rule enforcement system. The daemon already reads enforcement entries from rule frontmatter and evaluates them (hooks.rs). Extend it with missing entry types, revert the connector to calling the daemon, and block work when the daemon isn't running."
status: active
priority: P0
created: 2026-03-25
updated: 2026-03-25
horizon: active
relationships:
  - target: RES-2c959f47
    type: informed-by
    rationale: "Research identified the enforcement gap — 44 hook specs, only 3-4 implemented"
  - target: EPIC-a5501c18
    type: depends-on
    rationale: "Connector rebuild must be complete"
  - target: PD-1ef9f57c
    type: implements
    rationale: "AD resolved: daemon is business logic boundary"
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Mechanical enforcement is required for dogfooding milestone"
---

## Context

The daemon's `hooks.rs` already:

- Reads all active rules from the graph
- Parses `enforcement` entries from YAML frontmatter
- Evaluates `bash` pattern entries (regex match on commands)
- Evaluates `file` path entries (glob match on file paths)
- Checks plugin file ownership via manifest
- Returns allow/block/warn results

This was always the correct architecture (daemon = business logic boundary). During the connector rebuild session (EPIC-a5501c18), the rule-engine hook was incorrectly rewritten to do local enforcement in TypeScript, duplicating daemon logic. This must be reverted.

RES-2c959f47 found 20 rules declare 44 hook specs but only bash and file types are implemented in the daemon. The daemon needs: field-check (tool_input validation), tool-matcher (per-tool blocking), and session-state checks.

## Architectural Principle

- **Daemon evaluates rules.** All enforcement logic lives in `libs/validation/src/hooks.rs`.
- **Connector hooks are thin adapters.** They read stdin, call the daemon, format the response.
- **Daemon must be running.** Projects with the Claude connector installed must block work when the daemon is not reachable.
- **No duplicate enforcement in TypeScript.** The local bash-safety and file-ownership code added in EPIC-a5501c18 must be removed from the connector and verified to exist in the daemon.

## Tasks

### Phase 1: Revert + Verify Daemon

**TASK-1: Revert connector rule-engine.ts to daemon-calling thin adapter**

- Restore the original pattern: read stdin, call `POST /hook` on daemon, format result
- Remove the local bash-safety and file-ownership code from TypeScript
- Verify the daemon's `hooks.rs` already handles file ownership and bash patterns
- Acceptance criteria: rule-engine.ts is < 30 lines, all enforcement logic in daemon

**TASK-2: Add daemon-required enforcement to SessionStart**

- Add a SessionStart hook that checks daemon reachability (`GET /health`)
- If daemon is not running: output a blocking message telling the user to run `orqa daemon start`
- Acceptance criteria: new session without daemon running shows clear error and blocks

**TASK-3: Fix daemon rule evaluation for build commands**

- The original problem: daemon was blocking `cargo test` and `npx tsc`
- Investigate which rules' enforcement entries match these commands
- Fix the rules' patterns to not match legitimate build commands (or adjust action to warn)
- Acceptance criteria: `cargo test`, `npx tsc`, `cargo build` are allowed; `--no-verify`, `push --force` are blocked

### Phase 2: Extend Daemon Entry Types

**TASK-4: Add field-check entry type to hooks.rs**

- New enforcement entry type that checks tool_input fields
- Example: Agent tool must have `run_in_background: true` and `team_name` set
- Replaces the hand-written `enforce-background-agents.mjs`
- Acceptance criteria: field-check entries in rules are evaluated by daemon, tests pass

**TASK-5: Add tool-matcher entry type to hooks.rs**

- Entry type that matches specific tool names (not just Bash)
- Example: block `Write` to `.orqa/process/` files from implementer role
- Acceptance criteria: tool-matcher entries evaluated, tests pass

**TASK-6: Standardize enforcement entry schema**

- Define the canonical schema for enforcement entries in rule frontmatter
- Document all entry types: bash, file, field-check, tool-matcher
- Add schema validation to the Rust validation crate
- Acceptance criteria: schema documented, validation catches invalid entries

### Phase 3: Rule Migration

**TASK-7: Migrate 59 active rules to standardized enforcement schema**

- Audit all enforcement entries against the canonical schema
- Normalize format for the 20 rules with existing hook entries
- Add explicit `mechanism: behavioral` for advisory-only rules
- Acceptance criteria: all 59 rules have valid enforcement entries

### Phase 4: Remove Duplicates + Verify

**TASK-8: Remove hand-written enforcement scripts replaced by daemon**

- Delete `enforce-background-agents.mjs` (replaced by field-check in daemon)
- Delete `enforce-completion-gate.mjs` (if daemon handles it)
- Update `hooks.json` to remove entries for deleted scripts
- Acceptance criteria: no duplicate enforcement between daemon and connector

**TASK-9: End-to-end verification**

- Start daemon, verify all mechanical rules produce correct enforcement
- Test: bare Agent spawn → blocked
- Test: `--no-verify` → blocked
- Test: foreground agents → blocked
- Test: `cargo test` → allowed
- Test: file ownership → protected
- Acceptance criteria: all 44 hook entries produce correct allow/block/warn

**TASK-10: Enforcement coverage reporting**

- Add `orqa audit enforcement` CLI command
- Reports: which rules have enforcement, mechanism type, coverage gaps
- Acceptance criteria: command produces accurate report
