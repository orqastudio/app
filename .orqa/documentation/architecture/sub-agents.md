---
id: DOC-015
title: Sub-Agent Support Architecture
description: Architecture for sub-agent support enabling nested agent delegation within conversations.
created: "2026-03-05"
updated: "2026-03-05"
---

**Date:** 2026-03-05

Sub-agent support allows the orchestrator agent to delegate tasks to specialized sub-agents within a single OrqaStudio™ session. The `spawn_agent` tool spawns a sub-agent with a declared role and instructions. The agent registry reads `.orqa/team/agents/*.md` to discover available roles. Child tool calls are aggregated and not surfaced individually in the conversation view. Turn limits prevent runaway sub-agent loops.

---

## `spawn_agent` Tool

The `spawn_agent` tool is an OrqaStudio MCP tool, implemented in Rust, exposed to the sidecar via the MCP server interface.

### Tool Invocation Schema

```json
{
  "name": "spawn_agent",
  "description": "Delegate a task to a specialized sub-agent. The sub-agent executes autonomously within turn limits and returns a structured result.",
  "input_schema": {
    "type": "object",
    "properties": {
      "role": {
        "type": "string",
        "description": "Agent role name matching a definition in .claude/agents/. Example: 'backend-engineer', 'code-reviewer'."
      },
      "instructions": {
        "type": "string",
        "description": "Task instructions for the sub-agent. Should be specific and scoped. The sub-agent receives these instructions plus its role's Required Reading and skills."
      },
      "explore_mode": {
        "type": "boolean",
        "default": false,
        "description": "If true, runs a lightweight exploration agent with read-only tool access. No file writes, no bash commands. Used for codebase research."
      },
      "max_turns": {
        "type": "integer",
        "default": 20,
        "minimum": 1,
        "maximum": 50,
        "description": "Maximum number of turns the sub-agent may take before returning. Prevents runaway loops."
      }
    },
    "required": ["role", "instructions"]
  }
}
```

### Result Schema

```json
{
  "role": "backend-engineer",
  "status": "complete | turn_limit_reached | error",
  "summary": "What the sub-agent did and what it produced",
  "artifacts": [
    {
      "type": "file_written | file_read | command_run | decision",
      "path": "/absolute/path/to/file",
      "description": "Brief description of the artifact"
    }
  ],
  "violations": [
    {
      "entry_id": "RULE-004-001",
      "description": "Blocked: no `any` type in TypeScript",
      "verdict": "block"
    }
  ],
  "turns_used": 7
}
```

---

## Agent Registry

The agent registry reads `.orqa/team/agents/*.md` at startup and indexes available roles.

### Registry Entry

For each agent definition file, the registry extracts:

- **Role name** — derived from the filename (e.g., `backend-engineer.md` → `backend-engineer`)
- **Description** — from the agent file's YAML frontmatter `description` field
- **Skills** — from the `skills:` list in YAML frontmatter
- **Required Reading** — from the `required_reading:` list in YAML frontmatter
- **Capabilities** — from the `capabilities:` list in YAML frontmatter (optional)

### YAML Frontmatter Expected

```yaml
---
name: backend-engineer
description: "Rust / Tauri v2, IPC commands, domain logic, SQLite persistence"
skills:
  - chunkhound
  - rust-async-patterns
  - tauri-v2
required_reading:
  - .orqa/governance/decisions/
  - .orqa/documentation/development/coding-standards.md
capabilities:
  - write_files
  - run_commands
---
```

---

## Explore Mode

Explore mode is a lightweight variant of `spawn_agent` optimized for codebase research. The explore-mode sub-agent:

- Has access to read-only tools only: `Read`, `Glob`, `Grep`, ChunkHound MCP tools
- Cannot write files or run bash commands
- Does not require tool approval (all reads are auto-approved)
- Has a default turn limit of 10

Use explore mode when the orchestrator needs to gather information before deciding on an implementation approach. The result is a structured summary, not a set of file changes.

---

## Output Aggregation

When a sub-agent executes, it may call many tools: reading files, running commands, writing output. These tool calls are not surfaced individually in the parent conversation view. Instead:

1. The sub-agent's tool calls are recorded in the `sub_agent_turns` SQLite table
2. The parent conversation sees a single `spawn_agent` tool call card with a result summary
3. The sub-agent's full turn log is accessible via the "Expand details" action on the tool call card

This keeps the conversation view clean. The user sees what the orchestrator decided and what the sub-agent produced — not every intermediate step.

### Turn Recording Schema

```sql
CREATE TABLE sub_agent_turns (
    id              TEXT PRIMARY KEY,
    session_id      TEXT NOT NULL,
    spawn_call_id   TEXT NOT NULL,       -- ID of the spawn_agent tool call in parent
    role            TEXT NOT NULL,
    turn_number     INTEGER NOT NULL,
    tool_name       TEXT NOT NULL,
    tool_input      TEXT NOT NULL,       -- JSON
    tool_result     TEXT NOT NULL,       -- JSON
    created_at      TEXT NOT NULL
);
```

---

## Turn Limits

Turn limits prevent runaway sub-agent loops. When a sub-agent reaches its `max_turns` limit:

1. The sub-agent is instructed to produce a partial result and stop
2. The `spawn_agent` result has `status: "turn_limit_reached"`
3. The parent orchestrator receives the partial result and decides whether to re-invoke or report incomplete

Turn limits are enforced by the Rust runtime, not by the sidecar. The sidecar receives a termination signal when the limit is reached.

**Default limits by mode:**

| Mode | Default | Maximum |
|------|---------|---------|
| Normal | 20 turns | 50 turns |
| Explore | 10 turns | 20 turns |

---

## Enforcement in Sub-Agents

Sub-agent tool calls pass through the same enforcement engine as parent tool calls. A sub-agent writing a file with `: any` in TypeScript will be blocked by [RULE-004](RULE-004)-001, the same as if the orchestrator wrote it directly. Sub-agent violations are recorded in the `violations` table with the `session_id` of the parent session and a `sub_agent_spawn_id` reference.

---

## IPC Commands

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `list_agent_roles` | — | `Vec<AgentRole>` | List available roles from the registry |
| `get_agent_role` | `role: String` | `AgentRole` | Get role details including skills and required reading |
| `get_sub_agent_turns` | `spawn_call_id: String` | `Vec<SubAgentTurn>` | Get the full turn log for a spawn_agent call |

---

## Rust Module Structure

```text
backend/src-tauri/src/
  agents/
    mod.rs             -- AgentRegistry struct, spawn_agent tool handler
    registry.rs        -- Reads .orqa/team/agents/*.md, builds role index
    runner.rs          -- Sub-agent execution: skill loading, turn management, result aggregation
    explore.rs         -- Explore mode: read-only tool set, no approval required
    types.rs           -- AgentRole, SpawnAgentInput, SpawnAgentResult, SubAgentTurn
  commands/
    agents.rs          -- Tauri command handlers
  repo/
    sub_agent_turns.rs -- SQLite repository for sub_agent_turns table
```

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Learning Through Reflection | Sub-agent turn logs are persisted and contribute to session analytics. Explore mode enables the system to gather codebase context that feeds into more accurate governance recommendations. |
| Clarity Through Structure | Sub-agents execute within the enforcement engine boundary — all agent tool calls are evaluated against active rules. Delegation to specialized roles (code-reviewer, test-engineer) implements the governance review gates described in the process docs. |

---

## Related Documents

- [AD-010](AD-010) — Tool implementation as MCP (orqa_ tool namespace)
- `.orqa/documentation/architecture/enforcement.md` — Enforcement engine that sub-agent calls pass through
- `.orqa/documentation/architecture/tool-definitions.md` — Built-in tool definitions and approval matrix
- `.orqa/documentation/process/orchestration.md` — Orchestration model and task delegation workflow
