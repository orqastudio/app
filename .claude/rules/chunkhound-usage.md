# ChunkHound Usage (MANDATORY when available)

The `chunkhound` skill contains tool selection guides, query patterns, and anti-patterns. Load it. **Prefer ChunkHound over Grep/Glob for any search that spans more than one file or directory.**

## Enforcement

- The orchestrator and ALL subagents MUST prefer ChunkHound over Grep/Glob for multi-file searches
- Grep/Glob are only appropriate for single-file lookups or when ChunkHound is confirmed unavailable
- Every agent's YAML frontmatter MUST include `chunkhound` in its `skills:` list

## Documentation Review (MANDATORY before implementation)

Before writing ANY implementation code, check the project documentation for existing designs, plans, and architecture decisions related to the task. Use `code_research` with a query describing the feature area — it searches docs AND code together.

## When ChunkHound is Unavailable

If ChunkHound MCP tools are not available in the current session:

1. **Subagents** — Delegate research to a subagent that has ChunkHound access
2. **Direct fallback** — Only if subagent delegation is impractical, use Grep/Glob
3. **Always note** — State in the task summary that ChunkHound was unavailable so results may be incomplete

## Related Rules

- `skill-enforcement.md` — `chunkhound` is a universal skill required for every agent
- `error-ownership.md` — use `search_regex` to find function signatures before calling them
- `reusable-components.md` — use `search_semantic` to find similar components
- `end-to-end-completeness.md` — use `code_research` to map the full request chain
- `no-stubs.md` — use `search_regex` to verify implementations exist
