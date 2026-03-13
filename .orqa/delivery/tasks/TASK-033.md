---
id: TASK-033
title: Three-tier skill loading for agents
description: Updates all 16 agent definitions to carry only portable Tier 1 skills and the code-search wrapper, removing all project-specific and context-specific skills from their frontmatter.
status: done
created: "2026-03-09"
updated: "2026-03-09"
epic: EPIC-042
assignee: AGENT-003
skills:
  - SKILL-029
  - SKILL-011
acceptance:
  - "All agent skills: lists contain ONLY Tier 1 portable skills + code-search wrapper"
  - No agent lists chunkhound, orqa-native-search, or any orqa-* skill directly
  - Orchestrator agent updated with injection table reference
---
## What

Update the YAML `skills:` frontmatter on all 16 agent definitions to contain only
Tier 1 (portable) skills and the `code-search` wrapper. Remove all `orqa-*` project
skills and context-specific search skills.

## Scope

All agents in `.orqa/process/agents/`:
- orchestrator, backend-engineer, frontend-engineer, designer, debugger
- test-engineer, code-reviewer, data-engineer, devops-engineer
- documentation-writer, security-engineer, refactor-agent
- agent-maintainer, systems-architect, qa-tester, ux-reviewer

## Implementation Notes

For each agent:
1. Keep: `code-search`, `architecture`, language/framework skills relevant to their role
2. Remove: `chunkhound`, `orqa-native-search`, all `orqa-*` project skills
3. The `composability` meta-skill moves to Tier 2 (always injected by orchestrator)

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
