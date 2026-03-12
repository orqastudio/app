---
id: EPIC-050
title: Rule Enforcement Engine with Claude Code Companion Plugin
description: |
  Build a two-context rule enforcement engine: a Claude Code companion plugin
  (separate repo) that enforces .orqa/ rules in the CLI, then integrate the same
  enforcement logic into the OrqaStudio app for dogfooding parity.
status: in-progress
priority: P1
created: "2026-03-11"
updated: "2026-03-12"
deadline: null
milestone: MS-002
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on: []
blocks: []
research-refs: []
docs-required: []
docs-produced: []
scoring:
  dogfood-value: 5
  user-facing: 5
  foundation: 5
  complexity: 4
  score: 4.5
---

## Context

OrqaStudio has 39 rules in `.orqa/governance/rules/` with structured frontmatter
(id, layer, scope as agent ID arrays, status). But enforcement is fragile:

- **CLI context**: Rules are loaded into agent context via `.claude/rules/` symlinks.
  Enforcement is self-policed — agents read rules but nothing stops violations.
  Shell-script hooks in `.claude/settings.json` handle session-start checks and
  pre-commit reminders, but these are invisible to the app and broke when
  `.claude/hooks/` was gitignored (RES-036 finding F-01).

- **App context**: The app scans `.orqa/governance/rules/` for display but doesn't
  enforce rules during agent execution. The enforcement architecture doc
  (`.orqa/documentation/architecture/enforcement.md`) describes a pattern-matching
  engine that evaluates rules against file writes and bash commands, but it's not
  yet connected to the agent execution pipeline.

Both [IDEA-034](IDEA-034) (structured enforcement engine) and [IDEA-027](IDEA-027) (native hooks / CLI
parity) converge on the same solution: **rules are the enforcement layer, and
hooks are the mechanical implementation detail that users don't need to see**.

The Claude Code [hookify plugin](https://github.com/anthropics/claude-code/tree/main/plugins/hookify)
provides the pattern: markdown files with YAML frontmatter, loaded dynamically,
enforced via PreToolUse/PostToolUse hooks. We adapt this for OrqaStudio's existing
rule format rather than inventing a new one.

## Implementation Design

### Two-Phase Strategy

**Phase 1: Claude Code Companion Plugin** (separate repo: `orqa-studio/orqa-plugin`)
- A Claude Code plugin that reads `.orqa/governance/rules/` and enforces them
- Replaces the need for `.claude/hooks/` shell scripts and `.claude/rules/` symlinks
- Provides commands (`/orqa`, `/orqa:rules`, `/orqa:status`) for governance interaction
- Ships as an installable Claude Code plugin

**Phase 2: App-Native Enforcement** (this repo: `orqa-studio`)
- Port the enforcement logic into the Rust backend
- Integrate with the agent execution pipeline (tool approval, pre-delegation checks)
- Surface violations in the governance UI
- Complete dogfood parity: same rules, same enforcement, both contexts

### Phase 1: Companion Plugin Architecture

```
orqa-plugin/
  .claude-plugin/
    plugin.json
  hooks/
    pre-tool-use.md        # PreToolUse hook definition
    post-tool-use.md       # PostToolUse hook definition
    session-start.md       # SessionStart hook (replaces session-start-hook.sh)
    stop.md                # Stop hook (replaces pre-commit-reminder.sh)
  commands/
    orqa.md                # /orqa — main governance command
    orqa-rules.md          # /orqa:rules — list active rules
    orqa-status.md         # /orqa:status — governance health check
  skills/
    rule-enforcement.md    # How the enforcement engine works
  agents/
    rule-checker.md        # Agent that evaluates rules against context
  core/
    engine.py              # Rule loading, parsing, pattern matching
    matchers.py            # Regex, glob, contains matchers
    context.py             # Build enforcement context from tool calls
  README.md
```

#### Rule Loading

The engine reads `.orqa/governance/rules/RULE-NNN.md` files and extracts:
- `status` — only `active` rules are enforced
- `layer` — determines if the rule applies (core = always, project = this project)
- `scope` — agent ID array filtering (match against current agent context)
- `enforcement` frontmatter section (new) — machine-readable patterns

#### Enforcement Frontmatter (New Field)

Rules that need mechanical enforcement add an `enforcement` array to their
frontmatter:

```yaml
enforcement:
  - event: file
    pattern: "unwrap\\(\\)"
    paths: ["backend/src-tauri/src/**/*.rs"]
    action: block
    message: "No unwrap() in production code (RULE-006)"
  - event: bash
    pattern: "git commit.*--no-verify"
    action: block
    message: "Never bypass pre-commit hooks (RULE-013)"
```

Rules without `enforcement` entries are guidance-only — loaded into agent context
but not mechanically enforced. This is intentional: most rules are behavioral
guidelines that agents self-enforce. Only specific, pattern-matchable violations
get mechanical enforcement.

#### Hook Flow

```
Claude Code tool call (e.g., Bash, Edit, Write)
  → PreToolUse hook fires
  → Plugin loads active rules from .orqa/governance/rules/
  → Filter by: status=active, layer matches, scope matches current agent
  → For each rule with enforcement entries:
    → Match event type (file/bash/prompt/stop)
    → Evaluate pattern against tool call context
    → If match: return action (block with message, or warn via additionalContext)
  → If no violations: allow tool call
```

#### Context Fields by Event

| Event | Context Fields |
|-------|---------------|
| `bash` | `command` |
| `file` | `file_path`, `new_text`, `old_text` |
| `prompt` | `user_prompt` |
| `stop` | `transcript` |

#### Commands

| Command | Purpose |
|---------|---------|
| `/orqa` | Show governance summary: active rules, recent violations, health |
| `/orqa:rules` | List all active rules with enforcement status |
| `/orqa:status` | Governance health: rule coverage, broken refs, schema compliance |

### Phase 2: App-Native Enforcement

Port the same engine into Rust:
- Rule loading from `.orqa/governance/rules/` (already scanned by artifact reader)
- Pattern matching via `regex` crate
- Integration with tool approval pipeline (before tool execution in agent loop)
- Violations stored in SQLite for audit trail
- Violations surfaced in governance UI (rules view shows violation history)
- Same `enforcement` frontmatter consumed by both plugin (Python) and app (Rust)

### Schema Changes

Add `enforcement` to the rule schema:

```json
"enforcement": {
  "type": "array",
  "items": {
    "type": "object",
    "required": ["event", "pattern", "action"],
    "properties": {
      "event": { "enum": ["file", "bash", "prompt", "stop"] },
      "pattern": { "type": "string" },
      "paths": { "type": "array", "items": { "type": "string" } },
      "action": { "enum": ["block", "warn"] },
      "message": { "type": "string" }
    }
  }
}
```

### Agent & Skill Loading (Eliminates .claude/ Symlinks)

The plugin doesn't just enforce rules — it becomes the single bridge between
`.orqa/` and Claude Code, replacing the entire `.claude/` symlink architecture:

| Plugin Component | Reads From | Replaces |
|-----------------|-----------|----------|
| **Orchestrator agent** | `.orqa/team/agents/orchestrator.md` | `.claude/CLAUDE.md` symlink |
| **Agent definitions** | `.orqa/team/agents/*.md` | `.claude/agents/` symlink |
| **Skills** | `.orqa/team/skills/*/SKILL.md` | `.claude/skills/` symlink |
| **Rules (context)** | `.orqa/governance/rules/*.md` | `.claude/rules/` symlink |
| **Rules (enforcement)** | `enforcement` frontmatter entries | `.claude/hooks/` shell scripts |
| **Session hooks** | Plugin SessionStart/Stop hooks | `.claude/hooks/*.sh` scripts |

**How it works:**

- **SessionStart hook** — reads `.orqa/team/agents/orchestrator.md` and injects
  it as system context. This is what `.claude/CLAUDE.md` currently does via symlink.
- **Plugin agents directory** — exposes `.orqa/team/agents/` as the plugin's agents
  folder. Claude Code discovers them automatically.
- **Plugin skills directory** — exposes `.orqa/team/skills/` as the plugin's skills
  folder. Skills become available via `/skill-name` in Claude Code.
- **Plugin hooks** — replace shell scripts with structured hook definitions that
  read and enforce rules from `.orqa/governance/rules/`.

**After migration, `.claude/` contains only:**
- `settings.json` — enables the plugin, configures Claude Code preferences
- `settings.local.json` — user-specific overrides (gitignored)

Everything else comes from `.orqa/` via the plugin. The symlinks are deleted.

### What Replaces What

| Current | Replaced By |
|---------|-------------|
| `.claude/CLAUDE.md` → orchestrator.md | Plugin loads orchestrator directly |
| `.claude/rules/` → `.orqa/governance/rules/` | Plugin hooks enforce + load rules |
| `.claude/agents/` → `.orqa/team/agents/` | Plugin exposes agents natively |
| `.claude/skills/` → `.orqa/team/skills/` | Plugin exposes skills natively |
| `.claude/hooks/` → `.orqa/governance/hooks/` | Plugin hooks replace shell scripts |
| Self-policed rule compliance | Mechanical enforcement via PreToolUse |

### What Stays

| Kept | Reason |
|------|--------|
| `.githooks/pre-commit` | Git-level enforcement (schema validation, stub scanning) — independent of Claude |
| `.orqa/governance/rules/` | Source of truth — unchanged, just consumed by more systems |
| Rule frontmatter (id, layer, scope) | Extended with `enforcement`, not replaced |
| `.claude/settings.json` | Still needed to enable the plugin itself |

## Tasks

### Phase 1: Companion Plugin

- [x] [TASK-183](TASK-183): Add `enforcement` field to rule schema + add entries to key rules
- [x] [TASK-177](TASK-177): Create `orqa-plugin` repository with Claude Code plugin scaffold
- [x] [TASK-178](TASK-178): Implement rule engine core (loader, parser, pattern matcher)
- [x] [TASK-179](TASK-179): Implement agent & skill loading from `.orqa/team/`
- [x] [TASK-180](TASK-180): Implement PreToolUse hook (file + bash event enforcement)
- [x] [TASK-181](TASK-181): Implement SessionStart hook (orchestrator injection + session checks)
- [x] [TASK-182](TASK-182): Implement Stop hook (replaces pre-commit-reminder.sh)
- [ ] [TASK-184](TASK-184): Implement `/orqa`, `/orqa:rules`, `/orqa:status` commands
- [ ] [TASK-185](TASK-185): Test plugin against OrqaStudio's `.orqa/` governance artifacts
- [ ] [TASK-186](TASK-186): Remove `.claude/` symlinks and update [RULE-003](RULE-003) symlink documentation
- [x] [TASK-187](TASK-187): Document plugin installation and configuration

### Phase 2: App-Native Enforcement

- [ ] [TASK-188](TASK-188): Port enforcement engine to Rust backend
- [ ] [TASK-189](TASK-189): Integrate with agent tool approval pipeline
- [ ] [TASK-190](TASK-190): Surface violations in governance UI

## Out of Scope

- **HOOK-NNN artifact type** — deferred until the enforcement model is validated
- **Plugin distribution registry** — manual installation first, registry later
- **Rule editor in UI** — the `enforcement` field is hand-authored for now
- **Cross-project rule sharing** — one project at a time
