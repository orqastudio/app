---
id: RULE-017
title: Lessons Learned
description: "Two learning loops: implementation lessons in .orqa/governance/lessons/ and process retrospectives. Both are mandatory."
status: active
created: "2026-03-07"
updated: "2026-03-10"
layer: core
scope: [AGENT-003, AGENT-006]
---
The team maintains two learning loops to prevent mistakes from recurring across sessions. Both loops are mandatory — they are not guidelines.

## Implementation Lessons

Lessons are stored as individual markdown files in `.orqa/governance/lessons/`, one file per lesson with YAML frontmatter (id, title, category, recurrence count, promoted-to, tags). In the CLI, agents can also reference `.orqa/governance/lessons/` as a consolidated view.

When `code-reviewer`, `qa-tester`, or `ux-reviewer` reports a FAIL verdict:

1. **Check existing lessons** — search `.orqa/governance/lessons/` for the failure pattern before reporting it as a novel finding
2. **If the failure matches an existing lesson:** note the recurrence (increment the count in the lesson file's frontmatter)
3. **If the failure is new:** the reviewing agent creates a new `IMPL-NNN.md` file in `.orqa/governance/lessons/` before the fix-and-resubmit cycle begins
4. **When an IMPL entry reaches recurrence >= 2:** the `orchestrator` (with `governance-maintenance` skills) is triggered to promote it to a rule, coding standard addition, or skill update
5. **After promotion:** the lesson file's "promoted-to" frontmatter field is updated with the target artifact

## Process Retrospectives (`.orqa/documentation/process/retrospectives.md`)

When a process-level change occurs:

1. **Process changes** (new rule, new agent, workflow change, governance update) — the `orchestrator` adds a `RETRO-NNN` entry
2. **Ineffective rules** (violations continue despite the rule) — the `orchestrator` adds a RETRO entry and proposes stronger enforcement
3. **Session start** — the orchestrator checks `.orqa/documentation/process/retrospectives.md` to load current process context and avoid repeating known mistakes

## Promotion Pipeline

```text
Lesson documented -> recurrence tracked -> promoted at threshold -> enforcement verified -> recurrence re-tracked
```

If a promoted lesson still sees violations: escalate enforcement (rule -> hook -> scanner -> hard block).

## Review Agent Output Requirements

All review agents (`code-reviewer`, `qa-tester`, `ux-reviewer`) MUST include a "Lessons Logged" section in their output format listing:

- Any new IMPL entries added during this review
- Any recurrence updates to existing IMPL entries
- Confirmation that `.orqa/governance/lessons/` was checked for known patterns

## Lesson Status Vocabulary

Lessons carry a `status` field that reflects their promotion state:

| Status | Meaning | Indicator |
|--------|---------|-----------|
| `active` | Unpromoted lesson, normal state | Blue dot |
| `recurring` | Recurrence >= 2, pending promotion review | Amber dot |
| `promoted` | Promoted to a rule, coding standard, or skill | Purple dot |

When a lesson's recurrence is incremented to >= 2, update `status: recurring`. When promoted (the `promoted-to` field is set), update `status: promoted`.

## The learning loop is NOT optional

Review agents that skip lesson documentation are in violation of this rule. The `orchestrator` audits compliance during governance reviews.

## App-Managed Workflow

In OrqaStudio, the lesson pipeline (create, recurrence tracking, promotion) is managed through the UI. The app provides a lessons view where users can browse, filter, and promote lessons. Recurrence counts are updated automatically when the app detects matching failure patterns. Promotion to rules or coding standards is initiated from the UI and routed to the `orchestrator` for execution.

In the CLI, agents create lesson files manually in `.orqa/governance/lessons/` following the YAML frontmatter format, and the `orchestrator` handles promotion through the standard governance audit process.

## Related Rules

- [RULE-015](RULE-015) (honest-reporting) — completion reports must include lesson documentation status
- [RULE-022](RULE-022) (plan-mode-compliance) — verification gate protocol that triggers lesson logging
