## Session: 2026-03-21T20:10:00Z

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
- [ ] **Step 5:** Session state rule + enforcement
  - [x] Rule RULE-4f7e2a91 created
  - [x] Enforcement hook (freshness check + constant reminder in prompt-injector.mjs)
  - [x] Rule documents enforcement mechanism
  - [ ] Schema validation for enforcement field — needs validator check (Step 5d)
- [x] **Step 5a:** Create governance plugin — DONE (11 types, 2 additive relationships, display config)
- [ ] **Step 5b:** Plugin dependency system
  - [ ] Add `requires` field to plugin manifests for declaring dependencies on other plugins
  - [ ] Add `category` field to plugin manifests (thinking | delivery | governance | connector | tooling)
  - [ ] Validator checks: required categories are satisfied, plugin dependencies are met
  - [ ] Claude Code connector declares: requires `@orqastudio/plugin-systems-thinking`, `@orqastudio/plugin-agile-governance`
- [ ] **Step 5c:** Rename + split plugins
  - [ ] Rename `@orqastudio/plugin-governance` → `@orqastudio/plugin-agile-governance` (differentiates from other governance approaches)
  - [ ] Create `@orqastudio/plugin-systems-thinking` (methodology: systems thinking, diagnostic methodology, restructuring methodology)
  - [ ] Rename `@orqastudio/plugin-software-project` stays as-is (already specific enough)
- [ ] **Step 5d:** Migrate core.json content to plugins
  - [ ] Move all artifactTypes, relationships, semantics from core.json to owning plugins
  - [ ] core.json becomes structural definition only (schema ref, required categories, empty arrays)
  - [ ] Verify validator and graph scanner still work
- [ ] **Step 5e:** Add frontmatter.required validation check to CLI validator
- [ ] **Step 5d:** Add frontmatter.required validation check to CLI validator
- [ ] **Step 5e:** PostToolUse hooks validate via `orqa validate` CLI
- [x] **Step 6:** Migrate user preferences to rules — DONE (RULE-045 through RULE-048)
- [ ] **Step 7:** `orqa link` CLI command
- [ ] **Step 8:** Move Claude symlink logic to connector
- [ ] **Step 9:** Dogfood `orqa link`
- [ ] **Step 10:** Thinking mode — dogfood detection
- [ ] **Step 11:** Lessons as memory source (research)
- [x] **Step 12:** Decision pros/cons idea — DONE (IDEA-144)
- [ ] **Step 13:** Orchestrator prompt injection: teams + dependencies reminder

### Architecture Decisions

#### Three required plugin categories
- **Thinking** — methodology plugins. `@orqastudio/plugin-systems-thinking` (systems thinking, diagnostic methodology, restructuring methodology). Replaceable with alternative thinking frameworks.
- **Delivery** — work tracking plugins. `@orqastudio/plugin-software-project` (epics, tasks, milestones). Replaceable with e.g. kanban, scrum-specific, etc.
- **Governance** — structured decision-making. `@orqastudio/plugin-agile-governance` (rules, decisions, lessons, knowledge, learning loop). Replaceable with alternative governance models.
- App REQUIRES one of each category installed. Validator checks this.
- Plugins declare `category` in their manifest.
- Plugins declare `requires` for dependencies on specific other plugins.
- Learning loop stays in governance (operates on lesson→rule artifacts).
- Systems thinking is a separate thinking plugin (methodology, not artifact types).

#### Plugin dependency system
- Plugin manifests get `category` field (thinking | delivery | governance | connector | tooling)
- Plugin manifests get `requires` field (array of plugin names this plugin depends on)
- Claude Code connector requires: `@orqastudio/plugin-systems-thinking`, `@orqastudio/plugin-agile-governance`
- Validator checks: all 3 required categories have at least one installed plugin, all `requires` dependencies are met

#### Plugins own everything
- core.json = structural definition only (schema ref, required plugin slots, empty arrays)
- Each plugin owns its types, relationships, semantics, frontmatter schemas
- Validator merges all plugin schemas at runtime

#### 4-layer agent context
1. System prompt (static) → role + tools
2. UserPromptSubmit hook (ONNX) → thinking mode + where to find knowledge
3. Agent self-service → CLI/graph queries
4. Enforcement hooks → mechanical compliance

#### Other decisions
- CLI is platform-agnostic, connector owns Claude-specific setup
- Rules require enforcement (created together, documented in body)
- All plugins are replaceable (governance, thinking, delivery)

### Bugs
- **BUG: prompt-injector shows "Project: unknown | Dogfood: inactive | Plugins: none"** — the hook is outputting stale/wrong values and using old `|` separator format. Either the running hook is a different version than the source, or our edits to prompt-injector.mjs broke the context line. Need to investigate after commit.

### Lessons
- IMPL-071: Session state as living document → RULE-4f7e2a91
