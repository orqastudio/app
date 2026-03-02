# Sidebar Synchronization (NON-NEGOTIABLE)

The Forge documentation site uses sidebar files that must always be kept in sync. A navigation entry that exists in one sidebar but not the others produces broken links or missing items depending on which section the reader is in.

## Top-Level Sections (canonical order)

Every sidebar file MUST contain all top-level sections in this exact order:

1. **Introduction** — home link (`/`)
2. **Product** — vision, pillars, governance, roadmap
3. **Architecture** — decisions, IPC design, data flow, module structure
4. **User Interface** — per-page UI documentation, component library
5. **Development** — coding standards, workflow, tooling, testing
6. **Research** — background investigations, tech stack evaluations
7. **Process** — team, orchestration, workflow, governance, rules, skills, retrospectives

## Mandatory Synchronization Rules

> [!IMPORTANT]
> Every change to any sidebar MUST be reflected in ALL sidebar files in the same commit. Partial updates are rejected.

### Adding a new page

1. Identify which section owns the page
2. Add the entry to the **owning sidebar** in the expanded section at the correct position
3. Add the entry to the **root sidebar** in the collapsed or expanded view of that section
4. Verify all **other sidebar files** still match the root for that section

### Moving a page between sections

1. Remove the entry from the old section in ALL sidebar files
2. Add the entry to the new section in ALL sidebar files
3. Move the file on disk (`git mv`) to the new section's folder
4. Update the path in all sidebar files to match the new location

### Removing a page

Remove the entry from ALL sidebar files in the same commit that deletes the file. Never leave dead links.

### Reordering entries within a section

When changing the order of entries within a section, update the order in ALL sidebar files. The owning sidebar must match the root sidebar's ordering for that section.

## Verification Checklist (run before committing)

- [ ] Root sidebar — updated with the change
- [ ] Owning section's sidebar — expanded entry added/updated/removed
- [ ] All other sidebar files — top-level section list consistent with root
- [ ] No sidebar contains a link to a file that no longer exists
- [ ] No sidebar is missing a link to a file that does exist and is not exempt

## Common Mistakes

| Mistake | Consequence | Fix |
|---------|-------------|-----|
| Adding a page to only the root sidebar | Page unreachable from within its own section | Add to owning sidebar too |
| Adding a page to only the owning sidebar | Page missing from root navigation | Add to root sidebar too |
| Forgetting to update non-owning sidebars | Stale navigation in other sections | Update all sidebar files |
| Moving a file without updating all paths | 404 errors across all sidebar files | Search-replace old path across all files |

## Related Rules

- `documentation-first.md` — documentation changes require approval before implementation
