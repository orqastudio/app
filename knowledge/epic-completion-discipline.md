---
id: KNOW-SW-epic-complete
type: knowledge
title: "Epic completion discipline"
description: "Epics must be completed in their entirety — never left half-done. All tasks delivered before moving on."
status: active
created: 2026-03-21
updated: 2026-03-21
injection:
  paths: [".orqa/delivery/epics/", ".orqa/delivery/tasks/", ".orqa/delivery/milestones/"]
  artifact_types: ["epic", "task", "milestone"]
  keywords: ["epic", "milestone", "deliver", "complete", "finish", "wrap up", "next session"]
relationships: []
---

## Epic Completion Discipline

**Epics are atomic units of delivery. They are completed in their entirety or not at all.**

### Rules

1. **Never leave an epic half-done.** When you start an epic, complete all tasks before moving to the next one.
2. **Commit and push regularly** throughout the work — don't batch up changes for a single commit at the end.
3. **If the session must end before completion**, persist session state to `tmp/session-state.md` with:
   - Which tasks are done
   - Which are in progress (and their exact state)
   - What the next steps are
   - Any blockers
4. **Track progress visibly** — update task statuses as you complete them, not in bulk at the end.
5. **Definition of done for an epic:**
   - All tasks marked completed
   - All acceptance criteria checked off
   - `orqa validate` passes
   - Changes committed and pushed
   - Session state updated

### Why This Matters

Partial epics create:
- **Context loss** — the next session doesn't know what was planned vs what was done
- **Orphaned work** — tasks started but not finished become stale
- **Scope drift** — without completion discipline, epics grow indefinitely
- **False confidence** — reporting an epic as "in progress" when only 2 of 14 tasks are done

### Anti-Patterns

- Starting a new epic before the current one is complete
- Marking an epic as "review" when tasks are still pending
- Deferring "easy" tasks to a future session
- Reporting partial work as complete

### Enforcement

This knowledge is automatically injected when working on delivery artifacts (epics, tasks, milestones) via the `injection` metadata in this file's frontmatter.
