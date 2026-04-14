---
name: Commit at natural boundaries
description: Commit at task/phase completion — do not accumulate uncommitted changes across multiple tasks
type: feedback
---

Commit at natural boundaries (task completion, phase completion). Do not accumulate uncommitted changes across multiple tasks.

**Why:** Large uncommitted changesets are hard to review, hard to revert, and create merge conflicts. Multiple sessions have lost work or created confusion from accumulated changes.

**How to apply:** After each task is reviewed and accepted, commit the changes before starting the next task. Agents do NOT commit — the orchestrator commits after reviewing findings.
