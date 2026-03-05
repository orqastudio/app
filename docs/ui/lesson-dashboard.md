# Lesson Dashboard UI Spec

**Date:** 2026-03-05

The lesson dashboard provides navigation and display for implementation lessons captured in `.orqa/lessons/`. Users can browse lessons, filter by category and status, see recurrence trends, and promote lessons to rule enforcement entries when recurrence reaches the threshold.

---

## Purpose

The lesson dashboard makes the learning loop visible and actionable. Review agents add new lessons through the API; the dashboard surfaces them to users so they can track recurring mistakes, understand why they happen, and promote patterns into enforceable rules when they've recurred enough times.

---

## Components

### LessonList

The navigation section within the Lessons section of the Nav Sub-Panel. Shows a scrollable list of lessons with recurrence badges.

**Layout:**

```
┌──────────────────────────────────┐
│ Lessons                          │
│ [All] [Active] [Promoted]        │
│ ──────────────────────────────── │
│ IMPL-007  No `: any` in TS   [3]│  ← recurrence badge, amber at >=2
│ IMPL-006  unwrap() in cmd    [2]│  ← promotion candidate (highlighted)
│ IMPL-005  Missing IPC type   [1]│
│ IMPL-004  Stub in command    [4]│  ← promoted (checkmark icon)
│ IMPL-003  ...                   │
│                                  │
│ Showing 5 of 12                  │
│ [Load more]                      │
└──────────────────────────────────┘
```

**Filter tabs:**
- **All** — shows all lessons regardless of status
- **Active** — shows only `status: active` lessons
- **Promoted** — shows only `status: promoted` lessons

**Item display:**

Each lesson item shows:
- IMPL number (left, muted)
- Title (truncated to one line)
- Recurrence badge (right): count in amber when recurrence >= 2, green when promoted

**States:**

| State | Display |
|-------|---------|
| Loading | `LoadingSpinner` |
| Error | `ErrorDisplay` with retry |
| Empty | "No lessons recorded yet. Lessons are created when review agents identify recurring mistakes." |
| Loaded | Filtered list with load-more pagination |

---

### LessonViewer

Displays the full content of a single lesson, selected from the LessonList.

**Layout:**

```
┌────────────────────────────────────────────────────┐
│ IMPL-006                                   [Promote]│
│ unwrap() called in Tauri command handler            │
│                                                     │
│ Category: rust   Recurrence: 2                      │
│ Tags: tauri, error-handling                         │
│ First seen: 2026-03-03  Last seen: 2026-03-09       │
├────────────────────────────────────────────────────┤
│ What Happened                                       │
│                                                     │
│ The backend-engineer agent used `.unwrap()` on the  │
│ return value of `get_session()` inside a Tauri      │
│ command handler, causing a panic when the session   │
│ was not found.                                      │
│                                                     │
│ Why It Recurs                                       │
│                                                     │
│ The agent treats Tauri commands like internal       │
│ functions where panics are acceptable. The IPC      │
│ boundary requires returning Result<T, String>.      │
│                                                     │
│ Correct Approach                                    │
│                                                     │
│ Use `?` with `map_err` or `.ok_or_else()` to        │
│ convert the Option/Result into a command error:     │
│                                                     │
│ ```rust                                             │
│ let session = repo.get(id)                          │
│   .map_err(|e| e.to_string())?;                     │
│ ```                                                 │
│                                                     │
│ Detection                                           │
│                                                     │
│ Search for `.unwrap()` or `.expect()` in            │
│ `src-tauri/src/commands/`.                          │
├────────────────────────────────────────────────────┤
│ Sessions where this occurred (2)                    │
│  ● 2026-03-03  session-abc123...                    │
│  ● 2026-03-09  session-def456...                    │
└────────────────────────────────────────────────────┘
```

**Promote button:** Visible only when `status: active` and `recurrence >= 2`. Clicking it opens the promotion dialog.

**States:**

| State | Display |
|-------|---------|
| Loading | `LoadingSpinner` centered |
| Error | `ErrorDisplay` |
| Empty (no lesson selected) | "Select a lesson from the list to view details." |
| Active lesson | Full content with promote button if eligible |
| Promoted lesson | Full content with promotion reference ("Promoted to RULE-NNN entry RULE-NNN-001") and no promote button |

---

### Recurrence Badges

Recurrence badges appear in both the LessonList and within the LessonViewer metadata row. They communicate urgency at a glance.

| Recurrence | Badge Style | Meaning |
|------------|-------------|---------|
| 1 | Gray, no highlight | First occurrence — monitoring |
| 2 | Amber, highlighted | Meets promotion threshold — review recommended |
| 3+ | Amber, bold | Elevated recurrence — promotion strongly recommended |
| Promoted | Green with checkmark | Converted to enforcement rule |

---

### Promotion Candidates Section

A dedicated section at the top of the lesson list (or as a dashboard card) highlighting lessons that meet the promotion threshold.

**Layout (when promotion candidates exist):**

```
┌──────────────────────────────────┐
│ Promotion Candidates             │
│ 2 lessons meet the threshold     │
│                                  │
│  IMPL-006  unwrap() in cmd  [2] │
│  IMPL-007  No `: any`       [3] │
│                                  │
│ [Review candidates]              │
└──────────────────────────────────┘
```

This section is hidden when no lessons have recurrence >= 2.

---

### Promotion Dialog

A modal dialog that walks the user through promoting a lesson to a rule enforcement entry.

**Steps:**

1. **Confirm the lesson** — Summarizes the lesson being promoted.
2. **Choose the target rule** — Dropdown of rule files in `.claude/rules/`. Suggests the most relevant based on category.
3. **Configure the enforcement entry** — Pre-fills `description` from the lesson title. User provides:
   - `event` (file or bash)
   - `action` (block or warn)
   - `pattern` (regex)
   - `scope` (system or project)
4. **Confirm** — Shows a preview of the YAML that will be added to the rule's frontmatter.

**States:**

| State | Display |
|-------|---------|
| Step 1-4 | Form wizard with step indicator |
| Submitting | `LoadingSpinner` with "Saving..." |
| Error | Inline error message, form remains open |
| Success | Dialog closes, lesson badge turns green, lesson list refreshes |

---

## Component States Summary

| Component | States |
|-----------|--------|
| `LessonList` | loading, error, empty, loaded |
| `LessonViewer` | loading, error, empty (no selection), active, promoted |
| `RecurrenceBadge` | first-occurrence, threshold-met, elevated, promoted |
| `PromotionCandidates` | hidden (none), shown (candidates present) |
| `PromotionDialog` | step-1, step-2, step-3, step-4, submitting, error, success |

---

## User-Facing Language

| Internal concept | Display label |
|-----------------|---------------|
| `status: active` | "Active" |
| `status: promoted` | "Promoted" |
| `status: archived` | "Archived" |
| `recurrence` | Shows the count directly (e.g., "3") — no label |
| IMPL number | "IMPL-006" — shown as-is |
| Promotion threshold | Not exposed directly; surfaced as "meets the threshold" |

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | The lesson dashboard is the primary UI for the learning loop — it makes lesson capture, recurrence tracking, and pattern promotion visible and actionable to the user. |
| Process Governance | Lesson promotion converts documented mistakes into rule enforcement entries, directly strengthening governance. The dashboard tracks the pipeline from observed mistake to enforced standard. |

---

## Related Documents

- `docs/architecture/lessons.md` — Lesson storage, metadata schema, promotion workflow
- `docs/architecture/enforcement.md` — Enforcement engine that receives promoted lessons
- `docs/ui/enforcement-panel.md` — Enforcement sidebar showing active violations
- `docs/wireframes/dashboard.md` — Dashboard wireframes including learning loop cards
