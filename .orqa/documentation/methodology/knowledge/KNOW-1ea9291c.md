---
id: KNOW-1ea9291c
type: knowledge
title: Artifact Relationships
domain: methodology/governance
description: "Reference for all typed relationship verbs, their from/to constraints, and semantic categories used to connect OrqaStudio artifacts."
summary: "Artifact Relationships. Every connection between artifacts uses a typed relationship with a specific verb. The verb describes the nature of the connection and constrains which artifact types can participate."
status: active
relationships:
  - target: DOC-ae447f88
    type: synchronised-with
  - target: DOC-bad8e26f
    type: synchronised-with

---

# Artifact Relationships

Every connection between artifacts uses a typed relationship with a specific verb. The verb describes the nature of the connection and constrains which artifact types can participate. If a sentence doesn't read naturally with the verb, the relationship shouldn't exist.

## Core Platform Relationships

These ship with every OrqaStudio installation and define the foundational graph structure.

### Foundation — anchoring to principles

| From | Verb | To | Sentence |
| --- | --- | --- | --- |
| pillar | `upholds` | vision | "Pillar upholds Vision" |
| idea | `grounded-by` | pillar | "Idea grounded by Pillar" |
| idea | `benefits` | persona | "Idea benefits Persona" |
| pivot | `revises` | vision, persona, pillar | "Pivot revises Vision" |

The vision is the north star. Pillars uphold it. Ideas must ground to a pillar and benefit a persona — if they can't, either the idea is wrong or a pivot is needed.

### Lineage — ideas becoming other things

| From | Verb | To | Sentence |
| --- | --- | --- | --- |
| idea | `crystallises` | decision | "Idea crystallises into Decision" |
| idea | `spawns` | research | "Idea spawns Research" |
| idea | `merged-into` | idea | "Idea merged into Idea" |

Ideas are seeds. They crystallise into decisions (choices), spawn research (investigation), or get merged when they overlap.

### Governance — decisions directing behaviour

| From | Verb | To | Sentence |
| --- | --- | --- | --- |
| decision | `drives` | epic | "Decision drives Epic" |
| decision | `governs` | rule | "Decision governs Rule" |
| rule | `enforces` | decision | "Rule enforces Decision" |
| rule | `codifies` | lesson | "Rule codifies Lesson" |

Decisions fork into two paths: `drives` leads into delivery (epics → tasks), `governs` leads into the learning loop (rules → enforcement). When a lesson can be enforced, a rule `codifies` it — turning informal knowledge into governance.

### Knowledge flow — findings and learning

| From | Verb | To | Sentence |
| --- | --- | --- | --- |
| research | `informs` | decision | "Research informs Decision" |
| research | `guides` | epic | "Research guides Epic" |
| lesson | `teaches` | decision | "Lesson teaches Decision" |
| lesson | `cautions` | epic | "Lesson cautions Epic" |
| doc | `documents` | epic, decision, rule, milestone | "Doc documents Epic" |

Each verb is specific: research `informs` choices and `guides` work. Lessons `teach` future choices and `caution` current work. Docs `document` things for human reference.

### Agents and skills

| From | Verb | To | Sentence |
| --- | --- | --- | --- |
| agent | `observes` | epic, task, decision, rule, milestone | "Agent observes Epic" |
| agent | `employs` | skill | "Agent employs Skill" |
| skill | `synchronised-with` | doc | "Skill synchronised with Doc" |

Agents observe artifacts they're responsible for and employ skills they use. Skills and docs are paired — agent-facing and human-facing versions of the same knowledge.

## Plugin Relationships

Plugins register additional relationships for their artifact types. The software-project plugin adds delivery, dependency, and bug-tracking relationships. These are documented in KNOW-a700e25a.

## Rules for Creating Relationships

1. **Every relationship is bidirectional** — when you add `drives` on a decision, add `driven-by` on the epic
2. **The verb must read as a natural sentence** — "Decision drives Epic" ✓, "Task drives Pillar" ✗
3. **Check the from/to types** — the integrity validator rejects relationships between wrong types
4. **One relationship, one meaning** — don't use `informs` when `guides` is more accurate
5. **Trace to ideas** — every artifact should trace back to an idea through the graph
6. **Trace to pillars** — every idea should ground to a pillar. If it can't, question the idea or pivot

## Automatic Transitions

Status transitions are computed from relationship state:

- Tasks with unmet `depends-on` targets → `blocked`
- Tasks with all `depends-on` targets completed → `ready`
- Epics with all child tasks (`delivered-by`) completed → `review`
- Milestones with all child epics (`fulfilled-by`) completed → `review`
- Lessons without a `codified-by` rule → unaddressed learning (surfaced in dashboards)
- Ideas without `grounded-by` or `benefits` → ungrounded ideas (surfaced as warnings)
