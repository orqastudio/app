---
role: artifacts
label: "Milestones"
description: "Strategic goals that group related epics together."
icon: "flag"
sort: 3
---

# Milestones

Milestones are strategic goals that group related epics. Each milestone has a gate question — a concrete yes/no question that defines when the milestone is complete. Milestones are the top-level structure for delivery; all epics belong to a milestone.

## Pipeline Role

Milestones sit above epics in the delivery hierarchy:

```
Milestone → Epic → Task → Verification
```

They are the answer to "what are we trying to achieve?" where epics answer "how are we achieving it?" A milestone is not complete until all P1 epics are done and the gate question can honestly be answered yes.

## Lifecycle

```
planning → active → complete
```

- **Planning**: Milestone defined; epics being scoped
- **Active**: At least one epic is in-progress
- **Complete**: All P1 epics done; gate question answered yes
