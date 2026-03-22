---
id: IDEA-37a23ec0
type: idea
title: skills.sh compatibility plugin — import and reformat external skills
description: A plugin that installs skills from skills.sh and reformats/renames them to fit OrqaStudio conventions. Imported skills are related back to the skills.sh plugin's human-facing documentation as a reference of provenance.
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

# IDEA-37a23ec0: skills.sh Compatibility Plugin

## Concept

A plugin (`@orqastudio/plugin-skills-sh`) that bridges the skills.sh ecosystem with OrqaStudio:

1. **Import from skills.sh** — `orqa plugin skills-sh install <skill-name>` fetches a skill from skills.sh
2. **Reformat to OrqaStudio conventions** — the plugin transforms the imported skill into OrqaStudio's flat `.md` format with proper frontmatter (id, type, status, relationships)
3. **Provenance tracking** — each imported skill gets a `synchronised-with` relationship back to a documentation artifact that records where it came from (skills.sh URL, version, import date)
4. **Update detection** — the plugin can check if upstream skills.sh skills have been updated and offer to re-import

## Why

skills.sh is building a community ecosystem of reusable AI skills. OrqaStudio shouldn't compete with that — it should interoperate. Users should be able to pull in community skills and have them work within OrqaStudio's governance model (enforcement, validation, relationship tracking) without manual reformatting.

## Key Design Considerations

- Imported skills should be treated as plugin-provided (layer: plugin)
- The plugin creates a human-facing doc artifact per import source as provenance record
- Reformatting should preserve the original content but add OrqaStudio frontmatter
- Skills that don't map cleanly to OrqaStudio conventions should be imported with warnings, not rejected
- The plugin itself is a thin adapter — it doesn't host skills, it bridges two formats

## Pending: Skills Maintenance Knowledge

KNOW-2c8eead6 (Skills Maintenance) is currently archived. When this plugin is implemented,
reactivate it and move it here — it teaches how skills are maintained, updated, and
synchronized, which is directly relevant to the skills.sh import/update workflow.
