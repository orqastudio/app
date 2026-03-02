# Skills Inventory

**Date:** 2026-03-02

This page tracks all installed skills in the Forge project -- their source, purpose, and when they were added. It is maintained by the `agent-maintainer` agent.

Skills are stored in `.claude/skills/` and follow the open [Agent Skills](https://agentskills.io) standard.

---

## Skill Inventory

| Skill | Source | Purpose | Date Added |
|-------|--------|---------|------------|
| `tauri-v2` | skills.sh/nodnarbnitram/claude-code-extensions@tauri-v2 | Tauri v2 desktop app patterns, commands, events, plugins | 2026-03-02 |
| `svelte5-best-practices` | skills.sh/ejirocodes/agent-skills@svelte5-best-practices | Svelte 5 runes ($state, $derived, $effect), component patterns | 2026-03-02 |
| `typescript-advanced-types` | skills.sh/wshobson/agents@typescript-advanced-types | Strict TypeScript, advanced type patterns, type guards | 2026-03-02 |
| `rust-async-patterns` | skills.sh/wshobson/agents@rust-async-patterns | Rust async/await patterns, tokio, error handling | 2026-03-02 |
| `tailwind-design-system` | skills.sh/wshobson/agents@tailwind-design-system | Tailwind CSS design system, utilities, responsive patterns | 2026-03-02 |
| `chunkhound` | Custom (adapted from Alvarez) | Semantic code search: tool selection, query patterns, anti-patterns | 2026-03-02 |
| `planning` | Custom (adapted from Alvarez) | Discuss-Agree-Plan-Approve-Implement-Verify methodology, plan structure | 2026-03-02 |
| `architecture` | Custom (adapted from Alvarez) | ADR pattern, data flow mapping, identifying architectural violations | 2026-03-02 |
| `skills-maintenance` | Custom (adapted from Alvarez) | skills.sh CLI, skill lifecycle management, portability audit protocol | 2026-03-02 |

---

## Source Categories

| Category | Description | Examples |
|----------|-------------|---------|
| `skills.sh` | Downloaded directly from the skills.sh ecosystem -- fully portable, no project-specific content | tauri-v2, svelte5-best-practices, typescript-advanced-types, rust-async-patterns, tailwind-design-system |
| `skills.sh + modified` | Started as a skills.sh skill but modified for Forge patterns | (none yet) |
| `Custom (adapted)` | Adapted from Alvarez project for Forge's architecture and workflow | chunkhound, planning, architecture, skills-maintenance |

---

## Portability Rules

**All skills must be portable technology knowledge.** A skill is portable if it would be useful in a different project using the same technology stack.

**Portable (allowed in skills):**

- How Svelte 5 `$state` reactivity works
- How to write a Rust module with proper error handling
- How to structure a cargo test with fixtures
- How ChunkHound's three search modes work

**Not portable (must go in docs, agent instructions, or rules):**

- Any architecture decision or Forge architectural rule
- Any reference to `$lib/components/shared/` or Forge-specific paths
- Any Forge-specific Tauri command patterns
- Any product pillar or governance rule

The `agent-maintainer` audits skills periodically for portability violations. Any Forge-specific content found in a skill is extracted to the appropriate destination (docs, rules, or agent instructions).

---

## Skill Lifecycle

1. **Discovery** -- Find a skill need during a task. Search skills.sh: `npx skills find <query>`
2. **Installation** -- `npx skills add <source> -y` (installs to `.claude/skills/`)
3. **Documentation** -- Add entry to this skills-log.md with source, purpose, and date
4. **Loading** -- Add to relevant agent YAML frontmatter `skills:` list
5. **Updates** -- `npx skills check` to see available updates, `npx skills update` to apply
6. **Deprecation** -- When a skill is superseded or its domain is gone, mark it and remove after one session
7. **Removal** -- `npx skills remove <name>` and remove from skills-log.md and all agent frontmatter

For full lifecycle management guidance, load the `skills-maintenance` skill.

---

## Related Documents

- [Team Overview](/process/team) -- Which agents load which skills
- [Content Governance](/process/content-governance) -- Skills vs docs vs agent instructions vs rules
