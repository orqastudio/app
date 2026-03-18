---
id: RULE-017
title: Lessons Learned
description: "Two learning loops: implementation lessons in .orqa/process/lessons/ and process retrospectives. Both are mandatory."
status: active
created: 2026-03-07
updated: 2026-03-14
relationships:
  - target: AD-044
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-044
  - target: AD-048
    type: enforces
  - target: DOC-025
    type: documented-by
---
The team maintains two learning loops to prevent mistakes from recurring across sessions. Both loops are mandatory — they are not guidelines.

## Implementation Lessons

Lessons are stored as individual markdown files in `.orqa/process/lessons/`, one file per lesson with YAML frontmatter (id, title, category, recurrence count, promoted-to, tags). In the CLI, agents can also reference `.orqa/process/lessons/` as a consolidated view.

When `code-reviewer`, `qa-tester`, or `ux-reviewer` reports a FAIL verdict:

1. **Check existing lessons** — search `.orqa/process/lessons/` for the failure pattern before reporting it as a novel finding
2. **If the failure matches an existing lesson:** note the recurrence (increment the count in the lesson file's frontmatter)
3. **If the failure is new:** the reviewing agent creates a new `IMPL-NNN.md` file in `.orqa/process/lessons/` before the fix-and-resubmit cycle begins
4. **When an IMPL entry reaches recurrence >= 2:** the `orchestrator` (with `governance-maintenance` skills) is triggered to promote it to a rule, coding standard addition, or skill update
5. **After promotion:** the lesson file's "promoted-to" frontmatter field is updated with the target artifact

## Process Retrospectives

Process-level learnings are captured as lessons (`IMPL-NNN`) in `.orqa/process/lessons/`:

1. **Process changes** (new rule, new agent, workflow change, governance update) — the `orchestrator` creates or updates a lesson
2. **Ineffective rules** (violations continue despite the rule) — the `orchestrator` creates a lesson and proposes stronger enforcement
3. **Session start** — the orchestrator checks `.orqa/process/lessons/` for known patterns and recurring issues

## Promotion Pipeline

```text
Lesson documented -> recurrence tracked -> promoted at threshold -> enforcement attempted -> promotion completed -> recurrence re-tracked
```

### Enforcement Gate (NON-NEGOTIABLE — [AD-048](AD-048))

A lesson MUST NOT be promoted to a rule without attempting enforcement. Rules without enforcement are just lessons with a label. Before setting `evolves-into` on a lesson:

1. **Declare enforcement entries** on the target rule (event type, action, paths/patterns)
2. **Attempt mechanical enforcement** — can a hook, scanner, validator, or gate catch violations?
3. **If mechanical enforcement isn't possible** — use skill injection via [RULE-042](RULE-042) to inject context at the right moment
4. **All enforcement flows through the artifact graph** — never create raw platform hooks that bypass the system

Enforcement layers in priority order: artifact graph declaration → Rust application layer → Claude plugin → pre-commit hooks. Not every rule can be enforced at every layer, but at least one layer must be attempted.

If a promoted lesson still sees violations: escalate enforcement (context injection → process gate → scanner → hard block).

## Review Agent Output Requirements

All review agents (`code-reviewer`, `qa-tester`, `ux-reviewer`) MUST include a "Lessons Logged" section in their output format listing:

- Any new IMPL entries added during this review
- Any recurrence updates to existing IMPL entries
- Confirmation that `.orqa/process/lessons/` was checked for known patterns

## Lesson Status Vocabulary

Lessons carry a `status` field that reflects their promotion state:

| Status | Meaning | Indicator |
|--------|---------|-----------|
| `active` | Unpromoted lesson, normal state | Blue dot |
| `recurring` | Recurrence >= 2, pending promotion review | Amber dot |
| `promoted` | Promoted to a rule, coding standard, or skill | Purple dot |

When a lesson's recurrence is incremented to >= 2, update `status: recurring`. When promoted (the `evolves-into` field is set), update `status: promoted`.

## The learning loop is NOT optional

Review agents that skip lesson documentation are in violation of this rule. The `orchestrator` audits compliance during governance reviews.

## App-Managed Workflow

In OrqaStudio, the lesson pipeline (create, recurrence tracking, promotion) is managed through the UI. The app provides a lessons view where users can browse, filter, and promote lessons. Recurrence counts are updated automatically when the app detects matching failure patterns. Promotion to rules or coding standards is initiated from the UI and routed to the `orchestrator` for execution.

In the CLI, agents create lesson files manually in `.orqa/process/lessons/` following the YAML frontmatter format, and the `orchestrator` handles promotion through the standard governance audit process.

## Related Rules

- [RULE-015](RULE-015) (honest-reporting) — completion reports must include lesson documentation status
- [RULE-022](RULE-022) (plan-mode-compliance) — verification gate protocol that triggers lesson logging
