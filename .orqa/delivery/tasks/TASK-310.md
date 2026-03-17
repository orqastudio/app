---
id: TASK-310
title: "Plugin: maintain memory entries for unimplemented ADs"
description: "Use Claude Code's memory system to surface unimplemented architecture decisions. Session-start hook creates/updates concise memory entries for accepted ADs lacking enforced-by/practiced-by edges. Hook also cleans up memory entries when ADs mature. Leverages provider-native memory for compaction-resistant context."
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - Session-start hook scans .orqa/process/decisions/ for accepted ADs
  - ADs with enforced-by or practiced-by relationships are excluded
  - Memory file created per unimplemented AD with concise reference + summary
  - MEMORY.md updated with section listing unimplemented AD memory files
  - Memory files removed when ADs gain enforced-by or practiced-by edges
  - Existing session-start functionality unchanged
  - Stdout AD injection block replaced with memory-based approach
relationships:
  - target: IMPL-040
    type: informed-by
    rationale: Auto-generated inverse of grounded relationship from IMPL-040
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-349
    type: depended-on-by
  - target: IMPL-040
    type: informs
---
## What

Use Claude Code's native memory system to surface architecture decisions that haven't yet matured into enforcement artifacts (rules/skills). The session-start hook manages memory files — creating entries for unimplemented ADs, removing them when ADs mature. This leverages provider capabilities rather than reinventing context injection, and survives context compaction.

## How

1. Replace the stdout AD injection block (lines 147-176) in session-start.sh with memory management
2. Derive Claude memory directory path from project directory
3. On each session start:
   - Scan `.orqa/process/decisions/AD-*.md` for `status: accepted` without `enforced-by`/`grounded` edges
   - Create `ad_AD-NNN.md` memory files for new unimplemented ADs (type: reference, concise summary)
   - Remove `ad_AD-*.md` memory files for ADs that have matured
   - Update MEMORY.md with an "Unimplemented Architecture Decisions" section
4. Memory format: AD ID, title, one-line description, path to full decision

## Verification

- Unimplemented AD → memory file created, MEMORY.md entry added
- AD gains `enforced-by` relationship → memory file removed, MEMORY.md entry removed
- Existing session-start functionality unchanged
- Memory entries survive context compaction (verified by checking MEMORY.md is loaded)

## Lessons

- [IMPL-040](IMPL-040): Decisions scrolled out of view must be resurfaced — memory system addresses this
