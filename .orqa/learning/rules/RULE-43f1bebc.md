---
id: RULE-43f1bebc
type: rule
title: Systems Thinking First
description: "Every change must be evaluated as part of the whole system, not as an isolated fix. This is the foundational discipline that OrqaStudio exists to enable — it applies to every project the app manages."
status: active
enforcement_type: mechanical
created: 2026-03-07
updated: 2026-03-07
enforcement:

  - engine: behavioral

    message: "Every change must be evaluated as part of the whole system; plans addressing symptoms without identifying the system are rejected"

  - engine: hook

    type: PostToolUse
    event: file
    action: inject
    description: "systems-thinking knowledge injected on all implementation work"
relationships:

  - target: PD-af88bb69

    type: enforces
---

# Systems Thinking First (NON-NEGOTIABLE)

This is a universal rule. It applies to every project OrqaStudio manages, not just OrqaStudio itself. The app exists to provide a framework for systems thinking applied to complex problems — this rule defines what that means in practice.

## The Principle

Before touching code, answer these questions:

1. **What is the system?** Identify the full structure this change lives within — not just the file, but the data flow, the rendering pipeline, the user's mental model.
2. **What are the boundaries?** Where does this system start and end? What are its inputs, outputs, and invariants?
3. **What are the relationships?** How does this part connect to other parts? What depends on it? What does it depend on?
4. **What is the pattern?** Is there an existing pattern for how similar things work? If so, follow it. If not, design one that covers all cases — not just the one in front of you.
5. **What breaks if this changes?** Trace the consequences through the system before making the change.

## Anti-Patterns

| Anti-Pattern | Systems Alternative |
| --- | --- |
| Fix one instance, move on | Design for ALL instances of the pattern, implement uniformly |
| Add a special case for this one scenario | Find the general rule, implement it once, apply everywhere |
| Patch the symptom (wrong label, missing field) | Trace to the root cause (data model, pipeline, contract) |
| "It works for X, I'll do Y later" | Design for the abstraction, implement once |
| Fix the display without checking the data | Trace the full pipeline: source → processing → storage → presentation |

## When This Applies

- **Planning**: Every plan must describe the system being modified, not just the feature being added. What is the current system? What will it become?
- **Implementation**: Every code change must be evaluated against the system's invariants. A change to one type's behaviour must be checked against ALL types in that system.
- **Debugging**: Start with the system model. Where in the pipeline does the data diverge from expectation? Don't guess-and-patch.
- **Review**: Does this change maintain system consistency? Or does it introduce a special case that will need its own special cases later?

## The Uniform Base Principle

Every system should have ONE default behaviour applied uniformly to all members. Variations are optional enhancements layered on top of the base, not replacements for it.

- Rendering pipelines: one default rendering path for all types; type-specific views are plugins
- Data models: one base schema with type-specific extensions, not parallel schemas
- Navigation: one hierarchy pattern applied consistently across all content types
- Validation: one set of invariants checked uniformly, with type-specific rules as additions

If you find yourself writing `if (type === "X") { ... } else if (type === "Y") { ... }` for fundamental behaviour, you don't have a system — you have a collection of special cases.

See the `systems-thinking` skill for practical methodology — how to identify systems, map boundaries, trace relationships, find patterns, and discover the uniform base.

## Enforcement

- Plans that address symptoms without identifying the system are rejected
- Code reviews that find special-case handling where a general pattern exists are a FAIL
- The orchestrator must ask "what is the system here?" before delegating any implementation task
- Agents must describe the system they are modifying in their completion reports

## Related Rules

- [RULE-dccf4226](RULE-dccf4226) (plan-mode-compliance) — plans must describe the system being modified; the Systems Architecture Checklist operationalises this principle
- end-to-end-completeness — full-stack thinking is systems thinking applied to the four-layer feature structure
- [RULE-998da8ea](RULE-998da8ea) (dogfood-mode) — heightened systems awareness applies when editing the app you are running inside, as changes affect the live system
