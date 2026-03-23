---
id: TASK-e258e6cb
type: task
title: Create skill for epic-requirement inference during project setup
description: "Create a skill that informs the project-setup process about whether epics should be required, based on the project type, structure, and intended use. The skill provides heuristics and questions to determine the right workflow.epics-required setting."
status: completed
created: 2026-03-12
updated: 2026-03-12
docs: []
acceptance:
  - A skill exists that can recommend epics-required true or false based on project signals
  - KNOW-e0dec720 skill uses this recommendation during setup
  - project.json has a workflow.epics-required field after setup
  - Software-like projects default to true
  - Research/planning projects default to false
  - Ambiguous projects prompt the user for a choice
relationships:
  - target: EPIC-dc1e3e4b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-dd9c8538
    type: depended-on-by
---
## What

When a new project is set up via OrqaStudio, the setup process needs to determine
the right level of structure enforcement. The `workflow.epics-required` setting
controls whether every task must link to an epic (strict, for implementation work)
or whether standalone tasks are acceptable (flexible, for research/planning).

This skill provides the inference logic:

### Signals for `epics-required: true`

- Project has `src/`, `lib/`, or language-specific source directories
- Project has `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`
- Project has CI/CD configuration (`.github/workflows/`, `Jenkinsfile`)
- Project has test directories (`tests/`, `__tests__/`, `spec/`)
- User describes the project as "building", "developing", "implementing"

### Signals for `epics-required: false`

- Project is primarily documentation (`.md` files dominate)
- Project has research, notes, or planning directories
- No build tooling or compiled languages detected
- User describes the project as "researching", "planning", "exploring", "writing"

### Ambiguous Cases

When signals are mixed (e.g., a software project with heavy research), the skill
recommends asking the user: "This project has both implementation and research
characteristics. Should tasks require epic linkage?"

## Integration

The `project-setup` skill calls this inference during the setup flow:

```
1. Run project-inference (existing) → project profile
2. Run epic-requirement inference (this skill) → recommended setting
3. Present recommendation to user with rationale
4. Set workflow.epics-required in project.json
```

## How

1. Create a new skill directory `.orqa/process/skills/epic-requirement-inference/`
2. Write SKILL.md with heuristics for determining epic requirement
3. Update project-setup skill to consume the inference
4. Add `workflow` section to project.json schema

## Verification

- Skill exists and validates against schema
- project-setup skill references the inference skill
- project.json can accept a workflow.epics-required field