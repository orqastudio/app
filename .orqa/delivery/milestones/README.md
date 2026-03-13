---
role: artifacts
label: "Milestones"
description: "Strategic goals that group related epics together."
icon: "target"
sort: 1
---

# Milestones

Milestones are strategic goals that group related epics together. Each milestone has a gate question — a yes/no question that determines when the milestone is complete.

## Lifecycle

```
planning → active → complete
```

- **Planning**: Milestone defined; epics being scoped
- **Active**: At least one epic is in-progress
- **Complete**: All P1 epics are done and the gate question is answered "yes"

## Gate Questions

Every milestone has a gate question like "Can we use OrqaStudio to build OrqaStudio?" The milestone is complete when this question can honestly be answered yes.

## Related

- Milestones contain epics from the **Epics** section
- Priority scoring determines epic ordering within a milestone
