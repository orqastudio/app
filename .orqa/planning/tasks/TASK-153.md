---
id: TASK-153
title: Create schema compliance scanning skill and tools
description: Create a skill for schema compliance scanning methodology and design tools that can validate artifact frontmatter against schemas at any time, not just at commit time.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on:
  - TASK-152
scope:
  - Create a schema-compliance skill covering validation methodology, common violations, and remediation patterns
  - Design a validation tool (Tauri command or script) that scans all artifacts for schema compliance on demand
  - Tool reports all violations across the entire .orqa/ directory in one pass
  - Includes propertyOrder enforcement — flags fields out of schema-defined order
  - Supports both CLI usage (script) and app usage (Tauri command)
acceptance:
  - Schema compliance skill created in .orqa/team/skills/
  - On-demand validation tool exists that scans all .orqa/ artifacts
  - Tool output groups violations by type (missing field, invalid value, pattern mismatch, field ordering, missing body section)
  - Tool can be run standalone or integrated into the app dashboard
  - Skill documents common violation patterns and how to fix them
---
## What

Schema validation currently only runs at commit time (pre-commit hook). Agents and users need the ability to scan all artifacts for compliance at any time — during audits, after bulk edits, or before starting work. This requires both a skill (teaching agents how to think about schema compliance) and a tool (running the actual validation).

## How

1. Create `.orqa/team/skills/schema-compliance/SKILL.md` covering: schema discovery (how schemas map to directories), common violation patterns (missing required fields, invalid enums, malformed IDs, field ordering violations, missing body sections), remediation patterns, and how to use the validation tools
2. Create or enhance a validation script (Node.js with ajv) that walks all `.orqa/` directories, discovers `schema.json` files, validates every `.md` file against its schema, and reports all violations
3. Design the output format: grouped by violation type, with file path, field name, expected vs actual value
4. Plan a Tauri command wrapper so the app can surface compliance status in the dashboard
5. Document in the skill how agents should use the tool before and after bulk artifact modifications

## Verification

- [ ] Schema compliance skill created with methodology and common patterns
- [ ] Validation script scans all .orqa/ artifacts in one pass
- [ ] Output clearly groups violations by type with actionable fix information
- [ ] Script handles missing schema.json gracefully (skip directory with warning)
- [ ] propertyOrder enforcement included — flags out-of-order fields
- [ ] Body template section validation included (not just frontmatter)
