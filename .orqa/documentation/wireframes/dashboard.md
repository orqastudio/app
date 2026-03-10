---
id: DOC-060
title: "Wireframe: Dashboard Views"
description: Wireframe specification for dashboard views including metrics, status, and activity summaries.
created: "2026-03-02"
updated: "2026-03-04"
---

**Date:** 2026-03-02 | **Informed by:** [Information Architecture](DOC-040), [Frontend Research](RES-004)

Dashboard views appear in the Explorer Panel and provide operational visibility into scanning, metrics, and the learning loop. These are Phase 3-5 features designed early to validate the information architecture and panel system.

> **Note:** These wireframes represent future-phase features (Scanner Dashboard: Phase 3+, Metrics Dashboard: Phase 5, Learning Loop: Phase 5). They are designed now to ensure the panel system and data model accommodate these views without rework.

### How to Navigate to Dashboards

Dashboard views are Explorer Panel views, accessed the same way as the artifact browser or settings:

| Entry Point | Action |
|-------------|--------|
| **Activity Bar icons** | Scanners, Metrics, and Learning icons in the Activity Bar switch the Explorer Panel to the corresponding dashboard view. |
| **Project Dashboard** | Scanner status, metrics, and learning summaries on the Project Dashboard are clickable links that activate the corresponding Activity Bar icon. |
| **Activity Bar** | Click the Scanners, Metrics, or Learning icons in the Activity Bar. |

All dashboards share the Explorer Panel — switching to a dashboard replaces the current Explorer view. The conversation always remains visible in the Chat Panel.

---

## 1. Scanner Dashboard (Mixed Pass/Fail Results)

The scanner dashboard shows all configured scanners with their current status, accessible via the Scanners icon in the Activity Bar.

```plantuml
@startsalt
{+
  { <b>Scanners</b> (via Activity Bar) }
  ---
  { <b>Scanner Dashboard</b> | . | . | [Run All Scanners] }
  ---
  { Filter: | (X) All | () Passing | () Failing }
  ---
  {+
    <b>Pass/Fail Trend (Last 30 Days)</b>
    ---
    {
      100% |..****..**
      . 75% |****..****
      . 50% |..........
      . 25% |..........
      .  0% |__________
      .     . D1 . . . . D30
    }
    ---
    Pass rate: <b>78%</b> (7/9 scanners)
  }
  ---
  {SI
    {
      <color:green><&circle-check></color> <b>lint-rules</b>
      ESLint governance rules | Last run: 2 min ago
      Result: <b>PASS</b> | 0 violations
    }
    ---
    {
      <color:green><&circle-check></color> <b>type-check</b>
      TypeScript strict mode | Last run: 2 min ago
      Result: <b>PASS</b> | 0 violations
    }
    ---
    {
      <color:red><&circle-x></color> <b>architecture-boundaries</b>
      Module dependency rules | Last run: 5 min ago
      Result: <b>FAIL</b> | 3 violations
    }
    ---
    {
      <color:green><&circle-check></color> <b>api-contract</b>
      OpenAPI spec compliance | Last run: 5 min ago
      Result: <b>PASS</b> | 0 violations
    }
    ---
    {
      <color:red><&circle-x></color> <b>security-scan</b>
      Dependency vulnerability check | Last run: 12 min ago
      Result: <b>FAIL</b> | 2 violations
    }
    ---
    {
      <color:green><&circle-check></color> <b>naming-conventions</b>
      File and export naming rules | Last run: 2 min ago
      Result: <b>PASS</b> | 0 violations
    }
    ---
    .
  }
}
@endsalt
```

### Scanner List Elements

| Element | Behavior |
|---------|----------|
| **Status icon** | Green check for pass, red X for fail. |
| **Scanner name** | Click to expand violation details (see wireframe 2). |
| **Description** | Brief summary of what the scanner checks. |
| **Last run** | Relative timestamp of last execution. |
| **Result** | PASS/FAIL badge with violation count. |
| **Run All Scanners** | Triggers all scanners. Individual scanners run via their expanded view. |
| **Filter radio buttons** | All shows every scanner; Passing/Failing filters the list. |
| **Trend chart** | LayerChart area chart showing pass rate over last 30 days. |

---

## 2. Scanner Dashboard -- Violation Detail Expanded

When a failing scanner row is clicked, it expands to show individual violations with file locations and descriptions.

```plantuml
@startsalt
{+
  { <b>Scanners</b> (via Activity Bar) }
  ---
  { <b>Scanner Dashboard</b> | . | . | [Run All Scanners] }
  ---
  { Filter: | (X) All | () Passing | () Failing }
  ---
  {SI
    {
      <color:green><&circle-check></color> <b>lint-rules</b>
      ESLint governance rules | Last run: 2 min ago
      Result: <b>PASS</b> | 0 violations
    }
    ---
    {+
      <color:red><&circle-x></color> <b>architecture-boundaries</b>
      Module dependency rules | Last run: 5 min ago
      Result: <b>FAIL</b> | 3 violations | [Run Scanner]
      ---
      {#
        # | File | Violation | Severity
        1 | src/ui/api/direct-call.ts | UI layer imports core directly, must use service layer | Error
        2 | src/hooks/db-query.ts | Hook accesses database without repository abstraction | Error
        3 | ui/lib/utils/sidecar.ts | Utility module depends on Tauri API; use adapter pattern | Warning
      }
      ---
      { [Fix in Session] | [Dismiss] | . | 3 violations across 3 files }
    }
    ---
    {
      <color:green><&circle-check></color> <b>api-contract</b>
      OpenAPI spec compliance | Last run: 5 min ago
      Result: <b>PASS</b> | 0 violations
    }
    ---
    {+
      <color:red><&circle-x></color> <b>security-scan</b>
      Dependency vulnerability check | Last run: 12 min ago
      Result: <b>FAIL</b> | 2 violations | [Run Scanner]
      ---
      {#
        # | File | Violation | Severity
        1 | package.json | lodash@4.17.20 has prototype pollution CVE-2024-XXXX | Critical
        2 | Cargo.toml | hyper@0.14.18 has HTTP request smuggling vulnerability | High
      }
      ---
      { [Fix in Session] | [Dismiss] | . | 2 violations across 2 files }
    }
    ---
    {
      <color:green><&circle-check></color> <b>naming-conventions</b>
      File and export naming rules | Last run: 2 min ago
      Result: <b>PASS</b> | 0 violations
    }
    ---
    .
  }
}
@endsalt
```

### Violation Detail Elements

| Element | Behavior |
|---------|----------|
| **Violation table** | Sortable columns. Click file path to open in editor. |
| **Severity** | Critical, Error, Warning. Color-coded. |
| **Fix in Session** | Creates a new session pre-filled with scanner violations as context for AI-assisted fixing. |
| **Dismiss** | Collapses the violation detail back to the summary row. |
| **Run Scanner** | Re-runs just this individual scanner. |

---

## 3. Metrics Dashboard with KPI Cards and Chart

The metrics dashboard provides operational KPIs with trend indicators and sparklines, plus a detailed time-series chart area.

```plantuml
@startsalt
{+
  { <b>Metrics</b> (via Activity Bar) }
  ---
  { <b>Metrics Dashboard</b> | . | . | Period: ^Last 7 days^ }
  ---
  {+
    {
      {+
        <b>Sessions/Day</b>
        ---
        <b>12.4</b> <color:green>^ +18%</color>
        ---
        ._.--^._.^.--.
      } | {+
        <b>Tool Calls/Session</b>
        ---
        <b>8.2</b> <color:green>^ +5%</color>
        ---
        ._.--.__.--^^.
      } | {+
        <b>Avg Response Time</b>
        ---
        <b>3.4s</b> <color:red>v +12%</color>
        ---
        .--._.--.^^._.
      }
    }
    ---
    {
      {+
        <b>Governance Coverage</b>
        ---
        <b>87%</b> <color:green>^ +3%</color>
        ---
        ._.--._.--^^_.
      } | {+
        <b>Scan Pass Rate</b>
        ---
        <b>78%</b> <color:red>v -4%</color>
        ---
        .^^.--._.--.__
      } | {+
        <b>Active Rules</b>
        ---
        <b>24</b> <color:green>^ +2</color>
        ---
        ._.__.--.--.^^
      }
    }
  }
  ---
  {+
    <b>Sessions/Day</b> (selected KPI detail)
    ---
    {
      . 20 |..........
      . 15 |....**....
      . 10 |.***..****
      .  5 |*.........
      .  0 |__________
      .    . Mon Tue Wed Thu Fri Sat Sun
    }
    ---
    { Avg: <b>12.4</b> | Min: <b>4</b> | Max: <b>19</b> | Trend: <b>+18%</b> }
  }
  ---
  {+
    <b>Lesson Log</b> (recent entries)
    ---
    {#
      Type | Title | Date | Status
      IMPL | Add retry logic to sidecar calls | Mar 2 | Promoted
      RETRO | Session context window overflow | Mar 1 | Draft
      IMPL | Use adapter pattern for Tauri APIs | Feb 28 | Promoted
      RETRO | Scanner false positives on test files | Feb 27 | Review
    }
  }
}
@endsalt
```

### KPI Card Elements

| Element | Behavior |
|---------|----------|
| **Card title** | Metric name. Click to load detailed chart below. |
| **Current value** | Large numeric display of the current/average value. |
| **Trend indicator** | Up arrow (green) = improving, down arrow (red) = degrading. Percentage or absolute change. |
| **Sparkline** | Miniature inline trend line rendered by LayerChart. Shows last 7 data points. |
| **Detail chart** | Full-width LayerChart time-series for the selected KPI. Appears below the card grid. |
| **Period selector** | Dropdown: Last 7 days, Last 30 days, Last 90 days, All time. |

### Lesson Log Elements

| Element | Behavior |
|---------|----------|
| **Type column** | IMPL (implementation lesson) or RETRO (retrospective insight). |
| **Title** | Click to open the full lesson entry in the Explorer Panel. |
| **Status** | Draft, Review, Promoted, Archived. |
| **Promoted entries** | Promoted lessons have been converted into active rules or agent instructions. |

---

## 4. Learning Loop -- IMPL/RETRO Cards

The learning loop view manages implementation lessons and retrospective insights, with a promotion workflow for converting them into governance artifacts.

```plantuml
@startsalt
{+
  { <b>Learning</b> (via Activity Bar) }
  ---
  { <b>Learning Loop</b> | . | . | [+ New Entry] }
  ---
  { Filter: | (X) All | () IMPL | () RETRO | . | Status: ^All Statuses^ }
  ---
  {SI
    {+
      { <b>[IMPL]</b> | <b>Add retry logic to sidecar calls</b> | . | [Promoted] }
      ---
      Sidecar connections drop intermittently. Added
      exponential backoff with 3 retries. This pattern
      should be standard for all sidecar communication.
      ---
      { Date: Mar 2, 2026 | Source: <&link-intact> Session #42 }
      ---
      { Promoted to: <&document> Rule: sidecar-retry-pattern }
    }
    ---
    {+
      { <b>[RETRO]</b> | <b>Session context window overflow</b> | . | [Draft] }
      ---
      Long sessions with many tool calls exceed the
      context window. Need to implement summarization
      or sliding window for sessions > 50 messages.
      ---
      { Date: Mar 1, 2026 | Source: <&link-intact> Session #38 }
      ---
      { [Promote to Rule] | [Promote to Lesson] | [Archive] }
    }
    ---
    {+
      { <b>[IMPL]</b> | <b>Use adapter pattern for Tauri APIs</b> | . | [Promoted] }
      ---
      Direct Tauri API calls in components make testing
      impossible. Adapter pattern wraps all Tauri invoke
      calls so they can be mocked in tests.
      ---
      { Date: Feb 28, 2026 | Source: <&link-intact> Session #35 }
      ---
      { Promoted to: <&document> Rule: tauri-adapter-pattern }
    }
    ---
    {+
      { <b>[RETRO]</b> | <b>Scanner false positives on test files</b> | . | [Review] }
      ---
      Architecture boundary scanner flags test helper
      files that intentionally cross module boundaries.
      Need scanner exclude patterns for test directories.
      ---
      { Date: Feb 27, 2026 | Source: <&link-intact> Session #31 }
      ---
      { Workflow: Draft -> <b>Review</b> -> Promoted }
      ---
      { [Approve & Promote] | [Return to Draft] | [Archive] }
    }
    ---
    {+
      { <b>[RETRO]</b> | <b>Model selection affects code quality</b> | . | [Archived] }
      ---
      Smaller models produce more verbose, less idiomatic
      Rust code. Default to opus for Rust backend tasks.
      ---
      { Date: Feb 25, 2026 | Source: <&link-intact> Session #28 }
      ---
      { <i>Archived on Mar 1 -- superseded by model routing rule</i> }
    }
    ---
    .
  }
}
@endsalt
```

### Promotion Workflow

```
Draft --> Review --> Promoted
  |                    |
  +---> Archived <-----+
```

| Stage | Description |
|-------|-------------|
| **Draft** | Initial entry captured from a session. Author can edit freely. |
| **Review** | Submitted for review. Shows approval/rejection actions. |
| **Promoted** | Converted into a governance artifact (rule, agent instruction, or lesson). Links to the created artifact. |
| **Archived** | No longer active. Retained for historical reference. Can be restored to Draft. |

### Learning Loop Card Elements

| Element | Behavior |
|---------|----------|
| **Type badge** | [IMPL] or [RETRO] -- color-coded (blue for IMPL, orange for RETRO). |
| **Title** | Bold text. Click to expand/collapse card body. |
| **Status badge** | Draft (gray), Review (yellow), Promoted (green), Archived (muted). |
| **Description** | Free-text body of the lesson or retrospective. Markdown supported. |
| **Source link** | Links back to the originating session. Opens in the Chat Panel. |
| **Promote to Rule** | Creates a new rule artifact pre-filled from the lesson content. |
| **Promote to Lesson** | Promotes as a general lesson referenced by agents. |
| **Archive** | Moves entry to archived status. |
| **Promoted to** | Read-only link to the governance artifact created from this entry. |
| **New Entry** | Opens a form to create a new IMPL or RETRO entry manually. |

---

## Keyboard Navigation

| Shortcut | Action |
|----------|--------|
| Activity Bar icon click | Open corresponding dashboard in Explorer Panel |
| `Up/Down` | Navigate between scanner rows or learning cards |
| `Enter` | Expand selected scanner or card |
| `Escape` | Collapse expanded detail |
