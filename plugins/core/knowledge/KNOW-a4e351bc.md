---
id: KNOW-a4e351bc
type: knowledge
title: Governance Migration Methodology
description: |
summary: "|. Methodology for migrating an existing project's governance into a structured governance framework. The goal is coexistence, not replacement — existing tools continue to work, and the governance framework becomes the single source of truth that feeds them."
  Methodology for migrating an existing project to a structured governance
  framework. Detects existing rules, instructions, and conventions from other
  tools, maps them to governance artifacts, and establishes coexistence so
  existing tooling continues to work.
  Use when: Onboarding a project that already uses AI tools or has documented
  conventions, or adding a governance layer to a project mid-lifecycle.
status: active
created: 2026-03-22
updated: 2026-03-22
category: methodology
version: 1.0.0
user-invocable: true
---

Methodology for migrating an existing project's governance into a structured governance framework. The goal is coexistence, not replacement — existing tools continue to work, and the governance framework becomes the single source of truth that feeds them.

## When to Use This

Migration applies when project type detection (see `project-type-detection`) finds:
- Existing AI tool configuration files with rules or instructions
- Convention documents (`CONVENTIONS.md`, `CONTRIBUTING.md`)
- Pre-commit hooks with embedded rules
- Any other source of documented project constraints

Do not skip migration. Layering new governance on top of undiscovered existing governance creates conflicts that surface unexpectedly during development.

## Supported Source Types

### Monolithic Instruction Files

Many tools use a single large file for all instructions. These must be decomposed into individual rules during migration.

**Decomposition pattern:**
```
Monolithic file contains:
  "Always use strict TypeScript"      → rule: typescript-strict.md
  "Never use console.log in prod"     → rule: no-console-log.md
  "Run tests before committing"       → rule: pre-commit-tests.md
```

Each extracted rule gets proper structured frontmatter: unique ID, title, description, status, and scope.

### Rule Files

If the source tool already uses individual rule files, they map directly to individual governance rules. Check for:
- Duplicates against existing core rules (do not create duplicates)
- Conflicts with core rules (surface to the user, do not silently override)

### Hooks and Automation

Pre-commit hooks, CI configuration, and automation scripts map to the governance framework's hook and automation layer. Preserve the existing hook behavior — extract the logic into the governance hook format but keep the original configuration active during transition.

### Project Settings and Configuration

Model settings, tool preferences, and project metadata map to the project configuration file (`project.json` or equivalent). These are settings, not rules — they do not belong in the rules directory.

### Convention Documents

`CONVENTIONS.md`, `CONTRIBUTING.md`, and similar documents are usually a mix of rules, guidelines, and onboarding documentation. Classify each section:
- Clear constraint with PASS/FAIL criteria → rule
- Best practice guidance → knowledge artifact
- Setup instructions → documentation artifact
- Historical context → decision artifact

## Migration Procedure

### 1. Detect

Run `project-type-detection` to get the full list of detected governance sources. Do not start migration without the complete detection output.

### 2. Read

Parse each source file and inventory its content. Produce a migration manifest listing every piece of content found and its proposed mapping.

### 3. Classify

For each content item, determine what governance artifact type it maps to:

| Content Type | Maps To |
|-------------|---------|
| Constraint with PASS/FAIL criteria | Rule artifact |
| Agent or role instructions | Agent definition or skill |
| Hook or automation | Hook artifact |
| Project configuration | Project settings |
| Guidance without clear criteria | Knowledge artifact |
| Architecture rationale | Decision artifact |

### 4. Deduplicate

Before creating any new artifact, check whether the content already exists in the governance framework's core artifacts. Common rules (no `console.log`, strict types, test coverage) often exist in core already. Do not create duplicates.

When content overlaps with an existing rule:
- If they agree: link the migration source to the existing rule (no new artifact needed)
- If they conflict: surface the conflict to the user; do not resolve silently

### 5. Create

Write the extracted governance artifacts with proper structured frontmatter. Do not flatten everything into a single artifact — each distinct rule, constraint, or guideline gets its own artifact.

### 6. Establish Coexistence

Existing tool configurations should become generated outputs, not sources of truth, once migration is complete. The governance framework generates them:

```
Governance framework (source of truth)
  → generates tool-specific config files
  → existing tools read their native format unchanged
```

During the transition period, the original files may remain active. Mark them as "generated" or "managed by governance" so future contributors know not to edit them directly.

### 7. Report

After migration, produce a summary for the user:

- What was found (source files and content count)
- What was migrated (new artifacts created, with types)
- What was deduplicated (existing artifacts that already covered the content)
- What needs manual review (ambiguous content, conflicts, content that couldn't be classified automatically)

Do not complete migration without presenting this summary. The user must be able to verify what changed.

## Governance Hub Model

Once migration is complete, the governance framework operates as a hub:

1. The governance directory is the single source of truth for all rules, decisions, and knowledge
2. Tool-specific configuration files are generated from governance artifacts
3. Changes flow in one direction: governance → generated configs → tools read their format
4. A file watcher (or CI step) can auto-regenerate tool configs when governance changes

This eliminates drift between tools. Adding a rule once propagates it to all configured tools.

## Critical Rules

- NEVER delete existing tool configurations during migration — coexistence first
- NEVER assume the user wants to migrate everything — present the manifest and confirm scope before creating artifacts
- NEVER resolve a conflict between existing rules and core rules silently — surface it
- Always preserve original content as a reference (in a migration log or comments) until the migration is confirmed complete
- Confidence levels apply to classification: high (clear mapping), medium (reasonable interpretation), low (needs user decision)
- If content doesn't clearly map to any governance artifact type, flag it for manual review rather than forcing a classification
