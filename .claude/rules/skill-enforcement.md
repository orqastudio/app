# Skill Enforcement (NON-NEGOTIABLE)

Every agent MUST have a `skills:` list in its YAML frontmatter that includes ALL skills it needs for its domain.

## Universal Skills

- The `chunkhound` skill MUST be in every agent's skill list — it is a universal skill for code search
- The orchestrator loads `chunkhound` and `planning` skills on every session (via CLAUDE.md)

## Agent-Specific Skills

- When the orchestrator delegates to an agent, the agent's YAML-declared skills are auto-loaded
- Skill lists should match the agent's Required Reading domains — if an agent reads frontend docs, it should have frontend-related skills

## Audit

- The `agent-maintainer` periodically audits that agent skill lists match their Required Reading domains
- Missing skills are added; irrelevant skills are removed
- All skill changes are documented in `docs/process/skills-log.md`

## Related Rules

- `required-reading.md` — docs that agents must load (complementary to skills)
- `chunkhound-usage.md` — enforcement of ChunkHound as the preferred search tool
