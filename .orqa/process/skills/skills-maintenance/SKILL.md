---
id: SKILL-029
title: Skills Maintenance
description: |
  Full skills.sh lifecycle management: CLI reference, skill evaluation criteria, SKILL.md format,
  portability rules, provenance tracking, and skill audit protocol.
  Use when: Adding, updating, removing, or auditing agent skills; evaluating whether to install
  a community skill or create a project-specific one; managing the .orqa/process/skills/ directory.
status: active
created: 2026-03-01
updated: 2026-03-10
layer: core
category: methodology
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Skill evaluation, portability rules, and provenance tracking ensure domain knowledge is curated, versioned, and discoverable
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
---


Agent skills are portable domain knowledge that agents load on demand. This skill covers the full lifecycle of managing skills: discovering them, installing them, evaluating their portability, updating them, and removing them when no longer needed.

## Quick Start

### CLI Reference (skills.sh)

```bash
# Search for skills by domain or keyword
npx skills find [query]

# Install a skill (auto-confirm, project scope)
npx skills add <owner/repo@skill-name> --copy -y

# List all installed skills
npx skills list

# Check for updates to installed skills
npx skills check

# Update all installed skills
npx skills update

# Remove a skill
npx skills remove [skill-name]

# Initialize a new empty skill
npx skills init [skill-name]
```

**Installation flags:**

- `--copy`: Copy files to `.orqa/process/skills/` instead of symlinking (preferred for this project)
- `-y` or `--yes`: Skip confirmation prompts
- `-g` or `--global`: Global scope — installs to `~/.claude/skills/`, available across all projects

Always use project scope (default) with `--copy` for skills relevant to the project.

## Key Concepts

| Concept | Description |
|---------|-------------|
| Skill | A `SKILL.md` file with YAML frontmatter — portable domain knowledge for agents |
| Project skill | In `.orqa/process/skills/` — committed with the codebase, shared by all agents |
| Global skill | In `~/.claude/skills/` — personal, not committed, available everywhere |
| Registry skill | Installed from skills.sh ecosystem via `npx skills add` |
| Custom skill | Created manually in `.orqa/process/skills/<name>/SKILL.md` |
| Provenance | Where a skill came from: skills.sh, custom-created, or downloaded+modified |
| Portability | A skill is portable if it contains no project-specific paths, IDs, or rules |

## SKILL.md Format

Every skill is a `SKILL.md` file in a directory under `.orqa/process/skills/`:

```markdown
---
name: skill-name
description: |
  One-paragraph description of what the skill covers.
  Use when: [trigger conditions for loading this skill]
version: 1.0.0
tags: [tag1, tag2, tag3]
user-invocable: true
---

# Skill Title

[Introduction paragraph]

## Quick Start
[Minimal working example]

## Key Concepts
[Table of concepts]

## Common Patterns
[Practical patterns and examples]

## See Also
[External references]

## Related Skills
[Cross-references to other skills]
```

### YAML Frontmatter Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Skill identifier (kebab-case, matches directory name) |
| `description` | Yes | What it covers + "Use when:" trigger conditions |
| `version` | Recommended | SemVer version of this skill |
| `tags` | Recommended | Searchable keywords |
| `user-invocable` | Recommended | `true` if users can invoke with `Skill(name)`, `false` if agent-only |
| `allowed-tools` | Optional | MCP tools this skill grants access to |

## Skill Evaluation Criteria

When deciding whether to install a community skill or create a new one:

### Install from skills.sh When

- The domain is well-established (Rust, Svelte, Tailwind, TypeScript)
- The skill covers a technology not specific to this project
- The source is reputable (high install count, known maintainer)
- The skill requires minimal or no modification for this project

### Create a Project Skill When

- The domain is project-specific (the project's own patterns, conventions, entity types)
- No community skill exists for the domain
- An existing community skill would require major modification (better to fork)
- The knowledge is specific to the project's architecture or workflow

### Skip (Neither Install Nor Create) When

- The domain is adequately covered by an existing skill
- The knowledge belongs in project documentation (not portable enough)
- The skill would duplicate content already in `.orqa/process/rules/` or docs

## Portability Rules (NON-NEGOTIABLE)

See [RULE-035](RULE-035) for the full skill portability constraint and enforcement.

A skill is portable if a different project could use it unchanged. Skills MUST NOT contain:

- Project-specific file paths (e.g., `backend/src-tauri/src/domain/sessions.rs`)
- Architecture decision numbers from this project (e.g., `[AD-001](AD-001)`, `[AD-005](AD-005)`)
- Project-specific config values (hardcoded URLs, service names, environment variables)
- Enforcement rules (those belong in `.orqa/process/rules/`)
- Product decisions (those belong in `.orqa/documentation/about/`)
- Implementation patterns specific to this codebase's conventions

**Test:** Would this skill be useful on a different project? If yes, it's portable. If no, move the content to project docs or rules.

## Provenance Tracking

Every installed skill is tracked through its SKILL.md frontmatter (YAML metadata including layer, category, relationships). Key tracking fields:

| Field | Description |
|-------|-------------|
| Name | Skill identifier |
| Source | `skills.sh/<owner>/<repo>@<name>` / `custom` / `downloaded+modified` |
| Purpose | Why it was installed |
| Date Added | ISO date |
| Portability | `portable` / `modified` / `project-specific` |

## Skill Lifecycle

```text
Discovery → Evaluation → Installation → Loading → Use → Update → Deprecation → Removal
```

| Phase | Action |
|-------|--------|
| Discovery | `npx skills find [query]` |
| Evaluation | Check portability, source reputation, relevance |
| Installation | `npx skills add <source> --copy -y` |
| Loading | Agent loads via `Skill(name)` |
| Update | `npx skills check` → `npx skills update` |
| Deprecation | Add deprecation note to SKILL.md, update skills log |
| Removal | `npx skills remove <name>`, update skills log |

## Skill Audit Protocol

Run a full skills audit periodically (or when the agent-maintainer role is triggered):

### Step 1: Inventory

```bash
npx skills list
ls .orqa/process/skills/
```

Verify: every directory in `.orqa/process/skills/` has a SKILL.md, and every entry in the skills log matches an installed skill.

### Step 2: Portability Check

For each skill, verify it contains no project-specific content (see Portability Rules above).

### Step 3: Currency Check

```bash
npx skills check
```

Review the output. For each skill with an available update, review the changelog and update if safe.

### Step 4: Relevance Check

For each skill, ask:

- Is this skill still being loaded by at least one agent?
- Has the domain become obsolete or replaced?
- Is there a better community skill available?

Deprecate or remove skills that are no longer relevant.

## See Also

- [skills.sh](https://skills.sh) — Open agent skills ecosystem
- [Agent Skills Standard](https://agentskills.io) — SKILL.md specification

## Related Skills

- See the **chunkhound** skill for code search (frequently used during skill audits)
- See the **architecture** skill for evaluating whether skill content is portable
