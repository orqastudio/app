---
role: artifacts
label: "Lessons"
description: "Implementation discoveries that prevent recurring mistakes."
icon: "book-open"
sort: 2
---

# Lessons

Lessons record implementation discoveries and prevent recurring mistakes. When a code reviewer, QA tester, or UX reviewer finds an issue, the pattern is captured here so it isn't repeated.

## Lifecycle

```
documented → recurrence tracked → promoted → enforcement verified
```

- **Documented**: The lesson is captured with context, the correct approach, and tags
- **Recurrence tracked**: Each time the same pattern is found again, the count increments
- **Promoted**: At recurrence >= 2, the lesson is promoted to a rule, coding standard, or skill
- **Enforcement verified**: After promotion, recurrence is re-tracked to verify the fix works

## Status Values

| Status | Meaning |
|--------|---------|
| `active` | Unpromoted lesson, normal state |
| `recurring` | Recurrence >= 2, pending promotion review |
| `promoted` | Promoted to a rule, coding standard, or skill |

## Promotion

Lessons that keep recurring are promoted to stronger enforcement: rule, hook, scanner, or hard block. The promotion target is recorded in the lesson's `promoted-to` field.
