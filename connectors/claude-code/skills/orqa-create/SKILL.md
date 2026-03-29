---
name: orqa-create
description: "Create a new governance artifact (task, epic, decision, rule, knowledge, etc.) via the OrqaStudio daemon."
user-invocable: true
---

# Create Artifact

Create a new governance artifact with proper ID generation, frontmatter schema compliance, and graph registration.

## Usage

```bash
orqa create <type> --title "<title>" [options]
```

## Artifact Types

| Type                 | ID Prefix | Location                                    |
| -------------------- | --------- | ------------------------------------------- |
| `task`               | TASK      | `.orqa/implementation/tasks/`               |
| `epic`               | EPIC      | `.orqa/implementation/epics/`               |
| `milestone`          | MS        | `.orqa/implementation/milestones/`          |
| `discovery-idea`     | IDEA      | `.orqa/discovery/ideas/`                    |
| `planning-idea`      | IDEA      | `.orqa/planning/ideas/`                     |
| `implementation-idea`| IDEA      | `.orqa/implementation/ideas/`               |
| `discovery-research` | RES       | `.orqa/discovery/research/`                 |
| `planning-research`  | RES       | `.orqa/planning/research/`                  |
| `wireframe`          | WIRE      | `.orqa/discovery/wireframes/`               |
| `principle-decision` | PD        | `.orqa/learning/decisions/`                 |
| `planning-decision`  | PAD       | `.orqa/planning/decisions/`                 |
| `rule`               | RULE      | `.orqa/learning/rules/`                     |
| `lesson`             | IMPL      | `.orqa/learning/lessons/`                   |
| `knowledge`          | KNOW      | `.orqa/documentation/<category>/knowledge/` |
| `doc`                | DOC       | `.orqa/documentation/<category>/`           |
| `persona`            | PERSONA   | `.orqa/discovery/personas/`                 |
| `pillar`             | PILLAR    | `.orqa/discovery/pillars/`                  |
| `vision`             | VISION    | `.orqa/discovery/vision/`                   |
| `pivot`              | PIVOT     | `.orqa/discovery/pivots/`                   |
| `discovery-decision` | AD        | `.orqa/discovery/decisions/`                |

The daemon validates the artifact against the composed schema and registers it in the artifact graph. Relationships, status values, and required fields are enforced by the schema.
