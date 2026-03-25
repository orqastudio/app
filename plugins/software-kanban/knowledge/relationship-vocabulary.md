# Software — Relationship Vocabulary

The software plugin defines 11 relationship types that model how delivery and engineering artifacts connect. Each relationship has a forward direction (stored on the source artifact) and a computed inverse (resolved by the graph engine at query time). Under forward-only storage, you write only the forward key on the artifact that initiates the relationship.

---

## Hierarchy (2 types)

Relationships that structure work into nested delivery units.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `delivers` | `delivered-by` | task | epic | Task delivers work to an epic. Required, min 1. Has status rules: epic moves to review when all delivering tasks are completed. |
| `fulfils` | `fulfilled-by` | epic | milestone | Epic fulfils a milestone checkpoint. Has status rules: milestone moves to review when all fulfilling epics are completed. |

**When to use:**
- `delivers` — every task must belong to exactly one epic
- `fulfils` — epics that contribute to a milestone checkpoint

---

## Lineage (2 types)

Relationships that trace how artifacts evolve through the pipeline.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `realises` | `realised-by` | idea | epic, task | Idea realised through delivery work. |
| `produces` | `produced-by` | research | wireframe | Research produces a visual specification. |

**When to use:**
- `realises` — an idea has moved from discovery into delivery as an epic or task
- `produces` — research has generated a wireframe or visual spec

---

## Dependency (1 type)

Relationships that express blocking dependencies between work items.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `depends-on` | `depended-on-by` | task, epic | task, epic | Task or epic cannot proceed until dependency is completed. Has status rules: blocked when any dependency is incomplete, unblocked (ready) when all are completed. |

**When to use:**
- `depends-on` — a task or epic cannot start until another piece of work is finished. Always store on the dependent item (the one that is blocked), pointing to the item it waits for.

---

## Corrective (3 types)

Relationships that connect bugs to the work they affect and the work that fixes them.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `reports` | `reported-by` | bug | epic, task, milestone | Bug reports an issue against delivery work. Required, min 1. |
| `fixes` | `fixed-by` | task | bug | Task fixes a reported bug. |
| `affects` | `affected-by` | bug | persona | Bug affects a target persona. |

**When to use:**
- `reports` — every bug must identify which delivery work it was found in
- `fixes` — a task has been created specifically to resolve a bug
- `affects` — a bug impacts a specific user persona

---

## Knowledge Flow (1 type)

Relationships that capture operational learning from delivery work.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `yields` | `yielded-by` | task | lesson | Task yields a lesson learned. |

**When to use:**
- `yields` — a completed task produced an insight worth capturing as a lesson

---

## Delivery (2 types)

Relationships that trace delivery work back to governance decisions and lessons.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `implements` | `implemented-by` | task, epic | decision | Work directly implements a decision — traces delivery back to governance. |
| `addresses` | `addressed-by` | task, epic | lesson | Task or epic addresses a lesson or implementation finding. |

**When to use:**
- `implements` — work that directly carries out an architecture decision
- `addresses` — work that responds to a lesson learned or implementation finding
