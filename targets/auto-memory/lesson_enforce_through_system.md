---
name: Enforce through OrqaStudio's system
description: Use the artifact graph enforcement system, not raw platform hooks or manual checks
type: feedback
---

When adding enforcement for a rule, use OrqaStudio's own enforcement system (artifact graph, composed schema, plugin hooks) — never create raw platform hooks in `.claude/hooks/` or equivalent.

**Why:** IMPL-1dbed312 — the orchestrator created a raw Claude Code Stop hook instead of using the artifact graph enforcement system, bypassing the very system we're building. Dogfooding means using our own tools.

**How to apply:** New enforcement goes through: RULE artifact → enforcement chain (linter/hook/gate/injection) → plugin-generated hooks. Never bypass this by writing directly to the target tool's config.
