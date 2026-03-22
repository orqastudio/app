---
id: "TASK-732d8f12"
title: "Deduplicate rule/skill content — rules keep constraints, skills keep methodology"
description: "Remove duplicated methodology, templates, and code examples from three rules and move them into their companion skills, leaving rules with only constraints and FORBIDDEN sections."
status: "completed"
created: "2026-03-11"
updated: "2026-03-11"
acceptance:
  - "RULE-303c1cc8 contains only constraints, gates, and FORBIDDEN — references planning skill"
  - "RULE-4d4f540d contains only phase structure, constraints, and FORBIDDEN — references uat-process skill"
  - "RULE-1acb1602 contains only layer requirements, checklist, and FORBIDDEN — references orqa-ipc-patterns skill"
  - "Skills contain the full methodology, templates, and code examples"
  - "No content lost — everything moved, nothing deleted"
  - "Rules are significantly shorter after deduplication"
relationships:
  - target: "EPIC-4a7aeacb"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-ec136ce9"
    type: "depended-on-by"
---
## What

Three rule/skill pairs have significant content duplication. Apply the principle: rules keep constraints and FORBIDDEN sections, skills keep methodology, templates, and code examples. Rules reference skills for the "how."

| Rule | Skill | Action |
|------|-------|--------|
| [RULE-303c1cc8](RULE-303c1cc8) (Plan Mode Compliance) | `planning` | Move plan template from rule to skill. Rule keeps mandatory requirements and verification gates. Add "See `planning` skill for template and methodology." |
| [RULE-4d4f540d](RULE-4d4f540d) (UAT Process) | `uat-process` | Move 4-phase methodology detail from rule to skill. Rule keeps phase names, FORBIDDEN section, and process improvement encoding requirement. Add skill reference. |
| [RULE-1acb1602](RULE-1acb1602) (End-to-End Completeness) | `orqa-ipc-patterns` | Move 100+ lines of code examples from rule to skill. Rule keeps the 4-layer requirement, verification checklist, and FORBIDDEN patterns (keep these concise). Add skill reference. |

Also consider [RULE-cb65b5d0](RULE-cb65b5d0) (Reusable Components) which has a component inventory that is knowledge, not constraint — after [TASK-cca73736](TASK-cca73736) completes, evaluate whether the inventory belongs in a companion skill instead.

## How

1. For each rule/skill pair: copy the methodology/template/example content into the skill file first
2. Verify the skill now contains the full detail (nothing missing)
3. Then remove the duplicated content from the rule, leaving only constraints, gates, and FORBIDDEN sections
4. Add a "See `<skill-name>` skill for full methodology and examples" reference sentence in each rule
5. For [RULE-1acb1602](RULE-1acb1602), add the moved code examples to the `orqa-ipc-patterns` skill's existing content

## Verification

- [ ] [RULE-303c1cc8](RULE-303c1cc8) contains only constraints, gates, and FORBIDDEN — references planning skill
- [ ] [RULE-4d4f540d](RULE-4d4f540d) contains only phase structure, constraints, and FORBIDDEN — references uat-process skill
- [ ] [RULE-1acb1602](RULE-1acb1602) contains only layer requirements, checklist, and FORBIDDEN — references orqa-ipc-patterns skill
- [ ] Skills contain the full methodology, templates, and code examples
- [ ] No content lost — everything moved, nothing deleted
- [ ] Rules are significantly shorter after deduplication
