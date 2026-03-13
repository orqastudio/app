---
role: artifacts
label: "Epics"
description: "Trackable work units that carry implementation design and documentation gates."
icon: "layers"
sort: 4
---

# Epics

Epics are the primary planning unit for implementation. Each epic carries its implementation design in the body, documentation gates that must be satisfied before and after implementation, and priority scoring that determines when it gets worked on.

## Pipeline Role

Epics sit at the centre of the delivery pipeline:

```
Idea → Research → Epic → Task → Verification
```

An epic is not just a container for tasks — it is the design document. The epic body holds the data model, IPC contracts, component breakdown, and approach. Tasks are the atomic units that implement what the epic designs.

## Lifecycle

```
draft → ready → in-progress → review → done
```

- **Draft**: Epic scoped; `docs-required` gate being satisfied
- **Ready**: All required docs exist; implementation can begin
- **In-progress**: Implementation underway in a worktree
- **Review**: Code complete; verification gates in progress
- **Done**: All gates passed; merged to main

## Documentation Gates

- **`docs-required`**: Must exist before implementation starts (`draft → ready`)
- **`docs-produced`**: Must be created or updated on completion (`review → done`)
