---
id: AD-e7ca1e94
type: discovery-decision
title: "Relationship schema: complete knowledge flow traceability from vision to implementation"
status: exploring
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: EPIC-2d63dff3
    type: drives
---

# AD-063: Relationship Schema — Complete Knowledge Flow Traceability from Vision to Implementation

## Context

The artifact graph exists to trace knowledge flow — how an idea grounded in a pillar evolves through research, crystallises into a decision, drives an epic, and is delivered by tasks. Currently, `core.json` defines 19 relationship types but is missing the most critical delivery chain relationships:

- `delivers` (task → epic) — used in 600+ artifacts but NOT defined in `core.json`
- `fulfils` (epic → milestone) — used extensively but NOT in schema
- `depends-on` (task → task) — used for dependency chains but NOT in schema
- `delivered-by`, `fulfilled-by`, `depended-on-by` — inverses also undefined

This means the delivery hierarchy — the actual "how work gets done" chain — is completely unvalidated by the schema. Any artifact can use these relationship types without constraint, so the structural integrity checks in the `libs/validation` crate (AD-062) have no schema entry to validate against.

Additionally:

- There is no `implements` relationship (task, epic → decision) to directly trace what decision a task implements
- `observes` (agent → epic, task, decision, rule, milestone) is structurally incorrect for the knowledge model: agents discover delivery work dynamically at runtime via graph traversal, not through static relationships. A static `observes` edge says nothing about knowledge flow; it says "this agent is watching this artifact," which is an operational fact, not a governance fact
- Agent relationships currently connect agents only to `knowledge` artifacts. There is no relationship connecting agents back to the principles they serve (pillars, personas), leaving the governance → agent connection untraceable in the graph
- Ideas can reference epics via `crystallises → decision → drives → epic`, which is correct for the knowledge model. This chain must remain the path — direct idea-to-epic shortcuts would bypass the decision layer

## Decision

### 1. Add delivery relationships to core.json

The delivery hierarchy must be schema-defined so `libs/validation` can enforce it:

| Relationship | From | To | Semantic | Required |
|---|---|---|---|---|
| `delivers` | task | epic | delivery | yes (task must have ≥1) |
| `delivered-by` | epic | task | delivery | — |
| `fulfils` | epic | milestone | delivery | yes (epic must have ≥1) |
| `fulfilled-by` | milestone | epic | delivery | — |
| `depends-on` | task | task | delivery | — |
| `depended-on-by` | task | task | delivery | — |

A new `delivery` semantic group is created to contain these relationships. This separates the "how work is structured" concern from the "how decisions govern behaviour" concern already expressed in the `governance` group.

### 2. Add implementation tracing

A task or epic that implements a decision should be able to declare that fact directly, rather than requiring a chain traversal through the epic's `driven-by` edge:

| Relationship | From | To | Semantic |
|---|---|---|---|
| `implements` | task, epic | decision | governance |
| `implemented-by` | decision | task, epic | governance |

`implements` is the explicit delivery-to-governance bridge. Not required — a task that delivers through an epic that was driven by a decision already has the chain. But when a task directly implements a decision (e.g., a compliance task enforcing an architectural rule), it should be able to say so without an intermediate epic.

### 3. Replace `observes` with `serves`

Remove `observes` from `core.json` entirely. Agents do not statically observe delivery artifacts. They discover the delivery context they need at delegation time via `graph_query` — this is runtime traversal, not a governance relationship.

In its place, add `serves` to connect agents upward to the principles that define their purpose:

| Relationship | From | To | Semantic | Required |
|---|---|---|---|---|
| `serves` | agent | pillar, persona | foundation | yes (agent must have ≥1) |
| `served-by` | pillar, persona | agent | foundation | — |

With this change, the agent relationship set becomes:
- `employs` → knowledge: what domain context an agent consumes (already in schema, required)
- `serves` → pillar, persona: which foundational principles an agent is accountable to (new, required)

Agents connect UP to principles and OUT to knowledge. They do not connect down into the delivery hierarchy — they discover that at runtime.

### 4. Required relationships per artifact type

Formalise minimum cardinality requirements in `core.json` constraints. These are enforced by `libs/validation` and by the PreToolUse hook (AD-062):

| Artifact type | Required relationship | Minimum |
|---|---|---|
| task | `delivers` → epic | 1 |
| epic | `fulfils` → milestone | 1 |
| idea | `grounded` → pillar | 1 |
| idea | `benefits` → persona | 1 |
| rule | `enforces` → decision | 1 |
| agent | `employs` → knowledge | 1 |
| agent | `serves` → pillar or persona | 1 |
| pillar | `upholds` → vision | 1 |
| pivot | `revises` → vision, persona, or pillar | 1 |

Decision and knowledge artifacts are not given hard minimums at this stage — decisions crystallise from ideas or are informed by research (either path is valid), and knowledge is employed by agents (the inverse is checked on the agent side).

### 5. Allow `merged-into` for ideas and ideas

`merged-into` already allows `[idea, research] → [idea, research]`. No change needed here — this is confirmed correct.

### 6. Context-aware knowledge injection via artifact metadata

Knowledge artifacts declare when they should be injected via an `injection` property in their own frontmatter — not on the relationship edge, and not in a separate mapping file:

```yaml
# In a knowledge artifact's frontmatter
injection:
  file_patterns: ["*.svelte", "*.svelte.ts"]
  paths: ["ui/src/"]
  keywords: ["store", "reactive", "$state", "$derived", "runes"]
  artifact_types: ["rule", "decision"]
```

Three trigger mechanisms, checked in order:
1. **File context** — `file_patterns` and `paths` match against the file being edited (PreToolUse/PostToolUse). Deterministic, no latency.
2. **Prompt keywords** — `keywords` match against the user's prompt (UserPromptSubmit). Fast keyword scan, fallback when semantic search unavailable.
3. **Semantic search** — embed the prompt and search the knowledge corpus (UserPromptSubmit). Most accurate, requires the search engine.

The injection metadata lives on the knowledge artifact because the knowledge knows when it's relevant, regardless of which agent is active. A Svelte knowledge file is relevant when editing `.svelte` files whether the implementer, reviewer, or designer is working.

This replaces the INTENT_MAP entirely — injection configuration is graph-native, declarative, and maintained alongside the knowledge it describes.

### 7. The knowledge flow model

Every relationship type in `core.json` must serve this flow. Relationships that do not trace knowledge and cannot be read as a meaningful governance statement should not exist.

```
Vision ← Pillar ← Idea → Research → Decision → Epic → Task
              ↑                            ↓
          Agent → Knowledge            Rule → Lesson
```

Reading the graph:
- **Foundation layer**: Vision ← pillars (upholds). Pillars ← ideas (grounded). Ideas ← personas (benefits).
- **Ideation layer**: Ideas → research (spawns). Ideas → decisions (crystallises). Research → decisions (informs).
- **Governance layer**: Decisions → epics (drives). Decisions → rules (governs). Rules → decisions (enforces). Rules → lessons (codifies). Lessons → decisions (teaches). Lessons → epics (cautions).
- **Delivery layer**: Epics → tasks (delivered-by / delivers). Epics → milestones (fulfils). Tasks → tasks (depends-on). Tasks, epics → decisions (implements).
- **Agent layer**: Agents → knowledge (employs). Agents → pillars/personas (serves). Knowledge → docs (synchronised-with).

Agents connect UP to principles and OUT to knowledge. They discover the delivery context at runtime via graph traversal, not via static edges.

### 8. Impact-triggered investigation for high-blast-radius changes

When an artifact with high graph connectivity is modified, the system should automatically assess downstream impact and alert the orchestrator.

**PostToolUse hook on `.orqa/` writes:**
1. Identify the modified artifact
2. Query its impact radius via `graph_relationships` (2-3 hops downstream)
3. If impact exceeds a threshold → inject a system message listing affected artifacts and prompting the orchestrator to investigate

**Always-trigger types** (changes to these always have wide blast radius):
- Vision, Pillar, Persona → foundational; everything grounds to them
- Decision → affects all enforcing rules + all driven epics
- Rule → affects all governed agents + all codified lessons

**Threshold-trigger** (trigger when downstream count > N):
- Any artifact where impact radius (depth 2) exceeds 20 affected artifacts
- Milestone changes → all fulfilling epics + their tasks
- Knowledge changes → all employing agents

The system message provides:
- Count of affected artifacts by type
- List of directly connected artifacts (depth 1)
- Summary of the change's scope relative to the pillar it traces to

This is not blocking — it injects context so the orchestrator makes an informed decision about whether downstream artifacts need updating. The graph's own structure determines what's important, not a hardcoded list.

## Impact

- `core.json`: add 8 new relationship type entries (`delivers`, `delivered-by`, `fulfils`, `fulfilled-by`, `depends-on`, `depended-on-by`, `implements`, `implemented-by`, `serves`, `served-by`), remove `observes`/`observed-by`, add `delivery` semantic group
- `libs/validation` (AD-062): enforce required relationships per type in `cardinality.rs`; add checks for `observes` removal migration
- Scanner / integrity checks: validate delivery relationships against schema, flag `observes` edges as deprecated
- Agent artifacts: add `serves` → pillar or persona to every agent that currently lacks it
- Existing artifacts: 600+ tasks already use `delivers` — formalisation validates what exists, not what is new

## Risks

- **Retroactive required-relationship enforcement** — existing artifacts that lack `delivers`, `fulfils`, or `serves` will become invalid. The migration strategy is: warn before fail. The validator emits warnings for a migration window before promoting to errors
- **`observes` removal** — any existing agent artifact with `observes` edges will need those edges removed. Audit is required before enforcement is activated
- **`implements` overuse** — teams may add `implements` edges to every task, creating a dense graph that is harder to read. The constraint that `implements` is not required, and is meant for direct decision-to-task tracing only, must be documented in the governance knowledge artifact

## Alternatives Considered

1. **Keep `observes`, just document it better** — rejected: static observation edges are not knowledge flow; they add noise to the graph without enabling any governance query. The dynamic graph traversal model is strictly more correct
2. **Make `delivers` and `fulfils` optional** — rejected: a task with no epic is ungoverned work. The point of the graph is to trace all work to its principles. Optional delivery relationships undermine the entire purpose
3. **Add direct idea → epic relationship** — rejected: bypassing the decision layer breaks the knowledge model. Ideas must crystallise into decisions before decisions drive epics. The decision is the record of what was chosen and why

## Related Decisions

- AD-062 (`AD-628b058b`) — validation consolidation: `libs/validation` is the enforcement engine that will apply the constraints defined here