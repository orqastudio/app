---
id: TASK-6827dc2b
title: Implement code_research compound search tool
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-cdb03816
    type: delivers
  - target: TASK-9fb85edf
    type: depends-on
  - target: TASK-945e12fa
    type: depended-on-by
---

# TASK-6827dc2b: code_research Compound Search

## Acceptance Criteria

1. MCP tool `search.research({ question })` implements compound query
2. Pipeline: semantic search → symbol extraction (regex-based) → regex follow-up → assembled context
3. Returns coherent answer with source file paths and relevance
4. No AI dependency for symbol extraction — regex-based pattern matching
5. Exposed via MCP alongside search.regex and search.semantic
