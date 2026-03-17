---
id: SKILL-047
title: Epic Requirement Inference
description: |
  Evaluates project context to recommend whether epics should be required
  for task creation. Analyses directory structure, build tooling, and
  project type signals to determine the workflow.epics-required setting.
  Use when: Running project setup or configuring workflow enforcement.
status: active
layer: setup
category: tool
version: 1.0.0
user-invocable: false
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Automatically configures task-to-epic linkage requirements based on project signals, ensuring workflow governance matches project complexity
  - target: PILLAR-001
    type: informs
---

Determines whether a project should require epic linkage for all tasks
(`workflow.epics-required: true`) or allow standalone tasks
(`workflow.epics-required: false`).

## Decision Logic

### Strong signals for `epics-required: true`

These indicate an implementation-heavy project where structured planning prevents scope creep:

| Signal | Weight | Check |
|--------|--------|-------|
| Source code directories | High | `src/`, `lib/`, `app/`, `pkg/`, `cmd/` exist |
| Build tooling | High | `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`, `Makefile` exist |
| CI/CD configuration | Medium | `.github/workflows/`, `Jenkinsfile`, `.gitlab-ci.yml` exist |
| Test directories | Medium | `tests/`, `__tests__/`, `spec/`, `test/` exist |
| Compiled output | Medium | `target/`, `dist/`, `build/` in `.gitignore` |
| User describes "building" or "developing" | High | Natural language signal during setup |

### Strong signals for `epics-required: false`

These indicate a research/planning project where lightweight task tracking is appropriate:

| Signal | Weight | Check |
|--------|--------|-------|
| Documentation-dominant | High | >70% of files are `.md` |
| Research/notes directories | High | `research/`, `notes/`, `docs/`, `journal/` exist without source code |
| No build tooling | High | No `Cargo.toml`, `package.json`, etc. |
| No compiled languages | Medium | No `.rs`, `.go`, `.java`, `.cs`, `.cpp` files |
| User describes "researching", "planning", "exploring" | High | Natural language signal |

### Ambiguous cases

When signals are mixed (e.g., a software project with a heavy research phase), recommend asking the user:

> "This project has both implementation and research characteristics.
> Should tasks require epic linkage? (Recommended: yes for implementation-focused, no for research-focused)"

## Integration with Project Setup

The `project-setup` skill should call this inference after `project-inference` runs:

```
1. project-inference → project profile (languages, frameworks, type)
2. epic-requirement-inference → recommended epics-required setting
3. Present recommendation with rationale to user
4. Set workflow.epics-required in project.json
```

## project.json Schema Addition

Add a `workflow` section to `project.json`:

```json
{
  "workflow": {
    "epics-required": true
  }
}
```

- `true` (default for software projects): Tasks without an `epic` field trigger a warning from the enforcement engine. The schema allows it, but the engine flags it.
- `false` (default for research/planning): Tasks without an `epic` field are normal. No warning.

## Enforcement Behaviour

| `epics-required` | Task without `epic` | Enforcement |
|-------------------|---------------------|-------------|
| `true` | Created | Warn: "This task has no epic. Consider linking it to an epic for traceability." |
| `false` | Created | No warning |
| Not set | Created | Treat as `true` for software projects (detected by `project-inference`) |
