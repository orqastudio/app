---
id: DOC-05d5eaee
type: doc
title: "How To: Build an OrqaStudio Plugin"
category: how-to
description: "Step-by-step guide for building a Claude Code companion plugin that adds hooks, commands, and knowledge to an OrqaStudio project."
created: 2026-03-14
updated: 2026-03-14
sort: 1
relationships: []
---

## What a Plugin Is

An OrqaStudio plugin is a Claude Code companion plugin — a directory registered in
`.claude/settings.json` that Claude Code loads at session start. It can:

- **Run hooks** before/after tool calls and at session boundaries
- **Add slash commands** (e.g., `/orqa`) that agents can invoke
- **Inject knowledge** as system context when agents write to specific files

The OrqaStudio companion plugin lives at `.orqa/plugins/orqastudio-claude-plugin/` and
is the reference implementation for everything in this guide.

---

## Plugin Directory Structure

```
.claude-plugin/
├── package.json          # Plugin metadata and npm dependencies
├── hooks/
│   ├── hooks.json        # Hook registrations (which hooks fire on which events)
│   └── scripts/          # Hook implementation scripts
│       ├── rule-engine.mjs
│       ├── prompt-injector.ts
│       └── session-start.sh
├── commands/
│   └── orqa.md           # Slash command: /orqa
└── knowledge/
    ├── my-knowledge.md
    └── another-knowledge.md
```

Register the plugin in `.claude/settings.json`:

```json
{
  "plugins": [
    { "path": ".claude-plugin" }
  ]
}
```

Use `$CLAUDE_PLUGIN_ROOT` in hook commands — Claude Code resolves it to the absolute
path of your plugin directory.

---

## hooks.json Format

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Write|Edit|Bash",
        "hooks": [
          {
            "type": "command",
            "command": "node \"$CLAUDE_PLUGIN_ROOT/hooks/scripts/rule-engine.mjs\"",
            "timeout": 10
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "node \"$CLAUDE_PLUGIN_ROOT/hooks/scripts/graph-guardian.mjs\"",
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
            "command": "bash \"$CLAUDE_PLUGIN_ROOT/hooks/scripts/session-start.sh\"",
            "timeout": 15
          }
        ]
      }
    ],
    "UserPromptSubmit": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "node \"$CLAUDE_PLUGIN_ROOT/hooks/scripts/prompt-injector.ts\"",
            "timeout": 10
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
            "command": "bash \"$CLAUDE_PLUGIN_ROOT/hooks/scripts/stop-checklist.sh\"",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
```

**Hook types:**

| Type | Fires When |
|------|-----------|
| `PreToolUse` | Before a tool call executes — can block the call by exiting non-zero |
| `PostToolUse` | After a tool call completes — use for graph integrity checks, auto-linking |
| `SessionStart` | When a new Claude Code session begins |
| `UserPromptSubmit` | When the user submits a prompt — use for context injection |
| `Stop` | When Claude Code exits |

**`matcher`** is a regex matched against the tool name. `"Write|Edit|Bash"` fires on any
of those tools. `"*"` matches everything.

Hook scripts receive tool call context via stdin (JSON). A non-zero exit code from a
`PreToolUse` hook blocks the tool call and shows the script's stdout as the error message.

---

## Creating a Command

Commands are markdown files in `commands/`. The filename becomes the slash command name:
`commands/orqa.md` → `/orqa`.

```markdown
---
name: orqa
description: Show OrqaStudio governance summary — active rules, epics, and tasks
---

Read the OrqaStudio governance state and present a summary.

## Instructions

1. Read `.orqa/project.json` to get the project name
2. Count active rules in `.orqa/process/rules/`
3. Count epics by status in `.orqa/delivery/epics/`
4. Count tasks by status in `.orqa/delivery/tasks/`

Present the summary as a compact table.
```

The `description` field appears in `/help`. The body is injected as a system prompt
when the command is invoked — write it as instructions for the agent.

---

## Creating a Knowledge Artifact

Knowledge artifacts are flat markdown files in `knowledge/`. The filename (without `.md`) is how
agents refer to the knowledge. Knowledge is injected into agent context by hook scripts based on
file path patterns.

```markdown
---
id: KNOW-NNN
title: My Domain Knowledge
description: Describes specific patterns for working in the foo/ module.
status: active
created: "2026-03-14"
updated: "2026-03-14"
user-invocable: false
version: 0.1.0
---

# My Domain Knowledge

Content here is injected into the agent's context window when the knowledge is loaded.
Write it as reference documentation the agent reads and applies.

## Key Patterns

...
```

The `layer: plugin` field marks this as a portable plugin knowledge artifact (not project-specific).

---

## Reference: OrqaStudio Plugin

The live plugin at `.orqa/plugins/orqastudio-claude-plugin/` demonstrates all of these
patterns working together:

- `hooks/scripts/rule-engine.mjs` — reads enforcement entries from rule frontmatter
  and blocks tool calls that violate them (PreToolUse)
- `hooks/scripts/prompt-injector.ts` — injects project context on every prompt (UserPromptSubmit)
- `hooks/scripts/graph-guardian.mjs` — validates artifact cross-references after writes (PostToolUse)
- `hooks/scripts/session-start.sh` — runs `git status` and `git stash list` checks (SessionStart)
- `commands/orqa.md` — the `/orqa` governance summary command
- `knowledge/rule-enforcement.md` — teaches agents how enforcement entries work

---
