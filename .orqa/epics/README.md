# Epics

Epics are trackable work units that group related tasks together. Each epic has documentation gates, priority scoring, and verification requirements.

## Lifecycle

```
draft → ready → in-progress → review → done
```

- **Draft**: Epic scoped; documentation gates being satisfied
- **Ready**: All required docs exist; ready to implement
- **In-progress**: Implementation underway in a worktree
- **Review**: Code complete; passing verification gates
- **Done**: All gates passed; merged to main

## Documentation Gates

- **docs-required**: Documentation that must exist before implementation starts
- **docs-produced**: Documentation that the implementation must create or update

## Priority

Epics are scored across configurable dimensions and assigned P1/P2/P3 priority bands.

## Related

- Epics belong to milestones in the **Milestones** section
- Epics may contain tasks in the **Tasks** section
- Epics originate from promoted ideas in the **Ideas** section
