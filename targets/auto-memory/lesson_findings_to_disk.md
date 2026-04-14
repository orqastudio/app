---
name: Agent findings must be written to disk
description: Agent work products go in findings files — orchestrator reads files, not accumulated context
type: feedback
---

Agent task completion requires findings written to disk. The orchestrator reads findings files — it does not accumulate agent output in its own context window.

**Why:** P6 (Hub-Spoke Orchestration) and P2 (One Context Window Per Task). Accumulating agent output in the orchestrator's context causes it to exceed token limits and lose coherence. Findings files are the structured handoff mechanism.

**How to apply:** Every agent writes a findings file on task completion. The orchestrator reads the file to assess results. Never inline large agent outputs into the orchestrator conversation.
