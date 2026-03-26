---
id: "AD-c6c2d9fb"
type: discovery-decision
title: "Rule promotion requires enforcement — rules without enforcement are lessons"
description: "A lesson MUST NOT be promoted to a rule unless an enforcement mechanism is attempted. Enforcement flows through the artifact graph enforcement system at three layers: artifact graph declaration, Rust application, and Claude plugin. Raw platform hooks bypass the system and are forbidden."
status: completed
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
relationships:
  - target: "EPIC-12fba656"
    type: "drives"
---
## Decision

A lesson (IMPL-NNN) MUST NOT be promoted to a rule (RULE-NNN) unless enforcement is attempted at the appropriate layers. A rule without enforcement is just a lesson with a label — it provides no mechanical guarantee.

### Enforcement Layers

Every rule's enforcement should be attempted across three layers, in order of priority:

| Layer | Mechanism | When |
|-------|-----------|------|
| **Artifact graph** | Enforcement entries on the rule, consumed by the enforcement engine | Always — this is the declaration layer |
| **Rust application** | Process gates, validators, integrity checks in `backend/src-tauri/` | When the app can mechanically verify compliance |
| **Claude plugin** | Plugin hooks that consume enforcement entries from the artifact graph | When CLI agents need real-time enforcement |

Not every rule can be enforced at every layer. Some rules are only enforceable through context injection (making the agent aware of the constraint). This is acceptable — but it must be the artifact graph's skill injection system doing the injection, not a raw platform hook.

### Promotion Checklist

Before promoting IMPL-NNN to RULE-NNN:

1. **Declare enforcement** — Add enforcement entries to the rule's artifact (event type, action, paths/patterns)
2. **Attempt mechanical enforcement** — Can a hook, scanner, validator, or gate catch violations?
3. **If mechanical enforcement isn't possible** — Use skill injection via RULE-e1f1afc1 to inject context at the right moment
4. **Document what's enforced and what isn't** — The rule body must state how enforcement works

### What Counts as Enforcement

| Enforcement Type | Example | Mechanical? |
|-----------------|---------|-------------|
| Pre-commit hook | Schema validation blocks commit | Yes |
| Integrity check | Broken link detection in artifact_graph.rs | Yes |
| Skill injection | Domain skill injected when writing to a path | Soft — agent awareness |
| Process gate | Epic can't move to ready without docs-required | Yes |
| Plugin Stop hook | Block output matching forbidden patterns | Yes |
| Context injection | Rule loaded into agent prompt | Soft — agent awareness |

### The Enforcement System

All enforcement flows through OrqaStudio's enforcement system:

```
Rule declares enforcement entries (artifact graph)
  → Plugin reads entries, registers hooks (CLI context)
  → App reads entries, applies process gates (App context)
  → Pre-commit reads entries, runs validators (commit context)
```

**FORBIDDEN**: Creating raw platform hooks (`.claude/hooks/`, git hooks outside `.githooks/`) that bypass the artifact graph. The artifact graph is the single source of truth for what is enforced.

## Rationale

- A rule that relies on agents reading it and self-complying is indistinguishable from a lesson
- OrqaStudio's value proposition is mechanical enforcement of structured thinking — rules without enforcement undermine this
- Three-layer enforcement provides defense in depth: app catches it in real-time, plugin catches it in CLI, pre-commit catches it at commit
- Enforcement entries in the artifact graph are auditable, version-controlled, and queryable — raw hooks are invisible

## Consequences

- Every future IMPL → RULE promotion must include enforcement entries
- Existing rules should be audited for enforcement coverage (this is a backlog item, not a blocker)
- The enforcement engine (RULE-e1f1afc1) becomes the central mechanism for all rule enforcement
- Raw `.claude/hooks/` files are forbidden for governance enforcement — use the plugin's hook system which reads from the artifact graph