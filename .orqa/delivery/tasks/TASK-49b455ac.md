---
id: "TASK-49b455ac"
type: "task"
title: "Consolidate search skills — merge chunkhound, orqa-code-search, orqa-native-search into search"
status: "captured"
created: 2026-03-19T00:00:00.000Z
updated: 2026-03-19T00:00:00.000Z
relationships:
  - target: "EPIC-1358323e"
    type: "delivers"
  - target: "TASK-ba25d426"
    type: "depends-on"
---

# TASK-49b455ac: Search Skill Consolidation

## Acceptance Criteria

1. Single `search` skill replaces chunkhound, orqa-code-search, and orqa-native-search
2. Skill documents three tools: search.regex, search.semantic, search.research
3. No context-switching logic (CLI vs App) — MCP is the universal interface
4. Old skills removed from app/.orqa/process/skills/
5. Agent relationships updated (employed-by references)
6. Connector skill sync picks up the new consolidated skill
7. Works for non-code projects (searches content, not "code")