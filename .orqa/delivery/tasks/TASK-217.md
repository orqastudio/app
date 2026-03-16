---
id: TASK-217
title: Add web_fetch and web_search capabilities to research and planner agents
description: Ensure Researcher and Planner agent definitions include web_fetch and web_search capabilities so they can access external knowledge during investigation.
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - Researcher and Planner agents can use WebSearch and WebFetch in CLI context
  - SKILL-046 skill is in Researcher and Planner agent definitions
  - Capability mappings are correct in RULE-040
  - Tool access restrictions in RULE-037 are updated
relationships:
  - target: EPIC-053
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-343
    type: depended-on-by
---
## What

Web search is a first-class research tool. Researchers should be able to search the web during investigation, and Planners should be able to look up external references. This connects external knowledge to the artifact graph through research documents that cite their sources.

The `research-methodology` skill (`.orqa/process/skills/research-methodology/SKILL.md`) provides source verification protocol, credibility tiers (T1-T4), cross-referencing rules, and confidence levels to ensure research findings are verified to an acceptable level of certainty.

## How

1. Add `web_fetch` and `web_search` to Researcher and Planner agent capabilities
2. Add [SKILL-046](SKILL-046) (research-methodology) to both agents' skills lists
3. Update [RULE-037](RULE-037) tool access matrix with web capabilities
4. Verify [RULE-040](RULE-040) already has the mappings

## Verification

- Researcher agent definition has web_fetch and web_search capabilities
- Planner agent definition has web_fetch and web_search capabilities
- Both agents have [SKILL-046](SKILL-046) in their skills lists
- [RULE-037](RULE-037) matrix shows web capabilities for appropriate roles
- [RULE-040](RULE-040) mappings already cover web_fetch and web_search
