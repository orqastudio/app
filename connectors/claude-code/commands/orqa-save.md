# /orqa-save — Save Session State

Write session state to `.state/session-state.md` so the next session can resume where this one left off.

## What to write

Gather the following and write to `.state/session-state.md`:

```markdown
## Session: <current UTC timestamp>

### Scope
- Epic: <active epic ID and title>
- Tasks: <active task IDs and statuses>
- Persona: <Alex/Sam/Jordan>
- Pillars served: <which pillars this session's work served>

### What Was Done
- <completed items this session>

### In Progress
- <partially done work with current state>

### Next Steps
- <what should happen next session>

### Blockers
- <anything blocking progress, or "None">

### Commits This Session
- <list commits made this session>
```

## How to gather the information

1. **Scope** — check which epic/task you've been working on this session
2. **What Was Done** — summarise completed work from this conversation
3. **In Progress** — note any partially completed tasks
4. **Next Steps** — what the next session should pick up
5. **Blockers** — anything preventing progress
6. **Commits** — run `git log --oneline` to find commits from this session

## Rules

- Always overwrite the existing `.state/session-state.md` — it's for the next session, not a log
- Use absolute dates, not relative ("2026-03-21", not "today")
- Be specific about in-progress state — the next session starts cold
- Include commit hashes so the next session can verify what landed
