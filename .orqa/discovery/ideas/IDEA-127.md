---
id: IDEA-9c58fbb6
type: idea
title: "Git as a plugin — centralised githooks enforcement across organisation repos"
description: "Git integration as a first-party plugin that sets up git and githooks, enforcing consistent quality checks across all repos in an organisation. Ships enabled by default with the app. Acts as the central source of truth for githooks config. Disabling shows a user warning that version history and enforcement of artifact data integrity will be lost."
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: MS-654badde
    type: grounded-by
  - target: PILLAR-569581e0
    type: grounded
---
  - target: PERSONA-cda6edd6
    type: benefits
---

# IDEA-9c58fbb6: Git as a Plugin

## Concept

Git integration should be a first-party OrqaStudio plugin (`@orqastudio/plugin-git`) that:

1. **Ships enabled by default** — every OrqaStudio project gets git enforcement out of the box
2. **Centralises githooks** — the plugin is the single source of truth for pre-commit, pre-push, and commit-msg hooks across all repos in an organisation
3. **Enforces quality checks** — same checks (integrity validation, frontmatter compliance, relationship bidirectionality) run consistently across every repo
4. **Organisation-mode sync** — in org mode, hook config propagates from the organisation root to all child projects, with override tracking
5. **User warning on disable** — disabling the plugin shows a clear warning that version history tracking and enforcement of artifact data integrity will be lost

## Why

Currently git setup is manual and inconsistent. Each project may or may not have hooks, and there's no guarantee they enforce the same checks. Making git a plugin means:

- Enforcement is declarative and version-controlled
- New projects get hooks automatically
- Organisation-level policy is enforceable
- The app can surface hook violations in the UI (not just the terminal)

**Strategic:** By packaging version control as a plugin rather than a hard dependency, we create the platform to build custom cloud hosting into the app in the future without relying on git. A cloud-hosted artifact store could be a second version-control plugin that implements the same enforcement interfaces — the app doesn't care whether persistence comes from git or a hosted service, it just needs a plugin that satisfies the contract.

## Key Design Considerations

- Hook templates should be generated from rules (same pattern as coding standards plugins)
- The plugin should detect existing `.git/hooks/` and offer migration
- Must work with husky/lint-staged if already installed (coexistence or migration)
- Pre-commit hook should run `orqa validate` (or a fast subset)
- Commit-msg hook could enforce conventional commits or artifact ID references
