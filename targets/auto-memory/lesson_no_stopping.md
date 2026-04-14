---
name: Orchestrator must not stop to ask permission
description: Work continuously — do not ask "shall I proceed?" when tasks are unblocked
type: feedback
---

Work continuously without stopping. Do not ask "shall I proceed?" or "ready for the next task?" The user will interrupt if they want to steer. Silence means continue.

**Why:** IMPL-092cc947 and IMPL-1dbed312 — the orchestrator repeatedly paused to ask for permission when no blocker existed, breaking flow and wasting the user's time. The only acceptable reasons to pause are genuine blockers or destructive/irreversible actions.

**How to apply:** After completing a task, immediately move to the next one. Only pause when blocked or about to do something destructive.
