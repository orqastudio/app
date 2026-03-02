# Sidebar Synchronization (NON-NEGOTIABLE)

Forge uses a **single root sidebar** (`docs/_sidebar.md`) for the entire documentation site. There are no subdirectory sidebar files. This matches Alvarez's pattern and avoids synchronization drift between multiple copies of the same navigation.

## Single Sidebar Rule

- The **only** sidebar file is `docs/_sidebar.md`
- **NEVER** create `_sidebar.md` files in subdirectories — Docsify inherits the root sidebar for all pages
- If a subdirectory `_sidebar.md` is found, delete it immediately — it will override the root and drift out of sync

## Top-Level Sections (canonical order)

The root sidebar MUST contain all top-level sections in this exact order:

1. **Introduction** — home link (`/`)
2. **Product** — vision, pillars, governance, roadmap
3. **Architecture** — decisions, IPC design, data flow, module structure
4. **User Interface** — per-page UI documentation, component library
5. **Wireframes** — layout and view wireframes (top-level, not nested under UI)
6. **Development** — coding standards, workflow, tooling, testing
7. **Research** — background investigations, tech stack evaluations
8. **Process** — team, orchestration, workflow, governance, rules, skills, retrospectives

## Adding a new page

1. Identify which section owns the page
2. Add the entry to `docs/_sidebar.md` in the correct section at the correct position
3. Verify the link path matches the file's actual location on disk

## Moving a page between sections

1. Remove the entry from the old section in `docs/_sidebar.md`
2. Add the entry to the new section in `docs/_sidebar.md`
3. Move the file on disk (`git mv`) to the new section's folder
4. Update the path in the sidebar to match the new location

## Removing a page

Remove the entry from `docs/_sidebar.md` in the same commit that deletes the file. Never leave dead links.

## Verification Checklist (run before committing)

- [ ] `docs/_sidebar.md` — updated with the change
- [ ] No sidebar entry links to a file that no longer exists
- [ ] No documentation file exists that is missing from the sidebar (unless exempt)
- [ ] No subdirectory `_sidebar.md` files exist

## Related Rules

- `documentation-first.md` — documentation changes require approval before implementation
