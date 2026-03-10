---
id: TASK-049
title: Classify rules and hooks with layer field
status: done
epic: EPIC-044
description: Adds layer classification to all rule and hook files, separating platform-level canon rules from OrqaStudio-specific project rules.
created: 2026-03-09
updated: 2026-03-09
assignee: agent-maintainer
skills: [orqa-governance]
scope: [.orqa/governance/rules/, .orqa/governance/hooks/]
acceptance: ["All rule files have `layer:` field in YAML frontmatter", All hook files have classification documented, Canon rules enforce platform principles, Project rules enforce project-specific conventions]
---

## Classification Plan

### Rules — Canon (platform principles)

vision-alignment, systems-thinking, documentation-first, honest-reporting, no-stubs,
error-ownership, plan-mode-compliance, lessons-learned, enforcement-before-code,
historical-artifacts, no-deferred-deliverables, no-aliases-or-hacks, reusable-components,
skill-enforcement, required-reading, agent-delegation, uat-process, pillar-alignment-docs,
artifact-lifecycle, architecture-decisions, root-cleanliness, git-workflow,
testing-standards, coding-standards, chunkhound-usage, artifact-config-integrity,
end-to-end-completeness, structure-before-work

### Rules — Project (OrqaStudio-specific)

development-commands (make targets), dogfood-mode (dogfood flag)

### Hooks — Canon

session-start-hook.sh (session awareness), pre-commit-reminder.sh (quality gates)
