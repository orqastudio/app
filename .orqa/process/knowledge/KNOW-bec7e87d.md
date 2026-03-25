---
id: KNOW-bec7e87d
type: knowledge
title: UX Compliance Review
description: |
  UX compliance review methodology: label auditing, state coverage verification,
  shared component usage checks, layout and accessibility auditing, and structured
  UX review output format.
  Use when: Reviewing UI implementations against specifications, auditing accessibility,
  checking design system compliance, or verifying state coverage.
status: active
created: 2026-03-01
updated: 2026-03-10
category: methodology
version: 1.0.0
user-invocable: true
relationships:
  - target: DOC-4554ff3e
    type: synchronised-with
---


UX compliance review methodology. This skill teaches the *review process and checklist structure* — the specific design system tokens, component libraries, and UI specs come from the project's design and frontend skills.

## Label Audit

Check every user-facing text element:

- **Buttons:** Label matches the action (use the exact wording from the spec)
- **Headings:** Match the spec exactly — case, wording, hierarchy level
- **Empty states:** Messages are helpful, not generic
- **Error messages:** Describe what went wrong and what the user can do about it
- **Tooltips:** Present on all icon-only buttons, describe the action

### Label Consistency Rules

- Same concept uses same label everywhere
- Action labels use imperative verbs ("Create", "Delete", "Export")
- Status labels use adjectives or past participles ("Active", "Completed", "Failed")
- Search across the UI codebase to find label variants and catch inconsistencies

## State Audit

Every component that displays data must handle ALL states:

### 1. Loading

- Loading indicator appears promptly after action start
- No blank screens during loading
- Loading state is distinguishable from empty state

### 2. Empty

- Clear message explaining why there is no data
- Call-to-action to create/add the first item
- Visually distinct from loading and error states

### 3. Error

- Message explains what went wrong in user-friendly language
- Retry action is available where applicable
- Error state does not break the rest of the UI

### 4. Loaded (populated)

- Data is displayed according to spec layout
- Lists handle 1 item, few items, and many items gracefully
- Long text is truncated with ellipsis or scrollable, not overflowing
- Interactive elements are clearly interactive (hover states, cursors)

## Shared Component Audit

Verify consistent use of the project's shared component library:

- All buttons use the project's button component — no raw HTML buttons
- All form inputs use the project's input components
- All dialogs use the project's dialog components
- All status indicators use the project's badge/status component
- No duplicate implementations of the same UI pattern

## Layout Audit

- Spacing follows the design system's scale — no arbitrary pixel values
- Colors use design tokens — no hardcoded hex/rgb values
- Dark/light mode support where applicable
- No inline style attributes — use the project's styling system

## Accessibility Audit

- All interactive elements are keyboard-navigable (Tab order makes sense)
- Focus indicators are visible
- Color contrast meets WCAG AA standards (4.5:1 for text)
- Screen reader content is present (aria-labels, semantic HTML)
- No information conveyed by color alone (use icons/text alongside)

## UX Review Output Format

```markdown
## UX Review: [Feature/Component/Page]

### Label Audit
- Labels match spec: PASS / [list of mismatches]
- Label consistency: PASS / [list of inconsistencies]
- Tooltips present: PASS / [list of missing tooltips]

### State Audit
- Loading state: PRESENT / MISSING — [details]
- Empty state: PRESENT / MISSING — [details]
- Error state: PRESENT / MISSING — [details]
- Loaded state: CORRECT / ISSUES — [details]

### Shared Component Audit
- Component library usage: COMPLIANT / [violations]

### Layout Audit
- Design token compliance: PASS / [hardcoded values found]

### Accessibility Audit
- Keyboard navigation: PASS / [issues]
- Focus indicators: PASS / [missing on specific elements]
- Color contrast: PASS / [failing elements]
- Screen reader: PASS / [missing labels]

### Lessons Logged
- New entries: [list or none]
- Recurrence updates: [list or none]
- Checked lessons: YES

### Findings
1. [Severity] Description — File — Expected vs Actual

### Verdict: APPROVED / NEEDS REVISION
```

## Critical Rules

- NEVER approve a component missing any of the four states (loading, empty, error, loaded)
- NEVER approve raw HTML elements where project components should be used
- NEVER approve hardcoded color values — always use design tokens
- NEVER approve UI that is not keyboard-accessible
- Always verify against the spec document — personal aesthetic preference is not the standard
- When the spec is ambiguous, flag it for clarification rather than making assumptions
