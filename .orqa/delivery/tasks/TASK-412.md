---
id: TASK-412
title: Implement full skill content injection in rule-engine.mjs
description: Currently skill injection returns skill names as a list. Change it to read the actual SKILL.md file content and inject it as systemMessage so agents receive the knowledge, not just a reference.
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-064
depends-on: []
assignee: null
skills:
  - SKILL-011
  - SKILL-020
acceptance:
  - collectSkillIds() resolves skill names to .orqa/process/skills/{name}/SKILL.md paths
  - Skill file content is read, YAML frontmatter stripped, body returned as systemMessage
  - Deduplication still works via .injected-skills.json (no re-injection of already-loaded skills)
  - Missing skill files produce a warning message, not a crash
  - Injected content is returned alongside any warn/block verdicts
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Core task — makes skill injection meaningful instead of just naming skills
  - target: RULE-042
    type: enforces
    rationale: RULE-042's path-to-skill injection depends on actual skill content being loaded
  - target: EPIC-064
    type: belongs-to
    rationale: Task belongs to this epic
  - target: TASK-467
    type: informs
    rationale: "Auto-generated inverse of informs relationship from TASK-467"
---
## Scope

### rule-engine.mjs Changes (lines 156-177)

Current `collectSkillIds()`:
```js
return `**Read before implementing:**\n${newIds.map((id) => `- ${id}`).join("\n")}`;
```

Target:
```js
const contents = newIds.map(id => {
  const path = resolve(ROOT, '.orqa/process/skills', id, 'SKILL.md');
  if (!existsSync(path)) return `[WARNING: Skill '${id}' not found at ${path}]`;
  const raw = readFileSync(path, 'utf-8');
  // Strip YAML frontmatter
  const body = raw.replace(/^---[\s\S]*?---\n*/, '');
  return `## Skill: ${id}\n\n${body}`;
}).join('\n\n---\n\n');
return contents;
```

### Edge Cases

- Skill directory exists but SKILL.md missing → warning message
- Skill file has no frontmatter → return raw content
- Large skill files → consider truncation if systemMessage has size limits
