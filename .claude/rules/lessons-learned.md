# Lessons Learned (NON-NEGOTIABLE)

The team maintains two learning loops to prevent mistakes from recurring across sessions. Both loops are mandatory — they are not guidelines.

## Implementation Lessons (`docs/development/lessons.md`)

When `code-reviewer`, `qa-tester`, or `ux-reviewer` reports a FAIL verdict:

1. **Check existing lessons** — search `docs/development/lessons.md` for the failure pattern before reporting it as a novel finding
2. **If the failure matches an existing lesson:** note the recurrence (increment the count in the entry)
3. **If the failure is new:** the reviewing agent adds a new `IMPL-NNN` entry before the fix-and-resubmit cycle begins
4. **When an IMPL entry reaches recurrence >= 2:** the `agent-maintainer` is triggered to promote it to a rule, coding standard addition, or skill update
5. **After promotion:** the IMPL entry's "Promoted to" field is updated with the target artifact

## Process Retrospectives (`docs/process/retrospectives.md`)

When a process-level change occurs:

1. **Process changes** (new rule, new agent, workflow change, governance update) — the `agent-maintainer` adds a `RETRO-NNN` entry
2. **Ineffective rules** (violations continue despite the rule) — the `agent-maintainer` adds a RETRO entry and proposes stronger enforcement
3. **Session start** — the orchestrator checks `docs/process/retrospectives.md` to load current process context and avoid repeating known mistakes

## Promotion Pipeline

```text
Lesson documented -> recurrence tracked -> promoted at threshold -> enforcement verified -> recurrence re-tracked
```

If a promoted lesson still sees violations: escalate enforcement (rule -> hook -> scanner -> hard block).

## Review Agent Output Requirements

All review agents (`code-reviewer`, `qa-tester`, `ux-reviewer`) MUST include a "Lessons Logged" section in their output format listing:

- Any new IMPL entries added during this review
- Any recurrence updates to existing IMPL entries
- Confirmation that `docs/development/lessons.md` was checked for known patterns

## The learning loop is NOT optional

Review agents that skip lesson documentation are in violation of this rule. The `agent-maintainer` audits compliance during governance reviews.

## Related Rules

- `honest-reporting.md` — completion reports must include lesson documentation status
- `plan-mode-compliance.md` — verification gate protocol that triggers lesson logging
