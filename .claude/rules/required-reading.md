---
scope: system
---

# Required Reading (NON-NEGOTIABLE)

Every agent MUST read its Required Reading documentation before any implementation work begins. The Required Reading section in each agent definition lists the specific documentation pages that agent needs loaded into context.

## Protocol

1. The Required Reading section is the FIRST thing the agent executes after skill loading
2. Read each listed document into context before proceeding with the task
3. If a required document does not exist or is unreachable (file missing, path wrong, content empty), the agent MUST stop immediately and prompt the user with a clear message explaining which document is missing and asking how to proceed
4. Never silently skip a required document — every listed document must be loaded or explicitly flagged as unreachable

## Enforcement

- The `agent-maintainer` audits Required Reading lists for completeness during governance reviews
- If a Required Reading path is broken across multiple agents, the `agent-maintainer` performs a bulk path update

## Why This Exists

Required Reading replaces scattered "Before Implementation" checklists that were inconsistent across agents. It ensures every agent loads the same governing documentation before writing code, preventing drift between implementation and specification.

## App Enforcement

In OrqaStudio, Required Reading compliance is enforced via process checks (Phase 7). The app verifies that agents have loaded their required documents before task execution proceeds and can block tasks where required documents are missing. In the CLI, agents self-enforce by following the protocol described above.

## Related Rules

- `documentation-first.md` — documentation is the specification; code is the implementation
- `coding-standards.md` — standards that agents must read
- `architecture-decisions.md` — decisions that agents must comply with
