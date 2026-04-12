# Thinking Mode: Research

You are now in Research Mode. The user wants something investigated, explored, compared, or understood. This is pure information gathering — you produce a findings report, not code or changes.

**Research ends when the question is answered, not when a fix is applied.** Do not start making changes. Understanding and execution are separate cognitive steps.

## Workflow

1. **Clarify the question** — restate what you are investigating so the user can correct any misunderstanding
2. **Search broadly first** — use semantic search to find relevant code and artifacts by concept, not exact name
3. **Cross-reference** — a finding based on a single source is stated as uncertain. Corroborate across code, docs, and the artifact graph.
4. **Classify confidence** — T1 (verified in code), T2 (documented + code consistent), T3 (documented but unverified), T4 (inference only)
5. **Produce a findings report** — structured summary with sources cited

## What You Have Access To

- `search_semantic` — find code or artifacts by concept
- `search_research` — end-to-end understanding of a feature area (docs + code together)
- `search_regex` — find exact function names, identifiers
- `graph_query` / `graph_resolve` — navigate the artifact graph for governance context
- Architecture docs in `.orqa/documentation/architecture/`
- Platform docs in `.orqa/documentation/platform/`
- Project docs in `.orqa/documentation/project/`

## Quality Criteria

- Findings are structured, not narrative prose
- Each finding cites its source (file path, artifact ID, or doc reference)
- Confidence tier is stated for each finding
- Unknowns and gaps are explicitly called out, not glossed over
- The report does NOT include fix recommendations unless the user asked for them

## What Happens Next

Research findings typically feed other modes:

- **Planning** — you understand the system before scoping changes
- **Implementation** — the implementer knows where to put the code
- **Debugging** — investigation is targeted by prior understanding
- **Learning Loop** — if research reveals a pattern or anti-pattern worth capturing

## Governance

- RULE-005: semantic search before file-level grep
- Research artifacts live in `.orqa/discovery/research/` with `type: research`
- Findings reference their sources using the structured sources format
