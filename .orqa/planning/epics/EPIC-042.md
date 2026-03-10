---
id: EPIC-042
title: Three-Tier Skill Loading
description: "Implement AD-028: restructure skill loading into three tiers — portable agent skills (Tier 1), orchestrator-injected project skills (Tier 2), and wrapper skills that resolve context-dependent implementations (Tier 3)."
status: done
priority: P1
created: 2026-03-09
updated: 2026-03-09
milestone: MS-001
pillars:
  - PILLAR-001
research-refs: []
docs-required: []
docs-produced:
  - .orqa/governance/decisions/AD-028.md
scoring:
  user-value: 4
  pillar-alignment: 5
  dependency-weight: 5
  effort: 3
  risk: 2
  score: 19
---
## Implementation Design

### Overview

Migrate from the current model (all skills on agent definitions) to three tiers:
- **Tier 1 (Agent):** Portable language/framework skills + wrapper skills in agent YAML
- **Tier 2 (Orchestrator):** Project-specific `orqa-*` skills injected at task creation
- **Tier 3 (Wrapper):** `code-search` wrapper that detects CLI vs App and loads the right skill

### Task Breakdown

1. Create `code-search` wrapper skill with context detection logic
2. Update all agent definitions: remove `orqa-*` and `chunkhound`/`orqa-native-search`, add `code-search`
3. Create orchestrator skill injection table mapping task scope → project skills
4. Update orchestrator to inject Tier 2 skills in delegation prompts
5. Update `skill-enforcement.md` to reflect three-tier model
6. Verify: dry-run a delegation to confirm all three tiers load correctly

### Skill Classification

| Tier | Skills |
|------|--------|
| Tier 1 (Agent) | `code-search`, `architecture`, `rust-async-patterns`, `svelte5-best-practices`, `typescript-advanced-types`, `tailwind-design-system`, `planning`, `skills-maintenance` |
| Tier 2 (Orchestrator) | `orqa-composability`, `orqa-ipc-patterns`, `orqa-store-patterns`, `orqa-store-orchestration`, `orqa-streaming`, `orqa-domain-services`, `orqa-repository-pattern`, `orqa-error-composition`, `orqa-governance`, `orqa-testing`, `orqa-native-search` |
| Tier 3 (Wrapper) | `code-search` → resolves to `chunkhound` (CLI) or `orqa-native-search` (App) |

### Orchestrator Injection Table

| Task Scope | Injected Skills |
|-----------|----------------|
| `src-tauri/src/commands/` | `orqa-ipc-patterns`, `orqa-error-composition` |
| `src-tauri/src/domain/` | `orqa-domain-services`, `orqa-error-composition` |
| `src-tauri/src/repo/`, `src-tauri/src/db.rs` | `orqa-repository-pattern` |
| `src-tauri/src/search/` | `orqa-native-search` |
| `sidecar/src/` | `orqa-streaming` |
| `ui/lib/stores/` | `orqa-store-patterns`, `orqa-store-orchestration` |
| `ui/lib/components/` | `orqa-store-patterns` |
| `.orqa/` | `orqa-governance` |
| Any streaming work | `orqa-streaming` |
| Any cross-boundary work | `orqa-composability` (always injected) |
| Any test work | `orqa-testing` |

## Acceptance Criteria

- [ ] `code-search` wrapper skill exists and resolves correctly in both CLI and App
- [ ] No agent definition contains `chunkhound`, `orqa-native-search`, or any `orqa-*` skill directly
- [ ] Orchestrator delegation prompts include Tier 2 skill injection based on task scope
- [ ] `skill-enforcement.md` documents the three-tier model
- [ ] A test delegation (dry-run) confirms all three tiers load
