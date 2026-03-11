![License](https://img.shields.io/badge/license-Apache%202.0-blue)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# OrqaStudio Claude Code Plugin

> **Governance companion plugin for Claude Code.** Mechanically enforces OrqaStudio governance rules, loads orchestrator context, and provides governance commands.

---

## What It Does

This plugin bridges OrqaStudio's governance framework with Claude Code's hook system:

- **Rule enforcement** — Intercepts `Write`, `Edit`, and `Bash` tool calls and evaluates them against rules that have `enforcement` entries in their YAML frontmatter. Blocks or warns on violations.
- **Session start checks** — Detects stale worktrees, uncommitted files, orphaned directories, and previous session state on session start.
- **Stop checklist** — Reminds the agent to commit, update session state, and clean up before ending a session.
- **Governance commands** — `/orqa` shows a governance summary (active rules, epics, tasks).

## How Rule Enforcement Works

Rules in `.orqa/governance/rules/RULE-NNN.md` can have an `enforcement` array in their YAML frontmatter:

```yaml
enforcement:
  - event: file
    pattern: "unwrap\\(\\)"
    paths: ["src-tauri/src/**/*.rs"]
    action: block
    message: "No unwrap() in production Rust code."
  - event: bash
    pattern: "git commit.*--no-verify"
    action: block
    message: "Never bypass pre-commit hooks."
```

The plugin's `PreToolUse` hook loads all active rules, parses their enforcement entries, and evaluates patterns against the tool call context. Violations are either blocked (tool call denied) or warned (message shown, tool call proceeds).

### Supported Event Types

| Event | Triggered By | Pattern Matched Against |
|-------|-------------|------------------------|
| `file` | `Write`, `Edit` tool calls | File content (`new_string` for Edit, `content` for Write) |
| `bash` | `Bash` tool calls | The command string |

### Currently Enforced Rules

| Rule | What It Catches | Action |
|------|----------------|--------|
| RULE-006 | `unwrap()`, `expect()`, `panic!()` in Rust production code | Block |
| RULE-007 | Raw `cargo`/`npm run` commands instead of `make` targets | Warn |
| RULE-013 | `--no-verify` on git commits, destructive git operations | Block/Warn |
| RULE-020 | `TODO`/`FIXME`/`HACK`/`XXX` comments in production code | Warn |

## Installation

### From the OrqaStudio marketplace (recommended)

Add the marketplace to your project's `.claude/settings.json`:

```json
{
  "extraKnownMarketplaces": {
    "orqa-local": {
      "source": {
        "source": "directory",
        "path": "../orqastudio-claude-plugin"
      }
    }
  },
  "enabledPlugins": {
    "orqa-plugin@orqa-local": true
  }
}
```

### Manual installation

1. Clone the repository alongside your project:

```bash
git clone git@github.com:orqastudio/orqastudio-claude-plugin.git
```

2. Add the hooks directly to your project's `.claude/settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Write|Edit|Bash",
        "hooks": [
          {
            "type": "command",
            "command": "node \"../orqastudio-claude-plugin/hooks/scripts/rule-engine.mjs\"",
            "timeout": 10
          }
        ]
      }
    ],
    "SessionStart": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "bash \"../orqastudio-claude-plugin/hooks/scripts/session-start.sh\"",
            "timeout": 15
          }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "bash \"../orqastudio-claude-plugin/hooks/scripts/stop-checklist.sh\"",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
```

## Usage

### Rule Enforcement (automatic)

Rule enforcement runs automatically on every `Write`, `Edit`, and `Bash` tool call. No action needed — the plugin reads your `.orqa/governance/rules/` directory and enforces rules that have `enforcement` entries.

To add enforcement to a rule:

1. Open a rule file in `.orqa/governance/rules/`
2. Add an `enforcement` array to the YAML frontmatter
3. Each entry needs: `event`, `pattern`, `action`, `message`
4. Optional: `paths` array of glob patterns to restrict file matching

### Commands

| Command | Description |
|---------|-------------|
| `/orqa` | Show governance summary — active rules, epics, tasks |

### Session Hooks (automatic)

- **Session start** — Runs on first prompt, checks for stashes, worktrees, uncommitted files, and previous session state
- **Stop** — Runs when the agent is about to stop, shows a pre-commit checklist

## Plugin Structure

```
.claude-plugin/
  plugin.json          # Plugin manifest
  marketplace.json     # Local marketplace registration
hooks/
  hooks.json           # Hook configuration
  scripts/
    rule-engine.mjs    # Node.js rule enforcement engine
    session-start.sh   # Session start health checks
    stop-checklist.sh  # Pre-stop checklist
commands/
  orqa.md              # /orqa governance summary command
skills/
  rule-enforcement/
    SKILL.md           # Rule enforcement skill documentation
```

## Requirements

- [Claude Code](https://claude.com/claude-code) CLI
- Node.js (for the rule engine)
- An OrqaStudio project with `.orqa/governance/rules/` containing rules with `enforcement` entries

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.
