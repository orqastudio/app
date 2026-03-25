# Agile Governance — Relationship Vocabulary

The agile-governance plugin defines 20 relationship types that model how governance artifacts connect. Each relationship has a forward direction (the one you store on the source artifact) and a computed inverse (resolved by the graph engine at query time). Under forward-only storage, you write only the forward key on the artifact that initiates the relationship.

---

## Foundation (5 types)

Relationships that anchor work to the project's guiding principles.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `upholds` | `upheld-by` | pillar | vision | Pillar supports the vision. Required, min 1. |
| `grounded` | `grounded-by` | idea, epic | pillar | Idea or epic anchored to a foundational principle. |
| `benefits` | `benefited-by` | idea | persona | Idea serves a target persona. Required, min 1. |
| `serves` | `served-by` | agent | pillar, persona | Agent is accountable to a foundational principle. Required, min 1. |
| `revises` | `revised-by` | pivot | vision, persona, pillar | Pivot revises a foundational artifact. Required, min 1. |

**When to use:**
- `upholds` — every pillar must declare which vision it supports
- `grounded` — every idea and epic should trace to at least one pillar for alignment
- `benefits` — every idea must identify the persona(s) it serves
- `serves` — every agent must be accountable to at least one pillar or persona
- `revises` — when a pivot changes the project's foundational direction

---

## Lineage (3 types)

Relationships that trace how artifacts evolve from one form to another.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `crystallises` | `crystallised-by` | idea | decision | Idea crystallises into an architecture decision. |
| `spawns` | `spawned-by` | idea | research | Idea spawns an investigation. |
| `merged-into` | `merged-from` | idea, research | idea, research | Artifacts consolidated — multiple ideas or research threads become one. |

**When to use:**
- `crystallises` — an idea has matured enough to become a formal decision
- `spawns` — an idea needs investigation before it can proceed
- `merged-into` — two or more ideas or research threads are consolidated into one

---

## Governance (4 types)

Relationships that connect decisions to their enforcement mechanisms.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `drives` | `driven-by` | decision | epic | Decision motivates a body of delivery work. |
| `governs` | `governed-by` | decision | rule | Decision establishes governance — leads into the learning loop. |
| `enforces` | `enforced-by` | rule | decision | Rule enforces a decision. Min 1 (not required). |
| `codifies` | `codified-by` | rule | lesson | Rule codifies a lesson into enforceable governance. |
| `promoted-to` | `promoted-from` | lesson | rule | Lesson promoted to an enforceable rule. |

**When to use:**
- `drives` — a decision has been made and work needs to begin
- `governs` — a decision creates or modifies a rule
- `enforces` — a rule exists to enforce a specific decision
- `codifies` — a rule was created from an operational lesson
- `promoted-to` — a lesson has been elevated to rule status

---

## Knowledge Flow (5 types)

Relationships that trace how knowledge moves through the system.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `informs` | `informed-by` | research | decision, research | Research findings inform a decision or another research thread. |
| `teaches` | `taught-by` | lesson | decision | Lesson teaches a future decision. |
| `guides` | `guided-by` | research | epic | Research guides a body of delivery work. |
| `cautions` | `cautioned-by` | lesson | epic | Lesson cautions a body of delivery work. |
| `documents` | `documented-by` | doc | epic, decision, rule, milestone | Document describes an artifact for human reference. |

**When to use:**
- `informs` — research has produced findings that should shape a decision
- `teaches` — a lesson learned applies to an upcoming decision
- `guides` — research provides direction for an epic's implementation
- `cautions` — a past lesson warns against an approach in an epic
- `documents` — a doc artifact describes another artifact for human consumption

---

## Agency (1 type)

Relationships that connect agents to their capabilities.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `employs` | `employed-by` | agent | knowledge | Agent employs a knowledge capability. Required, min 1. |

**When to use:**
- `employs` — every agent must declare the knowledge it uses to perform its role

---

## Synchronisation (1 type)

Relationships that keep paired content in sync.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `synchronised-with` | `synchronised-with` | knowledge, doc | knowledge, doc | Paired content kept in sync — agent-facing and human-facing versions. |

**Note:** This is a symmetric relationship — the forward and inverse keys are identical. Both sides of a sync pair should store the relationship. This is the one exception to the "store only forward" rule, because there is no directionality to determine which side is "forward."

**When to use:**
- `synchronised-with` — when a knowledge artifact (agent-facing) and a doc artifact (human-facing) describe the same content and must be updated together
