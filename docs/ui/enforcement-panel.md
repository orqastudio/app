---
title: "Enforcement Panel UI Spec"
category: ui
tags: []
created: 2026-03-05
updated: 2026-03-05
---

# Enforcement Panel UI Spec

**Date:** 2026-03-05

The enforcement panel surfaces active enforcement rules and their violations within the OrqaStudio™ UI. Violations from blocked and warned tool calls are displayed inline on the relevant tool call card and aggregated in the enforcement sidebar. The governance scan results view shows enforcement coverage across the rule inventory.

---

## Purpose

The enforcement panel makes governance enforcement visible and actionable. Users can see which rules are active, what violations have occurred in the current session, and the severity of each violation. This replaces silent failure modes where rules are violated but no feedback is given.

---

## Components

### ViolationBadge

A compact inline indicator displayed on a `ToolCallCard` when the tool call produced a block or warn verdict.

**Layout:**

```
[icon] [label]
```

**States:**

| State | Icon | Color | Label |
|-------|------|-------|-------|
| Blocked | X | Red (`destructive`) | "Blocked" |
| Warned | Triangle | Amber (`warning`) | "Warning" |

**Behavior:**
- Clicking the badge opens a popover showing the rule description and matched text snippet.
- The badge appears in the top-right corner of the tool call card.
- If multiple violations exist on one tool call, only the most severe badge is shown (block takes priority over warn). The popover lists all violations.

**Props:**
```typescript
interface ViolationBadgeProps {
  violations: Violation[];
  // If any violation has verdict "block", badge shows blocked state
  // Otherwise shows warned state
}
```

---

### EnforcementPanel

A sidebar panel (accessible from the Activity Bar) showing all enforcement entries and violations for the current session.

**Layout:**

```
┌─────────────────────────────────┐
│ Enforcement                  [x]│
├─────────────────────────────────┤
│ Session Summary                 │
│  [n blocked] [n warned]         │
├─────────────────────────────────┤
│ Active Rules (n)                │
│  ▸ RULE-004  No `any` type      │
│  ▸ RULE-004  No Svelte 4        │
│  ▸ RULE-007  No todo!()         │
│  ▼ RULE-008  No --no-verify     │
│    └─ 2 violations this session │
│       ● 09:14 bash: git commit  │
│         --no-verify             │
├─────────────────────────────────┤
│ [View all violations]           │
└─────────────────────────────────┘
```

**Sections:**

1. **Session Summary** — Two counters: total blocks and total warns for the current session. Clicking a counter filters the violation list to that verdict type.

2. **Active Rules** — A list of all compiled enforcement entries, grouped by rule file. Each entry shows:
   - Rule ID and short description
   - Violation count for the current session (hidden if zero)
   - Expandable violation list showing timestamp, matched context, and the tool call that triggered it

**States:**

| State | Display |
|-------|---------|
| Loading | `LoadingSpinner` centered in the panel |
| Error | `ErrorDisplay` with retry action |
| Empty (no violations) | "No violations in this session." with a checkmark icon |
| Violations present | Rule list with expandable violation groups |

**Behavior:**
- Clicking a violation in the list scrolls the conversation view to the relevant tool call card.
- Violations persist for the session — refreshing the panel re-queries from SQLite.
- Dismissed violations are shown with strikethrough text and reduced opacity.

---

### Inline Violation Display on ToolCallCard

When a tool call produces a violation, the `ToolCallCard` component shows an inline violation section below the tool call header.

**Layout (blocked tool call):**

```
┌─────────────────────────────────────────────────────┐
│ Write  src/main.ts                     [Blocked]    │
├─────────────────────────────────────────────────────┤
│ Blocked by RULE-004-001: No `any` type in TypeScript│
│  Matched: ": any" at line 42                        │
└─────────────────────────────────────────────────────┘
```

**Layout (warned tool call):**

```
┌─────────────────────────────────────────────────────┐
│ Bash  git reset --hard HEAD~1          [Warning]    │
├─────────────────────────────────────────────────────┤
│ Warning from RULE-008-003: Destructive git command  │
│  Command matched: "reset --hard"                    │
│  [Proceed anyway]                                   │
└─────────────────────────────────────────────────────┘
```

**Behavior:**
- Blocked tool calls cannot be re-executed without modifying the offending content.
- Warned tool calls show a "Proceed anyway" button that records a user acknowledgment in the violation record.
- The inline violation section is collapsed by default if the tool call is older than 5 minutes; the badge remains visible.

---

### GovernanceScanResults

Displays the output of a governance scan, including enforcement coverage — how many rules have pattern enforcement vs context-only.

**Layout:**

```
┌──────────────────────────────────────────────────────┐
│ Governance Scan Results                              │
├──────────────────────────────────────────────────────┤
│ Enforcement Coverage                                 │
│  Pattern-enforced rules: 8 / 19                     │
│  Context-only rules: 11 / 19                        │
│  ██████░░░░░░░░░░░░░░  42%                           │
├──────────────────────────────────────────────────────┤
│ Rule Files                                           │
│  ✓ coding-standards.md       3 entries              │
│  ✓ git-workflow.md            3 entries              │
│  ✓ no-stubs.md                1 entry               │
│  ○ vision-alignment.md        context-only           │
│  ○ documentation-first.md     context-only           │
│  ... (14 more)                                       │
│  [Show all]                                          │
└──────────────────────────────────────────────────────┘
```

**States:**

| State | Display |
|-------|---------|
| Loading | `LoadingSpinner` |
| Error | `ErrorDisplay` with retry |
| Empty (no rules found) | "No rule files found in .claude/rules/" |
| Loaded | Coverage bar and rule list |

---

## Component States Summary

| Component | States |
|-----------|--------|
| `ViolationBadge` | blocked, warned |
| `EnforcementPanel` | loading, error, empty, violations-present |
| `ToolCallCard` (with violation) | blocked, warned, warned-acknowledged |
| `GovernanceScanResults` | loading, error, empty, loaded |

---

## User-Facing Language

| Internal concept | Display label |
|-----------------|---------------|
| `verdict: "block"` | "Blocked" |
| `verdict: "warn"` | "Warning" |
| enforcement entry | "Rule" |
| `matched_text` | "Matched" |
| `entry_id` | Not shown directly — shown as rule description |

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | N/A |
| Process Governance | The enforcement panel makes rule violations visible in the conversation UI, turning abstract governance standards into concrete, actionable feedback on agent tool calls. |

---

## Related Documents

- `docs/architecture/enforcement.md` — Enforcement engine architecture (patterns, verdicts, persistence)
- `docs/ui/component-inventory.md` — Full component inventory
- `docs/wireframes/dashboard.md` — Dashboard wireframes including governance scan results
- `docs/process/rules.md` — Rule inventory and frontmatter schema
