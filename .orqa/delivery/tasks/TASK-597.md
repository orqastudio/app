---
id: TASK-597
type: task
title: "Implement code_research compound search tool"
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-096
    type: delivers
  - target: TASK-596
    type: depends-on
---

# TASK-597: code_research Compound Search

## Acceptance Criteria

1. MCP tool `search.research({ question })` implements compound query
2. Pipeline: semantic search → symbol extraction (regex-based) → regex follow-up → assembled context
3. Returns coherent answer with source file paths and relevance
4. No AI dependency for symbol extraction — regex-based pattern matching
5. Exposed via MCP alongside search.regex and search.semantic
