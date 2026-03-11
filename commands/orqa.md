---
name: orqa
description: Show OrqaStudio governance summary — active rules, epics, and tasks
---

Read the OrqaStudio governance state and present a summary.

## Instructions

1. Read `.orqa/project.json` to get the project name and configuration
2. Count active rules: list files in `.orqa/governance/rules/` matching `RULE-*.md`, read each file's frontmatter, count those with `status: active`
3. Count active epics: list files in `.orqa/planning/epics/` matching `EPIC-*.md`, read each file's frontmatter, count by status (draft, ready, in-progress, review, done)
4. Count tasks: list files in `.orqa/planning/tasks/` matching `TASK-*.md`, read each file's frontmatter, count by status (todo, in-progress, done)
5. Check for enforcement rules: count rules that have `enforcement:` entries in their frontmatter

Present the summary in a compact table format:

```
## Governance Summary

| Category | Count |
|----------|-------|
| Active rules | N |
| Rules with enforcement | N |
| Epics (draft/ready/in-progress/review/done) | N/N/N/N/N |
| Tasks (todo/in-progress/done) | N/N/N |
| Active lessons | N |
```

Then list the in-progress epics and todo tasks briefly.
