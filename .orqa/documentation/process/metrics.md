---
id: DOC-029
title: Process Metrics
description: Process metrics tracked across development including pass/fail rates, coverage, and violation trends.
created: "2026-03-02"
updated: "2026-03-08"
---

**Date:** 2026-03-02

KPIs tracked by the team to measure process health and improvement over time. Updated by the `agent-maintainer` during retrospective additions.

---

## Metrics Table

| Metric | Source | Updated | Period | Value | Trend | Notes |
|--------|--------|---------|--------|-------|-------|-------|
| Review failure rate | Code-reviewer FAIL count / total reviews | Per retrospective | -- | -- | -- | Baseline not yet established |
| Lesson promotion rate | IMPL entries promoted / total IMPL entries | Per retrospective | -- | -- | -- | Baseline not yet established |
| DoR compliance | Tasks that met all DoR items / total tasks started | Per retrospective | -- | -- | -- | Baseline not yet established |
| DoD compliance | Tasks that met all DoD items / total tasks completed | Per retrospective | -- | -- | -- | Baseline not yet established |
| Context window incidents | Sessions that died from context overflow | Per retrospective | -- | -- | -- | Baseline not yet established |
| Stale worktree incidents | Worktrees found unmerged at session start | Per retrospective | -- | -- | -- | Baseline not yet established |
| Build verification pass rate | `cargo build && npm run build` passes after merge / total merges | Per retrospective | -- | -- | -- | Baseline not yet established |
| Skill load rate | Skills loaded per session / total available skills | Future automation | -- | -- | -- | Requires automated instrumentation |

---

## How to Update

The `agent-maintainer` updates this table during retrospective entries:

1. When adding a `RETRO-NNN` entry to `.orqa/documentation/process/retrospectives.md`, also update the relevant metrics
2. Fill in the Period column with the date range being measured
3. Update the Value column with the current measurement
4. Set the Trend column to one of: improving, stable, declining, or -- (insufficient data)
5. Add context in the Notes column explaining significant changes

## Metric Definitions

### Review Failure Rate

**Formula:** Number of `code-reviewer` FAIL verdicts / total code reviews conducted

**Why it matters:** A high failure rate indicates agents are not following coding standards or making the same mistakes repeatedly. A declining rate indicates the learning loops are working.

### Lesson Promotion Rate

**Formula:** IMPL entries with a non-empty "Promoted to" field / total IMPL entries

**Why it matters:** Lessons that are documented but never promoted to rules or standards don't prevent recurrence. A low promotion rate means the learning loop is incomplete.

### DoR Compliance

**Formula:** Tasks where all applicable Definition of Ready items were verified / total tasks started

**Why it matters:** Tasks that skip DoR often require rework. Measuring compliance highlights whether the gate is being enforced.

### DoD Compliance

**Formula:** Tasks where all applicable Definition of Done items were verified / total tasks completed

**Why it matters:** Tasks that skip DoD may be reported as complete but have gaps.

### Context Window Incidents

**Formula:** Count of sessions that ended prematurely due to context window overflow

**Why it matters:** Context overflow causes lost work and session restarts. A declining count indicates the context management rules are effective.

### Stale Worktree Incidents

**Formula:** Count of worktrees found unmerged at session start

**Why it matters:** Stale worktrees indicate incomplete task cleanup, risk merge conflicts, and can cause data loss.

### Build Verification Pass Rate

**Formula:** Number of `cargo build && npm run build` runs that succeed after merge / total merges

**Why it matters:** A failing post-merge build indicates the merge introduced integration issues. Tracking this helps catch regressions early.

### Skill Load Rate

**Formula:** Number of skills actually loaded per session / total available skills

**Why it matters:** Skills that are never loaded may be unnecessary or poorly targeted. Skills that are always loaded may need to be made universal.

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | N/A |
| Learning Through Reflection | Process metrics quantify whether the learning loops are working — tracking lesson promotion rates, review failure trends, and DoR/DoD compliance over time to surface where governance improvements are needed. |

---

## Related Documents

- [Process Retrospectives](DOC-031) -- Where metrics are updated
- [Implementation Lessons](DOC-006) -- Source data for lesson promotion rate
- [Definition of Ready](DOC-028) -- DoR compliance source
- [Definition of Done](DOC-027) -- DoD compliance source
