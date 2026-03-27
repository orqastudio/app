# Audit Criteria

> This is part of the OrqaStudio Architecture Reference. See ARCHITECTURE.md for the complete document.

---

## 14. Audit Criteria

When reviewing files against this architecture, each file should be assessed on:

1. **Does it belong in this location?** Would the finished app have created this file here?
2. **Does it have the correct artifact type?** (e.g., PERSONA in personas/, not DOC)
3. **Does it serve the plugin-composed architecture?** Or does it assume the old monolithic model?
4. **Is it a duplicate?** Content installed from a plugin AND also defined manually.
5. **Is it correctly scoped?** Knowledge should be 500-2,000 tokens, atomic, self-contained.
6. **Does it have proper frontmatter?** (id, type, title, description, relationships)
7. **Is it actively used?** Or is it a leftover from a superseded approach?
8. **Does it cross boundaries?** (e.g., connector doing engine's job, app hardcoding governance patterns)
9. **Is it organized for human navigation?** Hash-only filenames in flat directories are not navigable.
