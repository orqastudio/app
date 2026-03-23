---
id: "RES-dd35bbf0"
type: research
title: "Principle enforcement gap audit"
description: "Comprehensive audit of the OrqaStudio governance framework to identify where principles, rules, architecture decisions, and pipeline stages lack mechanical enforcement. Findings feed into EPIC-4e6e9eae priority dimensions and task planning."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
sources: []
relationships:
  - target: "EPIC-4e6e9eae"
    type: "guides"
    rationale: "Research findings informed the design of Principle enforcement foundations"
---
## Research Questions

1. **Rules without mechanical enforcement**: Which active rules are only enforced by agent self-compliance — no hook, linter, schema validation, or tooling backs them up?
2. **Pillar enforcement coverage**: For each pillar's gate questions, what tooling can answer them? Where are the gaps?
3. **AD enforcement chains**: Which accepted architecture decisions have no `enforced-by` relationship and no corresponding rule?
4. **Unpromoted lessons**: Which active/recurring lessons should have been promoted to rules or skills but haven't been?
5. **Pipeline stage transitions**: At each stage of the knowledge maturity pipeline, does tooling exist to detect when artifacts should advance? Where are the dead ends?
6. **Priority dimension signals**: What patterns emerge from the gaps that suggest the right priority dimensions for this project?

## Audit Scope

| Artifact Type | Location | Count | What to Check |
|---------------|----------|-------|---------------|
| Rules | `.orqa/process/rules/` | ~44 | Enforcement mechanism per rule |
| Pillars | `.orqa/process/pillars/` | ~3 | Gate question coverage |
| Lessons | `.orqa/process/lessons/` | 47 | Promotion status |
| Decisions | `.orqa/process/decisions/` | 46 | Enforcement chain |
| Ideas | `.orqa/delivery/ideas/` | ~78 | Principle gap signals |

## Findings

### 1. Rules Without Mechanical Enforcement

All 45 rules have `status: active`. The pre-commit hook (`.githooks/pre-commit`) runs the following mechanical checks:

- **Rust files**: `make format-check`, `make lint-backend`, `make test-rust`
- **Frontend files**: `make typecheck`, `make lint-frontend`, `make test-frontend`
- **Artifact files (.orqa/)**: `autolink-artifacts.mjs`, `validate-artifacts.sh` (schema validation via `validate-schema.mjs`), `verify-links.mjs --staged --check-bidirectional`, `verify-pipeline-integrity.mjs --staged`
- **Production code**: `scan-stubs.sh` (stub/TODO/unwrap scanner)
- **Task files**: `validate-task-deps.sh` (dependency gate)
- **Epic files**: `validate-epic-readiness.sh` (docs-required gate)
- **Core graph protection**: blocks commits touching firmware files (unless dogfood=true)

| Rule | Title | Enforcement Mechanism | Gap |
|------|-------|----------------------|-----|
| [RULE-532100d9](RULE-532100d9) | Agent Delegation | Self-compliance only | No tooling verifies the orchestrator delegates rather than implements. Purely behavioral. |
| [RULE-65973a88](RULE-65973a88) | Architecture Decisions | Self-compliance only | No check that code changes comply with accepted ADs. Agents must read them manually. |
| [RULE-6c0496e0](RULE-6c0496e0) | Artifact Config Integrity | Partial: schema validation checks frontmatter; `verify-links.mjs --check-paths` checks stale paths | No automatic check that `project.json` artifact paths match disk directories. |
| [RULE-7b770593](RULE-7b770593) | Artifact Lifecycle | Hook: `validate-task-deps.sh` (dependency gate), `validate-epic-readiness.sh` (docs-required gate), schema validation (status enums) | Status transition order not enforced (e.g., nothing blocks `draft->in-progress` skipping `ready`). Milestone completion gate not enforced. |
| [RULE-5e03e67b](RULE-5e03e67b) | Code Search Usage | Self-compliance only | No check that agents use semantic search before grep/glob. Purely behavioral. |
| [RULE-b49142be](RULE-b49142be) | Coding Standards | Linter: clippy, eslint, rustfmt, svelte-check; Hook: `scan-stubs.sh` for unwrap/TODO | Function size limits (<=50 lines) not mechanically enforced. Coverage threshold (80%) not enforced in hook. |
| [RULE-c71f1c3f](RULE-c71f1c3f) | Development Commands | Self-compliance only | No check that `make` targets are used instead of raw cargo/npm. Purely behavioral. |
| [RULE-9daf29c0](RULE-9daf29c0) | Documentation-First | Partial: `validate-epic-readiness.sh` enforces docs-required gate | No check that docs are updated BEFORE code. The hook only checks epic status transitions, not temporal ordering. |
| [RULE-6083347d](RULE-6083347d) | Dogfood Mode | Partial: core graph protection in pre-commit checks dogfood flag | Restart protocol, sidecar self-edit warnings are self-compliance only. |
| [RULE-1acb1602](RULE-1acb1602) | End-to-End Completeness | Self-compliance only | No check that all four layers (command, IPC type, component, store) exist for a feature. |
| [RULE-3eccebf3](RULE-3eccebf3) | Enforcement Before Code | Self-compliance only | No check that enforcement artifacts are created before implementation code. |
| [RULE-57ccb4a3](RULE-57ccb4a3) | Error Ownership | Partial: pre-commit hook runs checks; clippy catches unwrap | The principle "all errors are YOUR responsibility" is behavioral. The hook enforces the checks, not the ownership mindset. |
| [RULE-633e636d](RULE-633e636d) | Git Workflow | Partial: hook blocks `--no-verify`; stub scanner runs | Worktree creation/cleanup, stash policy, untracked file policy are self-compliance only. |
| [RULE-6d1c8dc7](RULE-6d1c8dc7) | Historical Artifact Preservation | Self-compliance only | No check that research/tasks are marked surpassed rather than deleted. No check that outdated docs are deleted. |
| [RULE-878e5422](RULE-878e5422) | Honest Reporting | Self-compliance only | No tooling verifies agent completion reports include "What Is NOT Done" section. |
| [RULE-22783309](RULE-22783309) | IDs Are Not Priority | Self-compliance only | No check that agents don't sort by ID to imply priority. Purely behavioral. |
| [RULE-551bde31](RULE-551bde31) | Lessons Learned | Self-compliance only | No check that review agents create IMPL entries on FAIL verdicts. Recurrence tracking is manual. |
| [RULE-9ba80a19](RULE-9ba80a19) | No Aliases or Hacks | Self-compliance only | No linter rule catches alias maps or fallback patterns. |
| [RULE-e120bb70](RULE-e120bb70) | No Deferred Deliverables | Self-compliance only | No check that epic scope matches roadmap. |
| [RULE-e9c54567](RULE-e9c54567) | No Stubs or Placeholders | Hook: `scan-stubs.sh` catches TODO/FIXME/HACK/unwrap/dbg!/console.log | Hardcoded return values pretending to be real data are not caught by the scanner. Only syntactic patterns, not semantic stubs. |
| [RULE-39169bcd](RULE-39169bcd) | Pillar Alignment in Docs | Self-compliance only | No check that documentation pages have a Pillar Alignment section. |
| [RULE-303c1cc8](RULE-303c1cc8) | Plan Mode Compliance | Self-compliance only | No check that plans include architectural compliance or UX-first sections. |
| [RULE-b2753bad](RULE-b2753bad) | Required Reading | Self-compliance only | No check that agents read their required reading before implementation. |
| [RULE-cb65b5d0](RULE-cb65b5d0) | Reusable Components | Self-compliance only | No linter rule catches inline empty/loading/error states vs shared components. |
| [RULE-1f30904a](RULE-1f30904a) | Root Directory Cleanliness | Self-compliance only | No check on what files exist in root. |
| [RULE-deab6ea7](RULE-deab6ea7) | Skill Enforcement | Self-compliance only | No check that agents load skills before starting work. |
| [RULE-8035e176](RULE-8035e176) | Structure Before Work | Self-compliance only | No check that epics/tasks exist before implementation begins. |
| [RULE-d90112d9](RULE-d90112d9) | Systems Thinking First | Self-compliance only | Purely philosophical/behavioral. |
| [RULE-f10bb5de](RULE-f10bb5de) | Testing Standards | Partial: `make test-rust` and `make test-frontend` run in hook | Coverage threshold (80%) not enforced. Test organisation conventions not checked. |
| [RULE-4d4f540d](RULE-4d4f540d) | UAT Process | Self-compliance only | No tooling enforces UAT phases or process improvement encoding. |
| [RULE-1e8a1914](RULE-1e8a1914) | Vision Alignment | Partial: pillar relationships required by `verify-pipeline-integrity.mjs` | No check that features trace to a pillar gate question. Relationship existence is checked, not semantic accuracy. |
| [RULE-a764b2ae](RULE-a764b2ae) | Artifact Schema Compliance | Hook: `validate-schema.mjs` via `validate-artifacts.sh` | Fully mechanically enforced for staged .orqa/ files. |
| [RULE-89155a7f](RULE-89155a7f) | Tooltips over title attributes | Self-compliance only | No linter rule catches `title=` on interactive elements. |
| [RULE-2f7b6a31](RULE-2f7b6a31) | Artifact Link Format | Hook: `autolink-artifacts.mjs` auto-links bare IDs; `verify-links.mjs` checks link validity | Mechanically enforced. Bare IDs are auto-linked; broken links are flagged. |
| [RULE-11c29c9e](RULE-11c29c9e) | Skill Portability | Self-compliance only | No check that core skills avoid project-specific paths. |
| [RULE-df24948b](RULE-df24948b) | Context Window Management | Self-compliance only | Purely behavioral. |
| [RULE-f809076f](RULE-f809076f) | Tool Access Restrictions | Self-compliance only | No enforcement mechanism restricts which tools each role can use. |
| [RULE-5ee43922](RULE-5ee43922) | User-Invocable Skill Semantics | Schema: `user-invocable` field is required by skill schema | Schema enforces field presence but not correctness of the value. |
| [RULE-e352fd0a](RULE-e352fd0a) | Session Management | Self-compliance only | No check that session state is written or that commits happen before session end. |
| [RULE-92dba0cb](RULE-92dba0cb) | Provider-Agnostic Capabilities | Self-compliance only | No check that agent definitions use capabilities instead of tools. |
| [RULE-c95f4444](RULE-c95f4444) | Data Persistence Boundaries | Self-compliance only | No check that governance data stays in files and conversation data stays in SQLite. |
| [RULE-f9d0279c](RULE-f9d0279c) | Automated Skill Injection | Self-compliance only (designed for app enforcement engine, not yet built) | Path-to-skill mapping exists in the rule but no tooling implements injection. |
| [RULE-7f416d7d](RULE-7f416d7d) | Tooling Ecosystem Management | Linter: the linters themselves are the enforcement | Meta-rule about maintaining linter-to-standard mappings. No check that mappings are complete. |
| [RULE-98682b5e](RULE-98682b5e) | Core Graph Firmware Protection | Hook: pre-commit blocks commits touching core paths (unless dogfood=true or ORQA_CORE_OVERRIDE=1) | Fully mechanically enforced. |
| [RULE-130f1f63](RULE-130f1f63) | Data Integrity | Hook: `verify-links.mjs --check-bidirectional`, `verify-pipeline-integrity.mjs` | Mechanically enforced for staged files. Link resolution, bidirectional consistency, and pipeline integrity all checked. |

**Summary**: Of 45 active rules, **7 have full mechanical enforcement** (RULE-a764b2ae, [RULE-2f7b6a31](RULE-2f7b6a31), [RULE-98682b5e](RULE-98682b5e), [RULE-130f1f63](RULE-130f1f63), and partially [RULE-b49142be](RULE-b49142be), [RULE-e9c54567](RULE-e9c54567), RULE-7b770593). **27 rules rely entirely on self-compliance** with no mechanical check whatsoever. The remaining **11 rules have partial enforcement** where some aspects are checked but significant gaps remain.

### 2. Pillar Gate Coverage

#### [PILLAR-569581e0](PILLAR-569581e0): Clarity Through Structure

| Gate Question | Tooling Coverage | Gap |
|---------------|-----------------|-----|
| Does this make governance artifacts visible and manageable? | `verify-links.mjs` checks artifact integrity; schema validation ensures well-formed artifacts | No check that artifacts are *browsable in the app*. Artifact scanner existence is assumed, not verified per-commit. |
| Does it produce structured knowledge (plans, decisions, rules)? | Schema validation ensures correct frontmatter structure | No check that work PRODUCES artifacts. Only checks existing artifact validity. |
| Does it enforce a workflow that ensures understanding precedes action? | `validate-epic-readiness.sh` checks docs-required; `validate-task-deps.sh` checks dependencies | Partial coverage. Documentation-first ordering is not enforced. |
| Does it surface what would otherwise be hidden? | None | No tooling measures visibility. This is a product design question, not a commit-time check. |

#### [PILLAR-cdf756ff](PILLAR-cdf756ff): Learning Through Reflection

| Gate Question | Tooling Coverage | Gap |
|---------------|-----------------|-----|
| Does this capture lessons, discoveries, or patterns? | `verify-pipeline-integrity.mjs` checks that governance artifacts have relationships | No check that review agents create IMPL entries. Lesson creation is entirely manual. |
| Does it track metrics or outcomes that show improvement? | None | No metrics tracking exists. No trend analysis tooling. |
| Does it feed retrospectives back into the governance framework? | None | Lesson promotion is manual. No tooling detects when recurrence >= 2 and suggests promotion. |
| Does it accumulate knowledge across sessions? | None | Session state is manual (`tmp/session-state.md`). No cross-session knowledge accumulation tooling. |

#### [PILLAR-94b281db](PILLAR-94b281db): Purpose Through Continuity

| Gate Question | Tooling Coverage | Gap |
|---------------|-----------------|-----|
| Does this help users stay oriented toward their original purpose? | None | No mid-cycle orientation tooling. No scope drift detection. |
| Does this prevent knowledge/decisions/context from being silently lost? | None | No decision persistence or pending-question tracking. |
| Does this make scope drift visible and require explicit approval? | None | No scope tracking between epic/task definition and actual implementation. |
| Does this reduce the user's cognitive burden? | None | No cognitive load measurement or indicator. |

**Summary**: Pillar 1 has partial tooling coverage (schema validation, link verification, dependency gates). Pillar 2 has almost no tooling coverage (only pipeline integrity checking for relationship presence). Pillar 3 has zero tooling coverage.

### 3. Architecture Decisions Without Enforcement Chain

Of 46 ADs, 4 are superseded (AD-dffc3d30, [AD-8b91f5a4](AD-8b91f5a4), [AD-69072318](AD-69072318), AD-40b3de0a), leaving 41 accepted decisions (AD-29b5eb06 has no explicit status in the data but is accepted). Only 4 ADs have any relationship pointing to a rule.

| AD | Title | Has enforced-by/practices relationship? | Corresponding Rule? | Gap |
|----|-------|----------------------------------------|---------------------|-----|
| [AD-e513c9e4](AD-e513c9e4) | Thick Backend Architecture | No | No direct rule | No enforcement that domain logic stays in backend. |
| [AD-a334623b](AD-a334623b) | IPC Boundary Design | No | [RULE-1acb1602](RULE-1acb1602) covers end-to-end but not IPC-specific | No check that `invoke()` is the only frontend-backend interface. |
| [AD-1ad08e5f](AD-1ad08e5f) | Error Propagation via Result Types | No | [RULE-b49142be](RULE-b49142be) references this; clippy catches unwrap | Partially enforced via linter. No explicit AD->RULE relationship. |
| [AD-8d552e96](AD-8d552e96) | Svelte 5 Runes Only | No | [RULE-b49142be](RULE-b49142be) references this; svelte-check catches Svelte 4 | Partially enforced via linter. No explicit AD->RULE relationship. |
| [AD-61087142](AD-61087142) | Component Purity | No | [RULE-b49142be](RULE-b49142be) references this | No linter check that components don't call `invoke()`. Self-compliance only. |
| [AD-dc919e52](AD-dc919e52) | Agent SDK Sidecar Integration | No | No rule | No enforcement. Architecture pattern, not a constraint. |
| [AD-22650b3b](AD-22650b3b) | Max Subscription Authentication | No | No rule | No enforcement. Business decision. |
| [AD-fcd55d44](AD-fcd55d44) | Streaming Pipeline | No | No rule | No enforcement on streaming architecture. |
| [AD-0dbba717](AD-0dbba717) | Tool Implementation as MCP | No | No rule | No enforcement. |
| [AD-5d9ac6bd](AD-5d9ac6bd) | Security Model | No | No rule | No enforcement of security boundaries. |
| [AD-834fc71a](AD-834fc71a) | Tauri Plugin Selections | No | No rule | No enforcement. Technology selection, not a constraint. |
| [AD-afc78f6e](AD-afc78f6e) | Frontend Library Selections | No | No rule | No enforcement. |
| [AD-4047ceb1](AD-4047ceb1) | Onboarding Strategy | No | No rule | No enforcement. Strategy, not a constraint. |
| [AD-1d928079](AD-1d928079) | Composability Principle | No | No rule (skill: composability) | Enforced via skill injection, not via rule. No mechanical check. |
| [AD-4f5277f0](AD-4f5277f0) | Three-Zone Layout | No | No rule | No enforcement of layout structure. |
| [AD-8b7c4ac5](AD-8b7c4ac5) | Filesystem-Driven Doc Browsing | No | [RULE-6c0496e0](RULE-6c0496e0) partially | No direct enforcement chain. |
| [AD-6ce44025](AD-6ce44025) | .orqa/ as Single Source of Truth | No | [RULE-6c0496e0](RULE-6c0496e0) partially | No check that .claude/ only contains symlinks. |
| [AD-0c56aa90](AD-0c56aa90) | Config-Driven Artifact Scanning | No | [RULE-6c0496e0](RULE-6c0496e0) | No direct relationship declared. |
| [AD-c3700062](AD-c3700062) | Plans Merged Into Research | No | No rule | Structural decision. No enforcement needed. |
| [AD-99c2a969](AD-99c2a969) | Native Search Engine | No | No rule | Architecture decision. No enforcement needed. |
| [AD-ff88ecea](AD-ff88ecea) | Provider-Agnostic AI Integration | No | [RULE-92dba0cb](RULE-92dba0cb) partially | No check on provider-agnosticism in code. |
| [AD-6cd1ff6f](AD-6cd1ff6f) | Domain Service Extraction | No | No rule | No enforcement of extraction pattern. |
| [AD-44a033cf](AD-44a033cf) | Vision Evolution | No | [RULE-1e8a1914](RULE-1e8a1914) covers alignment | No direct relationship. |
| [AD-53e80192](AD-53e80192) | Three-Tier Skill Loading | No | [RULE-deab6ea7](RULE-deab6ea7) | No direct relationship. |
| [AD-774cc3d0](AD-774cc3d0) | Universal Roles | No | [RULE-532100d9](RULE-532100d9) | No direct relationship. |
| [AD-6f0dea5e](AD-6f0dea5e) | Skill-Driven Project Init | No | No rule | No enforcement. |
| [AD-47c41f0c](AD-47c41f0c) | Pillars as First-Class Artifacts | No | No rule | Schema enforces pillar structure. |
| [AD-2aa4d6db](AD-2aa4d6db) | SQLite for Conversations Only | No | [RULE-c95f4444](RULE-c95f4444) | No direct relationship. No enforcement. |
| [AD-0291fa65](AD-0291fa65) | Core UI Boundary | No | No rule | No enforcement of navigate/search/edit scope. |
| [AD-89391ab6](AD-89391ab6) | Schema-Driven Filtering | No | No rule | No enforcement. |
| [AD-2f1991c2](AD-2f1991c2) | Config-Driven Nav Defaults | No | No rule | No enforcement. |
| [AD-d8ea4d2b](AD-d8ea4d2b) | Cross-Linking as Default | No | [RULE-2f7b6a31](RULE-2f7b6a31), [RULE-130f1f63](RULE-130f1f63) | Cross-linking is enforced by tooling but no AD->RULE relationship. |
| [AD-0f6286cd](AD-0f6286cd) | AI-Driven Cross-Artifact Search | No | No rule | No enforcement. |
| [AD-2783985c](AD-2783985c) | Graph-Based Knowledge Injection | Yes: enforced-by [RULE-98682b5e](RULE-98682b5e) | [RULE-98682b5e](RULE-98682b5e) | Has relationship. |
| [AD-e8ab8572](AD-e8ab8572) | Core Graph Firmware | Yes: enforced-by [RULE-98682b5e](RULE-98682b5e) | [RULE-98682b5e](RULE-98682b5e) | Has relationship. |
| [AD-6dfbba70](AD-6dfbba70) | Task-First Audit Trail | Yes: enforced-by [RULE-7b770593](RULE-7b770593), [RULE-8035e176](RULE-8035e176) | [RULE-7b770593](RULE-7b770593), [RULE-8035e176](RULE-8035e176) | Has relationships. |
| [AD-20d6719d](AD-20d6719d) | CLI Rule Loading | No | No rule | Interim decision. No enforcement needed. |
| [AD-a76663db](AD-a76663db) | Knowledge Maturity Pipeline | No | No rule | Framework decision. `verify-pipeline-integrity.mjs` partially enforces. |
| [AD-71d44f5c](AD-71d44f5c) | Three-Level Directory Structure | No | [RULE-6c0496e0](RULE-6c0496e0) partially | No direct relationship. |
| [AD-8b3962f6](AD-8b3962f6) | Standards Distribution Pattern | No | No rule | Framework decision. No enforcement. |
| [AD-ea4a5979](AD-ea4a5979) | Design for App First, Port to CLI | Yes: enforced-by [RULE-98682b5e](RULE-98682b5e), practices [RULE-f9d0279c](RULE-f9d0279c) | [RULE-98682b5e](RULE-98682b5e), [RULE-f9d0279c](RULE-f9d0279c) | Has relationships. |
| [AD-29b5eb06](AD-29b5eb06) | Conciseness over Granularity | No | No rule | Meta-decision. No enforcement. |

**Summary**: Only **4 out of 41 accepted ADs** have explicit `enforced-by` or `grounded-by` relationships to rules. Many ADs that DO have corresponding rules lack the relationship edge (e.g., [AD-1ad08e5f](AD-1ad08e5f) is enforced by clippy via [RULE-b49142be](RULE-b49142be) but has no relationship declaring this). The graph is structurally incomplete — enforcement chains exist in practice but are not declared in the artifact relationships.

### 4. Unpromoted Lessons

Of 47 lessons:
- 22 have `status: promoted` but ALL have empty `evolves-into` fields
- 25 have `status: active`
- 1 (IMPL-6ba34dea) has `recurrence: 2`, all others have `recurrence: 1`

**Promoted lessons with missing `evolves-into` targets:**

| Lesson | Title | Status | Recurrence | promoted-to | Issue |
|--------|-------|--------|------------|-------------|-------|
| [IMPL-5331c7b9](IMPL-5331c7b9) | Config paths must match disk paths exactly | promoted | 1 | (empty) | Promoted to [RULE-6c0496e0](RULE-6c0496e0) likely, but relationship not set |
| [IMPL-b3e47100](IMPL-b3e47100) | Symlinks Prevent Governance Divergence | promoted | 1 | (empty) | Promoted to [RULE-6c0496e0](RULE-6c0496e0) likely, but relationship not set |
| [IMPL-91ad864c](IMPL-91ad864c) | Investigate systemically before fixing | promoted | 1 | (empty) | Promoted to [RULE-d90112d9](RULE-d90112d9) likely, but not linked |
| [IMPL-8a971261](IMPL-8a971261) | Encode Improvements in Artifacts | promoted | 1 | (empty) | Promoted to [RULE-3eccebf3](RULE-3eccebf3) likely, but not linked |
| [IMPL-1bd6ed0c](IMPL-1bd6ed0c) | Epic titles describe outcomes not process | promoted | 1 | (empty) | Promoted to [RULE-7b770593](RULE-7b770593) likely, but not linked |
| [IMPL-2e30e4ab](IMPL-2e30e4ab) | Commit at natural boundaries | promoted | 1 | (empty) | Promoted to [RULE-633e636d](RULE-633e636d) likely, but not linked |
| [IMPL-c306b136](IMPL-c306b136) | Hardcoded .orqa/ paths should be configurable | promoted | 1 | (empty) | Target unknown |
| [IMPL-e22b63b4](IMPL-e22b63b4) | Plugins and sidecars are paired | promoted | 1 | (empty) | Target unknown |
| [IMPL-a97eccb6](IMPL-a97eccb6) | Plugins from other sidecars should be disabled | promoted | 1 | (empty) | Target unknown |
| [IMPL-97e2788f](IMPL-97e2788f) | Open items not tracked as tasks | promoted | 1 | (empty) | Target unknown |
| [IMPL-b149653d](IMPL-b149653d) | Epics with all tasks done should surface | promoted | 1 | (empty) | Target unknown |
| [IMPL-f2b140da](IMPL-f2b140da) | Observation logging should be automated | promoted | 1 | (empty) | Target unknown |
| [IMPL-ccb23768](IMPL-ccb23768) | Lessons should be recorded on task completion | promoted | 1 | (empty) | Target unknown |
| [IMPL-257c8303](IMPL-257c8303) | Lessons must flow forward | promoted | 1 | (empty) | Target unknown |
| [IMPL-6ba34dea](IMPL-6ba34dea) | Orchestrator writes directly instead of delegating | promoted | 2 | (empty) | Only lesson with recurrence >= 2. Has promoted-to relationship to [RULE-532100d9](RULE-532100d9) in relationships array but `evolves-into` field is empty |
| [IMPL-7b47abee](IMPL-7b47abee) | Recording observations is not scope creep | promoted | 1 | (empty) | Target unknown |
| [IMPL-9d2eaf01](IMPL-9d2eaf01) | Observation creation should be delegated | promoted | 1 | (empty) | Target unknown |
| [IMPL-4e9b3f2d](IMPL-4e9b3f2d) | Decisions scrolled out must be resurfaced | promoted | 1 | (empty) | Target unknown |
| [IMPL-336adfc0](IMPL-336adfc0) | Orchestrator should think critically about suggestions | promoted | 1 | (empty) | Target unknown |
| [IMPL-f27a1550](IMPL-f27a1550) | Epic body drifts from actual work | promoted | 1 | (empty) | Target unknown |
| [IMPL-40b0b5ca](IMPL-40b0b5ca) | Idea promotion should surface related ideas | promoted | 1 | (empty) | Target unknown |
| [IMPL-8d666f0c](IMPL-8d666f0c) | User prompt should infer intent and auto-record | promoted | 1 | (empty) | Target unknown |

**Active lessons that may warrant review for promotion:**

| Lesson | Title | Status | Recurrence | Maturity | Should Promote? | Why |
|--------|-------|--------|------------|----------|----------------|-----|
| [IMPL-eb748de2](IMPL-eb748de2) | Vite Optimize After New Dependencies | active | 1 | observation | No | Single occurrence. Operational, not a principle. |
| [IMPL-7d4e3af9](IMPL-7d4e3af9) | Kill existing dev server processes | active | 1 | observation | No | Already covered by [RULE-c71f1c3f](RULE-c71f1c3f). |
| [IMPL-e53df28b](IMPL-e53df28b) | Orchestrator must manage dev lifecycle | active | 1 | understanding | No | Already covered by [RULE-c71f1c3f](RULE-c71f1c3f). |
| [IMPL-ec3ab819](IMPL-ec3ab819) | Svelte 5 $derived requires .by() | active | 1 | understanding | Yes | Common gotcha. Could be added to svelte5-best-practices skill. |
| [IMPL-d5cbc9c7](IMPL-d5cbc9c7) | Don't refactor agentic structure while delegating | active | 1 | understanding | Yes | Important meta-principle. Could become a rule or skill addition. |
| [IMPL-bb34f3cf](IMPL-bb34f3cf) | Extract domain logic before commands become monolithic | active | 1 | understanding | No | Covered by [AD-6cd1ff6f](AD-6cd1ff6f) (Domain Service Extraction). |
| [IMPL-687d20fb](IMPL-687d20fb) | Domain-neutral naming avoids renames | active | 1 | understanding | No | Naming convention. Low urgency. |
| [IMPL-3de505e8](IMPL-3de505e8) | Documentation must be app-native | active | 1 | understanding | No | Product decision, not enforcement. |
| [IMPL-a4643199](IMPL-a4643199) | Process skills at orchestration level | active | 1 | understanding | No | Covered by [RULE-deab6ea7](RULE-deab6ea7) tier model. |
| [IMPL-f40a935f](IMPL-f40a935f) | Orchestrator deferred scoped deliverable | active | 1 | understanding | No | Covered by [RULE-e120bb70](RULE-e120bb70). |
| [IMPL-262e63e1](IMPL-262e63e1) | Directory reorgs leave stale paths | active | 1 | understanding | Yes | Recurring pattern. `verify-links.mjs --check-paths` partially addresses, but could become a stronger enforcement. |
| [IMPL-994d2276](IMPL-994d2276) | Present decisions one at a time | active | 1 | observation | No | UX pattern, not enforcement. |
| [IMPL-bc5fb64a](IMPL-bc5fb64a) | Plugin requirements shaped by plugin type | active | 1 | observation | No | Design insight, not enforcement. |
| [IMPL-2f360a49](IMPL-2f360a49) | AI provider requirements are bidirectional | active | 1 | observation | No | Design insight. |
| [IMPL-3b5fe409](IMPL-3b5fe409) | Capability fulfilment should be user choice | active | 1 | observation | No | Design insight. |
| [IMPL-6c76ba27](IMPL-6c76ba27) | Capability config is per-project | active | 1 | observation | No | Design insight. |
| [IMPL-3201693d](IMPL-3201693d) | Plugin installation must wire capabilities | active | 1 | observation | No | Design insight. |
| [IMPL-4c101745](IMPL-4c101745) | Blocking vs non-blocking observation handling | active | 1 | observation | No | Design insight. |
| [IMPL-a2bec916](IMPL-a2bec916) | Orchestrator should periodically sanity-check scope | active | 1 | observation | Yes | Relates to Pillar 3 (scope drift). No rule covers this. |
| [IMPL-230f6f2e](IMPL-230f6f2e) | Design discussions need structured mode | active | 1 | observation | No | Feature idea, not enforcement. |
| [IMPL-3148326e](IMPL-3148326e) | Design discussion mode should be a skill | active | 1 | understanding | No | Architecture insight. |
| [IMPL-79ad82af](IMPL-79ad82af) | App UI save can destructively overwrite project.json | active | 1 | understanding | Yes | Data integrity bug. Should become a fix task, not a rule. |
| [IMPL-6936bcf3](IMPL-6936bcf3) | Null relationship targets should link to tracking artifact | active | 1 | understanding | No | Already being addressed by tooling. |
| [IMPL-eed5b00a](IMPL-eed5b00a) | Project-level prioritization needed | active | 1 | observation | No | Design insight for priority system. |
| [IMPL-6f4106d7](IMPL-6f4106d7) | Research artifact not triggered before investigation | active | 1 | observation | No | Single occurrence. Covered by [RULE-8035e176](RULE-8035e176). |

### 5. Pipeline Stage Transition Gaps

The knowledge maturity pipeline ([AD-a76663db](AD-a76663db), [AD-8b3962f6](AD-8b3962f6)) defines stages: Observation -> Understanding -> Principle (AD) -> Practice (Skill) -> Enforcement (Rule) -> Verification.

| Transition | Tooling Exists? | Gap |
|------------|----------------|-----|
| Observation -> Understanding | No | No tooling detects when a lesson's maturity should advance. Maturity is set manually. No trigger for "this observation has been analysed." |
| Understanding -> Principle (AD) | No | No tooling detects when an understood pattern should become an architecture decision. Manual promotion only. |
| Principle -> Practice (Skill) | No | No tooling links ADs to skills that implement them. No check that accepted ADs have corresponding skills. |
| Practice -> Enforcement (Rule) | No | No tooling detects when a skill pattern needs rule enforcement. The `evolves-into` field on lessons is supposed to track this but is empty on ALL 22 promoted lessons. |
| Enforcement -> Verification | Partial: `verify-pipeline-integrity.mjs` checks relationship integrity; pre-commit hook runs relevant checks per rule | The pre-commit hook implements verification for rules that have mechanical enforcement. But 27 rules have no verification at all. |
| Recurrence detection (any stage) | No | No tooling detects recurring patterns across lessons. Recurrence is manually incremented. Only 1 of 47 lessons has recurrence > 1. |
| Promotion suggestion (recurrence >= 2) | No | [RULE-551bde31](RULE-551bde31) says recurrence >= 2 triggers promotion review, but no tooling surfaces this. The one lesson with recurrence=2 (IMPL-6ba34dea) was manually promoted. |

**Summary**: The pipeline has **zero automated stage transitions**. Every advancement through the pipeline is manual. The `verify-pipeline-integrity.mjs` tool checks structural integrity (relationships exist, no deprecated fields) but does not assess readiness for advancement. The pipeline is a conceptual framework with no automation driving artifacts through it.

### 6. Summary: Gap Patterns

**Pattern 1: Relationship graph is declared but unpopulated.** The pipeline model requires `enforced-by`, `grounded`, and `evolves-into` edges to trace enforcement chains. In practice: 37 of 41 accepted ADs lack enforcement relationships; all 22 promoted lessons lack `evolves-into` targets; rules reference ADs in their prose but not in structured relationships.

**Pattern 2: Self-compliance is the dominant enforcement mechanism.** 27 of 45 rules (60%) have no mechanical enforcement. These rules are enforced entirely by agents reading them and choosing to comply. This works when agents are reliable, but creates no safety net for compliance drift.

**Pattern 3: Pillar 3 (Purpose Through Continuity) has zero tooling.** Pillar 1 has schema validation, link verification, and dependency gates. Pillar 2 has pipeline integrity checking. Pillar 3 has nothing — no scope drift detection, no decision persistence, no cognitive load indicators, no mid-cycle orientation.

**Pattern 4: The learning loop is conceptual, not operational.** The Observation -> Enforcement pipeline exists as a framework, but no automation drives artifacts through it. Recurrence tracking is manual (and rarely incremented). Promotion is manual (and the target is never recorded). The loop runs only when agents remember to run it.

**Pattern 5: Linter-enforceable rules are under-leveraged.** Several rules describe patterns that linters COULD enforce but don't: component purity (no `invoke()` in `$lib/components/`), function size limits (>50 lines), tooltip usage (`title=` on interactive elements), reusable component usage (inline empty states). These are low-hanging fruit for mechanical enforcement.

**Pattern 6: Hook enforcement is strong where it exists.** The pre-commit hook covers: schema validation, link verification, pipeline integrity, stub scanning, task dependencies, epic readiness, and core graph protection. The gap is not in hook infrastructure but in the number of rules that have no hook check at all.

**Priority dimension signals for [EPIC-4e6e9eae](EPIC-4e6e9eae):**

1. **Data integrity** (relationship completeness, promoted-to targets) — the graph is structurally incomplete
2. **Pipeline automation** (stage transition detection, recurrence tracking) — the learning loop needs mechanical drivers
3. **Linter expansion** (component purity, function size, tooltip usage) — add enforcement to rules that CAN be mechanically checked
4. **Pillar 3 tooling** (scope drift, decision persistence) — the newest pillar has no supporting infrastructure
5. **Relationship backfill** (AD->Rule edges, lesson->Rule promoted-to) — the graph edges that exist in prose but not in structured data