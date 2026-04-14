# Thinking Mode: Learning Loop

You are now in Learning Loop Mode. The user is teaching the system — sharing an observation, feedback, or lesson learned. This is NOT a request for work to be done. It is a signal that the governance system should grow.

**This is the most important mode to detect correctly.** Missing it means the observation is lost — it never becomes a lesson, never becomes a rule, and never becomes enforcement.

## The Governance Learning Pipeline

```
Observation → Lesson artifact → Pattern check → Rule (if recurring) → Enforcement
```

## Workflow

1. **Capture** — write a lesson artifact in `.orqa/learning/lessons/` with the observation documented
2. **Check recurrence** — search existing lessons for the same pattern. Has this been observed before?
3. **Promote if recurring** — if the pattern has occurred 2+ times, promotion to a rule is **required** (non-negotiable)
4. **Close the enforcement chain** — a rule without enforcement is just documentation. The chain must close: rule → linter/hook/gate/injection

## Quality Criteria

- The observation is captured as a structured lesson artifact with clear description
- Existing lessons are searched for the same pattern before creating a new one
- Recurring patterns (2+ occurrences) are promoted to rules — this is mandatory
- Promoted rules have a complete enforcement chain (not just a document)
- The lesson links back to what triggered it (a debugging session, a review FAIL, a user observation)

## What Happens Next

The learning loop is what makes OrqaStudio's governance compound over time. Every observation that is captured and promoted strengthens the system for all future sessions.

Inputs to the learning loop come from:
- **Debugging** — root cause reveals a systemic governance gap
- **Review** — FAIL verdict reveals a missing rule
- **Research** — investigation reveals an anti-pattern worth naming
- **User observation** — the user notices a recurring pattern

## Governance

- Lesson artifacts: `.orqa/learning/lessons/` with `type: lesson`
- Rule artifacts: `.orqa/learning/rules/` with `type: rule`
- Recurring lessons (2+ times) MUST be promoted to rules
- RULE-009: enforcement gaps discovered during development are immediately CRITICAL (dogfood mode)
