---
id: project-migration
layer: canon
title: "Project Migration"
name: project-migration
description: |
  Reads existing agentic tool configurations (Claude Code, Cursor, Copilot, Aider)
  and maps their rules, instructions, and settings into OrqaStudio's governance
  structure. Preserves existing governance while adding OrqaStudio's layer.
  Use when: Setting up OrqaStudio on a project that already uses other AI tools,
  or syncing governance between OrqaStudio and external tools.
version: 1.0.0
tags: [setup, migration, claude-code, cursor, copilot, governance-hub]
user-invocable: true
---


Maps existing agentic tool configurations into OrqaStudio's governance structure. OrqaStudio coexists with other tools — it doesn't replace them. Migration creates the bridge between OrqaStudio's structured governance and each tool's native configuration format.

## Supported Source Formats

### Claude Code

| Source | Content | Maps To |
|--------|---------|---------|
| `CLAUDE.md` | Project instructions | `.orqa/team/agents/orchestrator.md` + project rules |
| `.claude/rules/*.md` | Rule files | `.orqa/governance/rules/` (if not already canon) |
| `.claude/settings.json` | Hooks, permissions | `.orqa/governance/hooks/` |
| `AGENTS.md` | Cross-agent instructions | Agent skill content |

**Coexistence strategy:** `.claude/` becomes a symlink layer pointing to `.orqa/`. Claude Code reads `.claude/CLAUDE.md` which symlinks to `.orqa/team/agents/orchestrator.md`. Both tools read the same source of truth.

### Cursor

| Source | Content | Maps To |
|--------|---------|---------|
| `.cursorrules` | Project-wide instructions | `.orqa/governance/rules/` (extracted as individual rules) |
| `.cursor/rules/*.md` | Rule files | `.orqa/governance/rules/` |

**Coexistence strategy:** OrqaStudio can generate `.cursorrules` from its governance rules. The `.cursorrules` file becomes a generated artifact, not a source of truth.

### GitHub Copilot

| Source | Content | Maps To |
|--------|---------|---------|
| `.github/copilot-instructions.md` | Instructions | `.orqa/governance/rules/` (extracted as individual rules) |

**Coexistence strategy:** OrqaStudio can generate `copilot-instructions.md` from its governance rules.

### Aider

| Source | Content | Maps To |
|--------|---------|---------|
| `.aider.conf.yml` | Configuration | Project settings in `project.json` |
| `.aider.model.settings.yml` | Model config | Model settings in `project.json` |
| `CONVENTIONS.md` | Conventions | `.orqa/governance/rules/` (extracted as individual rules) |

**Coexistence strategy:** OrqaStudio can generate convention files from its governance rules.

## Migration Procedure

1. **Detect** — Run `project-inference` to find which tools are configured
2. **Read** — Parse each tool's configuration files
3. **Classify** — For each piece of content, determine:
   - Is this a rule? → `.orqa/governance/rules/`
   - Is this agent instructions? → Agent definition or skill content
   - Is this a hook/automation? → `.orqa/governance/hooks/`
   - Is this project settings? → `project.json`
4. **Deduplicate** — Check if the content already exists in canon rules/skills
5. **Create** — Write the extracted governance artifacts
6. **Link** — Set up symlinks/generation for coexistence
7. **Report** — Show the user what was migrated and what needs manual review

## Content Extraction Patterns

### From Monolithic Instructions to Individual Rules

Many tools use a single large instruction file. Break it into individual rules:

```
CLAUDE.md contains:
  "Always use TypeScript strict mode"     → rule: typescript-strict.md
  "Never use console.log in production"   → rule: no-console-log.md
  "Run tests before committing"           → rule: pre-commit-tests.md
```

Each extracted rule gets proper YAML frontmatter with id, title, description, scope.

### From Implicit to Explicit

Instructions like "be careful with..." become explicit rules with clear PASS/FAIL criteria. Vague guidance becomes structured governance.

## Governance Hub Mode

When OrqaStudio manages governance for a project that uses multiple AI tools:

1. `.orqa/` is the single source of truth for all governance
2. Tool-specific configs are generated from `.orqa/` content
3. Changes flow: `.orqa/` → generated configs → tools read their native format
4. A file watcher (future) can auto-regenerate when `.orqa/` changes

## Critical Rules

- NEVER delete existing tool configurations during migration — coexistence, not replacement
- NEVER assume the user wants to migrate everything — present what was found and ask
- Always preserve the original file content as a reference (in comments or a migration log)
- If content doesn't clearly map to a governance artifact, flag it for manual review
- Confidence levels: high (clear mapping), medium (reasonable guess), low (needs user decision)
