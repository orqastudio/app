---
role: artifacts
label: "Tasks"
description: "Scoped work items within an epic."
icon: "check-square"
sort: 3
---

# Tasks

Tasks are scoped work items within an epic. They represent individual units of work that can be assigned to a single agent.

## Lifecycle

```
todo → in-progress → done
```

- **Todo**: Task defined with acceptance criteria
- **In-progress**: Agent assigned and working
- **Done**: Acceptance criteria met and verified

## What Makes a Good Task

- Belongs to a parent epic
- Has clear, testable acceptance criteria
- Can be completed by a single agent in one session
- Does not overlap with other tasks in the same epic

## Frontmatter Schema

Most tasks live as markdown checklist items in their parent epic. A task graduates to a separate `TASK-NNN.md` file when it needs its own detailed tracking — acceptance criteria, agent assignment, or discussion thread.

See `schema.json` in this directory for the complete field reference.

## The Skills Field

The `skills` field creates a traceability chain from plan to execution:

- **Plan** defines what needs doing
- **Task** specifies who does it (`assignee`) and what knowledge they need (`skills`)
- **Agent** loads those skills before starting
- **Implementation** is done with the right context loaded

When an orchestrator creates a task, it populates `skills` based on the domains the task touches. An agent that picks up the task loads every skill listed before reading any code.

Common skill combinations by domain:

| Domain | Typical Skills |
|--------|---------------|
| Rust backend | `orqa-code-search`, `orqa-ipc-patterns`, `orqa-repository-pattern`, `rust-async-patterns` |
| Svelte frontend | `orqa-code-search`, `orqa-store-patterns`, `orqa-ipc-patterns`, `svelte5-best-practices` |
| Streaming pipeline | `orqa-code-search`, `orqa-streaming`, `orqa-ipc-patterns` |
| Data / SQLite | `orqa-code-search`, `orqa-repository-pattern`, `orqa-domain-services` |
| Governance / agents | `orqa-code-search`, `orqa-governance` |

## Related

- Tasks belong to epics in the **Epics** section
- See `.orqa/documentation/product/artifact-framework.md` for the full task schema and lifecycle rules
