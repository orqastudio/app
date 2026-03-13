---
id: DOC-031
title: Process Retrospectives
description: Log of process retrospectives capturing what went well, friction points, and improvement actions.
created: "2026-03-02"
updated: "2026-03-08"
---

**Date:** 2026-03-02

A living record of process-level lessons: how the agentic development system was shaped by real failures and deliberate improvements. Each entry documents a problem, its root cause, the change made, and whether the change worked.

For implementation-specific lessons (coding mistakes, architectural gotchas), see Implementation Lessons.

New entries are added by the `agent-maintainer` when process changes are made.

---

## Lessons Learned from Alvarez

The following RETRO entries are imported from the Alvarez project. They represent patterns discovered during Alvarez development that informed OrqaStudio™'s governance framework from day one.

---

## RETRO-007: Process Retrospectives Are Worth the Time

*2026-03-02 -- Imported from Alvarez bootstrap guide.*

- **Context:** Each RETRO entry in Alvarez represents a real failure and a real fix. Starting a new project with the Alvarez retrospectives provides a significant head start in avoiding known process failures.
- **Root cause:** Without a retrospectives log, each new project repeats the same mistakes the prior project already solved.
- **Action taken (OrqaStudio):** Imported Alvarez lessons as RETRO-001 through RETRO-007 at project start. Established `process/retrospectives.md` on day one.
- **Outcome:** Pending -- OrqaStudio development has not yet started implementation. Expected: known failure modes avoided from the first sprint.
- **Status:** Active

---

## RETRO-006: Use Git Worktrees from Day One

*2026-03-02 -- Imported from Alvarez bootstrap guide.*

- **Context:** In Alvarez, direct work on main caused merge conflicts and data loss risks. A stash-related incident (RETRO-001 in Alvarez) resulted in the loss of 145 documentation files. Worktree discipline was retrofitted, but establishing it earlier would have prevented the incident entirely.
- **Root cause:** Working directly on `main` means uncommitted changes are exposed to every git operation. Worktrees provide isolation by default.
- **Action taken (OrqaStudio):** Git worktree workflow established as mandatory from project start. Rules defined in `.orqa/process/rules/git-workflow.md` including data loss prevention, stash policy, and background process discipline.
- **Outcome:** Pending -- no incidents yet. Framework is in place.
- **Status:** Active

---

## RETRO-005: Log Implementation Lessons Immediately

*2026-03-02 -- Imported from Alvarez bootstrap guide.*

- **Context:** In Alvarez, implementation lessons were not documented until the self-learning loop was established late in the project. Patterns were much harder to document retroactively. Early sessions repeated the same mistakes because no lesson log existed.
- **Root cause:** No structured learning loop between review failures and process improvement from the start.
- **Action taken (OrqaStudio):** Created `.orqa/process/rules/lessons-learned.md` and `.orqa/documentation/development/lessons.md` structure at project start. Review agents required to log failures immediately. Promotion pipeline defined (recurrence >= 2 -> rules/standards).
- **Outcome:** Pending -- system established, awaiting first implementation sessions.
- **Status:** Active

---

## RETRO-004: ChunkHound Is Mandatory from the Start

*2026-03-02 -- Imported from Alvarez bootstrap guide.*

- **Context:** Every Alvarez session that used ChunkHound instead of manual grepping saved significant time and context budget. Sessions without ChunkHound consumed context on manual file exploration that could have been delegated.
- **Root cause:** Manual code search fills orchestrator context with raw file contents. ChunkHound delegates the search to an MCP tool that returns targeted results.
- **Action taken (OrqaStudio):** `chunkhound` skill listed as mandatory in every agent's YAML frontmatter. `.orqa/process/rules/chunkhound-usage.md` enforces ChunkHound preference over Grep/Glob. `.mcp.json` configured for ChunkHound from project start.
- **Outcome:** Pending -- awaiting first implementation sessions.
- **Status:** Active

---

## RETRO-003: Make DoR and DoD Explicit from the Start

*2026-03-02 -- Imported from Alvarez bootstrap guide.*

- **Context:** In Alvarez, vague completion criteria led to "done" tasks that did not actually work. The Definition of Ready and Definition of Done were retrofitted after multiple incidents of incomplete work being reported as complete.
- **Root cause:** Without explicit gate checklists, "done" is subjective. Agents naturally report work as complete when the code compiles, not when the feature works end-to-end.
- **Action taken (OrqaStudio):** Created `.orqa/documentation/process/definition-of-ready.md` and `.orqa/documentation/process/definition-of-done.md` at project start. Orchestrator configuration references both gates as non-negotiable. Orchestrator verifies DoR before delegation and DoD before marking complete.
- **Outcome:** Pending -- gates established before first sprint.
- **Status:** Active

---

## RETRO-002: Create All Sidebar Files Immediately

*2026-03-02 -- Imported from Alvarez bootstrap guide.*

- **Context:** In Alvarez, sidebar synchronization was a recurring source of broken navigation. The `sidebar-synchronization.md` rule was created after multiple incidents of pages added to one sidebar but not others, producing dead links.
- **Root cause:** Sidebar files were created incrementally as sections were populated. By the time the synchronization rule was established, many sidebars were already out of sync.
- **Action taken (OrqaStudio):** Initially adopted from Alvarez. Superseded by [AD-020](AD-020) — Docsify replaced by OrqaStudio's built-in doc viewer with filesystem-driven navigation. Rule and sidebar files removed.
- **Outcome:** Resolved — sidebar synchronization no longer applicable. Doc navigation is generated dynamically from the project's `docs/` directory.
- **Status:** Closed [AD-020](AD-020)

---

## RETRO-001: Establish Content Governance on Day One

*2026-03-02 -- Imported from Alvarez bootstrap guide.*

- **Context:** In Alvarez, the four-layer content governance framework (docs, agents, skills, rules) was established after an audit revealed that 10 of 11 agents restated the same rules with stale variations, and skill files contained project-specific AD numbers and paths. Retrofitting governance required auditing every agent file and skill file, extracting Alvarez-specific content, and replacing duplication with references.
- **Root cause:** No ownership boundary between content layers from the start. Agents found it convenient to add context directly rather than referencing docs, and the violation pattern propagated across agents.
- **Action taken (OrqaStudio):** Content governance model defined at project start in `.orqa/documentation/process/content-governance.md`. Rules, agents, skills, and docs each have clear ownership. `agent-maintainer` has periodic audit responsibility. `code-reviewer` includes doc-layer compliance in reviews.
- **Outcome:** Pending -- framework established before first sprint. Expected: no content duplication from the start.
- **Status:** Active

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | N/A |
| Learning Through Reflection | Retrospectives are the primary mechanism through which process failures become structured improvements — each RETRO entry converts a real failure into an action that is tracked, enforced, and reflected in future sessions. |

---

## Related Documents

- Implementation Lessons -- Implementation-level patterns (coding mistakes, architectural gotchas)
- Rules Reference -- Rules created as a result of retrospective actions
- Content Governance -- Framework established in response to Alvarez lessons
- Definition of Done -- Review gate informed by Alvarez experience
- Metrics -- Quantitative tracking of process health
