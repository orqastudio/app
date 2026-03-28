---
id: TASK-90eecd23
type: task
title: "Verify P7 Resolved Workflow Is a File on Disk"
status: active
description: "Verify that runtime reads .orqa/workflows/*.resolved.yaml — no install-time re-resolution at runtime"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 4 — Principle Verification"
  - target: TASK-c9fa5dd0
    type: depends-on
    rationale: "Phase 3 (knowledge structure verified) must complete before principle verification"
acceptance:
  - "Audit report with PASS/FAIL verdict for P7"
  - "Runtime code paths verified to read .orqa/workflows/ only"
  - "Install-time resolution code confirmed to run only during orqa install"
---

## What

Verify Principle 7 from RES-d6e8ab11 section 2:

> **P7: The Resolved Workflow Is a File on Disk** — After plugin contributions are merged, the resolved workflow must be a YAML file on disk -- deterministic, diffable, and inspectable. The runtime reads this resolved file; it does not re-merge contributions on every evaluation. This makes debugging straightforward: read the file, see the workflow.

Confirm that the runtime (daemon, connector) reads only resolved workflow files, and that workflow resolution happens exclusively at install time.

## Knowledge Needed

- `.orqa/discovery/research/RES-d6e8ab11.md` section 2 (P7 definition)
- `libs/cli/src/lib/workflow-resolver.ts` — the install-time resolver
- `libs/cli/src/commands/install.ts` — where the resolver is called
- `libs/daemon/src/` — daemon runtime code (should read resolved files only)
- `connectors/claude-code/src/` — connector runtime code (should read resolved files only)
- `.orqa/workflows/*.resolved.yaml` — the resolved output files

## Agent Role

Researcher — read-only audit producing a PASS/FAIL verdict with evidence.

## Steps

1. Read RES-d6e8ab11 section 2 to confirm the exact P7 principle text
2. Read `libs/cli/src/lib/workflow-resolver.ts` — confirm it runs at install time and writes `.orqa/workflows/*.resolved.yaml`
3. Read `libs/cli/src/commands/install.ts` — confirm the resolver is called only from the install command
4. Search daemon code for workflow file reads — must read `.orqa/workflows/` not `plugins/`
5. Search connector code for workflow file reads — must read `.orqa/workflows/` not `plugins/`
6. Search for any runtime code path that calls the workflow resolver (would be a violation — resolver should only run at install time)
7. Verify `.orqa/workflows/*.resolved.yaml` files exist and are valid YAML
8. Document evidence for the install-time vs runtime boundary
9. Produce a PASS/FAIL verdict

## Verification

- `ls .orqa/workflows/*.resolved.yaml | wc -l` — resolved files exist
- `grep -rn "workflow-resolver\|resolveWorkflow" libs/daemon/src/` — should return 0 (resolver not called at runtime)
- `grep -rn "workflow-resolver\|resolveWorkflow" connectors/` — should return 0 (resolver not called at runtime)
- `grep -rn "plugins/" libs/daemon/src/ --include="*.rs"` — should return 0 runtime reads of plugin dirs
- `grep -rn "\.orqa/workflows" libs/daemon/src/ --include="*.rs"` — should return matches (daemon reads resolved files)
