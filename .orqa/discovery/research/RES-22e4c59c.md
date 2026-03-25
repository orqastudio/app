---
id: "RES-22e4c59c"
type: research
title: "README and Schema Audit — Canonical Definitions and Auto-Generation"
description: "Audit of all README.md and schema.json files for accuracy, duplication, canonical definition ownership, and auto-generation feasibility"
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
sources:
  - "url: \"file://.orqa/**/README.md"
  - "url: \"file://.orqa/**/schema.json"
  - "url: \"file://.orqa/documentation/about/artifact-framework.md"
relationships:
  - target: "EPIC-770f9ce9"
    type: "guides"
    rationale: "Research findings informed the design of Artifact Graph Alignment Audit"
---
## Findings

### Triple Source of Truth Problem

Artifact type definitions live in three places that already disagree:

| Source | Role | Status |
|--------|------|--------|
| `schema.json` | Machine-readable truth, pre-commit validation | Authoritative for field shapes |
| `README.md` | Human-readable guide, navigation metadata | Drifts from schema |
| `artifact-framework.md` | Canonical prose definitions per [RULE-b10fe6d1](RULE-b10fe6d1) | Most comprehensive but not machine-readable |

**Example disagreement (Tasks):**
- README says `epic` is required; schema says optional
- README omits `docs` field entirely; schema defines it
- CLAUDE.md references `scope` field; schema doesn't have it (removed tonight)

### README Accuracy Issues

| README | Issue |
|--------|-------|
| Planning group | Mentions "plans" as artifact type (deprecated) |
| Documentation group | Uses web-style link paths (`/product/`, `/architecture/`) violating RULE-2f7b6a31; lists "Research" as doc section (lives under Planning) |
| Skills | Says `chunkhound` is universal; should be `orqa-code-search` per [RULE-dd5b69e6](RULE-dd5b69e6) |
| Tasks | Field table duplicates and diverges from schema |
| Lessons | Lifecycle prose uses process names, not actual status enum values |
| Decisions | Doesn't mention `layer` field or `proposed` status |

### Schema Inventory

All 11 schemas have `propertyOrder`. 9 of 11 have `bodyTemplate` sections. Field descriptions exist on some schemas but not all.

| Schema | bodyTemplate? | Field descriptions? |
|--------|--------------|-------------------|
| epics | Yes (4 sections) | No |
| pillars | Yes (4 sections) | gate has description |
| milestones | Yes (3 sections) | No |
| ideas | Yes (2 sections) | No |
| tasks | Yes (3 sections) | docs, skills have descriptions |
| research | No | sources has description |
| lessons | Yes (2 sections) | No |
| decisions | Yes (3 sections) | No |
| rules | Yes (2 sections + notes) | capabilities, scope, enforcement |
| agents | No | capabilities |
| skills | No | scope, file-patterns |

### Auto-Generation Feasibility

**Can be generated from schema:**
- Field reference table (properties + required + propertyOrder)
- Valid status/priority values (enum arrays)
- YAML frontmatter template (required fields + defaults)
- Body section template (bodyTemplate)
- Relationship map (fields with `^PREFIX-\d+$` patterns)

**Must be human-authored:**
- Opening paragraph (what the artifact IS and why)
- Lifecycle explanation (what status transitions mean)
- Process guidance ("What Makes a Good X")
- Anti-patterns and examples

**Verdict:** Hybrid approach — auto-generate field reference, human-author process guidance. This eliminates the Tasks README's 30+ lines of field documentation that duplicate (and diverge from) the schema.

### README Role Recommendation

READMEs should serve exactly two roles:
1. **Navigation metadata** (frontmatter: role, label, icon, sort) — consumed by artifact scanner
2. **Human authoring guide** (body: what, why, lifecycle, process) — consumed by agents/humans

They should NOT serve as field reference — that duplicates and diverges from schema.json.

### Canonical Definition Location

**Recommendation:** `schema.json` is the single source of truth for field shapes. `artifact-framework.md` is the single source of truth for lifecycle and process semantics. READMEs provide a quick orientation but defer to both. No README should contain a field table.

## Recommendations

1. Remove field table from Tasks README (replace with "See schema.json")
2. Add `docs` field awareness or remove field tables entirely
3. Fix Planning README — remove "plans" mention
4. Fix Documentation README — fix link format, remove Research reference
5. Fix Skills README — `chunkhound` → `orqa-code-search`
6. Add actual status enum values to Lessons README
7. Add field descriptions to all schema properties (enables future auto-generation)
8. Consider a `make readme-gen` target that builds field reference sections from schemas
9. Establish convention: schema = fields truth, artifact-framework = process truth, README = orientation