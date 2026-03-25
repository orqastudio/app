---
id: "RES-b7062d7b"
type: research
title: "Governance Layer Audit — Core vs Project vs Plugin Alignment"
description: "Full audit of all governance artifacts (rules, decisions, lessons, skills, agents, docs) for layer classification accuracy, portability violations, and alignment with graph-based injection principles"
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
sources:
  - "url: file://.orqa/process/rules/"
  - "url: file://.orqa/process/decisions/"
  - "url: file://.orqa/process/lessons/"
  - "url: file://.orqa/process/skills/"
  - "url: file://.orqa/process/agents/"
  - "url: file://.orqa/documentation/"
  - "url: file://.orqa/plugins/orqastudio-claude-plugin/"
relationships:
  - target: "EPIC-770f9ce9"
    type: "guides"
    rationale: "Research findings informed epic design"
  - target: "AD-efe10e1d"
    type: "informs"
---
## Findings

### Layer Classification of Rules

44 rules audited. 8 rules marked `layer: core` contain project-specific content that violates portability:

| Rule | Universal Principle | Project-Specific Content |
|------|--------------------|-----------------------|
| [RULE-ec9462d8](RULE-ec9462d8) | Documentation-first | References Tauri/Svelte/Rust paths in body |
| [RULE-0be7765e](RULE-0be7765e) | Error ownership | Mentions cargo/clippy/npm |
| [RULE-c382e053](RULE-c382e053) | No aliases/hacks | Rust serde examples, TypeScript union examples |
| [RULE-af5771e3](RULE-af5771e3) | No stubs | Tauri invoke patterns, OrqaStudio-specific examples |
| [RULE-dccf4226](RULE-dccf4226) | Plan compliance | Systems Architecture Checklist lists Tauri commands, Svelte components |
| [RULE-97e96528](RULE-97e96528) | Root cleanliness | "What Belongs in Root" table lists Cargo.toml, svelte.config.js |
| [RULE-dd5b69e6](RULE-dd5b69e6) | Skill enforcement | Tier 2 injection table lists all orqa-* skills with file-pattern triggers |
| [RULE-1b238fc8](RULE-1b238fc8) | Vision alignment | "Foundational Principles" lists Tauri v2 + Svelte 5 + Rust + SQLite stack |

**Pattern:** Each rule correctly identifies a universal principle but embeds project-specific examples/checklists in the body. Fix: extract project content into project-layer companion rules or skills.

### Decisions Assessment

40 decisions. 4 supersession chains — all consistent (AD-75bb14ae→[AD-859ed163](AD-859ed163), [AD-b08f456d](AD-b08f456d)→[AD-859ed163](AD-859ed163), [AD-0dfa4d52](AD-0dfa4d52)→[AD-4ea9a290](AD-4ea9a290), [AD-85d45674](AD-85d45674)→AD-7cb83077). Recent graph decisions (AD-f079c196, [AD-45f32bab](AD-45f32bab), AD-7fa3f280) are current and aligned.

**Issue:** Early decisions (AD-7121ec20 through AD-33e315cc) have no `layer` field. Later decisions (AD-80f39962+) all have `layer: core`. Schema may not enforce `layer` on decisions.

### Lessons Assessment

15 lessons. 5 promoted (chains verified). 10 active — all still relevant. No stale or contradictory lessons found. [IMPL-9468f103](IMPL-9468f103) (don't refactor while delegating to agents) is a strong promotion candidate.

### Skills Assessment — Critical Issues

**[KNOW-f5ee4e0d](KNOW-f5ee4e0d) ID collision (CRITICAL):** Three skills share ID [KNOW-f5ee4e0d](KNOW-f5ee4e0d):
1. `plugin-setup` (plugin directory)
2. `plugin-setup` (symlinked to team/skills)
3. `research-methodology`

**KNOW-bcfeb64e duplication without symlink:** `rule-enforcement` exists as separate copies in both `team/skills/` (layer: core) and `plugin/skills/` (layer: plugin). Content has diverged.

**Core-layer portability violations:**
- `composability` (KNOW-0619a413): 37 project-specific references (Svelte components, sidecar paths, Tauri patterns)
- `orqa-native-search` (KNOW-40addb7a): Contains OrqaStudio-specific implementation details, should be `layer: project`
- `rule-enforcement` (team/skills copy): References OrqaStudio Rust enforcement engine paths, should be `layer: project`

**Layer/universality contradiction:** `orqa-code-search` (KNOW-1c5e752e) is `layer: project` but [RULE-dd5b69e6](RULE-dd5b69e6) lists it as universal (every agent must have it).

### Agents Assessment

7 agents, all using `capabilities` field (aligned with RULE-8abcbfd5). [KNOW-f5ee4e0d](KNOW-f5ee4e0d) ID collision means agents referencing it get undefined behavior. The Planner and Researcher agents both reference [KNOW-f5ee4e0d](KNOW-f5ee4e0d).

### Documentation Assessment

- `orchestration.md` and `workflow.md` describe pre-graph-based patterns (ChunkHound-specific)
- No architecture doc exists for graph-based injection model (AD-f079c196 accepted but no implementation doc)
- 15 wireframe/UI docs frozen at 2026-03-04 through 2026-03-07, likely outdated
- 3 rules have `scope: []` (empty) — [RULE-09a238ab](RULE-09a238ab), [RULE-e1f1afc1](RULE-e1f1afc1), [RULE-42d17086](RULE-42d17086)

### Plugin Assessment

Plugin hooks correctly implement graph-based injection. No remnants of old index-based approaches. The `rule-enforcement` skill duplication (not symlinked like `plugin-setup`) is the only issue.

### Rule Loading Tension

All 44 rules are loaded via `.claude/rules/` symlinks into every CLI session. This contradicts the "inject IDs only" principle — full rule bodies are injected regardless of relevance. This is the most significant architectural tension with the graph model.

## Recommendations

1. Fix [KNOW-f5ee4e0d](KNOW-f5ee4e0d) ID collision immediately
2. Fix KNOW-bcfeb64e duplication — symlink or assign distinct IDs
3. Split 8 core rules: extract project-specific content into project-layer companions
4. Split composability skill: core principle + project examples
5. Fix skill layer misclassifications (orqa-native-search, rule-enforcement)
6. Update stale process docs (orchestration.md, workflow.md)
7. Create graph injection architecture doc
8. Address `.claude/rules/` full-body loading vs graph-based injection tension