---
id: TASK-b5f9240b
title: Agent mapping — 9 Claude Code subagent definitions + new skills
status: surpassed
created: 2026-03-19
updated: 2026-03-21
relationships:
  - target: EPIC-6967c7dc
    type: delivers
  - target: TASK-367f0026
    type: depends-on
  - target: TASK-cd24193f
    type: depended-on-by
---

# TASK-b5f9240b: Agent Mapping + New Skills

## Acceptance Criteria

1. 9 agent `.md` files created in `connectors/claude-code/agents/`
2. Each maps to an OrqaStudio agent (AGENT-c5284fde through AGENT-bedeffd1)
3. Each has Claude Code frontmatter: name, description, model, tools, skills
4. System prompt derived from OrqaStudio agent body
5. `delegation-patterns` skill created in `skills/`
6. `governance-context` skill created in `skills/`
7. `artifact-creation` skill created in `skills/`
8. SessionStart hook updated to sync agent definitions
