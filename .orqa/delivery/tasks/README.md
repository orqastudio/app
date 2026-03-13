---
role: artifacts
label: "Tasks"
description: "Atomic work items assigned to a single agent with clear acceptance criteria."
icon: "check-square"
sort: 5
---

# Tasks

Tasks are atomic work items within an epic — the smallest unit of tracked delivery. Each task is scoped to a single agent, carries acceptance criteria, and specifies the skills the agent must load before starting.

## Pipeline Role

Tasks are the execution layer of the delivery pipeline:

```
Epic (design) → Tasks (execution) → Verification (proof)
```

A task cannot start until all its `depends-on` tasks are done. It cannot be marked done until acceptance criteria are met and verified by a reviewer — the implementing agent cannot self-certify.

## Lifecycle

```
todo → in-progress → done
```

- **Todo**: Defined with acceptance criteria; dependencies not yet complete
- **In-progress**: Agent assigned; `depends-on` tasks all done
- **Done**: Acceptance criteria met and verified

## The Skills Field

The `skills` field is a traceability chain: the orchestrator populates it based on the domains the task touches; the assigned agent loads every listed skill before reading any code. This bridges the plan (what needs doing) to the execution (how it gets done correctly).
