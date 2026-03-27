# Migration Task List: Phases 6-8

**Generated:** 2026-03-26
**Inputs:** ARCHITECTURE.md section 13, phase2-01-governance-gaps.md, phase2-05-root-infra-gaps.md, phase2-06-proposed-restructure.md, 03-orqa-process.md, 04-orqa-delivery-discovery.md

---

## Phase 6: Content Cleanup (Zero Dead Weight)

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Every deletion and content change must be validated against ARCHITECTURE.md to confirm alignment.

### 6.1 — Delete completed migration scripts from `scripts/`

**What:** Delete the following 6 files from `scripts/`:
- `scripts/migrate-artifact-ids.mjs`
- `scripts/standardise-ids.mjs`
- `scripts/fix-duplicate-frontmatter-keys.mjs`
- `scripts/fix-missing-inverses.mjs`
- `scripts/link-skills-to-docs.mjs`
- `scripts/monorepo-merge.sh`

**Keep:** `scripts/install.sh`, `scripts/sync-versions.sh`, `scripts/link-all.sh`, `scripts/validate-artifacts.mjs` (active/needed)

**Acceptance Criteria:**
- [ ] All 6 listed files are deleted from `scripts/`
- [ ] `scripts/install.sh`, `scripts/sync-versions.sh`, `scripts/link-all.sh`, `scripts/validate-artifacts.mjs` remain untouched
- [ ] No broken references to deleted scripts in Makefile, package.json, or CLAUDE.md

**Reviewer Checks:**
- Verify each file is actually a completed one-time migration (not still referenced anywhere)
- Grep the codebase for any remaining imports/references to the deleted script names
- Verify the 4 kept scripts are still valid

---

### 6.2 — Delete migration manifest files from `scripts/`

**What:** Delete the following 2 files from `scripts/`:
- `scripts/id-migration-manifest.json`
- `scripts/id-standardise-manifest.json`

These are output artifacts of completed migrations. Not needed at repo root.

**Acceptance Criteria:**
- [ ] Both files deleted
- [ ] No code references these manifest files

**Reviewer Checks:**
- Grep for `id-migration-manifest` and `id-standardise-manifest` across the codebase

---

### 6.3 — Delete completed migration scripts from `tools/`

**What:** Delete the following 2 files from `tools/`:
- `tools/remove-inverse-relationships.mjs`
- `tools/migrate-types.mjs`

**Keep:** `tools/debug/` (dev dashboard — active)

**Acceptance Criteria:**
- [ ] Both files deleted from `tools/`
- [ ] `tools/debug/dev.mjs` and `tools/debug/dev-dashboard.html` remain untouched
- [ ] No broken references to deleted scripts

**Reviewer Checks:**
- Grep for `remove-inverse-relationships` and `migrate-types` across the codebase
- Verify `tools/debug/` is intact

---

### 6.4 — Delete completed migration scripts from `app/scripts/`

**What:** Delete the following 3 files from `app/scripts/`:
- `app/scripts/rebuild-artifacts.mjs`
- `app/scripts/migration-manifest.json`
- `app/scripts/rewire-icons.mjs`

**Acceptance Criteria:**
- [ ] All 3 files deleted
- [ ] No references to these files in app/package.json scripts or elsewhere

**Reviewer Checks:**
- Check `app/package.json` for script references to deleted files
- Grep codebase for `rebuild-artifacts`, `migration-manifest`, `rewire-icons`

---

### 6.5 — Delete completed migration/backfill scripts from `app/tools/`

**What:** Delete the following files from `app/tools/` that are one-time migration/backfill scripts:
- `app/tools/apply-decision-backfill.mjs`
- `app/tools/apply-lesson-backfill.mjs`
- `app/tools/apply-rule-backfill.mjs`
- `app/tools/apply-skill-backfill.mjs`
- `app/tools/backfill-relationships.mjs`
- `app/tools/fix-lesson-order.mjs`
- `app/tools/migrate-deprecated-fields.mjs`
- `app/tools/path-manifest.json`

**Keep** (active verification/dev tools):
- `app/tools/check-cognitive-load.mjs`
- `app/tools/check-orientation.mjs`
- `app/tools/audit-enforcement-gaps.mjs`
- `app/tools/verify-enforcement-rules.mjs`
- `app/tools/verify-pipeline-integrity.mjs`
- `app/tools/verify-scope-drift.mjs`
- `app/tools/lib/` (shared utilities)

**Note:** `app/tools/verify-links.mjs` was listed in the gap analysis but does NOT exist on disk — no action needed.

**Acceptance Criteria:**
- [ ] All 8 listed files deleted
- [ ] All 6 kept files + lib/ remain untouched
- [ ] No broken imports in kept files that referenced deleted files

**Reviewer Checks:**
- Verify each deleted file is truly a one-time operation (check file contents for migration/backfill indicators)
- Check that kept `app/tools/lib/` does not import from deleted files
- Grep for references to deleted file names

---

### 6.6 — Delete legacy root files

**What:** Delete the following files from the repo root:
- `validation_stderr.txt` (debug output, not a source file)

**Acceptance Criteria:**
- [ ] File deleted
- [ ] File is already gitignored (verify before deleting — if tracked, need a commit)

**Reviewer Checks:**
- Verify the file is gitignored (if not, the deletion needs to be committed)
- Check that no script writes to this specific path (may need to redirect output elsewhere)

---

### 6.7 — Delete `tmp/` directory

**What:** Delete the `tmp/` directory at the repo root. Superseded by `.state/` per AD-8727f99a.

**Acceptance Criteria:**
- [ ] `tmp/` directory deleted
- [ ] No code references `tmp/` as a write destination (all should use `.state/`)

**Reviewer Checks:**
- Grep codebase for `tmp/` references (excluding node_modules, .git) — any hits must be updated to `.state/`
- Verify `.state/` is correctly gitignored

---

### 6.8 — Delete `app/WORKING-DOCUMENT.md`

**What:** Delete `app/WORKING-DOCUMENT.md`. Legacy working document — content has been superseded by ARCHITECTURE.md.

**Acceptance Criteria:**
- [ ] File deleted
- [ ] No references to this file in CLAUDE.md or other configuration

**Reviewer Checks:**
- Grep for `WORKING-DOCUMENT` across the codebase

---

### 6.9 — Fix CLI version hardcoding

**What:** In `libs/cli/src/cli.ts`, lines 28 and 66 hardcode `0.1.0-dev`. Update to read version from `package.json` dynamically.

**Files:**
- `libs/cli/src/cli.ts` (lines 28, 66)

**Acceptance Criteria:**
- [ ] Version string in CLI banner (line 28) reads from `package.json`
- [ ] Version string in `--version` output (line 66) reads from `package.json`
- [ ] Running `orqa --version` outputs `0.1.4-dev` (matching package.json)
- [ ] No hardcoded version strings remain in `cli.ts`

**Reviewer Checks:**
- Verify the version import mechanism (createRequire, fs.readFileSync, or import assertion)
- Run `orqa --version` and confirm output matches `package.json`

---

### 6.10 — Remove legacy CLI aliases

**What:** Remove the 8 legacy command aliases from `libs/cli/src/cli.ts` that route old command names to new locations: `setup`, `link`, `verify`, `audit`, `enforce`, `repo`, `hosting`, `index`, `log`.

Per architecture: "No backwards compatibility -- pre-release, breaking changes expected."

**Files:**
- `libs/cli/src/cli.ts` (alias definitions)

**Acceptance Criteria:**
- [ ] All legacy command aliases removed from CLI
- [ ] Only current, documented commands remain
- [ ] CLI help output (`orqa --help`) shows no legacy aliases
- [ ] No backwards-compatibility shims remain

**Reviewer Checks:**
- Run `orqa --help` and verify only current commands appear
- Grep `cli.ts` for "alias", "legacy", "deprecated"
- Verify no other files reference the removed alias names as CLI commands

---

### 6.11 — .state/team/ cleanup — delete empty directories

**What:** Delete the following empty team directory:
- `.state/team/fix-sot/` (confirmed empty — 0 files)

Scan for and delete any other empty team directories.

**Acceptance Criteria:**
- [ ] `fix-sot/` deleted
- [ ] All other empty directories in `.state/team/` deleted
- [ ] Non-empty team directories remain untouched

**Reviewer Checks:**
- Run `find .state/team/ -type d -empty` to verify no empty directories remain
- Verify no team directory with findings files was accidentally deleted

---

### 6.12 — .state/team/ cleanup — prune stale team directories

**What:** Review all 40+ team directories in `.state/team/`. For each directory:
1. Check if it contains findings that have already been committed (the work is done)
2. If all findings are committed/stale, delete the directory
3. If any findings contain unpromoted valuable content, document it for promotion

Current directories (40+): ac-completion, ad-fixes, agent-lifecycle, audit, audit-fixes, audits, cleanup, content-migration, dedup-epic, enforcement, enforcement-targets, epic-rewrite, final-gate, fix-pass, fixes-and-audits, graph-foundation, graph-foundation-p2, graph-perf, graph-relationships-fix, human-gates, implementer, migration-planning, phase1-decomposition, port-epic, port-fix, post-migration-fixes, prompt-pipeline, remaining-fixes, remediation, research, session-priorities, sot-fixes, statusbar-fix, target-plugin, target-schema, task-creation, team-design-research, validator-fix, workflow-engine, zero-warnings

Also: `.state/team/restructure-findings.md` (stale file, not a directory — delete)

**Acceptance Criteria:**
- [ ] All stale team directories (work already committed) are deleted
- [ ] `restructure-findings.md` (file, not directory) deleted if stale
- [ ] Any valuable unpromoted content is documented in a list before deletion
- [ ] `.state/team/` contains only actively-needed directories (likely zero)

**Reviewer Checks:**
- For each deleted directory, verify the associated work was committed to git
- Verify no directory contains findings that were never acted on
- Check git log for commits referencing each team name

---

### 6.13 — Review and clean documentation artifacts (20 root files)

**What:** Review all 20 DOC files in `.orqa/documentation/` (root level, not in platform/ or project/):

```
DOC-13c73ecf.md  DOC-1f4aba8f.md  DOC-22783288.md  DOC-2372ed36.md
DOC-4554ff3e.md  DOC-586bfa9a.md  DOC-7062bce9.md  DOC-7068f40a.md
DOC-8cf6ef38.md  DOC-9505a5b5.md  DOC-a06f2a63.md  DOC-a16b7bc7.md
DOC-af962d42.md  DOC-db794473.md  DOC-dd5062c9.md  DOC-e16aea3b.md
DOC-e89753ad.md  DOC-ecc181cb.md  DOC-f0a1c9b5.md  DOC-fd1d12bb.md
```

For each file:
1. Read and assess accuracy against current architecture
2. Check for duplication with KNOW- files (e.g., DOC-586bfa9a shares hash with KNOW-586bfa9a)
3. Determine: keep (update if stale), archive (no longer relevant), merge (duplicate)
4. Fix frontmatter: ensure `title` (not `name`), add `status` if missing
5. Assign to a topic subdirectory (architecture, reference, how-to, onboarding, concept)

**Acceptance Criteria:**
- [ ] Every file reviewed and disposition documented
- [ ] All stale/inaccurate content updated or archived
- [ ] All duplicates between DOC and KNOW resolved (one copy, not two)
- [ ] All files use `title` field (not `name`)
- [ ] All files have a `status` field
- [ ] Report produced listing each file, its disposition, and its target subdirectory

**Reviewer Checks:**
- Verify each keep/archive/merge decision is justified
- Spot-check 5 files for accuracy of the review
- Verify no DOC/KNOW duplicates remain

---

### 6.14 — Review and clean documentation artifacts (37 platform/ files)

**What:** Review all 37 DOC files in `.orqa/documentation/platform/`. Same review process as task 6.13.

For each file:
1. Assess accuracy against current architecture
2. Check for duplication
3. Fix frontmatter (title, status)
4. Reassign to topic subdirectory if the platform/ categorization is insufficient

**Acceptance Criteria:**
- [ ] Every file reviewed and disposition documented
- [ ] Stale content updated or archived
- [ ] Frontmatter standardized
- [ ] Report produced

**Reviewer Checks:**
- Spot-check 5 files for accuracy
- Verify frontmatter compliance

---

### 6.15 — Review and clean documentation artifacts (34 project/ files)

**What:** Review all 34 DOC files in `.orqa/documentation/project/`. Same review process as tasks 6.13/6.14.

**Acceptance Criteria:**
- [ ] Every file reviewed and disposition documented
- [ ] Stale content updated or archived
- [ ] Frontmatter standardized
- [ ] Report produced

**Reviewer Checks:**
- Spot-check 5 files for accuracy
- Verify frontmatter compliance

---

### 6.16 — Review and clean knowledge artifacts (batch 1: KNOW-0* through KNOW-3*)

**What:** Review 29 knowledge files (KNOW-0188373b through KNOW-3f307edb). For each:
1. Assess content accuracy against current architecture
2. Check for duplication with DOC files or other KNOW files
3. Verify/add injection metadata: `tier`, `roles`, `paths`, `tags`, `summary`, `priority`
4. Fix frontmatter: ensure `title` (not `name`), add `status` if missing
5. Assign to a domain category for later directory reorganization: platform, development/rust, development/frontend, development/typescript, methodology, documentation, plugin, integration, standards, project
6. Flag files that are too large (>2000 tokens) for splitting

**Files (29):**
```
KNOW-0188373b  KNOW-03421ec0  KNOW-0444355f  KNOW-0619a413  KNOW-0d6c1ece
KNOW-126aa140  KNOW-1314ac47  KNOW-13348442  KNOW-16e91c20  KNOW-1a4f41f7
KNOW-1afbc656  KNOW-1b7fa054  KNOW-1c2d005d  KNOW-1da7ecd8  KNOW-1ea9291c
KNOW-1f4aba8f  KNOW-207d9e2c  KNOW-21d28aa0  KNOW-22783288  KNOW-2876afc7
KNOW-2a846fb7  KNOW-2bf2b321  KNOW-2f38309a  KNOW-33b2dc14  KNOW-3642842e
KNOW-36befd20  KNOW-37496474  KNOW-3d946f9a  KNOW-3f307edb
```

**Acceptance Criteria:**
- [ ] Every file reviewed with disposition recorded
- [ ] All files have complete injection metadata (tier, roles, paths, tags, summary, priority)
- [ ] All files use `title` (not `name`)
- [ ] Domain category assigned to each file
- [ ] Duplicates identified and resolved
- [ ] Over-sized files flagged

**Reviewer Checks:**
- Verify injection metadata is semantically correct (not just present but meaningful)
- Spot-check 5 files for content accuracy
- Verify no `name` field remains (should be `title`)

---

### 6.17 — Review and clean knowledge artifacts (batch 2: KNOW-4* through KNOW-7*)

**What:** Review 31 knowledge files (KNOW-40be8113 through KNOW-7fadba3f). Same review process as 6.16.

**Files (31):**
```
KNOW-40be8113  KNOW-40e2eb99  KNOW-41849545  KNOW-4260613a  KNOW-45b5f8a8
KNOW-46f68631  KNOW-477f2c9c  KNOW-481059d2  KNOW-498ca38a  KNOW-4a4241a5
KNOW-4a58e7dd  KNOW-4f81ddc5  KNOW-50382247  KNOW-51de8fb7  KNOW-5611351f
KNOW-5704b089  KNOW-57365826  KNOW-586bfa9a  KNOW-59077955  KNOW-5efbe925
KNOW-5f4db8f7  KNOW-60aefbbc  KNOW-694ff7cb  KNOW-6cfacbb2  KNOW-6d80cf39
KNOW-71352dc8  KNOW-72ca209f  KNOW-73490bde  KNOW-7a4e45d4  KNOW-7c871921
KNOW-7fadba3f
```

**Acceptance Criteria:**
- [ ] Same as 6.16

**Reviewer Checks:**
- Same as 6.16

---

### 6.18 — Review and clean knowledge artifacts (batch 3: KNOW-8* through KNOW-b*)

**What:** Review 32 knowledge files (KNOW-83039175 through KNOW-bf70068c). Same review process as 6.16.

**Files (32):**
```
KNOW-83039175  KNOW-8564d52c  KNOW-85a449e7  KNOW-85e392ea  KNOW-8615fee2
KNOW-882d8c4f  KNOW-8c359ea4  KNOW-8cc0f5e4  KNOW-8d1c4be6  KNOW-8d2e5eef
KNOW-8d76c3c7  KNOW-91a7a6c1  KNOW-936e5944  KNOW-96aaa407  KNOW-990e4f85
KNOW-9ff8c63f  KNOW-a0947420  KNOW-a16b7bc7  KNOW-a1a195c1  KNOW-a274d90d
KNOW-a3dcdd05  KNOW-a4e351bc  KNOW-a53d826c  KNOW-a700e25a  KNOW-abb08445
KNOW-afaa4e88  KNOW-b320cae8  KNOW-b5f520d5  KNOW-b95ec6e3  KNOW-be54e4de
KNOW-bec7e87d  KNOW-bf70068c
```

**Acceptance Criteria:**
- [ ] Same as 6.16

**Reviewer Checks:**
- Same as 6.16

---

### 6.19 — Review and clean knowledge artifacts (batch 4: KNOW-c* through KNOW-f*)

**What:** Review 22 knowledge files (KNOW-c4d3e52b through KNOW-fd636a56). Same review process as 6.16.

**Files (22):**
```
KNOW-c4d3e52b  KNOW-c89f28b3  KNOW-d00093e7  KNOW-d03337ac  KNOW-d13d80e1
KNOW-d4095bd9  KNOW-dd5062c9  KNOW-df3c489e  KNOW-e3432947  KNOW-e484802a
KNOW-e6fee7a0  KNOW-e89753ad  KNOW-ea7898e4  KNOW-ea78c8e4  KNOW-ecc181cb
KNOW-ee860ed9  KNOW-eeceaabf  KNOW-f5ee4e0d  KNOW-f7d03a2c  KNOW-f7fb7aa7
KNOW-fbc200e6  KNOW-fd636a56
```

**Acceptance Criteria:**
- [ ] Same as 6.16

**Reviewer Checks:**
- Same as 6.16

---

### 6.20 — Review and clean SKILL.md subdirectories in knowledge/

**What:** Review the 5 SKILL.md subdirectories in `.orqa/documentation/*/knowledge/` (migrated from former `.orqa/process/knowledge/`):
- `diagnostic-methodology/SKILL.md` — root cause analysis
- `governance-context/SKILL.md` — artifact graph reading
- `planning/SKILL.md` — documentation-first planning
- `plugin-setup/SKILL.md` — plugin setup for Claude Code
- `search/SKILL.md` — unified MCP search

For each:
1. Determine if the content is agent-internal knowledge or user-facing command
2. If knowledge: convert to standard KNOW-*.md files with proper frontmatter and injection metadata
3. If user-facing command: document for later migration to connector-generated skill definitions
4. Delete the SKILL.md file and subdirectory after conversion

**Acceptance Criteria:**
- [ ] All 5 SKILL.md files converted to standard KNOW-*.md format or documented as user-facing
- [ ] All 5 subdirectories removed (content promoted to flat KNOW-*.md files)
- [ ] Converted files have complete injection metadata
- [ ] No SKILL.md files remain in knowledge/

**Reviewer Checks:**
- Verify each conversion preserves all meaningful content
- Verify the skill/knowledge classification is correct per user's feedback (skills = user-facing, knowledge = agent-internal)
- Check that any user-invocable skills are documented for connector migration

---

### 6.21 — Review and clean decision artifacts (70 files)

**What:** Review all 70 AD-*.md files in `.orqa/learning/decisions/` and `.orqa/planning/decisions/` (migrated from former `.orqa/process/decisions/`). For each:
1. Assess accuracy against current architecture
2. Classify as `principle-decision` (architecture/approach) or `planning-decision` (implementation/tactical)
3. Determine: keep, archive (superseded/no longer applies), update (partially stale)
4. Fix frontmatter if needed

**Files:** All 70 AD-*.md files listed in 03-orqa-process.md section 4

**Acceptance Criteria:**
- [ ] Every decision reviewed with disposition recorded
- [ ] Each decision classified as principle or planning
- [ ] Superseded decisions archived (status changed to `archived`)
- [ ] Partially stale decisions updated for accuracy
- [ ] Classification report produced (which files go to `.orqa/learning/decisions/` vs `.orqa/planning/decisions/`)

**Reviewer Checks:**
- Spot-check 10 decisions for correct classification
- Verify archived decisions are truly superseded
- Verify no active decisions contradict ARCHITECTURE.md

---

### 6.22 — Review and clean lesson artifacts (84 files)

**What:** Review all 84 IMPL-*.md files in `.orqa/learning/lessons/` (migrated from former `.orqa/process/lessons/`). For each:
1. Assess ongoing relevance — archive lessons about superseded approaches
2. Identify lessons that can become mechanical guards (rules, validation checks, workflow guards)
3. Convert applicable lessons to enforcement immediately (per architecture: "no recurrence threshold needed at this stage")
4. Archive lessons that are no longer relevant

**Files:** All 84 IMPL-*.md files listed in 03-orqa-process.md section 5

**Acceptance Criteria:**
- [ ] Every lesson reviewed with disposition recorded
- [ ] Irrelevant/superseded lessons archived (status: `archived`)
- [ ] Lessons convertible to mechanical guards identified and converted (new RULE-*.md created or existing rule updated)
- [ ] Remaining active lessons verified as still valuable and not mechanically enforceable
- [ ] Report produced listing each lesson's disposition

**Reviewer Checks:**
- Spot-check 10 lessons for correct disposition
- Verify converted-to-rule lessons have corresponding RULE-*.md entries
- Verify no active lessons contradict current architecture

---

### 6.23 — Review and clean rule artifacts (59 files)

**What:** Review all 59 RULE-*.md files in `.orqa/learning/rules/` (migrated from former `.orqa/process/rules/`). For each:
1. Assess accuracy against current architecture
2. Classify as **mechanical** (enforced by tooling — hooks, linters, validation) or **advisory** (guidance for agents/humans)
3. Add classification field to frontmatter (e.g., `enforcement: mechanical` or `enforcement: advisory`)
4. Remove rules that contradict the plugin-composed architecture
5. Remove rules made redundant by the architecture

**Files:** All 59 RULE-*.md files listed in 03-orqa-process.md section 3

**Acceptance Criteria:**
- [ ] Every rule reviewed with disposition recorded
- [ ] Each rule classified as mechanical or advisory
- [ ] Classification field added to frontmatter of all kept rules
- [ ] Contradictory/redundant rules archived or deleted
- [ ] Report produced

**Reviewer Checks:**
- Spot-check 10 rules for correct classification
- Verify "mechanical" rules actually have corresponding enforcement hooks/scripts
- Verify no deleted rules were the only enforcement for an important constraint

---

### 6.24 — Review and clean epic artifacts (128 files)

**What:** Review all 128 EPIC-*.md files in `.orqa/implementation/epics/` (migrated from former `.orqa/delivery/epics/`). For each:
1. Assess relevance to the migration plan and path forward
2. Archive epics not about the path forward (status: `archived`)
3. Clean status values — normalize YAML quoting (pick one convention: unquoted)
4. Verify status values are valid per the workflow

**Acceptance Criteria:**
- [ ] Every epic reviewed with disposition recorded
- [ ] Irrelevant epics archived
- [ ] All status values use consistent quoting (unquoted)
- [ ] All status values are valid per `epic.resolved.yaml`
- [ ] Remaining active/captured epics align with this migration plan

**Reviewer Checks:**
- Verify archived epics are truly irrelevant to future work
- Verify no important in-progress work was accidentally archived
- Check 10 random epics for consistent quoting

---

### 6.25 — Review and clean task artifacts (731 files)

**What:** Review all 731 TASK-*.md files in `.orqa/implementation/tasks/` (migrated from former `.orqa/delivery/tasks/`). This is a bulk operation:
1. Archive all completed tasks that belong to archived epics (from 6.24)
2. Archive all tasks with status `surpassed` (9 files)
3. Verify all active/captured/ready/blocked tasks still align with the migration plan
4. Clean status values — normalize YAML quoting (unquoted)
5. Verify all status values are valid per `task.resolved.yaml`

Note: Because of the volume (731 files), this task should use scripted automation for bulk operations (quoting normalization, status validation, mass archive of tasks delivering to archived epics) with manual review only for non-completed tasks (~140 files).

**Acceptance Criteria:**
- [ ] Tasks delivering to archived epics are archived
- [ ] All surpassed tasks archived
- [ ] Status quoting normalized across all 731 files
- [ ] All status values valid per workflow
- [ ] Active/captured/ready/blocked tasks (~140) reviewed for relevance
- [ ] Report produced with counts

**Reviewer Checks:**
- Verify archive script correctly identifies tasks by epic relationship
- Spot-check 10 archived tasks
- Spot-check 10 non-archived active tasks
- Run `orqa validate` or schema check against task artifacts

---

### 6.26 — Review and clean idea artifacts (172 files: 12 delivery + 160 discovery)

**What:** Review all 172 idea files across both directories:
- `.orqa/implementation/ideas/` (12 files, migrated from former `.orqa/delivery/ideas/`)
- `.orqa/discovery/ideas/` (160 files)

For each:
1. Combine/group ideas that are thematically the same
2. Archive ideas no longer relevant to the architecture (status: `archived`)
3. Verify remaining ideas are actionable and not superseded

Note: The 28 already-surpassed discovery ideas need verification. The 116 captured discovery ideas need relevance review. This can be done in sub-batches.

**Acceptance Criteria:**
- [ ] Thematically duplicate ideas merged (one kept, others archived with reference)
- [ ] Irrelevant ideas archived
- [ ] All remaining ideas are actionable and relevant
- [ ] Report produced with merge/archive decisions

**Reviewer Checks:**
- Verify merged ideas preserve the best content from all sources
- Verify archived ideas are truly irrelevant
- Spot-check 10 remaining ideas for relevance

---

### 6.27 — Delete `prompt-registry.json` from `.orqa/`

**What:** Delete `.orqa/prompt-registry.json` (3,654 lines, 177 knowledge entries). This is not in the target structure. The runtime prompt pipeline replaces it.

**Acceptance Criteria:**
- [ ] File deleted
- [ ] No code depends on reading this file at runtime (verify before deleting)
- [ ] If code references exist, they are updated to use the prompt pipeline instead

**Reviewer Checks:**
- Grep for `prompt-registry.json` across the codebase
- Verify any code that reads this file has been updated or confirmed unused

---

### 6.28 — Verify `node_modules` gitignore coverage for `integrations/claude-agent-sdk/`

**What:** The gap analysis flagged vendored `node_modules` as committed binary blobs, but actual git state shows `node_modules/` is already gitignored and NOT tracked (`git ls-files` returns 0 files, `git check-ignore` confirms coverage). This task verifies the gitignore coverage is correct and optionally removes the local `node_modules/` directory to reduce disk footprint.

1. Verify `node_modules/` is gitignored (already confirmed — just document)
2. Optionally delete the local `integrations/claude-agent-sdk/node_modules/` directory
3. Ensure `integrations/claude-agent-sdk/package.json` has correct dependencies for `npm install`

**Acceptance Criteria:**
- [ ] `node_modules/` is confirmed gitignored for this path (via `git check-ignore`)
- [ ] `git ls-files integrations/claude-agent-sdk/node_modules/` returns zero results
- [ ] `npm install` in the directory installs the correct dependencies
- [ ] No tracked binary blobs exist in this path

**Reviewer Checks:**
- Verify `.gitignore` coverage via `git check-ignore -v`
- Run `npm install` in the directory and verify it works

---

### 6.29 — Sync infrastructure/sync-bridge version

**What:** Update `infrastructure/sync-bridge/package.json` version from `0.1.0` to `0.1.4-dev` to match the monorepo.

**Acceptance Criteria:**
- [ ] Version in `infrastructure/sync-bridge/package.json` is `0.1.4-dev`
- [ ] No other version-bearing files in sync-bridge are out of sync

**Reviewer Checks:**
- Verify version matches root `VERSION` file
- Check for any other version strings in the sync-bridge directory

---

### 6.30 — Sync integrations/claude-agent-sdk version

**What:** Update `integrations/claude-agent-sdk/package.json` version from `0.1.0-dev` to `0.1.4-dev` to match the monorepo.

**Acceptance Criteria:**
- [ ] Version in package.json is `0.1.4-dev`

**Reviewer Checks:**
- Verify version matches root `VERSION` file

---

## Phase 7: Governance Artifact Migration

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Every directory move and artifact reclassification must be validated against ARCHITECTURE.md to confirm the target structure.

### 7.1 — Move decisions to stage-first structure

**What:** Move `.orqa/process/decisions/` to the stage-first structure. Decisions split by level per ARCHITECTURE.md 5.1:
- Principle/architecture decisions -> `.orqa/learning/decisions/`
- Planning/tactical decisions -> `.orqa/planning/decisions/`

**Note:** This task moves all decisions to a staging location. Task 7.11 handles the principle/planning classification split. If classification has not yet been done (task 6.21), move all files to `.orqa/learning/decisions/` initially, then split in 7.11.

**Steps:**
1. Create `.orqa/learning/decisions/` and `.orqa/planning/decisions/`
2. Move all 70 AD-*.md files from `.orqa/process/decisions/` — principle-classified to `.orqa/learning/decisions/`, planning-classified to `.orqa/planning/decisions/`
3. Remove empty `.orqa/process/decisions/` directory

**Acceptance Criteria:**
- [ ] All 70 AD-*.md files exist in `.orqa/learning/decisions/` or `.orqa/planning/decisions/`
- [ ] `.orqa/process/decisions/` no longer exists
- [ ] File contents are unchanged (no modifications during move)
- [ ] `orqa validate` or graph checks pass with new paths

**Reviewer Checks:**
- Verify file count: 70 files in new location
- Diff a sample of 3 files to confirm content unchanged
- Check that the graph/engine finds artifacts at new paths

---

### 7.2 — Move knowledge to documentation stage structure

**What:** Move `.orqa/process/knowledge/` into `.orqa/documentation/<category>/knowledge/` subdirectories. Per ARCHITECTURE.md 5.1, knowledge lives WITH documentation — knowledge is documentation split into agent-consumable chunks with injection metadata.

**Steps:**
1. Create `.orqa/documentation/<category>/knowledge/` subdirectories per task 7.12 domain assignments
2. Move all 114 KNOW-*.md files from `.orqa/process/knowledge/` to their assigned `.orqa/documentation/<category>/knowledge/` subdirectory
3. Do NOT move the 5 SKILL.md subdirectories (they should have been removed in task 6.20)
4. Remove empty `.orqa/process/knowledge/` directory

**Acceptance Criteria:**
- [ ] All 114 KNOW-*.md files exist in `.orqa/documentation/<category>/knowledge/` subdirectories
- [ ] No SKILL.md subdirectories remain (verified removed in 6.20)
- [ ] `.orqa/process/knowledge/` no longer exists
- [ ] File contents unchanged

**Reviewer Checks:**
- Verify file count: 114 files in new location
- Verify no subdirectories were carried over
- Check graph/engine finds artifacts

---

### 7.3 — Move rules to learning stage

**What:** Move `.orqa/process/rules/` to `.orqa/learning/rules/` (learning stage per ARCHITECTURE.md 5.1).

**Steps:**
1. Create `.orqa/learning/rules/`
2. Move all 59 RULE-*.md files
3. Remove empty `.orqa/process/rules/`

**Acceptance Criteria:**
- [ ] All 59 RULE-*.md files exist in `.orqa/learning/rules/`
- [ ] `.orqa/process/rules/` no longer exists
- [ ] File contents unchanged

**Reviewer Checks:**
- Verify file count: 59 files
- Check graph/engine finds artifacts

---

### 7.4 — Move lessons to learning stage

**What:** Move `.orqa/process/lessons/` to `.orqa/learning/lessons/` (learning stage per ARCHITECTURE.md 5.1).

**Steps:**
1. Create `.orqa/learning/lessons/`
2. Move all 84 IMPL-*.md files
3. Remove empty `.orqa/process/lessons/`

**Acceptance Criteria:**
- [ ] All 84 IMPL-*.md files exist in `.orqa/learning/lessons/`
- [ ] `.orqa/process/lessons/` no longer exists
- [ ] File contents unchanged

**Reviewer Checks:**
- Verify file count: 84 files
- Check graph/engine finds artifacts

---

### 7.5a — Research: determine correct location for source workflow definitions

**What:** Research where the 14 plugin-sourced workflow YAML files (contribution/lifecycle definitions, NOT resolved `.resolved.yaml` files) should live after migration. These are currently at `.orqa/process/workflows/`. Per ARCHITECTURE.md 5.1, source workflow definitions stay in plugin directories; only resolved output goes to `.orqa/workflows/`.

**Steps:**
1. Determine if `orqa install` has an opinion on where it places workflow definitions
2. Check whether `.orqa/workflows/` should contain both source and resolved files, or if source goes elsewhere
3. Determine naming convention to distinguish source (`.workflow.yaml`) from resolved (`.resolved.yaml`)
4. Write findings with recommendation

**Files (14 source definitions):**
```
agent.workflow.yaml             decision.workflow.yaml
doc.workflow.yaml               documentation.contribution.workflow.yaml
knowledge.workflow.yaml         learning.contribution.workflow.yaml
lesson.workflow.yaml            planning.contribution.workflow.yaml
planning-decision.workflow.yaml planning-idea.workflow.yaml
planning-research.workflow.yaml review.contribution.workflow.yaml
rule.workflow.yaml              wireframe.workflow.yaml
```

**Acceptance Criteria:**
- [ ] Target location documented with rationale
- [ ] Naming convention for source vs resolved documented
- [ ] Impact on `orqa install` assessed

**Reviewer Checks:**
- Verify recommendation is consistent with other `.orqa/` restructuring decisions

---

### 7.5b — Move source workflow definitions to determined location

**What:** Move the 14 source workflow YAML files from `.orqa/process/workflows/` back to their owning plugin directories (per ARCHITECTURE.md 5.1: source definitions stay in plugin dirs, only resolved output at `.orqa/workflows/`). Update any path references.

**Depends on:** 7.5a

**Steps:**
1. Identify which plugin owns each source workflow definition
2. Move each file to its owning plugin's `workflows/` directory
3. Remove empty `.orqa/process/workflows/`
4. Update `project.json` or manifest if it references the old path

**Acceptance Criteria:**
- [ ] All 14 source workflow files at the location from 7.5a
- [ ] Clear naming distinction from resolved workflows
- [ ] `orqa install` can still find/generate these
- [ ] `.orqa/process/workflows/` no longer exists
- [ ] Source workflow definitions are in their owning plugin directories

**Reviewer Checks:**
- Verify `orqa install` works with new paths
- Verify each source workflow is in the correct plugin directory

---

### 7.6 — Remove `process/agents/` directory (19 legacy AGENT-*.md files)

**What:** Delete the entire `.orqa/process/agents/` directory and all 19 AGENT-*.md files. These are legacy monolithic agent definitions that are replaced by:
- Base roles in the methodology plugin
- Domain knowledge injection at runtime
- Generated task-specific agents

**Files to delete (19):**
```
AGENT-065a25cc.md (Rust Specialist)
AGENT-0aad40f4.md (Designer)
AGENT-26e5029d.md (Rust Standards Agent)
AGENT-336e4d7d.md (Integration Specialist)
AGENT-4c94fe14.md (Orchestrator)
AGENT-5de8c14f.md (Svelte Specialist)
AGENT-65b56a0b.md (Tauri Standards Agent)
AGENT-6f55de0d.md (Svelte Standards Agent)
AGENT-7a06d10e.md (Governance Enforcer)
AGENT-85be6ace.md (Planner)
AGENT-867da593.md (Rust Backend Specialist)
AGENT-8e58cd87.md (Reviewer)
AGENT-ae63c406.md (Governance Steward)
AGENT-bbad3d30.md (Writer)
AGENT-ce86fb50.md (Plugin Developer)
AGENT-d1be3776.md (Installer)
AGENT-e333508b.md (Researcher)
AGENT-e5a1b6bf.md (Svelte Frontend Specialist)
AGENT-e5dd38e4.md (Implementer)
```

**Prerequisite:** Before deleting, verify that:
- Base role definitions exist in methodology plugin (`plugins/agile-workflow/agents/`)
- `.claude/agents/` files exist as operational truth for Claude Code

**Acceptance Criteria:**
- [ ] All 19 AGENT-*.md files deleted
- [ ] `.orqa/process/agents/` directory deleted
- [ ] No code references AGENT-*.md files by path
- [ ] Methodology plugin has base role definitions
- [ ] `.claude/agents/` operational definitions are intact

**Reviewer Checks:**
- Grep for `AGENT-` IDs across the codebase (relationships in other artifacts may reference these IDs)
- If other artifacts have `employs: AGENT-xxx` relationships, those relationships need to be updated or removed
- Verify methodology plugin agent definitions cover the base roles

---

### 7.7 — Update relationship references after AGENT-*.md deletion

**What:** After task 7.6, fix all artifacts that reference AGENT-*.md IDs in their relationships. Knowledge files reference agents via `employed-by` relationships, and agents reference knowledge via `employs` relationships.

**Steps:**
1. Grep all `.orqa/` artifacts for AGENT-* IDs
2. Remove or update `employed-by: AGENT-xxx` relationships in knowledge files
3. Remove any other dangling AGENT references

**Acceptance Criteria:**
- [ ] No artifact contains a relationship targeting a deleted AGENT ID
- [ ] All relationship arrays updated
- [ ] Graph validation passes with no broken links to AGENT IDs

**Reviewer Checks:**
- Run graph validation / broken link check
- Grep for all 19 AGENT IDs across `.orqa/`

---

### 7.8 — Remove `process/` directory entirely

**What:** After tasks 7.1-7.7 have moved all contents out, remove the empty `.orqa/process/` directory.

**Prerequisite:** Tasks 7.1-7.7 all complete.

**Acceptance Criteria:**
- [ ] `.orqa/process/` directory does not exist
- [ ] All former contents are in their new top-level locations
- [ ] No code references `.orqa/process/` as a path

**Reviewer Checks:**
- Grep for `.orqa/process/` and `orqa/process/` across the entire codebase
- Update any matches (config files, CLI scan paths, plugin manifests)

---

### 7.9 — Remove `principles/grounding/` directory (5 DOC files)

**What:** Delete `.orqa/principles/grounding/` (or `.orqa/discovery/grounding/` if already migrated) and its 5 files. These grounding docs should have been converted to `tier: always` knowledge artifacts in appropriate plugins during Phase 6 content review (tasks 6.16-6.19). Note: per ARCHITECTURE.md 5.1, the `principles/` directory is replaced by `discovery/` (for vision, pillars, personas).

**Files to delete:**
- `DOC-a0490c49.md` (Product Purpose grounding)
- `DOC-bdb520ae.md` (Research Principles grounding)
- `DOC-40b1498a.md` (Design Principles grounding)
- `DOC-ebf19a16.md` (Code Principles grounding)
- `DOC-0ea4c263.md` (Artifact Principles grounding)

**Prerequisite:** Verify equivalent KNOW-*.md files with `tier: always` exist for each grounding doc's content. If they don't exist yet, create them before deleting.

**Acceptance Criteria:**
- [ ] All 5 DOC files deleted
- [ ] `principles/grounding/` directory deleted
- [ ] Equivalent knowledge artifacts exist with `tier: always` in appropriate plugins or project knowledge
- [ ] No broken references to these DOC IDs

**Reviewer Checks:**
- Verify each grounding doc's content is preserved as knowledge
- Grep for the 5 DOC IDs across `.orqa/`
- Verify `discovery/` now contains only `vision/`, `pillars/`, `personas/`, `ideas/`, `research/`, `wireframes/`

---

### 7.10 — Fix personas directory: remove DOC-1ff7a9ba

**What:** Remove `DOC-1ff7a9ba.md` from `.orqa/discovery/personas/` (migrated from former `.orqa/principles/personas/`). This DOC file uses the wrong artifact type for the location. The 3 PERSONA files already contain the essential content.

**Steps:**
1. Verify content of DOC-1ff7a9ba is adequately covered by PERSONA-c4afd86b, PERSONA-477971bf, PERSONA-2721ae35
2. If DOC-1ff7a9ba has unique content not in the PERSONAs, move it to `.orqa/documentation/`
3. If fully duplicated, delete it

**Acceptance Criteria:**
- [ ] DOC-1ff7a9ba.md removed from `personas/`
- [ ] `personas/` contains only PERSONA-*.md files (3 files)
- [ ] No unique content lost (either migrated to documentation or confirmed duplicate)

**Reviewer Checks:**
- Compare DOC-1ff7a9ba content with the 3 PERSONA files
- Verify no information loss

---

### 7.11 — Verify decision split between learning/ and planning/ stages

**What:** Using the classification from task 6.21, verify decisions are correctly split between stage directories:
- `.orqa/learning/decisions/` — principle/architecture decisions (PRINCIPLE-DECISION artifacts)
- `.orqa/planning/decisions/` — planning/tactical decisions (PLANNING-DECISION artifacts)

**Prerequisite:** Task 6.21 (classification) and 7.1 (move to top-level) complete.

**Acceptance Criteria:**
- [ ] `.orqa/learning/decisions/` contains all principle-classified decisions
- [ ] `.orqa/planning/decisions/` contains all planning-classified decisions
- [ ] No AD-*.md files remain outside these two directories
- [ ] Total file count across both subdirectories equals original 70 (minus any archived in 6.21)
- [ ] File contents unchanged

**Reviewer Checks:**
- Verify file counts match classification report from 6.21
- Spot-check 5 files in each directory for correct classification
- Verify graph/engine finds artifacts at new paths

---

### 7.12 — Categorize knowledge into documentation topic subdirectories

**What:** Using the domain assignments from tasks 6.16-6.19, place knowledge files into `.orqa/documentation/<category>/knowledge/` subdirectories. Per ARCHITECTURE.md 5.1, knowledge lives WITH documentation — organized by topic, not standalone.

Target subdirectories:
```
documentation/
  platform/knowledge/        — core framework, governance, enforcement (~25 files)
  development/
    rust/knowledge/           — Rust patterns, testing, clippy, Tauri (~10 files)
    frontend/knowledge/       — Svelte 5, stores, component patterns (~10 files)
    typescript/knowledge/     — TypeScript patterns, plugin skills (~5 files)
  methodology/knowledge/      — planning, research, delivery discipline, thinking modes (~15 files)
  authoring/knowledge/        — documentation authoring, README standards (~5 files)
  plugin/knowledge/           — plugin development, architecture (~5 files)
  integration/knowledge/      — IPC, CLI, search, streaming, hooks (~10 files)
  standards/knowledge/        — coding standards, naming, design system (~10 files)
  architecture/knowledge/     — project-specific architecture knowledge (~19 files)
```

**Prerequisite:** Tasks 6.16-6.19 (review and classify) and 7.2 (move from process/) complete.

**Acceptance Criteria:**
- [ ] All `documentation/<category>/knowledge/` subdirectories created
- [ ] All 114 KNOW-*.md files placed in their assigned `knowledge/` subdirectory
- [ ] No KNOW-*.md files exist outside `documentation/*/knowledge/` directories
- [ ] Total file count across all subdirectories equals 114 (minus any removed in review)
- [ ] File contents unchanged

**Reviewer Checks:**
- Verify each file is in the correct domain's knowledge/ subdirectory
- Verify total count matches
- Verify graph/engine scans documentation/*/knowledge/ subdirectories correctly

---

### 7.13 — Categorize documentation into topic subdirectories

**What:** Using the assignments from tasks 6.13-6.15, restructure `.orqa/documentation/` into topic-based subdirectories:

Target subdirectories:
```
documentation/
  architecture/     — architecture docs
  reference/        — reference material
  how-to/           — how-to guides
  onboarding/       — getting started, setup
  concept/          — conceptual docs
  platform/         — platform-specific docs (keep existing, but review)
  project/          — project-specific docs (keep existing, but review)
```

**Prerequisite:** Tasks 6.13-6.15 complete.

**Acceptance Criteria:**
- [ ] Topic subdirectories created
- [ ] All DOC files from root level moved to appropriate subdirectory
- [ ] No DOC files remain at `.orqa/documentation/` root
- [ ] platform/ and project/ retained (or merged into topic dirs if that's what the review decided)
- [ ] File contents unchanged

**Reviewer Checks:**
- Verify no files at documentation root
- Spot-check 5 files for correct categorization
- Verify total count matches (91 minus any archived in review)

---

### 7.14 — Fix wireframe artifact type (doc -> wireframe)

**What:** Update all 5 wireframe files in `.orqa/discovery/wireframes/` to use `type: wireframe` and rename from `DOC-` to `WIRE-` prefix (or whatever prefix the wireframe schema specifies).

**Files:**
- `DOC-6c91572c.md` -> `WIRE-6c91572c.md`
- `DOC-65a3c4e8.md` -> `WIRE-65a3c4e8.md`
- `DOC-93a0f6c1.md` -> `WIRE-93a0f6c1.md`
- `DOC-4ac7f17a.md` -> `WIRE-4ac7f17a.md`
- `DOC-796d7f01.md` -> `WIRE-796d7f01.md`

For each file:
1. Update frontmatter `type: doc` -> `type: wireframe`
2. Update frontmatter `id: DOC-xxxx` -> `id: WIRE-xxxx`
3. Rename the file from `DOC-xxxx.md` to `WIRE-xxxx.md`

**Prerequisite:** Verify that `wireframe` is a valid artifact type in the schema and has the correct prefix defined.

**Acceptance Criteria:**
- [ ] All 5 files renamed with WIRE- prefix
- [ ] All 5 files have `type: wireframe` in frontmatter
- [ ] All 5 files have `id: WIRE-xxxx` matching their filename
- [ ] No DOC-*.md files remain in wireframes/
- [ ] `wireframe.resolved.yaml` workflow can apply to these artifacts

**Reviewer Checks:**
- Verify schema has `wireframe` type defined
- Verify ID format matches schema expectations
- Run artifact validation against the updated files

---

### 7.15 — Standardize frontmatter: `name` -> `title` across all artifacts

**What:** Find and fix all artifacts using `name` instead of `title` in frontmatter. Per ARCHITECTURE.md, the canonical field is `title`.

Known files using `name`: DOC-2372ed36, DOC-4554ff3e, DOC-db794473, DOC-743f9c71, DOC-7b9b45f0, DOC-ae447f88, DOC-e42efeaf, plus various knowledge files.

**Steps:**
1. Grep all `.orqa/` artifacts for `^name:` in frontmatter
2. For each match, rename `name:` to `title:` (preserving the value)
3. If both `name` and `title` exist, keep `title`, remove `name`

**Acceptance Criteria:**
- [ ] No artifact in `.orqa/` uses `name:` where `title:` is expected
- [ ] Files with both `name` and `title` have only `title`
- [ ] No content changes — only field name changes

**Reviewer Checks:**
- Grep `.orqa/` for `^name:` — should return zero matches
- Spot-check 5 converted files for correct values

---

### 7.16 — Standardize frontmatter: add missing `status` fields

**What:** Find all artifacts missing a `status` field and add one. Per artifact lifecycle workflows, all artifacts should have a status with appropriate initial state.

**Steps:**
1. Grep all `.orqa/` artifacts for files WITHOUT a `status:` field
2. For each, add `status: captured` (or the appropriate initial state per the artifact's workflow)
3. Ensure status values match the artifact's resolved workflow

**Acceptance Criteria:**
- [ ] Every artifact in `.orqa/` has a `status` field
- [ ] Status values are valid per the artifact's resolved workflow
- [ ] No new content changes — only metadata additions

**Reviewer Checks:**
- Run a script to verify every artifact has a `status` field
- Cross-check status values against resolved workflows

---

### 7.17 — Standardize frontmatter: normalize YAML quoting

**What:** Normalize all YAML string quoting across `.orqa/` artifacts. Pick one convention (unquoted for simple strings) and apply consistently.

Targets:
- Status values: `"completed"` -> `completed`
- Other string fields that have inconsistent quoting

**Steps:**
1. Script to find all quoted status values and other unnecessarily quoted strings
2. Normalize to unquoted form (except where quotes are required for YAML correctness)

**Acceptance Criteria:**
- [ ] All status values use consistent quoting (unquoted)
- [ ] No semantic changes — only formatting
- [ ] YAML still parses correctly after normalization

**Reviewer Checks:**
- Parse 10 random files before/after to verify YAML equivalence
- Verify no data loss from quoting changes

---

### 7.18a — Research: assess `.orqa/connectors/` runtime usage

**What:** Determine whether the files in `.orqa/connectors/claude-code/` are actively used at runtime before removing the directory.

**Steps:**
1. Grep for `injector-config.json` in connector source code — determine if it is read at runtime
2. Grep for `enforce-background-agents.mjs` in connector source code — determine if it is executed
3. Check if `orqa install` generates these files or if they are manually created
4. Write findings: for each file, document whether it is used, by what, and where it should go

**Acceptance Criteria:**
- [ ] Usage status documented for `injector-config.json` (used/unused, by what code)
- [ ] Usage status documented for hook scripts (used/unused, by what code)
- [ ] Relocation recommendation for each used file

**Reviewer Checks:**
- Verify grep was comprehensive (connector source, CLI, daemon)

---

### 7.18b — Remove `.orqa/connectors/` directory based on research

**What:** Based on findings from 7.18a, relocate any actively-used files and delete the `.orqa/connectors/` directory.

**Depends on:** 7.18a

**Steps:**
1. Relocate actively-used files per 7.18a recommendations
2. Update any code references to point to new locations
3. Delete `.orqa/connectors/` directory
4. Verify no runtime breakage

**Acceptance Criteria:**
- [ ] `.orqa/connectors/` directory removed
- [ ] Any actively-used runtime config relocated to correct location (per 7.18a)
- [ ] No code breaks from the removal
- [ ] Connector still functions after removal

**Reviewer Checks:**
- Grep for `injector-config` and `enforce-background-agents` — zero hits pointing to `.orqa/connectors/`
- Verify the connector still functions after removal

---

### 7.19 — Update `project.json` and `manifest.json` for new paths

**What:** After all .orqa/ restructuring (tasks 7.1-7.18), update `.orqa/project.json` and `.orqa/manifest.json` to reflect the new directory structure.

**Steps:**
1. Read current `project.json` — check for path references to `process/`, old directory names
2. Update any scan paths, directory references
3. Read current `manifest.json` — update artifact path references
4. Verify the CLI/engine can discover all artifacts at new paths

**Acceptance Criteria:**
- [ ] `project.json` has no references to removed directories (`process/`, `agents/`, `grounding/`, `connectors/`)
- [ ] `manifest.json` reflects actual file locations
- [ ] `orqa check` or equivalent passes with new paths
- [ ] Engine artifact scanner finds all artifacts

**Reviewer Checks:**
- Run `orqa check` or artifact scan
- Grep `project.json` and `manifest.json` for any `process/` references

---

### 7.20 — Update plugin content targets for new `.orqa/` structure

**What:** Update all plugin `orqa-plugin.json` files that reference `.orqa/process/` in their content installation targets.

**Files to check (all plugins):**
```
plugins/agile-workflow/orqa-plugin.json
plugins/agile-discovery/orqa-plugin.json
plugins/agile-planning/orqa-plugin.json
plugins/agile-documentation/orqa-plugin.json
plugins/agile-review/orqa-plugin.json
plugins/software-kanban/orqa-plugin.json
plugins/core/orqa-plugin.json
plugins/cli/orqa-plugin.json
plugins/rust/orqa-plugin.json
plugins/svelte/orqa-plugin.json
plugins/tauri/orqa-plugin.json
plugins/typescript/orqa-plugin.json
plugins/coding-standards/orqa-plugin.json
plugins/systems-thinking/orqa-plugin.json
plugins/plugin-dev/orqa-plugin.json
plugins/githooks/orqa-plugin.json
```

**Steps:**
1. Grep all `orqa-plugin.json` for `process/`
2. Update targets: `.orqa/process/rules` -> `.orqa/learning/rules`, `.orqa/process/knowledge` -> `.orqa/documentation/<category>/knowledge/`, `.orqa/process/decisions` -> `.orqa/learning/decisions/` or `.orqa/planning/decisions/`, `.orqa/process/lessons` -> `.orqa/learning/lessons/`, etc.
3. Verify `orqa install` works with new paths

**Acceptance Criteria:**
- [ ] No plugin manifest references `.orqa/process/`
- [ ] All content installation targets match the new `.orqa/` structure
- [ ] `orqa install` correctly installs plugin content to new locations

**Reviewer Checks:**
- Grep all `orqa-plugin.json` for `process/`
- Run `orqa install` and verify content lands in correct directories
- Spot-check 3 plugins for correct paths

---

### 7.21 — Update CLI/engine artifact scan paths

**What:** Update any hardcoded paths in `libs/cli/` and engine code that reference `.orqa/process/`.

**Steps:**
1. Grep `libs/cli/src/` for `process/`
2. Grep `libs/validation/src/` for `process/`
3. Grep `app/src-tauri/src/` for `process/`
4. Update all hardcoded paths to match new structure
5. Rebuild and test

**Acceptance Criteria:**
- [ ] No source code references `.orqa/process/` as a scan path
- [ ] Engine discovers artifacts at new locations
- [ ] CLI commands work with new paths
- [ ] Build succeeds

**Reviewer Checks:**
- Grep for `process/agents`, `process/knowledge`, `process/rules`, `process/lessons`, `process/decisions`, `process/workflows` in all source files
- Run `orqa check` to verify discovery

---

### 7.22 — Regenerate resolved workflows

**What:** After all governance artifact migration is complete, regenerate resolved workflows by running `orqa install`. Verify the 24 resolved workflows regenerate correctly at `.orqa/workflows/`.

Note: The ARCHITECTURE.md target calls for 7 purpose-named workflows instead of 24 artifact-type-named ones. However, this is an engine-level change (how `orqa install` names resolved outputs). For now, verify the current naming works with the new paths. Purpose-based naming is a future engine enhancement.

**Acceptance Criteria:**
- [ ] `orqa install` completes without errors
- [ ] All resolved workflows regenerated in `.orqa/workflows/`
- [ ] Resolved workflows reference correct artifact types and states
- [ ] No stale/orphaned resolved workflows

**Reviewer Checks:**
- Compare before/after resolved workflow count
- Verify `agent.resolved.yaml` no longer exists (agent type should be removed)
- Spot-check 3 resolved workflows for correctness

---

## Phase 8: Codebase Restructure

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Every directory move and workspace reconfiguration must be validated against ARCHITECTURE.md to confirm the target structure.

### 8.1 — Create `engine/` directory and move Rust crates from `libs/` to `engine/crates/`

**Note:** ARCHITECTURE.md Phase 8 item 1 says "Move engine crates to `libs/`" but this is a wording error — the correct direction is `libs/` -> `engine/crates/` as confirmed by the proposed restructure document.

**What:** Create `engine/crates/` and move the 4 Rust crates:
- `libs/validation/` -> `engine/crates/validation/`
- `libs/search/` -> `engine/crates/search/`
- `libs/mcp-server/` -> `engine/crates/mcp-server/`
- `libs/lsp-server/` -> `engine/crates/lsp-server/`

**Steps:**
1. Create `engine/` and `engine/crates/`
2. Create `engine/Cargo.toml` as workspace root
3. Move each crate directory
4. Update root `Cargo.toml` workspace members:
   - Remove: `libs/validation`, `libs/search`, `libs/mcp-server`, `libs/lsp-server`
   - Add: `engine/crates/validation`, `engine/crates/search`, `engine/crates/mcp-server`, `engine/crates/lsp-server`
5. Update `app/src-tauri/Cargo.toml` dependency paths
6. Update any inter-crate dependency paths
7. Verify `cargo build` succeeds

**Acceptance Criteria:**
- [ ] All 4 crates exist under `engine/crates/`
- [ ] `libs/` no longer contains Rust crates (only TypeScript packages remain)
- [ ] `engine/Cargo.toml` workspace file exists
- [ ] Root `Cargo.toml` workspace members updated
- [ ] `app/src-tauri/Cargo.toml` dependencies point to `engine/crates/`
- [ ] `cargo build` succeeds from repo root
- [ ] `cargo test` passes

**Reviewer Checks:**
- Verify no Rust source files remain in `libs/`
- Verify workspace resolution: `cargo metadata` shows correct crate paths
- Run full build and test suite

---

### 8.2 — Create `daemon/` top-level directory (placeholder)

**What:** Create `daemon/` as a top-level directory with a README explaining its future purpose. The daemon will be built in Phase 3 of the migration but the directory establishes the structure now.

**Steps:**
1. Create `daemon/`
2. Create `daemon/README.md` explaining: "Standalone Rust daemon process. Provides system tray, file watchers, MCP server, LSP server. Built in Migration Phase 3."

**Acceptance Criteria:**
- [ ] `daemon/` directory exists at repo root
- [ ] Contains a README explaining purpose and timeline
- [ ] No source code yet (placeholder only)

**Reviewer Checks:**
- Verify the README accurately describes the Phase 3 plan

---

### 8.3 — Restructure plugins: create taxonomy subdirectories

**What:** Create the plugin taxonomy subdirectory structure and the top-level `sidecars/` directory:
```
plugins/
  methodology/
  workflows/
  knowledge/
  infrastructure/
sidecars/         (top-level, NOT under plugins/)
```

**Acceptance Criteria:**
- [ ] All 4 plugin subdirectories exist under `plugins/`
- [ ] `sidecars/` exists at repo root
- [ ] No files at root of each new subdirectory (just the plugin directories inside them)

**Reviewer Checks:**
- Verify directory structure matches ARCHITECTURE.md section 4.1

---

### 8.4 — Move methodology plugin

**What:** Move `plugins/agile-workflow/` -> `plugins/methodology/agile-workflow/`

**Steps:**
1. Move directory
2. Update npm workspace in root `package.json` (if plugins/* glob doesn't match nested dirs)
3. Update any hardcoded paths in CLI, connector, or other code
4. Verify `npm install` succeeds
5. Verify `orqa install` finds the plugin

**Acceptance Criteria:**
- [ ] `plugins/methodology/agile-workflow/` exists with all contents
- [ ] `plugins/agile-workflow/` no longer exists
- [ ] npm workspace resolves correctly
- [ ] `orqa install` finds and installs the plugin
- [ ] No broken imports

**Reviewer Checks:**
- Run `npm install` from root
- Run `orqa plugin list` to verify discovery
- Grep for `plugins/agile-workflow` (old path) across codebase

---

### 8.5 — Move workflow plugins (6 plugins)

**What:** Move all 6 workflow plugins:
- `plugins/core/` -> `plugins/workflows/core/`
- `plugins/agile-discovery/` -> `plugins/workflows/agile-discovery/`
- `plugins/agile-planning/` -> `plugins/workflows/agile-planning/`
- `plugins/agile-documentation/` -> `plugins/workflows/agile-documentation/`
- `plugins/agile-review/` -> `plugins/workflows/agile-review/`
- `plugins/software-kanban/` -> `plugins/workflows/software-kanban/`

**Steps:**
1. Move each directory
2. Update npm workspace references
3. Update any hardcoded paths
4. Verify `npm install` and `orqa install`

**Acceptance Criteria:**
- [ ] All 6 plugins exist under `plugins/workflows/`
- [ ] None exist at old `plugins/` root location
- [ ] npm workspace resolves correctly
- [ ] `orqa install` finds all plugins
- [ ] No broken imports

**Reviewer Checks:**
- Run `npm install` and `orqa install`
- Grep for old paths: `plugins/core/`, `plugins/agile-discovery/`, etc.
- Verify `orqa plugin list` shows all 6

---

### 8.6 — Move knowledge plugins (7 plugins)

**What:** Move all 7 knowledge plugins:
- `plugins/cli/` -> `plugins/knowledge/cli/`
- `plugins/rust/` -> `plugins/knowledge/rust/`
- `plugins/svelte/` -> `plugins/knowledge/svelte/`
- `plugins/tauri/` -> `plugins/knowledge/tauri/`
- `plugins/typescript/` -> `plugins/knowledge/typescript/`
- `plugins/systems-thinking/` -> `plugins/knowledge/systems-thinking/`
- `plugins/plugin-dev/` -> `plugins/knowledge/plugin-dev/`

**Steps:**
1. Move each directory
2. Update npm workspace references
3. Update any hardcoded paths
4. Verify `npm install` and `orqa install`

**Acceptance Criteria:**
- [ ] All 7 plugins exist under `plugins/knowledge/`
- [ ] None exist at old `plugins/` root location
- [ ] npm workspace resolves correctly
- [ ] `orqa install` finds all plugins
- [ ] No broken imports

**Reviewer Checks:**
- Run `npm install` and `orqa install`
- Grep for old paths
- Verify `orqa plugin list` shows all 7

---

### 8.7 — Move infrastructure plugins (2 plugins)

**What:** Move both infrastructure plugins:
- `plugins/githooks/` -> `plugins/infrastructure/githooks/`
- `plugins/coding-standards/` -> `plugins/infrastructure/coding-standards/`

**Steps:**
1. Move each directory
2. Update npm workspace references
3. Update any hardcoded paths
4. Verify `npm install` and `orqa install`

**Acceptance Criteria:**
- [ ] Both plugins exist under `plugins/infrastructure/`
- [ ] None exist at old `plugins/` root location
- [ ] npm workspace resolves correctly
- [ ] `orqa install` finds both plugins
- [ ] No broken imports

**Reviewer Checks:**
- Run `npm install` and `orqa install`
- Grep for old paths
- Verify `orqa plugin list` shows both

---

### 8.8 — Move claude-agent-sdk to top-level `sidecars/`

**What:** Move `integrations/claude-agent-sdk/` -> `sidecars/claude-agent-sdk/`

**Note:** ARCHITECTURE.md Phase 8 item 5 specifies top-level `sidecars/`, not `plugins/sidecars/`. Sidecars are standalone processes, not plugins.

**Steps:**
1. Move directory
2. Update npm workspace: remove `integrations/*`, ensure sidecars are covered
3. Update any hardcoded paths in app or connector
4. Delete empty `integrations/` directory
5. Verify `npm install` succeeds

**Acceptance Criteria:**
- [ ] `sidecars/claude-agent-sdk/` exists with all contents
- [ ] `integrations/` directory no longer exists
- [ ] npm workspace resolves correctly
- [ ] No broken imports
- [ ] Sidecar can still be discovered by the app

**Reviewer Checks:**
- Verify `integrations/` is completely gone
- Grep for `integrations/claude-agent-sdk` across codebase
- Run `npm install`

---

### 8.9 — Remove sync-bridge

**What:** Delete `infrastructure/sync-bridge/` directory entirely. Per ARCHITECTURE.md Phase 8: "Remove sync-bridge (aspirational, not needed now)."

**Acceptance Criteria:**
- [ ] `infrastructure/sync-bridge/` deleted
- [ ] No code references sync-bridge
- [ ] `infrastructure/` still contains `orqastudio-git/`

**Reviewer Checks:**
- Grep for `sync-bridge` across codebase
- Verify no Docker Compose references to sync-bridge remain
- Verify `infrastructure/orqastudio-git/` is intact

---

### 8.10 — Update root npm workspace configuration

**What:** Update root `package.json` workspaces to reflect the new directory structure.

Current:
```json
"workspaces": [
  "libs/*",
  "plugins/*",
  "connectors/*",
  "integrations/*",
  "app"
]
```

Target:
```json
"workspaces": [
  "libs/*",
  "plugins/methodology/*",
  "plugins/workflows/*",
  "plugins/knowledge/*",
  "plugins/infrastructure/*",
  "sidecars/*",
  "connectors/*",
  "app"
]
```

**Acceptance Criteria:**
- [ ] Root `package.json` workspaces updated
- [ ] `integrations/*` removed from workspaces
- [ ] `npm install` succeeds from repo root
- [ ] All packages are discovered by npm workspace resolution

**Reviewer Checks:**
- Run `npm ls --all --depth=0` to verify all packages visible
- Verify no "missing workspace" warnings

---

### 8.11 — Update root Cargo.toml workspace configuration

**What:** Update root `Cargo.toml` workspace members to reflect engine crate moves.

Current:
```toml
members = [
  "libs/validation",
  "libs/search",
  "libs/mcp-server",
  "libs/lsp-server",
  "app/src-tauri",
]
```

Target:
```toml
members = [
  "engine/crates/core",
  "engine/crates/validation",
  "engine/crates/search",
  "engine/crates/mcp-server",
  "engine/crates/lsp-server",
  "app/src-tauri",
]
```

**Note:** Includes `engine/crates/core` (placeholder from 8.19). This should have been done as part of 8.1, but listed separately for verification.

**Acceptance Criteria:**
- [ ] Cargo workspace members point to `engine/crates/`
- [ ] `cargo build` succeeds
- [ ] `cargo test` passes

**Reviewer Checks:**
- Run `cargo build` and `cargo test`
- Verify `cargo metadata` shows correct paths

---

### 8.12 — Update template content paths

**What:** Update all plugin templates to use the new `.orqa/` structure (remove `process/` prefix from content paths). Also note missing template types for future creation.

**Files:**
- `templates/full/orqa-plugin.json`
- `templates/frontend/orqa-plugin.json`
- `templates/sidecar/orqa-plugin.json`
- `templates/cli-tool/orqa-plugin.json`

**Steps:**
1. Update content target paths: `.orqa/process/rules` -> `.orqa/learning/rules/`, `.orqa/process/knowledge` -> `.orqa/documentation/<category>/knowledge/`
2. Document missing template types (methodology, workflow, knowledge-only, infrastructure, connector, sidecar) in a TODO or task for future creation

**Acceptance Criteria:**
- [ ] No template references `.orqa/process/`
- [ ] Content paths match the new `.orqa/` structure
- [ ] Missing template types documented for future work

**Reviewer Checks:**
- Grep all templates for `process/`
- Verify content paths match the new target structure

---

### 8.13 — Update CI/CD workflows for new paths

**What:** Update `.forgejo/workflows/` CI files to reference new directory paths.

**Steps:**
1. Review all workflow YAML files for path references
2. Update plugin paths from `plugins/<name>/` to `plugins/<type>/<name>/`
3. Update Rust crate paths from `libs/<name>/` to `engine/crates/<name>/`
4. Update any test/build paths affected by the restructure

**Acceptance Criteria:**
- [ ] All CI workflows reference correct paths
- [ ] CI pipeline passes (if testable locally)
- [ ] No references to old paths

**Reviewer Checks:**
- Grep `.forgejo/workflows/` for old paths
- Verify path references match the new structure

---

### 8.14 — Update import paths across codebase

**What:** After all directory moves (8.1-8.13), do a comprehensive sweep for any remaining references to old paths. This is the catch-all task.

**Steps:**
1. Grep entire codebase for old paths:
   - `libs/validation` (now `engine/crates/validation`)
   - `libs/search` (now `engine/crates/search`)
   - `libs/mcp-server` (now `engine/crates/mcp-server`)
   - `libs/lsp-server` (now `engine/crates/lsp-server`)
   - `plugins/agile-workflow` (now `plugins/methodology/agile-workflow`)
   - `plugins/core` (now `plugins/workflows/core`)
   - `plugins/agile-discovery` through all moved plugins
   - `integrations/claude-agent-sdk` (now `sidecars/claude-agent-sdk`)
   - `infrastructure/sync-bridge` (deleted)
   - `.orqa/process/` (removed — content moved to stage directories)
   - `.orqa/delivery/` (now `.orqa/implementation/`)
   - `.orqa/principles/` (now `.orqa/discovery/`)
   - `.orqa/decisions/` (now `.orqa/learning/decisions/` and `.orqa/planning/decisions/`)
   - `.orqa/lessons/` (now `.orqa/learning/lessons/`)
   - `.orqa/rules/` (now `.orqa/learning/rules/`)
   - `.orqa/knowledge/` (now `.orqa/documentation/<category>/knowledge/`)
2. Fix any remaining references
3. Rebuild everything

**Acceptance Criteria:**
- [ ] Zero grep hits for any old path across the codebase (excluding git history, node_modules, .git)
- [ ] Full build succeeds (`cargo build` + `npm install` + `npm run build` where applicable)
- [ ] `orqa install` succeeds
- [ ] `orqa check` or equivalent passes

**Reviewer Checks:**
- Run the comprehensive grep sweep
- Run full build pipeline
- Run `orqa install` and `orqa check`

---

### 8.15 — Delete `file-audit/` directory

**What:** Delete the `file-audit/` directory at repo root. These are audit working files, not permanent project artifacts.

**Files to delete:**
- `file-audit/01-root-config.md`
- `file-audit/02-orqa-root.md`
- `file-audit/03-orqa-process.md`
- `file-audit/04-orqa-delivery-discovery.md`
- `file-audit/05-plugins.md`
- `file-audit/06-connectors.md`
- `file-audit/07-app.md`
- `file-audit/08-app-frontend.md`
- `file-audit/09-app-backend.md`
- `file-audit/10-app-libs-infra-misc.md`
- `file-audit/phase2-01-governance-gaps.md`
- `file-audit/phase2-02-plugin-manifest-gaps.md`
- `file-audit/phase2-03-connector-gaps.md`
- `file-audit/phase2-04-frontend-gaps.md`
- `file-audit/phase2-05-root-infra-gaps.md`
- `file-audit/phase2-06-proposed-restructure.md`
- `file-audit/migration-tasks-phase1-3.md`
- `file-audit/migration-tasks-phase4-5.md`
- `file-audit/migration-tasks-phase6-8.md` (this file — delete last)
- `file-audit/migration-tasks-phase9-11.md`
- Any other files in file-audit/

**Acceptance Criteria:**
- [ ] `file-audit/` directory deleted entirely
- [ ] No code references these files
- [ ] ARCHITECTURE.md does NOT reference file-audit/ (it should reference only `.orqa/` artifacts)

**Reviewer Checks:**
- Grep for `file-audit` across codebase
- Verify ARCHITECTURE.md and CLAUDE.md have no references to file-audit/

---

### 8.16 — Update CLAUDE.md for new architecture

**What:** Update `.claude/CLAUDE.md` to reflect the new codebase structure after all Phase 8 moves.

**Steps:**
1. Update the Architecture section with new directory structure
2. Update any path references
3. Update Key Design Decisions if any are affected
4. Remove references to deleted directories/files
5. Update the Reference Documents section if file paths changed

**Acceptance Criteria:**
- [ ] CLAUDE.md accurately reflects the new codebase structure
- [ ] No references to old paths (`.orqa/process/`, `.orqa/delivery/`, `.orqa/principles/`, `.orqa/decisions/`, `.orqa/lessons/`, `.orqa/rules/`, `.orqa/knowledge/`, `integrations/`, old plugin paths)
- [ ] All referenced files actually exist at the stated paths

**Reviewer Checks:**
- Verify every path mentioned in CLAUDE.md exists
- Verify the architecture description matches reality

---

### 8.17 — Reconcile relationship type count

**What:** Per ARCHITECTURE.md Phase 8 item 12: "Reconcile relationship type count (41 in plugins vs 30 stated in CLAUDE.md)."

**Steps:**
1. Enumerate all relationship types defined across all plugin schemas
2. Compare with the count stated in CLAUDE.md ("30 relationship types")
3. If the actual count is 41, update CLAUDE.md
4. If some types are duplicates or invalid, clean them up in plugin schemas
5. Document the authoritative list

**Acceptance Criteria:**
- [ ] Actual relationship type count verified across all plugin schemas
- [ ] CLAUDE.md states the correct count
- [ ] Any invalid/duplicate relationship types removed from schemas
- [ ] Authoritative relationship type list documented

**Reviewer Checks:**
- Independently count relationship types across plugin schemas
- Verify CLAUDE.md count matches
- Verify no duplicate type definitions across plugins

---

### 8.18 — Move CLI to top-level `cli/` (ARCHITECTURE.md Phase 8 item 4)

**What:** Move `libs/cli/` to a top-level `cli/` directory. ARCHITECTURE.md Phase 8 item 4 says "Move CLI to top-level `cli/`." The CLI is not a library — it is the primary user-facing command-line tool and deserves top-level placement.

**Steps:**
1. Create `cli/` at repo root
2. Move all contents from `libs/cli/` to `cli/`
3. Update npm workspace: remove `libs/cli` if explicitly listed, add `cli`
4. Update any import/require paths referencing `@orqastudio/cli` or `libs/cli`
5. Update `package.json` paths if needed
6. Verify `npm install` and `orqa` CLI commands work

**Acceptance Criteria:**
- [ ] `cli/` exists at repo root with all contents from `libs/cli/`
- [ ] `libs/cli/` no longer exists
- [ ] npm workspace resolves `@orqastudio/cli` from `cli/`
- [ ] All `orqa` CLI commands work as before
- [ ] No broken imports referencing old path
- [ ] `npm install` succeeds

**Reviewer Checks:**
- Grep for `libs/cli` across codebase — zero hits (excluding git history)
- Run `orqa --help` to verify CLI works
- Run `npm install` from root

---

### 8.19 — Add engine/crates/core/ extraction task placeholder (ARCHITECTURE.md Phase 2 + proposed restructure)

**What:** The proposed restructure describes extracting `app/src-tauri/src/domain/` business logic into `engine/crates/core/` as "the largest structural change" (~8,000+ lines). This is Phase 2 scope (engine extraction), not Phase 8, but the Cargo.toml workspace and directory structure should account for it.

This task creates the placeholder directory and updates the Cargo workspace to include it, so Phase 2 work can proceed against the correct structure.

**Steps:**
1. Create `engine/crates/core/` with a minimal `Cargo.toml` and `src/lib.rs`
2. Add `engine/crates/core` to workspace members in root `Cargo.toml`
3. Document that the actual extraction from `app/src-tauri/src/domain/` is Phase 2 scope

**Acceptance Criteria:**
- [ ] `engine/crates/core/` exists with valid Cargo.toml and minimal lib.rs
- [ ] Root Cargo.toml workspace includes `engine/crates/core`
- [ ] `cargo build` succeeds with the new crate
- [ ] README or comment in lib.rs documents that content extraction is Phase 2 scope

**Reviewer Checks:**
- Verify Cargo.toml is valid and workspace compiles
- Verify the placeholder does not interfere with existing crates

---

### 8.20 — Correct ARCHITECTURE.md Phase 8 wording errors

**What:** Fix known wording errors in ARCHITECTURE.md section 13 Phase 8:

1. Item 1: "Move engine crates to `libs/`" should say "Move engine crates from `libs/` to `engine/crates/`"
2. Item 5: Clarify that `sidecars/` is top-level (not under `plugins/`)

**Files modified:**
- `ARCHITECTURE.md` — fix Phase 8 item wording

**Acceptance Criteria:**
- [ ] Phase 8 item 1 correctly states direction: `libs/` -> `engine/crates/`
- [ ] Phase 8 item 5 clearly states top-level `sidecars/` directory
- [ ] No other ARCHITECTURE.md content changed

**Reviewer Checks:**
- Verify only the identified wording errors are changed
- Verify the corrections match the proposed restructure document

---

### 8.21 — Final build and validation

**What:** After all Phase 8 changes, do a comprehensive build, install, and validation pass.

**Steps:**
1. `cargo build` — verify all Rust code compiles
2. `cargo test` — verify all Rust tests pass
3. `npm install` — verify all npm packages resolve
4. `npm run build` (for app) — verify frontend builds
5. `orqa install` — verify plugin installation works
6. `orqa check` — verify governance validation passes
7. Start the app (`make dev` or equivalent) — verify it launches

**Acceptance Criteria:**
- [ ] `cargo build` succeeds
- [ ] `cargo test` passes (zero failures)
- [ ] `npm install` succeeds (zero errors)
- [ ] Frontend build succeeds
- [ ] `orqa install` succeeds
- [ ] `orqa check` passes
- [ ] App launches without errors

**Reviewer Checks:**
- Run each step independently
- Verify no warnings that indicate structural issues
- Verify the app can display governance artifacts from the new .orqa/ structure

---

## Summary

| Phase | Task Count | Description |
|-------|-----------|-------------|
| Phase 6 | 30 tasks (6.1-6.30) | Content cleanup: scripts, legacy files, artifact review, version fixes |
| Phase 7 | 24 tasks (7.1-7.22, with 7.5 and 7.18 split into a/b) | Governance migration: directory restructure, frontmatter standardization |
| Phase 8 | 21 tasks (8.1-8.21) | Codebase restructure: engine extraction, plugin taxonomy, CLI move, workspace updates |
| **Total** | **75 tasks** | |

### Dependency Chain

```
Phase 6 (Content Cleanup)
  6.1-6.12: Script/file deletion (independent, parallelizable)
  6.13-6.19: Artifact content review (parallelizable across batches)
  6.20: SKILL.md cleanup (depends on knowledge review awareness)
  6.21-6.26: Decision/lesson/rule/epic/task/idea review (parallelizable)
  6.27-6.30: Remaining cleanup (independent)

Phase 7 (Governance Migration)
  7.1-7.5: process/ nesting removal (parallelizable, but 7.8 depends on all)
  7.6-7.7: agents/ removal (sequential — 7.7 depends on 7.6)
  7.8: Remove process/ dir (depends on 7.1-7.7)
  7.9-7.10: grounding/ and personas/ fixes (independent)
  7.11: Decision splitting (depends on 6.21 + 7.1)
  7.12: Knowledge categorization (depends on 6.16-6.19 + 7.2)
  7.13: Documentation categorization (depends on 6.13-6.15)
  7.14: Wireframe type fix (independent)
  7.15-7.17: Frontmatter standardization (parallelizable)
  7.18a: connectors/ usage research (independent)
  7.18b: connectors/ removal (depends on 7.18a)
  7.19-7.21: Config/path updates (depend on 7.1-7.18b)
  7.22: Resolved workflow regeneration (depends on all above)

Phase 8 (Codebase Restructure)
  8.1: Engine crate move (independent, large)
  8.2: daemon/ placeholder (independent, trivial)
  8.3: Plugin taxonomy dirs (independent, trivial)
  8.4-8.8: Plugin moves (depend on 8.3, parallelizable within)
  8.9: sync-bridge removal (independent)
  8.10-8.11: Workspace config (depend on 8.1, 8.4-8.8)
  8.12: Template updates (independent)
  8.13: CI updates (depends on 8.1, 8.4-8.8)
  8.14: Import path sweep (depends on all moves)
  8.15: file-audit/ deletion (independent, do last)
  8.16: CLAUDE.md update (depends on all structure changes)
  8.17: Relationship type reconciliation (independent)
  8.18: CLI move to top-level (independent, can parallel with plugin moves)
  8.19: engine/crates/core/ placeholder (depends on 8.1)
  8.20: ARCHITECTURE.md wording fixes (independent)
  8.21: Final validation (depends on everything)
```
