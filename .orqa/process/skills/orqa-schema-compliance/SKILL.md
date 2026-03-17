---
id: SKILL-039
title: Schema Compliance
description: |
  Schema compliance validation methodology: schema discovery, common violation
  patterns, remediation, and on-demand scanning tools.
  Use when: Creating or modifying .orqa/ artifacts, running compliance audits,
  fixing schema validation errors, or batch-editing frontmatter.
status: active
created: 2026-03-11
updated: 2026-03-11
layer: project
category: methodology
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: JSON Schema validation of frontmatter ensures every artifact conforms to its type contract, preventing malformed graph edges
  - target: TASK-172
    type: informs
  - target: TASK-183
    type: informs
  - target: TASK-409
    type: informs
  - target: TASK-474
    type: informs
  - target: AGENT-008
    type: informs
  - target: PILLAR-001
    type: informs
---

How to create valid `.orqa/` artifacts and fix schema validation failures. Enforced by [RULE-032](RULE-032) via the pre-commit hook at `.githooks/validate-schema.mjs`.

## Schema Discovery

Each artifact type directory contains a `schema.json` (JSON Schema format). The validator discovers schemas by walking the `artifacts` array in `.orqa/project.json`.

| Directory | Schema governs |
|-----------|---------------|
| `.orqa/process/rules/` | Rules (RULE-NNN) |
| `.orqa/process/decisions/` | Architecture decisions (AD-NNN) |
| `.orqa/process/lessons/` | Implementation lessons (IMPL-NNN) |
| `.orqa/delivery/epics/` | Epics (EPIC-NNN) |
| `.orqa/delivery/tasks/` | Tasks (TASK-NNN) |
| `.orqa/delivery/ideas/` | Ideas (IDEA-NNN) |
| `.orqa/delivery/research/` | Research docs (RES-NNN) |
| `.orqa/delivery/milestones/` | Milestones (MS-NNN) |
| `.orqa/process/pillars/` | Pillars (PILLAR-NNN) |
| `.orqa/process/agents/` | Agent definitions |
| `.orqa/process/skills/` | Skill definitions |

Before creating any artifact, read the `schema.json` in its target directory for:
- Required fields (`"required"` array)
- Valid enum values for `status`, `priority`, `layer`
- Field ordering (`propertyOrder`)
- Body section requirements (`bodyTemplate.sections`)

## Common Violation Patterns

### 1. Missing Required Fields

The schema's `"required"` array lists fields that must be present.

```yaml
# WRONG — missing required fields 'created', 'updated', 'scope'
id: RULE-NNN
title: My Rule
status: active
layer: core
```

```yaml
# CORRECT
id: RULE-NNN
title: My Rule
description: One-line summary of what this rule enforces.
status: active
created: "2026-03-11"
updated: "2026-03-11"
layer: core
```

**Error message:** `(root) — must have required property 'created'`

### 2. Invalid Enum Values

Enum fields only accept values listed in the schema. Each artifact type has its own valid set.

```yaml
# WRONG — 'complete' is not a valid task status
status: complete

# CORRECT for tasks
status: done
```

**Error message:** `/status — must be equal to one of the allowed values (todo, in-progress, done)`

### 3. Malformed IDs

Every artifact type has an ID pattern enforced by regex.

```yaml
# WRONG — wrong zero-padding
id: EPIC-N
id: TASK-NN

# CORRECT — three-digit minimum
id: EPIC-001
id: TASK-005
id: TASK-153
```

**Error message:** `/id — must match pattern "^EPIC-\d{3,}$"`

To find the next available ID, scan existing files:
```bash
ls .orqa/delivery/tasks/TASK-*.md | sort | tail -1
```

### 4. Field Ordering Violations (propertyOrder)

Frontmatter fields must appear in the order defined by `propertyOrder` in `schema.json`. This is the most common violation when fields are added by hand.

```yaml
# WRONG — 'layer' and 'scope' appear before 'created'/'updated'
id: RULE-NNN
title: My Rule
description: ...
status: active
layer: core
created: "2026-03-11"
updated: "2026-03-11"
```

```yaml
# CORRECT — follows rule propertyOrder: id, title, description, status, created, updated, layer, scope
id: RULE-NNN
title: My Rule
description: ...
status: active
created: "2026-03-11"
updated: "2026-03-11"
layer: core
```

**Error message:**
```
field order does not match schema propertyOrder
  expected: id, title, description, status, created, updated, layer, scope
  actual:   id, title, description, status, layer, scope, created, updated
```

**Fix:** Reorder the YAML block to match the expected sequence. Only reorder — do not change values.

### 5. Missing Body Sections

`bodyTemplate.sections` lists required `## Heading` sections. The validator checks for their presence in the body (everything after the closing `---`).

```markdown
<!-- WRONG for a task — missing "## What" and "## How" -->
---
...frontmatter...
---

Implementation notes here with no section headings.
```

```markdown
<!-- CORRECT -->
---
...frontmatter...
---

## What
Describe what this task delivers.

## How
Implementation approach.

## Verification
How to confirm it's done.
```

**Error message:** `missing required section "## What"`

### 6. Unknown Fields

`"additionalProperties": false` means any field not in the schema's `properties` is rejected.

```yaml
# WRONG — 'tags' is not in the task schema
tags:
  - backend
  - rust
```

**Error message:** `(root) — unknown field 'tags'`

## Remediation Patterns

| Violation | Action |
|-----------|--------|
| Missing required field | Add the field with a valid value |
| Invalid enum value | Replace with an allowed value from the schema |
| Malformed ID | Fix zero-padding to match the pattern |
| Wrong field order | Reorder the YAML block to match `propertyOrder` |
| Missing body section | Add the `## Heading` section with content |
| Unknown field | Remove the field, or check if the schema needs updating |

## On-Demand Validation

Run the validator manually to check files before committing:

```bash
# Validate a single file
node .githooks/validate-schema.mjs .orqa/delivery/tasks/TASK-153.md

# Validate all files in a directory
find .orqa/process/rules -name '*.md' -not -name 'README.md' | \
  xargs node .githooks/validate-schema.mjs

# Validate all .orqa/ artifacts at once
find .orqa -name '*.md' -not -name 'README.md' | \
  xargs node .githooks/validate-schema.mjs
```

Exit code 0 = all valid. Exit code 1 = errors found (details printed to stderr).

The pre-commit hook runs automatically on staged `.orqa/**/*.md` files — no manual invocation needed for normal commits.

## Pre-Creation Checklist

Before creating any `.orqa/` artifact:

- [ ] Read the `schema.json` in the target directory
- [ ] Identify all required fields and add them
- [ ] Use only valid enum values for `status`, `priority`, `layer`, etc.
- [ ] Determine the next available ID (scan existing files)
- [ ] Follow the `propertyOrder` when writing frontmatter
- [ ] Include all required body sections from `bodyTemplate.sections`
- [ ] Run `node .githooks/validate-schema.mjs <file>` before staging

## Related Rules

- [RULE-032](RULE-032) — Schema compliance enforcement rule
- [RULE-004](RULE-004) — Artifact lifecycle and status transitions
- [RULE-027](RULE-027) — Structure must exist before implementation begins
