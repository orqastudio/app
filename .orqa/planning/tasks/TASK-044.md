---
id: TASK-044
title: Artifact type definitions and classification
status: done
epic: EPIC-043
description: Adds formal one-paragraph definitions for each artifact type to the framework documentation and reclassifies any misclassified artifacts, flattening the research directory to a relationship-via-YAML-fields model.
created: 2026-03-09
updated: 2026-03-09
assignee: agent-maintainer
skills: [orqa-governance, skills-maintenance]
scope: [.orqa/documentation/product/artifact-framework.md, .orqa/planning/research/, .orqa/planning/ideas/, .orqa/governance/rules/artifact-lifecycle.md]
acceptance: [Each artifact type has a clear one-paragraph definition of its purpose and when to use it, Definitions include "use this, NOT that" guidance (e.g., "research is investigation, not implementation planning — use epics for that"), Misclassified research documents reclassified (e.g., UX polish sprint → linked to its epic), Ideas with implemented features linked to decisions/epics via promoted-to field, Research directory flattened — no subfolders, relationships via YAML fields, artifact-framework.md updated with type definitions, artifact-lifecycle.md updated with classification rules]
---

## Findings Addressed

- **F22**: Research documents misclassified (some should be epics)
- **F23**: Artifact type definitions unclear
- **F24**: Ideas not linked to implemented decisions/epics
- **F29**: Research subfolder organisation inconsistent — flatten and use YAML fields

## Notes

This is primarily a data + documentation task. The artifact framework needs formal type definitions that answer "when do I create this type?" for each artifact. Then existing artifacts need auditing against those definitions.
