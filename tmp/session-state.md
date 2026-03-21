## Session: 2026-03-21T20:20:00Z

### Scope
- Epic: EPIC-6967c7dc (Claude Code connector rewrite)
- Persona: Alex (strategic, governance-focused)
- Pillars: Clarity, Continuity
- Team: connector-rewrite

### Steps

- [x] **Step 1:** Verify hook output
- [x] **Step 2:** Clean orchestrator.md
- [x] **Step 3:** Symlink .claude/CLAUDE.md → orchestrator.md
- [x] **Step 4:** Verify end-to-end
- [x] **Step 5:** Session state rule + enforcement (RULE-4f7e2a91 + hook)
- [x] **Step 5a:** Create governance plugin — DONE
- [x] **Step 5b:** Plugin dependency system — DONE
- [x] **Step 5c:** Rename + split — DONE (validator clean, 0 errors)
- [x] **Step 5d:** Migrate core.json → plugins — DONE (Option A: data in plugin, core.json deprecated duplicate, Rust migration separate task)
- [ ] **Step 5d:** Migrate core.json content to plugins
- [ ] **Step 5e:** Add frontmatter.required validation check to CLI validator
- [ ] **Step 5f:** PostToolUse hooks validate via `orqa validate` CLI
- [x] **Step 6:** Migrate user preferences to rules — DONE (RULE-045 through RULE-048)
- [ ] **Step 7:** `orqa link` CLI command
- [ ] **Step 8:** Move Claude symlink logic to connector
- [ ] **Step 9:** Dogfood `orqa link`
- [ ] **Step 10:** Thinking mode — dogfood detection
- [ ] **Step 11:** Lessons as memory source (research)
- [x] **Step 12:** Decision pros/cons idea — DONE (IDEA-144)
- [ ] **Step 13:** Orchestrator prompt injection: teams + dependencies reminder
- [ ] **Step 14:** Session state ↔ task list synchronisation

**COMMITTED:** 30fc6de — Steps 1-6, 5a, 12

### Architecture Decisions

#### Three required plugin categories
- **Thinking** — `@orqastudio/plugin-systems-thinking`. Replaceable.
- **Delivery** — `@orqastudio/plugin-software-project`. Replaceable.
- **Governance** — `@orqastudio/plugin-agile-governance`. Replaceable.
- App REQUIRES one of each. Validator checks.

#### Plugin dependency system
- Manifests get `category` + `requires` fields
- Connector requires systems-thinking + agile-governance
- Validator checks categories and dependencies

#### Plugins own everything
- core.json = structural definition only
- Each plugin owns types, relationships, semantics, frontmatter schemas

#### Session state ↔ task list sync needed

### Bugs
- **prompt-injector stale hooks** — ROOT CAUSE: `plugins/claude/hooks/hooks.json` declares hooks but has no scripts. Leftover from before connector split. Fix: remove hook declarations from plugins/claude. Pending approval.

### Lessons Captured
- IMPL-071: Session state as living document → promoted to RULE-4f7e2a91
- IMPL-072: Dogfooding proves the methodology before the app (CLI-first governance works)
- IMPL-073: Dogfood readiness signal — when lessons are observations not corrections, ready for next level
- IMPL-074: Orchestrator must trace artifacts to usage contexts → recurrence 3, PROMOTING TO RULE NOW
  - Occurrence 1: didn't trace IMPL-073 to dogfood milestone
  - Occurrence 2: said "worth noting" but didn't note it
  - Occurrence 3: said "eligible for promotion" but didn't promote it
  - Governance steward creating rule + updating lesson status
  - → RULE-049 created (RULE-67b91c13). Enforcement: Task #16 (thinking mode templates)

### Dogfood Milestone Readiness Signal
- Gate question: "Are recent lessons observations rather than corrections?"
- If yes → current infrastructure layer is mature, ready to dogfood at next level (CLI → app)
- Add as gate on Continuity pillar or readiness criterion in RULE-009 (dogfood mode)
