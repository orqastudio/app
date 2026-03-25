---
id: TASK-7093e0cb
type: task
name: "Full environment integration gate"
status: active
description: "Start orqa dev, connect Claude connector, and validate end-to-end: daemon enforcement, MCP search, prompt pipeline, workflow transitions"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 5 — Final Gate"
  - target: TASK-a06e8fc1
    type: depends-on
    rationale: "Compliance audit must pass before integration gate"
acceptance:
  - "orqa dev starts daemon + MCP + LSP without errors"
  - "Claude connector session starts and connects to all three services"
  - "A test artifact transitions through at least 3 workflow states successfully"
  - "Daemon enforcement blocks an invalid transition (evidence captured)"
  - "MCP semantic search returns results for a knowledge query"
  - "Agent prompt generated at delegation time contains three-layer composition"
  - "No business logic executes in the connector — all enforcement via daemon"
  - "Session completes without errors or service disconnections"
---

## What

The final integration gate: start the full development environment and validate that every architectural component works end-to-end in a live session. This is the definitive proof that the architecture alignment is complete.

## Knowledge Needed

- `.orqa/discovery/research/RES-d6e8ab11.md` — the architecture being validated end-to-end
- `libs/daemon/` — daemon service (enforcement, workflow evaluation)
- `connectors/claude-code/` — Claude connector (thin adapter layer)
- `.orqa/workflows/*.resolved.yaml` — resolved workflows for state transitions
- `.claude/agents/` — generated agent prompts (three-layer composition)
- `libs/cli/src/commands/dev.ts` — the `orqa dev` command

## Agent Role

Implementer — this requires running services, creating test artifacts, and executing commands.

## Steps

1. Start the development environment: `orqa dev`
   - Verify daemon starts (check process or port)
   - Verify MCP server starts (check process or port)
   - Verify LSP server starts (check process or port)
   - All three must start without errors

2. Test Claude connector connection:
   - Verify the connector can reach the daemon (daemon-gate passes)
   - Verify MCP tools are available (graph queries, semantic search)
   - Verify LSP provides diagnostics

3. Test workflow state transitions:
   - Create a test artifact (e.g., a task or idea)
   - Transition it through at least 3 valid workflow states
   - Verify each transition succeeds via daemon evaluation
   - Attempt an INVALID transition and capture the daemon's rejection

4. Test MCP semantic search:
   - Query for a knowledge artifact via MCP semantic search
   - Verify results are returned with relevant content

5. Test prompt pipeline:
   - Trigger agent prompt generation (e.g., delegate a task)
   - Verify the generated prompt contains three-layer composition:
     - Layer 1: Universal role
     - Layer 2: Stage context
     - Layer 3: Domain knowledge with titles

6. Verify connector is thin:
   - Monitor connector behavior during the above tests
   - Confirm all enforcement decisions flow through the daemon
   - No local business logic in the connector

7. Document all results with evidence (command output, log excerpts, artifact states)

## Verification

- `orqa dev` process starts 3 services (daemon, MCP, LSP) — all running
- Test artifact transitions: state1 -> state2 -> state3 -> state4 (3+ transitions)
- Invalid transition blocked with daemon error message captured
- MCP search query returns >= 1 result
- Agent prompt contains "Role:", "Stage:", and "Knowledge:" sections (or equivalent three-layer markers)
- Connector logs show only HTTP requests to daemon, no local evaluation
- Zero service disconnections during the test session
