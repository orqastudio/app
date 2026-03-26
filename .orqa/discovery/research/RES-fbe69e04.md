---
id: "RES-fbe69e04"
type: discovery-research
title: "Project Configuration, Plugin Pairing, and Open-Item Tracking"
description: "Investigates four observations from EPIC-88f359b0 implementation: hardcoded paths needing project config (IMPL-a73db2e6), plugin-sidecar pairing requirements (IMPL-b8ec72e2), plugin context scoping (IMPL-acea0394), and untracked open items during implementation (IMPL-0809b549)."
status: completed
created: "2026-03-13"
updated: "2026-03-13"
sources: []
relationships:
  - target: "EPIC-88f359b0"
    type: "guides"
    rationale: "Research findings informed epic design"
---
## Research Questions

### Q1: How should source code discover .orqa/ paths? (IMPL-a73db2e6)

**Current state:** `paths.rs` defines constants (`PILLARS_DIR`, `LESSONS_DIR`, etc.). `project_scanner.rs` hardcodes directory joins. `artifact_fs.rs` has a `governance_dir()` function with hardcoded mappings. Meanwhile, `project.json` has an `artifacts` array that already lists most of these paths — but source code doesn't read from it.

**Key tension:** [RULE-63cc16ad](RULE-63cc16ad) already says "no hardcoded paths in Rust or TypeScript — all artifact paths come from config." But the current `paths.rs` constants are the opposite of that. Why do they exist?

**Investigation:**

1. **What uses `paths.rs` constants?** These are used by the delivery workflow (task/epic/milestone creation and lookup). The scanner uses `project.json` config. So there are TWO path discovery mechanisms — config-driven scanning and constant-driven creation.

2. **Can `project.json` be the single source?** The `artifacts` array has `path` fields. The creation commands could read from config instead of constants. But config requires loading the project file, parsing JSON, and looking up the right entry by key — more complex than a constant.

3. **What about required vs optional directories?** Some directories MUST exist for the system to function (e.g., `tasks/` for task creation). Others are optional (e.g., `verification/` may not exist yet). The path manifest (`tools/path-manifest.json`) already distinguishes these, but it's a build-time tool, not a runtime config.

**Options:**

- **A. Config-only:** Remove `paths.rs`, all code reads from `project.json`. Pro: single source of truth. Con: every path lookup requires config parsing.
- **B. Config-generated constants:** A build step reads `project.json` and generates `paths.rs`. Pro: compile-time safety. Con: build complexity.
- **C. Runtime config cache:** Load `project.json` once at startup, cache paths in a struct passed to all services. Pro: single load, fast access. Con: requires passing config through the call chain.
- **D. Config for scanning, constants for creation:** Accept the duality but ensure constants are derived from config values. Document explicitly that `paths.rs` mirrors `project.json` and verify-links checks alignment.

**Recommendation:** Option C (runtime config cache) is the cleanest — load `project.json` once, build a `ProjectPaths` struct, pass it where needed. This aligns with how the scanner already works (reads config at startup) and eliminates the constant/config duality. But this is a non-trivial refactor touching many modules.

For the immediate term, Option D acknowledges the duality and adds enforcement (verify-links checks that `paths.rs` constants match `project.json` paths). This can be done now; Option C can be a future epic.

---

### Q2: How should plugins declare sidecar requirements? (IMPL-b8ec72e2)

**Current state:** The `orqastudio-claude-plugin` works with Claude Code CLI. Its hooks use Claude Code tool names, its skills reference Claude Code MCP servers, its agents assume Claude Code capabilities. Nothing in `plugin.json` declares this dependency.

**Investigation:**

1. **What is a sidecar?** In OrqaStudio's architecture, a sidecar is the bridge between the AI provider and the Rust backend. Claude Code CLI is one sidecar. The app's built-in Agent SDK is another. Cursor, Copilot, Windsurf would be others — each with their own plugin/extension ecosystems.

2. **What breaks without pairing?** If the Claude plugin's hooks fire in a Cursor context:
   - Tool name references (`Read`, `Edit`, `Bash`) are wrong — Cursor uses different names
   - MCP server availability assumptions fail
   - Skill content references unavailable capabilities
   - Agent definitions describe roles that can't be fulfilled

3. **What does a requirement look like?**

```jsonc
// plugin.json
{
  "name": "orqastudio-claude-plugin",
  "requires": {
    "sidecar": "claude-code"  // or an array for multi-sidecar plugins
  }
}
```

4. **How is sidecar identity detected?** The system needs to know which sidecar is active:
   - CLI: the sidecar IS the CLI tool (Claude Code, Cursor, etc.) — detectable from environment variables or process context
   - App: the app IS the sidecar (Agent SDK built-in) — always `orqa-app`
   - Context detection already exists in [RULE-8abcbfd5](RULE-8abcbfd5) capability resolution — extend it

**Recommendation:** Add a `requires.sidecar` field to the plugin schema. The sidecar identifier is a string enum (`claude-code`, `cursor`, `copilot`, `orqa-app`, etc.). The plugin loader checks this field against the detected sidecar context and skips incompatible plugins.

---

### Q3: How should the system scope plugin activation? (IMPL-acea0394)

**Current state:** All plugins in `.orqa/plugins/` are treated as active. Their rules, skills, agents, and hooks are symlinked into `.claude/` (for CLI) or loaded by the app scanner.

**Investigation:**

1. **Symlink architecture problem:** Currently, `.claude/rules/` → `.orqa/governance/rules/` (now `.orqa/process/rules/`). But plugin rules live in `.orqa/plugins/<plugin>/rules/`. How do plugin rules get into `.claude/rules/`? They don't — they need separate symlinks or the plugin loader needs to compose them.

2. **Activation model options:**
   - **A. Load-time filtering:** Plugin loader reads `requires.sidecar`, skips non-matching plugins entirely. Their artifacts never enter the active context.
   - **B. Runtime scoping:** All plugins load, but their hooks/skills/agents are tagged with sidecar context and only fire when the context matches.
   - **C. Symlink management:** The plugin installer creates/removes symlinks based on detected sidecar. Changing sidecars requires re-running plugin setup.

3. **App context:** The app shows ALL plugins in its plugin management UI regardless of sidecar — but only activates compatible ones for agent execution. This allows users to see what's installed across all contexts.

**Recommendation:** Option A (load-time filtering) is simplest and safest. When the system detects sidecar context, it filters `.orqa/plugins/` entries by their `requires.sidecar` field. Non-matching plugins are invisible to the hook engine, skill loader, and agent resolver. The app UI still shows them (greyed out) for management purposes.

---

### Q4: How should the process capture open items during implementation? (IMPL-0809b549)

**Current state:** When work reveals open items (dead code, cleanup needed, research questions), agents report them in conversation but don't create task artifacts. The items exist only in conversation history and Claude memory — both lossy. The epic gets marked done while untracked work remains.

**Investigation:**

1. **Why does this happen?** The agent is focused on completing the current task. Creating a new task for emergent work is a context switch that breaks flow. The agent notes the item for later but "later" never comes because context compacts.

2. **When do open items emerge?**
   - During code changes: "this Hook type is dead code" → needs a cleanup task
   - During research: "this question needs its own investigation" → needs a research doc
   - During review: "these broken links need fixing" → needs a fix task
   - During testing: "this test fixture assumes old structure" → needs an update task

3. **What's the right moment to capture?** Immediately when identified, not batched at end. The cost of creating a task file is low (2 minutes). The cost of losing the item is high (it may never be rediscovered).

4. **What gate should prevent premature epic completion?** The epic completion check should verify:
   - All tasks are done ✓ (already enforced)
   - No open items exist in the session that lack tasks ✗ (not enforced)
   - Session state doesn't mention pending work ✗ (not enforced)

**Recommendation:** Three changes:

1. **Discipline change:** When an agent identifies an open item during implementation, it creates a `TASK-NNN` immediately with `status: todo` and `epic: EPIC-NNN`. This is faster than noting it for later and ensures nothing is lost. The task body can be minimal — just enough to capture the item.

2. **Completion gate change:** Before marking an epic done, the orchestrator must:
   - Review the session for any mentioned-but-untracked items
   - Check that no `todo` tasks remain under the epic
   - Verify `.state/session-state.md` doesn't list pending work for this epic

3. **Human gate for epic completion:** The orchestrator MUST NOT mark an epic as `done` without explicit user approval. The orchestrator presents: (a) completed task summary, (b) any remaining `todo` tasks, (c) open items or observations logged during implementation, and (d) asks "Are you satisfied this epic is complete?" This gives the user the opportunity to ask clarifying questions, identify missed items, or flag concerns before the epic is closed. Agent-initiated epic completion is forbidden.

---

## Cross-Cutting Findings

### The Config/Constants Duality

Q1 and Q3 share a root cause: the system has two path discovery mechanisms that aren't unified. The scanner reads `project.json` (dynamic). Source code uses `paths.rs` constants (static). Plugins add a third layer — their own internal paths. A unified path resolution system (Option C from Q1) would solve this for all three.

### The Pairing Pattern

Q2 and Q3 are two sides of the same coin: declaration (Q2) and enforcement (Q3). This is exactly the Knowledge Maturity Pipeline pattern: an observation becomes understanding, understanding becomes a principle, the principle becomes enforcement. The plugin-sidecar pairing is currently at the observation stage. It needs:
- A principle (AD) establishing that plugins and sidecars are paired
- Enforcement (plugin schema + loader filtering)
- Verification (verify-links or similar checks that all active plugins match the sidecar)

### The Tracking Gap

Q4 reveals a gap in the pipeline itself: the process for capturing observations ([RULE-c603e90e](RULE-c603e90e)) exists, but the process for converting observations into tracked work items during active implementation doesn't. [RULE-d5d28fba](RULE-d5d28fba) says "structure before work" but there's no "structure during work" equivalent.

### Q5: How should agents automate observation logging and recurrence tracking? (IMPL-a1373533)

**Current state:** [RULE-c603e90e](RULE-c603e90e) requires review agents to check lessons and log findings. In practice, implementation agents — who encounter the most "why did that happen?" moments — don't log observations because it's a context switch that competes with their primary task. Recurrence counters stay at 1; promotion thresholds are never reached; the pipeline stalls.

**Investigation:**

1. **When do observation-worthy moments occur?** When an agent:
   - Encounters a surprising failure (test fails for non-obvious reason)
   - Discovers a gap between documentation and reality
   - Has to work around a missing capability
   - Fixes a bug that has a systemic root cause
   - Gets feedback from the user correcting their approach

2. **How could logging be automated?**
   - **Learning checkpoint:** After every fix or workaround, the agent pauses to search `.orqa/process/lessons/` for matching patterns. If found, increment recurrence. If new, create IMPL.
   - **Orchestrator pattern-matching:** The orchestrator monitors agent outputs for signals ("this happened because...", "I had to work around...", "the user corrected...") and triggers observation logging.
   - **Post-task hook:** When a task moves to `done`, a hook scans the task's conversation for surprise/fix patterns and prompts for lesson logging before the status change is committed.

3. **How to keep it lightweight?** The learning checkpoint must not double task time. Options:
   - A single `search_regex` against lesson titles/descriptions — 2 seconds
   - If no match: create a minimal IMPL with just a title and one-line pattern — 30 seconds
   - If match: increment recurrence counter — 10 seconds
   - Total overhead: under 1 minute per fix event

**Recommendation:** Make the learning checkpoint a required step in task completion, enforced by the orchestrator. When an agent reports a task done, the orchestrator asks: "What observations were logged or updated during this task?" If the answer is "none" for a task that involved any debugging, workaround, or user correction, the orchestrator prompts for lesson review before accepting completion.

### Q6: How should lessons be recorded on task completion artifacts? (IMPL-b27c458f)

**Current state:** Task artifacts have body sections: What, How, Verification. No section captures what was learned. The user has to read conversation history to discover observations.

**Investigation:**

1. **What should the section contain?**
   - IMPL-NNN references for new observations created during this task
   - IMPL-NNN references for existing observations with recurrence incremented
   - Brief description of each (one line)
   - "None" if the task was straightforward with no learning moments

2. **Should it be required?** Yes — but "None — straightforward implementation" is a valid entry. The section's purpose is to make the *absence* of learning as visible as the *presence* of it. A task with three workarounds and "Lessons: None" is a red flag.

3. **How does this aggregate at epic completion?** When the orchestrator presents the epic for human-gate review, it collects all Lessons sections from the epic's tasks and presents a summary: "During this epic, X new observations were created and Y existing observations had recurrence incremented. Z observations reached the promotion threshold."

**Recommendation:** Add a "Lessons" section to the task body template. Required, not optional. Format:

```markdown
## Lessons

- Created [IMPL-a73db2e6](IMPL-a73db2e6): Hardcoded paths should be project-configurable
- Updated [IMPL-ffb199b5](IMPL-ffb199b5): recurrence 2 → 3 (stale cache pattern)
- None — straightforward implementation
```

The orchestrator verifies this section exists and is non-empty before accepting task completion.

## Open Items Inventory

All previously-untracked open items that need tasks under [EPIC-88f359b0](EPIC-88f359b0):

| Item | Type | Description |
|------|------|-------------|
| [RES-98ab39e0](RES-98ab39e0) completion | Research | Behavioral directives in the pipeline — draft research awaiting discussion |
| Claude memory cleanup | Cleanup | 6 memory files duplicate artifact knowledge, should be pruned |
| ArtifactType::Hook removal | Dead code | Hook enum variant and all handling code — hooks are plugin implementation |
| Broken forward-references | Fix | 9 broken-link errors to non-existent artifacts (KNOW-bcfeb64e, AGENT-4c94fe14/4/5, VER-001) |
| [IMPL-a73db2e6](IMPL-a73db2e6) → configurable paths | Investigation | Source code paths should come from project config |
| [IMPL-b8ec72e2](IMPL-b8ec72e2) → plugin-sidecar pairing | Investigation | Plugins need a sidecar requirement mechanism |
| [IMPL-acea0394](IMPL-acea0394) → plugin context scoping | Investigation | Non-matching plugins should be disabled |
| [IMPL-0809b549](IMPL-0809b549) → open item tracking | Process | Emergent work items need immediate task creation |
| [IMPL-e13eb86c](IMPL-e13eb86c) → epic readiness visibility | UI/Process | Epics ready for review should be surfaced to user |
| [IMPL-a1373533](IMPL-a1373533) → automated observation logging | Process | Agents should auto-log observations, not rely on manual discipline |
| [IMPL-b27c458f](IMPL-b27c458f) → lessons on task artifacts | Process | Task completion should record what was learned, visible to user |
| [IMPL-db8027b6](IMPL-db8027b6) → lesson triage on epic close | Process | Observations must flow forward — implement, promote, or defer to idea |

## Decisions Made

| Question | Decision | Rationale |
|----------|----------|-----------|
| Q1: Path discovery | Option C — runtime config cache | User chose "do it properly". ProjectPaths struct loaded from project.json at startup. |
| Q2a: Plugin type taxonomy | Array of types: `ai-provider-integration`, `visualisation`, `governance-extension`, `data-connector` | Single plugin can span multiple types. Enum is extendable. |
| Q2a: Plugin requires shape | Determined by declared types — union of type-specific requires fields | `ai-provider-integration` adds `requires.ai-providers`, other types add their own fields |
| Q2a: ai-providers semantics | Array with `any-of` or `all` qualifier | Plugin may work with multiple providers but in practice only one may be necessary |
| Q2b: Provider definitions location | `.orqa/providers/<name>.json` — one file per provider | App-native, scannable, follows artifact pattern, easy to converge into plugins later (IDEA-d2a429c3) |
| Q2b: Provider-plugin relationship | Bidirectional — providers declare required/optional plugins, plugins declare supported providers | Adding a new provider triggers plugin ecosystem review |
| Missing sidecar field on plugin | No field = plugin doesn't affect AI provider integration | Valid for visualisation-only or governance-extension plugins |
| Q2c: Provider schema shape | Identity + detection + required plugins only. Capability mappings live in plugin, not provider. | Provider is just identity until [IDEA-d2a429c3](IDEA-d2a429c3) makes sidecars transportable as plugins |
| Q2c: Plugin type array | `type` is an array, single plugin can span multiple types | A provider integration plugin may also include visualisation |
| Q2d: Capability fulfilment | User choice between native provider tools and app MCP, per capability | Allows governance routing, new provider features before app support, consistent cross-provider behaviour |
| Q2d: Capability config location | Per-project in project.json, with plugin defaults and app baseline | Resolution: project config > plugin defaults > app baseline |
| Q2d: Plugin-provided defaults | Plugins ship default-capabilities with sane mappings, can ADD new capabilities | Extends capability vocabulary when provider supports something app doesn't |
| Q2d: Plugin installation wiring | Plugin installs capabilities + skills + agent updates as a complete package | Installation process wires everything, uninstallation unwires |
| Q2d: Provider declares required plugins | Yes for now — both sides declare the relationship | Becomes unnecessary when [IDEA-d2a429c3](IDEA-d2a429c3) (sidecar-as-plugin) lands |
| Q4 (3a): Epic completion gate | Human gate — orchestrator presents summary, user approves | Agent-initiated epic completion is forbidden |
| Q4 (3b): Open items tracking | Every open item becomes a task immediately, no threshold | Creates full audit trail, artifact graph IS institutional memory |
| Q4 (3b): Epic readiness surfacing | Auto-surface epics where all tasks are done but epic not verified by user | Prevents epics staying perpetually open by lack of attention |
| Q4 (3c): Observation logging timing | Blocking: immediate. Non-blocking: task completion. Borderline: ask user. | User context matters — overnight work favours continue with caveat, supervised favours block |
| Q4 (3d): Lessons section on tasks | Required body section, "None — straightforward" is valid | Makes learning visible, decreasing frequency signals pipeline health |
| Q4 (3e): Lesson triage on epic close | Auto-create triage task when first observation logged under an epic | Every observation must have forward path: implement, promote, or defer to idea |
| Scope boundary | Learning loop + path cache + cleanup = [EPIC-88f359b0](EPIC-88f359b0). Plugin/provider/capability = new epic | Plugin ecosystem is a different concern from philosophy alignment |