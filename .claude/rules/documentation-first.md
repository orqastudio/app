---
scope: system
---

# Documentation-First Implementation (NON-NEGOTIABLE)

## Principle

Documentation is the source of truth. Before writing ANY implementation code, verify that documentation exists for the feature area and read it. If documentation is missing or outdated, update the docs FIRST, get user approval, then implement.

## Before ANY Code Changes (MANDATORY)

1. Check `docs/ui/` for existing designs related to the task
2. Check `docs/architecture/` for relevant architecture decisions
3. Check `docs/development/coding-standards.md` for implementation patterns
4. Check `TODO.md` for phase requirements and constraints
5. Check `docs/product/roadmap.md` to verify work is prioritized

**Documentation priorities ALWAYS come before implementation priorities in TODO.md.** Never reorder to put code changes ahead of documentation corrections.

**When an audit reveals gaps between docs and code:** Update the docs FIRST to define the correct target state. Code is then changed to match the docs. Never fix code without first verifying the docs describe the intended behavior.

## For New Features

1. Read `docs/product/vision.md` — verify feature serves Pillar 1 or Pillar 2
2. Read `docs/product/governance.md` — verify feature passes governance criteria

## Architectural Principles (verify compliance on EVERY change)

- **Immutability** — Rust domain types should be immutable by default. No global mutable state outside of explicitly managed stores.
- **Error propagation** — All Rust functions return `Result` types. No `unwrap()` in production code. `thiserror` for typed errors.
- **IPC boundary** — Tauri commands are the ONLY interface between frontend and backend. No direct FFI calls, no side-channel communication.
- **Component purity** — Svelte display components receive props only. Data fetching happens in pages and containers, not in reusable components.

## During Implementation

- Re-read governing docs at the START of every phase (even across sessions)
- If code diverges from docs, fix the code (not the docs)
- If an improvement is found, update docs first with justification, then code
- No silent deviations from approved documentation

## Documentation File Placement

- New documentation files MUST be placed in the folder matching their section
- Moving a page between sections requires `git mv` to the new folder

## No Deprecated Documentation (NON-NEGOTIABLE)

This project is in alpha. Documentation describes the **active target state only**. There is no audience for migration guides, deprecation notices, or historical redirects.

**Rules:**

- When a feature or concept is consolidated or removed, **delete its documentation page entirely**. Do not leave redirect stubs, "moved to" notices, or migration tables.
- Remove the deleted page from any sidebar or index in the same commit.
- Update any cross-references that linked to the deleted page.
- If content was merged into another page, only the destination page should exist.

**FORBIDDEN:**

- Pages whose only content is "This has moved to X" or "See Y instead"
- Migration reference tables showing old-to-new mappings
- Keeping deprecated pages "for reference" — git history serves that purpose

## Bug Investigation Protocol (MANDATORY)

When investigating bugs, UI mismatches, or differences between user expectations and implementation:

1. **Reproduce** — Understand what the user expects vs. what they see
2. **Find governing docs** — Locate documentation that defines the expected behavior
3. **Compare docs to code** — Determine whether docs match user expectations and whether code matches docs
4. **Report alignment** — Tell the user whether docs are right, code is right, or both need updating
5. **Ask for direction** — Present options: fix code to match docs, update docs + code, or keep current behavior

**NEVER jump straight to fixing code without checking documentation first.** The fix direction depends on whether the documentation matches user intent.

## Related Rules

- `plan-mode-compliance.md` — plan structure requirements
- `chunkhound-usage.md` — using code_research for documentation discovery
- `vision-alignment.md` — pillar alignment and governance
- `architecture-decisions.md` — architecture decision compliance
- `coding-standards.md` — function size, typing, coverage requirements
