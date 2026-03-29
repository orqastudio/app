---
id: KNOW-6ac4abed
type: knowledge
status: active
title: "Audit Criteria for Files and Artifacts"
domain: architecture
description: "Nine questions to ask when reviewing any file or artifact against the target architecture — use for audit work and quality checks"
tier: stage-triggered
created: 2026-03-28
roles: [reviewer]
paths: [.orqa/]
tags: [audit, review, quality, artifacts, architecture]
relationships:
  - type: synchronised-with
    target: DOC-6ac4abed
---

# Audit Criteria for Files and Artifacts

When reviewing any file or artifact against the target architecture, ask all nine questions:

## The Nine Questions

1. **Does it belong here?** Would the finished app have created this file in this location? If not, it must be moved, reformatted, or deleted.

2. **Correct artifact type?** Every artifact must have the correct type for its location. A persona belongs in `discovery/personas/` as a `persona` type, not as a DOC in `documentation/`. Type mismatches create invalid relationships and break schema validation.

3. **Serves plugin-composed architecture?** Does this file assume the old monolithic model where knowledge, rules, and structure were hardcoded? Does it reference patterns that have been superseded? If so, it must be updated or deleted — not migrated as-is.

4. **Is it a duplicate?** Content installed from a plugin that is ALSO defined manually creates drift. The plugin-installed copy is the source of truth. Manual copies outside the installed hierarchy must be deleted.

5. **Correctly scoped?** Knowledge artifacts must be 500-2,000 tokens, atomic (covering ONE sub-topic), and self-contained (usable without reading other artifacts). An artifact at 300 tokens is incomplete. An artifact at 3,000 tokens needs splitting.

6. **Proper frontmatter?** Required fields: `id` (with correct prefix for type), `type`, `title`, `description`, `status`, `created`, `updated`. Use `title` not `name`. Status must be a valid value from the type's state machine. Relationships must use valid types from plugin schemas.

7. **Actively used?** Legacy artifacts from superseded approaches must be deleted, not archived in place. If it hasn't been referenced in workflows, relationships, or agent prompts — it is dead weight. Zero tech debt means nothing survives without purpose.

8. **Crosses boundaries?** Check: Is connector code doing engine work? Is frontend hardcoding governance patterns? Is CLI implementing business logic? Boundary violations must be corrected in the same phase that finds them — not deferred.

9. **Organized for human navigation?** Hash-only filenames in flat directories are not navigable. Artifacts must be in meaningful subdirectories with descriptive names. The `.orqa/` hierarchy should be something a human can browse without a search tool.

## Quick Reference: Verdict Table

| Question | Pass | Fail → Action |
| --------- | ---- | -------------- |
| Location correct | File in right stage dir | Move to correct location |
| Type correct | `type:` matches dir and ID prefix | Fix type or move file |
| Architecture-aware | Uses plugin-composed model | Update or delete |
| No duplicate | Not also installed from plugin | Delete the manual copy |
| Correct scope | 500-2000 tokens, one topic | Expand or split |
| Valid frontmatter | All required fields, valid values | Fix frontmatter |
| Actively used | Referenced in workflow or relationships | Delete if dead weight |
| Boundary-clean | No crossing | Fix boundary violation now |
| Navigable | Descriptive name in meaningful dir | Rename or reorganize |
