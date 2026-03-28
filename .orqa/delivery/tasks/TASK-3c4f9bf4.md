---
id: TASK-3c4f9bf4
type: task
title: "Full clean install from scratch"
status: active
description: "Delete .orqa/ and run orqa install from plugins only — verify complete system rebuild with zero errors"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 5 — Final Gate"
  - target: TASK-90eecd23
    type: depends-on
    rationale: "All Phase 4 principle verifications must complete before final gate"
acceptance:
  - "Clean install from deleted .orqa/ produces a working system with 0 errors"
  - "orqa check validate reports 0 errors"
  - "0 unfilled required contribution points in resolved workflows"
  - "All resolved workflows in .orqa/workflows/*.resolved.yaml are valid"
  - "All agent files generated within 1,500-4,000 token budget range"
  - "npx tsc --noEmit passes for all TypeScript packages"
  - "cargo clippy -- -D warnings passes"
---

## What

The definitive clean-room test: delete `.orqa/` entirely and rebuild from plugins only via `orqa install`. This proves the plugin-composed architecture is self-sufficient — everything in `.orqa/` is a derived artifact, and the plugins are the source of truth.

## Knowledge Needed

- `.orqa/discovery/research/RES-d6e8ab11.md` — the architecture being validated
- `libs/cli/src/commands/install.ts` — the install command that rebuilds `.orqa/`
- Plugin manifests in `plugins/*/orqa-plugin.json` — what each plugin provides
- `.orqa/workflows/*.resolved.yaml` — expected resolved workflow output

## Agent Role

Implementer — this requires running commands (delete, install, validate).

## Steps

1. Record the current state: `ls .orqa/` to document what exists before deletion
2. Back up any project-level content that is NOT plugin-derived (delivery artifacts, discovery artifacts, principles). Note: `orqa install` should preserve project-level content in `.orqa/delivery/`, `.orqa/discovery/`, `.orqa/principles/` — verify this.
3. Delete `.orqa/` completely: `rm -rf .orqa/`
4. Run `orqa install` — this must rebuild everything from plugins
5. Verify the install completed without errors (exit code 0, no error output)
6. Run validation checks:
   - `orqa check validate` — must report 0 errors
   - Check `.orqa/workflows/*.resolved.yaml` files exist and are valid YAML
   - Check all required contribution points are filled (planning-methodology, implementation-workflow, review-process)
   - Check `.claude/agents/` files are regenerated
7. Measure token counts on generated agent files — all within 1,500-4,000 range
8. Run build checks:
   - `npx tsc --noEmit` for all TypeScript packages (libs/cli, connectors/claude-code, etc.)
   - `cargo clippy -- -D warnings`
9. Document all results

## Verification

- `orqa install` exit code is 0
- `orqa check validate` exit code is 0 with "0 errors" in output
- `ls .orqa/workflows/*.resolved.yaml | wc -l` >= 1
- `ls .claude/agents/*.md | wc -l` >= 1
- `wc -w .claude/agents/*.md` — all files within expected word count range
- `npx tsc --noEmit` exit code 0 for each TS package
- `cargo clippy -- -D warnings` exit code 0
