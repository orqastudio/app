---
id: KNOW-6ac4abed
type: knowledge
status: active
title: Audit Criteria
domain: architecture
description: Nine questions to ask when reviewing any file or artifact against the target architecture — use for audit work and quality checks
tier: always
relationships:
  synchronised-with: DOC-6ac4abed
---

# Audit Criteria

When reviewing any file or artifact, ask all nine questions:

1. **Does it belong here?** Would the finished app have created this file in this location?
2. **Correct artifact type?** (e.g., PERSONA in `personas/`, not DOC)
3. **Serves plugin-composed architecture?** Or does it assume the old monolithic model?
4. **Is it a duplicate?** Content installed from a plugin AND also defined manually?
5. **Correctly scoped?** Knowledge must be 500-2,000 tokens, atomic, self-contained.
6. **Proper frontmatter?** (id, type, title, description, relationships)
7. **Actively used?** Or a leftover from a superseded approach?
8. **Crosses boundaries?** (e.g., connector doing engine's job, app hardcoding governance patterns)
9. **Organized for human navigation?** Hash-only filenames in flat directories are not navigable.
