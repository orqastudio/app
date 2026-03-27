# .orqa/process/ Inventory

Factual inventory of all artifacts in `.orqa/process/`. No recommendations or judgments.

---

## 1. agents/ (19 files)

All files follow the `AGENT-<8hex>.md` naming pattern. All are type `agent` with YAML frontmatter.

| ID | Title | Plugin / Scope | Role | Inherits | Model | Status | Key Relationships |
|----|-------|---------------|------|----------|-------|--------|-------------------|
| AGENT-065a25cc | Rust Specialist | @orqastudio/plugin-rust | Implementer (Rust) | AGENT-e5dd38e4 | sonnet | active | employs KNOW-59077955, KNOW-694ff7cb, KNOW-d4095bd9; serves PILLAR-c9e0a695, PERSONA-477971bf |
| AGENT-0aad40f4 | Designer | core | Designer | -- | sonnet | active | employs KNOW-13348442, KNOW-0619a413; serves PILLAR-c9e0a695, PERSONA-2721ae35 |
| AGENT-26e5029d | Rust Standards Agent | @orqastudio/plugin-rust | Task agent (assess/configure) | -- | sonnet | active | employs KNOW-d4095bd9, KNOW-694ff7cb; serves PILLAR-c9e0a695, PERSONA-477971bf |
| AGENT-336e4d7d | OrqaStudio Integration Specialist | project-specific | Implementer (integration) | -- | sonnet | active | employs KNOW-4f81ddc5, KNOW-33b2dc14, KNOW-207d9e2c, KNOW-b5f520d5, KNOW-8615fee2, KNOW-0d6c1ece; serves 3 pillars, 2 personas |
| AGENT-4c94fe14 | Orchestrator | core | Process coordinator | -- | sonnet | active | employs KNOW-13348442, KNOW-0619a413, KNOW-21d28aa0, KNOW-e3432947, KNOW-57365826; serves 3 pillars, 1 persona |
| AGENT-5de8c14f | Svelte Specialist | @orqastudio/plugin-svelte | Implementer (Svelte 5) | AGENT-e5dd38e4 | sonnet | active | employs KNOW-50382247, KNOW-4260613a, KNOW-3642842e; serves PILLAR-c9e0a695, PERSONA-477971bf |
| AGENT-65b56a0b | Tauri Standards Agent | @orqastudio/plugin-tauri | Task agent (assess/configure) | -- | sonnet | active | employs KNOW-73490bde; serves PILLAR-c9e0a695, PERSONA-477971bf |
| AGENT-6f55de0d | Svelte Standards Agent | @orqastudio/plugin-svelte | Task agent (assess/configure) | -- | sonnet | active | employs KNOW-6cfacbb2, KNOW-3642842e; serves PILLAR-c9e0a695, PERSONA-477971bf |
| AGENT-7a06d10e | Governance Enforcer | @orqastudio/plugin-agile-workflow | Enforcement specialist | -- | sonnet | active | employs KNOW-ee860ed9; serves PILLAR-c9e0a695 |
| AGENT-85be6ace | Planner | core | Planner | -- | inherit | active | employs KNOW-5611351f, KNOW-13348442, KNOW-0619a413, KNOW-21d28aa0, KNOW-9ff8c63f; serves PILLAR-a6a4bbbb, PERSONA-c4afd86b |
| AGENT-867da593 | OrqaStudio Rust Backend Specialist | project-specific | Implementer (Rust backend) | -- | sonnet | active | employs KNOW-8615fee2, KNOW-207d9e2c, KNOW-60aefbbc, KNOW-4f81ddc5, KNOW-fbc200e6; serves 2 pillars, 1 persona |
| AGENT-8e58cd87 | Reviewer | core | Reviewer | -- | inherit | active | employs KNOW-13348442, KNOW-0619a413; serves PILLAR-2acd86c1, PERSONA-477971bf |
| AGENT-ae63c406 | Governance Steward | core | Governance (3 roles) | -- | sonnet | active | employs KNOW-13348442, KNOW-0619a413, KNOW-e3432947, KNOW-57365826; serves PILLAR-a6a4bbbb, PILLAR-c9e0a695 |
| AGENT-bbad3d30 | Writer | core | Writer | -- | sonnet | active | employs KNOW-1afbc656, KNOW-13348442, KNOW-0619a413, KNOW-21d28aa0; serves PILLAR-c9e0a695, PERSONA-477971bf |
| AGENT-ce86fb50 | Plugin Developer | core | Plugin Developer | -- | sonnet | active | employs KNOW-13348442, KNOW-0619a413, KNOW-2f38309a, KNOW-e6fee7a0, KNOW-e3432947, KNOW-57365826; serves PILLAR-a6a4bbbb, PILLAR-c9e0a695 |
| AGENT-d1be3776 | Installer | core | Task agent (install) | -- | sonnet | active | employs KNOW-f5ee4e0d; serves PILLAR-a6a4bbbb, PERSONA-2721ae35 |
| AGENT-e333508b | Researcher | core | Researcher | -- | inherit | active | employs KNOW-13348442, KNOW-0619a413, KNOW-21d28aa0, KNOW-9ff8c63f; serves PILLAR-2acd86c1, PILLAR-c9e0a695 |
| AGENT-e5a1b6bf | OrqaStudio Svelte Frontend Specialist | project-specific | Implementer (Svelte frontend) | -- | sonnet | active | employs KNOW-0d6c1ece, KNOW-b5f520d5, KNOW-882d8c4f, KNOW-d00093e7, KNOW-50382247; serves 2 pillars, 1 persona |
| AGENT-e5dd38e4 | Implementer | core | Implementer (generic) | -- | sonnet | active | employs KNOW-13348442, KNOW-0619a413; serves PILLAR-c9e0a695, PERSONA-477971bf |

### Agent Taxonomy Summary

- **Core universal roles (8):** Orchestrator, Implementer, Reviewer, Researcher, Writer, Planner, Designer, Installer
- **Plugin-provided specialists (5):** Rust Specialist, Svelte Specialist, Governance Enforcer, Rust Standards Agent, Svelte Standards Agent, Tauri Standards Agent
- **Project-specific specialists (3):** OrqaStudio Rust Backend, OrqaStudio Svelte Frontend, OrqaStudio Integration
- **Plugin Developer (1):** Plugin Developer
- **Governance Steward (1):** Governance Steward (operates in 3 roles)
- **Task agents (non-conversational) (4):** Rust Standards, Svelte Standards, Tauri Standards, Installer

### Inheritance

Only 2 agents use `inherits`: Rust Specialist and Svelte Specialist both inherit from Implementer (AGENT-e5dd38e4).

### Model Distribution

- `sonnet`: 15 agents
- `inherit`: 4 agents (Planner, Reviewer, Researcher, Implementer)

---

## 2. knowledge/ (119 entries)

**Total count:** 119 items (114 KNOW-*.md files + 5 subdirectories)

### Non-KNOW Entries (5 subdirectories, each containing a SKILL.md)

| Directory | SKILL.md Description | user-invocable |
|-----------|---------------------|----------------|
| `diagnostic-methodology/` | Root cause analysis: capture, reproduce, isolate, fix, verify | true |
| `governance-context/` | How to read/use artifact graph, relationship vocabulary, status model | false |
| `planning/` | Documentation-first planning: Document -> Approve -> Implement -> Verify | false |
| `plugin-setup/` | OrqaStudio plugin setup for Claude Code (ID: KNOW-5ebf82dc) | true |
| `search/` | Unified MCP search: regex, semantic, research modes | true |

### Knowledge Samples (15 files)

| ID | Title | Has summary | Has tier | Has roles | Has paths | Has tags | Status | Approximate Size |
|----|-------|-------------|----------|-----------|-----------|----------|--------|-----------------|
| KNOW-0188373b | Delivery Unit Completion Discipline | yes | no | no | no | no | active | ~40 lines |
| KNOW-1afbc656 | Orqa Documentation Authoring | yes | on-demand | writer | .orqa/** | documentation, authoring, markdown, links | active | ~100 lines |
| KNOW-21d28aa0 | Planning | yes | no | no | no | no | active | ~30 lines |
| KNOW-2f38309a | Plugin Development | empty string | no | no | no | no | active | ~30+ lines |
| KNOW-4260613a | Svelte 5 Patterns | yes (in body) | no | no | no | no | active | ~30+ lines |
| KNOW-50382247 | Svelte 5 Best Practices | yes | no | no | ui/src/lib/** | no | active | ~30+ lines |
| KNOW-57365826 | Query Artifact Schemas Before Writing Frontmatter | yes | always | * | no | governance, schema, validation, frontmatter | active | ~30+ lines |
| KNOW-694ff7cb | Rust Testing Patterns | yes (in body) | no | no | no | no | active | ~30+ lines |
| KNOW-85e392ea | Thinking Mode - Learning Loop | yes | no | no | no | no | active | ~30+ lines |
| KNOW-9ff8c63f | Research Methodology | yes | no | no | no | no | active | ~30+ lines |
| KNOW-b5f520d5 | Orqa Store Patterns | yes | stage-triggered | implementer | app/ui/src/lib/stores/** | svelte5, stores, runes, state-management | active | ~30+ lines |
| KNOW-d4095bd9 | Clippy Config Management | yes (in body) | no | no | no | no | active | ~30+ lines |
| KNOW-e3432947 | Plugin-Canonical Architecture | yes | always | * | no | governance, plugin, architecture, content-placement | active | ~30+ lines |
| KNOW-ee860ed9 | Enforcement Patterns | yes | no | no | no | no | active | ~30+ lines |
| KNOW-f7fb7aa7 | Thinking Mode: Implementation | yes | no | no | no | no | active | ~30+ lines |

### Frontmatter Schema Observations

Knowledge files use **inconsistent frontmatter schemas**:
- Some use `title`, others use `name`, some use both
- Some have `tier`/`roles`/`paths`/`tags`/`priority` (newer format with injection metadata), others have none of these
- Some have `plugin` field (indicating plugin origin), others do not
- Some have `category`, `file-patterns`, `user-invocable`, `version` fields
- Some have `injection` field with `artifact_types` and `keywords` (newer injection format)
- The `summary` field is present in most but empty in a few

### Complete Knowledge File List

```
KNOW-0188373b.md  KNOW-4a4241a5.md  KNOW-8c359ea4.md
KNOW-03421ec0.md  KNOW-4a58e7dd.md  KNOW-8cc0f5e4.md
KNOW-0444355f.md  KNOW-4f81ddc5.md  KNOW-8d1c4be6.md
KNOW-0619a413.md  KNOW-50382247.md  KNOW-8d2e5eef.md
KNOW-0d6c1ece.md  KNOW-51de8fb7.md  KNOW-8d76c3c7.md
KNOW-126aa140.md  KNOW-5611351f.md  KNOW-91a7a6c1.md
KNOW-1314ac47.md  KNOW-5704b089.md  KNOW-936e5944.md
KNOW-13348442.md  KNOW-57365826.md  KNOW-96aaa407.md
KNOW-16e91c20.md  KNOW-586bfa9a.md  KNOW-990e4f85.md
KNOW-1a4f41f7.md  KNOW-59077955.md  KNOW-9ff8c63f.md
KNOW-1afbc656.md  KNOW-5efbe925.md  KNOW-a0947420.md
KNOW-1b7fa054.md  KNOW-5f4db8f7.md  KNOW-a16b7bc7.md
KNOW-1c2d005d.md  KNOW-60aefbbc.md  KNOW-a1a195c1.md
KNOW-1da7ecd8.md  KNOW-694ff7cb.md  KNOW-a274d90d.md
KNOW-1ea9291c.md  KNOW-6cfacbb2.md  KNOW-a3dcdd05.md
KNOW-1f4aba8f.md  KNOW-6d80cf39.md  KNOW-a4e351bc.md
KNOW-207d9e2c.md  KNOW-71352dc8.md  KNOW-a53d826c.md
KNOW-21d28aa0.md  KNOW-72ca209f.md  KNOW-a700e25a.md
KNOW-22783288.md  KNOW-73490bde.md  KNOW-abb08445.md
KNOW-2876afc7.md  KNOW-7a4e45d4.md  KNOW-afaa4e88.md
KNOW-2a846fb7.md  KNOW-7c871921.md  KNOW-b320cae8.md
KNOW-2bf2b321.md  KNOW-7fadba3f.md  KNOW-b5f520d5.md
KNOW-2f38309a.md  KNOW-83039175.md  KNOW-b95ec6e3.md
KNOW-33b2dc14.md  KNOW-8564d52c.md  KNOW-be54e4de.md
KNOW-3642842e.md  KNOW-85a449e7.md  KNOW-bec7e87d.md
KNOW-36befd20.md  KNOW-85e392ea.md  KNOW-bf70068c.md
KNOW-37496474.md  KNOW-8615fee2.md  KNOW-c4d3e52b.md
KNOW-3d946f9a.md  KNOW-882d8c4f.md  KNOW-c89f28b3.md
KNOW-3f307edb.md  KNOW-d00093e7.md  KNOW-ea7898e4.md
KNOW-40be8113.md  KNOW-d03337ac.md  KNOW-ea78c8e4.md
KNOW-40e2eb99.md  KNOW-d13d80e1.md  KNOW-ecc181cb.md
KNOW-41849545.md  KNOW-d4095bd9.md  KNOW-ee860ed9.md
KNOW-4260613a.md  KNOW-dd5062c9.md  KNOW-eeceaabf.md
KNOW-45b5f8a8.md  KNOW-df3c489e.md  KNOW-f5ee4e0d.md
KNOW-46f68631.md  KNOW-e3432947.md  KNOW-f7d03a2c.md
KNOW-477f2c9c.md  KNOW-e484802a.md  KNOW-f7fb7aa7.md
KNOW-481059d2.md  KNOW-e6fee7a0.md  KNOW-fbc200e6.md
KNOW-498ca38a.md  KNOW-e89753ad.md  KNOW-fd636a56.md
```

Plus 5 subdirectories: `diagnostic-methodology/`, `governance-context/`, `planning/`, `plugin-setup/`, `search/`

---

## 3. rules/ (59 files)

**Total count:** 59 files. All follow the `RULE-<8hex>.md` naming pattern.

### Rule Samples (15 files)

| ID | Title | Status | What It Constrains |
|----|-------|--------|-------------------|
| RULE-00700241 | System Command Safety | active | Blocks dangerous shell commands (rm -rf, sudo, eval, destructive SQL). Has PreToolUse blocking hooks. |
| RULE-0be7765e | Error Ownership | active | All errors are agent's responsibility. No skipping failures. Has pre-commit `make check`. |
| RULE-1b238fc8 | Vision Alignment | active | Every feature must serve at least one active pillar. Behavioral enforcement. |
| RULE-3c2da849 | Core Graph Firmware Protection | active | Core graph artifacts are non-editable except through update system or dogfood mode. PostToolUse warning hooks. |
| RULE-42d17086 | Tooling Ecosystem Management | active | Code quality belongs in linters, not regex. Linter config must match documented standards. |
| RULE-5dd9decd | Honest Reporting | active | Partial work must not be reported as complete. Stop hook checks for missing sections. |
| RULE-87ba1b81 | Agent Delegation | active | Orchestrator coordinates but does not implement. Stop hook verifies. |
| RULE-99abcea1 | Use agent teams for implementation | active | Orchestrator must delegate via Agent tool with teams. Behavioral enforcement. |
| RULE-998da8ea | Dogfood Mode | active | Conditional safety when `dogfood: true`. Tauri restart, sidecar, HMR safety. PreToolUse blocking hooks. |
| RULE-af5771e3 | No Stubs or Placeholders | active | No fake data, TODO functions, or scaffolded implementations. PostToolUse file checks. |
| RULE-b10fe6d1 | Artifact Lifecycle | active | Status transitions, promotion gates, documentation gates. `orqa enforce` tool + pre-commit. |
| RULE-d543d759 | Honest Status Reporting | active | Partial work must be reported as partial. Behavioral enforcement. |
| RULE-dccf4226 | Plan Mode Compliance | active | Plans need Architectural Compliance + Systems Architecture Checklist. Behavioral enforcement. |
| RULE-ec9462d8 | Documentation-First Implementation | active | Docs exist before code. Update docs before changing code. Behavioral enforcement. |
| RULE-f609242f | Git Workflow | active | Worktree-based, no --no-verify, no force push to main. PreToolUse blocking hooks. |

### Complete Rule File List

```
RULE-00700241.md  RULE-42d17086.md  RULE-8ee65d73.md  RULE-c603e90e.md
RULE-04684a16.md  RULE-43f1bebc.md  RULE-97e96528.md  RULE-cd426b0d.md
RULE-05562ed4.md  RULE-4603207a.md  RULE-9814ec3c.md  RULE-d2c2063a.md
RULE-05ae2ce7.md  RULE-484872ef.md  RULE-998da8ea.md  RULE-d543d759.md
RULE-09a238ab.md  RULE-49f66888.md  RULE-99abcea1.md  RULE-d5d28fba.md
RULE-0be7765e.md  RULE-4dbb3612.md  RULE-af1cd87d.md  RULE-dccf4226.md
RULE-0d29fc91.md  RULE-5965256d.md  RULE-af5771e3.md  RULE-dd5b69e6.md
RULE-145332dc.md  RULE-5d2d39b7.md  RULE-aff3c5db.md  RULE-e1f1afc1.md
RULE-1b238fc8.md  RULE-5dd9decd.md  RULE-b03009da.md  RULE-eb269afb.md
RULE-205d9c91.md  RULE-63cc16ad.md  RULE-b10fe6d1.md  RULE-ec9462d8.md
RULE-216e112e.md  RULE-71352dc8.md  RULE-b2584e59.md  RULE-ef822519.md
RULE-23699df2.md  RULE-83411442.md  RULE-b723ea53.md  RULE-f23392dc.md
RULE-25baac14.md  RULE-87ba1b81.md  RULE-c382e053.md  RULE-f3dca71e.md
RULE-2f64cc63.md  RULE-8aadfd6c.md                    RULE-f609242f.md
RULE-30a223ca.md  RULE-8abcbfd5.md
RULE-3c2da849.md  RULE-8cb4bd04.md
```

---

## 4. decisions/ (70 files)

**Total count:** 70 files. All follow the `AD-<8hex>.md` naming pattern. Type is `discovery-decision`.

### Decision Samples (8 files)

| ID | Title | Status | Summary |
|----|-------|--------|---------|
| AD-1ef9f57c | Team design v2 -- open question resolutions | active | Resolves 10 open questions from RES-d6e8ab11: no workflow inheritance, declarative guards only, forward-compatible versioning, etc. |
| AD-2d58941b | Error Propagation via Result Types | completed | All Rust functions return Result<T,E> with thiserror. No unwrap/expect/panic in production. |
| AD-45cfe1d1 | Config-Driven Artifact Scanning | completed | Scanner reads paths from project.json, not hardcoded constants. Recursive walking with frontmatter extraction. |
| AD-7fa3f280 | Task-First Audit Trail with Optional Epics | completed | Task is the universal unit of work. Epic linkage is configurable via `epics-required` setting. |
| AD-8727f99a | Rename tmp to .state | active | .state/ for session state, token metrics, hook telemetry -- operational data, not disposable temp files. |
| AD-c6c2d9fb | Rule promotion requires enforcement | completed | Lessons must not be promoted to rules without enforcement mechanism. Three layers: artifact graph, Rust app, Claude plugin. |
| AD-e711446e | Vision Evolution -- Domain-Agnostic Clarity Engine | completed | OrqaStudio evolved from dev tool to domain-agnostic clarity engine. Pillars renamed to Clarity Through Structure, Learning Through Reflection. |
| AD-fc4e9013 | Max Subscription Authentication | completed | Primary auth via Claude Max subscription through Agent SDK. API keys deferred to roadmap. |

### Complete Decision File List

```
AD-02a2a97b.md  AD-37894b70.md  AD-7121ec20.md  AD-a7fd7461.md  AD-d01b9e0a.md
AD-03d9007d.md  AD-39e2fb81.md  AD-74a2cb7a.md  AD-af88bb69.md  AD-e4a3b5da.md
AD-09fc4e65.md  AD-3b986859.md  AD-75bb14ae.md  AD-b08f456d.md  AD-e711446e.md
AD-0dfa4d52.md  AD-430829f1.md  AD-7cb83077.md  AD-b2f1f829.md  AD-e7ca1e94.md
AD-1ef9f57c.md  AD-45cfe1d1.md  AD-7d3d7521.md  AD-b741a7df.md  AD-e8a0f910.md
AD-23e27cf5.md  AD-45f32bab.md  AD-7fa3f280.md  AD-b8c11db0.md  AD-e8ea9fb9.md
AD-26b0eb9f.md  AD-487e045a.md  AD-80f39962.md  AD-bb125c2c.md  AD-ecc96aef.md
AD-26d8d45d.md  AD-48b310f9.md  AD-859ed163.md  AD-c1e5a39e.md  AD-ee2910b1.md
AD-2d58941b.md  AD-4e7faf0e.md  AD-85d45674.md  AD-c6c2d9fb.md  AD-efe10e1d.md
AD-2dc8ab96.md  AD-4ea9a290.md  AD-8727f99a.md  AD-c8f41d2e.md  AD-f079c196.md
AD-306d7320.md  AD-5d0f8814.md  AD-8746bd92.md  AD-cd06a937.md  AD-f9fbd1d7.md
AD-306eccf1.md  AD-5e87a65b.md  AD-9687b3cf.md  AD-33e315cc.md  AD-fc4e9013.md
AD-339e9223.md  AD-628b058b.md  AD-9a7d7256.md
AD-33e315cc.md  AD-6ccfb736.md  AD-9ab3e0a4.md
                                AD-9baf6ee9.md
                                AD-a1c2ca06.md
                                AD-a44384d1.md
                                AD-a4762810.md
                                AD-a47f313a.md
```

---

## 5. lessons/ (84 files)

**Total count:** 84 files. All follow the `IMPL-<8hex>.md` naming pattern. Type is `lesson`.

### Lesson Samples (8 files)

| ID | Title | Status | Maturity | Recurrence | Lesson Learned |
|----|-------|--------|----------|------------|---------------|
| IMPL-023a772e | Duplicate relationship types on same artifact | archived | observation | 1 | Auto-inverse system added both forward and inverse to same artifact instead of target |
| IMPL-286bdc1f | Symlinks Prevent Governance Divergence | completed | understanding | 1 | .claude/ as copies diverged from .orqa/ source of truth; must use symlinks |
| IMPL-5c77c349 | Dogfooding proves the methodology before the app | captured | observation | 1 | CLI-only session demonstrated full governance without the app running |
| IMPL-81eeac00 | Plugin installation must wire capabilities into agents and skills | active | observation | 1 | Installing a plugin must update agents and skills, not just register capability mappings |
| IMPL-a1373533 | Observation logging should be automated | completed | understanding | 1 | Learning loop depends on manual discipline which breaks under cognitive load |
| IMPL-c8e2803a | Research artifact not triggered before investigation | archived | observation | 1 | Orchestrator jumped to audit without creating RES-NNN artifact first |
| IMPL-e4b1aca1 | Frontend must route through Tauri invoke | active | -- | 1 | Direct fetch() to localhost blocked by Tauri CSP; must use invoke bridge |
| IMPL-f70ed197 | Recording observations is not scope creep | completed | understanding | 1 | Capture should be automatic; scope decisions are triage, not capture |

### Complete Lesson File List

```
IMPL-023a772e.md  IMPL-42dd183e.md  IMPL-81eeac00.md  IMPL-b86a46ad.md  IMPL-d66af988.md
IMPL-043f12f1.md  IMPL-45ad587c.md  IMPL-84bc013a.md  IMPL-b8ec72e2.md  IMPL-d7f93105.md
IMPL-0809b549.md  IMPL-4eed88ef.md  IMPL-9177c9bd.md  IMPL-bc62d0ba.md  IMPL-da5e11c9.md
IMPL-08d70280.md  IMPL-516733d4.md  IMPL-91d951b6.md  IMPL-c03fc142.md  IMPL-db7ab92c.md
IMPL-092cc947.md  IMPL-53fc59b5.md  IMPL-935110b6.md  IMPL-c57282b4.md  IMPL-db8027b6.md
IMPL-0c9a5882.md  IMPL-5b380b2e.md  IMPL-9468f103.md  IMPL-c66533e6.md  IMPL-df17079f.md
IMPL-1cccef7a.md  IMPL-5c4bc3d5.md  IMPL-9520fb0b.md  IMPL-c726abc2.md  IMPL-e13eb86c.md
IMPL-1dbed312.md  IMPL-5c77c349.md  IMPL-984941fb.md  IMPL-c8e2803a.md  IMPL-e3399a4d.md
IMPL-1f868d3b.md  IMPL-638deb6d.md  IMPL-9e486d51.md  IMPL-ca2f3f46.md  IMPL-e4b1aca1.md
IMPL-26576988.md  IMPL-66b7b217.md  IMPL-9eed2036.md  IMPL-cc7e9025.md  IMPL-e4b95213.md
IMPL-286bdc1f.md  IMPL-69246e36.md  IMPL-a1373533.md  IMPL-d331eb1e.md  IMPL-e77277bd.md
IMPL-28997580.md  IMPL-6a8f9612.md  IMPL-a73db2e6.md  IMPL-d3804cd0.md  IMPL-e95470f3.md
IMPL-2dc87f24.md  IMPL-6b8ab467.md  IMPL-a83c0678.md  IMPL-d6129c85.md  IMPL-f2822dfb.md
IMPL-30230427.md  IMPL-6e796c09.md  IMPL-a91ac743.md                    IMPL-f3629976.md
IMPL-30c3da78.md  IMPL-79f34490.md  IMPL-acea0394.md                    IMPL-f39f3824.md
IMPL-36b767ce.md                    IMPL-b080f0d4.md                    IMPL-f3ee3f16.md
IMPL-3f31499b.md                    IMPL-b19a7e02.md                    IMPL-f70ed197.md
                                    IMPL-b27c458f.md                    IMPL-f7201eb5.md
                                    IMPL-b36b00af.md                    IMPL-f7bc3b13.md
                                                                        IMPL-ffb199b5.md
```

---

## 6. workflows/ (14 YAML files)

### Artifact Lifecycle Workflows (7 files)

These govern individual artifact type lifecycles. All provided by `@orqastudio/plugin-core-framework`.

| Filename | Artifact Type | Plugin | States | Initial State |
|----------|--------------|--------|--------|---------------|
| agent.workflow.yaml | agent | plugin-core-framework | captured, exploring, active, hold, review, completed, surpassed, archived | captured |
| decision.workflow.yaml | decision | plugin-core-framework | captured, exploring, active, hold, review, completed, surpassed, archived | captured |
| doc.workflow.yaml | doc | plugin-core-framework | captured, exploring, active, hold, review, completed, surpassed, archived | captured |
| knowledge.workflow.yaml | knowledge | plugin-core-framework | captured, exploring, active, hold, review, completed, surpassed, archived | captured |
| lesson.workflow.yaml | lesson | plugin-core-framework | captured, exploring, active, recurring, review, completed, promoted, surpassed, archived | captured |
| rule.workflow.yaml | rule | plugin-core-framework | captured, exploring, active, inactive, hold, review, completed, surpassed, archived | captured |
| wireframe.workflow.yaml | wireframe | plugin-agile-planning | captured, exploring, active, hold, review, completed, surpassed, archived | captured |

### Planning Artifact Workflows (3 files)

These govern planning-stage artifact lifecycles. All provided by `@orqastudio/plugin-agile-planning`.

| Filename | Artifact Type | Plugin | States | Initial State |
|----------|--------------|--------|--------|---------------|
| planning-decision.workflow.yaml | planning-decision | plugin-agile-planning | proposed, reviewing, hold, resolved, deferred, archived | proposed |
| planning-idea.workflow.yaml | planning-idea | plugin-agile-planning | draft, evaluating, accepted, hold, rejected, archived | draft |
| planning-research.workflow.yaml | planning-research | plugin-agile-planning | proposed, investigating, hold, concluded, archived | proposed |

### Contribution Workflows (4 files)

These contribute sub-workflows to the `agile-methodology` skeleton workflow via contribution points. Each injects states into a specific stage of the epic lifecycle.

| Filename | Plugin | Contributes To | Contribution Point | Priority | States Added |
|----------|--------|---------------|-------------------|----------|-------------|
| documentation.contribution.workflow.yaml | plugin-agile-documentation | agile-methodology | documentation-standards | 10 | draft_docs, review_docs, publish_docs |
| learning.contribution.workflow.yaml | plugin-core-framework | agile-methodology | learning-pipeline | 10 | capture, retrospective, pattern-tracking, recurrence-detection, learning_complete |
| planning.contribution.workflow.yaml | plugin-agile-planning | agile-methodology | planning-methodology | 10 | scope_analysis, estimation, prioritisation, plan_finalised |
| review.contribution.workflow.yaml | plugin-agile-review | agile-methodology | review-process | 10 | code_review, gate_check, acceptance_verification, review_complete |

### Workflow Architecture Summary

The workflow system uses two patterns:
1. **Standalone workflows** -- govern individual artifact types with their own state machine (agent, decision, doc, knowledge, lesson, rule, wireframe, planning-decision, planning-idea, planning-research)
2. **Contribution workflows** -- inject sub-state-machines into extension points of the `agile-methodology` skeleton workflow (documentation, learning, planning, review)

All workflows include:
- `migration` sections mapping old status values to new ones
- `on_enter` actions (set_field, append_log) for state transitions
- `guards` on some transitions (only planning.contribution has a field_check guard)

The lesson workflow is unique in having a `recurring` state and `promoted` terminal state. The rule workflow is unique in having an `inactive` state for demoted rules.

---

## Summary Statistics

| Category | Count |
|----------|-------|
| Agents | 19 |
| Knowledge (KNOW-*.md) | 114 |
| Knowledge (subdirectories with SKILL.md) | 5 |
| Rules | 59 |
| Decisions | 70 |
| Lessons | 84 |
| Workflows | 14 |
| **Total artifacts** | **365** |
